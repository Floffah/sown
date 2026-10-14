use bevy_ecs::prelude::{ChildOf, Entity, World};
use project::world::{NodeSource, NodeType, WorldSource};

use super::{
    Ecs,
    scene::{LocalTransform, Material, Mesh, Node, Transform},
};

impl Ecs {
    /// Builds a new ECS from a parsed world, preserving its node hierarchy.
    ///
    /// # Errors
    /// Returns an error if a mesh node has no mesh or transforms cannot be
    /// represented as finite `f32` values. No partial ECS is returned.
    pub fn from_source(source: &WorldSource) -> Result<Self, String> {
        let mut ecs = Self::default();
        spawn_node(&mut ecs.world, &source.root, None, [0.0; 3], [1.0; 3])?;
        Ok(ecs)
    }
}

fn spawn_node(
    world: &mut World,
    source: &NodeSource,
    parent: Option<Entity>,
    parent_translation: [f32; 3],
    parent_scale: [f32; 3],
) -> Result<Entity, String> {
    if source.kind == NodeType::Mesh && source.mesh.is_none() {
        return Err(format!(
            "Mesh node '{}' requires mesh information",
            source.id
        ));
    }
    let local = Transform {
        translation: vector(
            source
                .transform
                .as_ref()
                .and_then(|t| t.position)
                .unwrap_or([0.0; 3]),
            &source.id,
        )?,
        rotation: [0.0, 0.0, 0.0, 1.0],
        scale: vector(
            source
                .transform
                .as_ref()
                .and_then(|t| t.scale)
                .unwrap_or([1.0; 3]),
            &source.id,
        )?,
    };
    // Compose translation and scale with the parent's world transform at load time.
    let translation = std::array::from_fn(|axis| {
        parent_translation[axis] + parent_scale[axis] * local.translation[axis]
    });
    let scale = std::array::from_fn(|axis| parent_scale[axis] * local.scale[axis]);
    if !translation
        .iter()
        .chain(&scale)
        .all(|value| value.is_finite())
    {
        return Err(format!(
            "Node '{}' has an overflowing world transform",
            source.id
        ));
    }
    let entity = world
        .spawn((
            Node {
                id: source.id.clone(),
                name: source.name.clone(),
                kind: source.kind,
            },
            LocalTransform(local),
            Transform {
                translation,
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale,
            },
        ))
        .id();
    if let Some(parent) = parent {
        world.entity_mut(entity).insert(ChildOf(parent));
    }
    if let Some(mesh) = &source.mesh {
        let mesh = Mesh::Primitive(mesh.primitive);
        world.entity_mut(entity).insert((
            mesh,
            Material {
                color: [0.9, 0.3, 0.15],
            },
        ));
    }
    for child in &source.children {
        spawn_node(world, child, Some(entity), translation, scale)?;
    }
    Ok(entity)
}

#[allow(clippy::cast_possible_truncation)]
fn vector(values: [f64; 3], node_id: &str) -> Result<[f32; 3], String> {
    let values = values.map(|value| value as f32);
    if !values.iter().all(|value| value.is_finite()) {
        return Err(format!(
            "Node '{node_id}' has a transform outside the finite f32 range"
        ));
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::{Children, With};
    use project::world::{self, TransformSource};

    const FIXTURE: &str = include_str!("../../../../fixtures/minimal-room/worlds/main.world");

    #[test]
    fn maps_fixture_nodes_components_and_hierarchy() {
        let source = world::parse(FIXTURE).unwrap();
        let mut ecs = Ecs::from_source(&source).unwrap();
        assert_eq!(ecs.world.query::<&Node>().iter(&ecs.world).count(), 6);
        assert_eq!(
            ecs.world
                .query_filtered::<Entity, With<Mesh>>()
                .iter(&ecs.world)
                .count(),
            5
        );
        let root = ecs
            .world
            .query::<(Entity, &Node)>()
            .iter(&ecs.world)
            .find(|(_, node)| node.id == source.root.id)
            .unwrap()
            .0;
        assert_eq!(ecs.world.get::<Children>(root).unwrap().len(), 5);
        assert!(ecs.world.get::<Mesh>(root).is_none());
        assert_eq!(ecs.world.get::<Transform>(root).unwrap().scale, [1.0; 3]);
        let floor = ecs
            .world
            .query::<(Entity, &Node)>()
            .iter(&ecs.world)
            .find(|(_, node)| node.name == "Floor")
            .unwrap()
            .0;
        assert_eq!(ecs.world.get::<ChildOf>(floor).unwrap().parent(), root);
        assert_eq!(
            ecs.world.get::<Mesh>(floor),
            Some(&Mesh::Primitive(
                source.root.children[0].mesh.as_ref().unwrap().primitive
            ))
        );
        assert_eq!(
            ecs.world.get::<Transform>(floor).unwrap().translation,
            [0.0, -0.5, 0.0]
        );
        assert_eq!(
            ecs.world.get::<Transform>(floor).unwrap().scale,
            [20.0, 1.0, 20.0]
        );
        assert_eq!(
            ecs.world.get::<Material>(floor).unwrap().color,
            [0.9, 0.3, 0.15]
        );
    }

    #[test]
    fn composes_parent_transforms_and_preserves_local_values() {
        let mut source = world::parse(FIXTURE).unwrap();
        source.root.transform = Some(TransformSource {
            position: Some([10.0, 2.0, 3.0]),
            scale: Some([2.0, 3.0, 4.0]),
        });
        let mut nested = source.root.children[1].clone();
        nested.id = "nested".into();
        nested.name = "Nested".into();
        source.root.children[0].children.push(nested);
        let mut ecs = Ecs::from_source(&source).unwrap();
        let (local, global) = ecs
            .world
            .query::<(&Node, &LocalTransform, &Transform)>()
            .iter(&ecs.world)
            .find(|(node, _, _)| node.name == "Floor")
            .map(|(_, local, global)| (local, global))
            .unwrap();
        assert_eq!(local.0.translation, [0.0, -0.5, 0.0]);
        assert_eq!(global.translation, [10.0, 0.5, 3.0]);
        assert_eq!(global.scale, [40.0, 3.0, 80.0]);
        let nested = ecs
            .world
            .query::<(&Node, &Transform)>()
            .iter(&ecs.world)
            .find(|(node, _)| node.name == "Nested")
            .unwrap()
            .1;
        assert_eq!(nested.translation, [10.0, 6.5, -837.0]);
        assert_eq!(nested.scale, [800.0, 15.0, 80.0]);
    }

    #[test]
    fn rejects_invalid_or_overflowing_transforms() {
        let mut source = world::parse(FIXTURE).unwrap();
        source.root.children[0].transform.as_mut().unwrap().position = Some([f64::MAX; 3]);
        assert!(Ecs::from_source(&source).is_err());
        source.root.children[0].transform.as_mut().unwrap().position = None;
        source.root.transform = Some(TransformSource {
            position: None,
            scale: Some([f64::from(f32::MAX); 3]),
        });
        assert!(Ecs::from_source(&source).is_err());
        source.root.transform = None;
        source.root.children[0].mesh = None;
        assert!(Ecs::from_source(&source).is_err());
    }
}
