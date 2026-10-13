use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorldSource {
    pub format_version: u32,
    pub name: String,
    pub root: NodeSource,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NodeSource {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: NodeType,
    pub transform: Option<TransformSource>,
    pub mesh: Option<MeshSource>,
    pub collider: Option<ColliderSource>,
    #[serde(default)]
    pub behaviours: Vec<String>,
    pub scope: Option<Scope>,
    #[serde(default)]
    pub children: Vec<NodeSource>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum NodeType {
    Node3D,
    Mesh,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MeshPrimitive {
    /// A unit cube centred on the origin.
    Cube,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Replicated,
    Server,
    Client,
}

// shortcut: behaviour attachment resolution is not wired up yet; remove when it is used.
#[allow(dead_code)]
pub(crate) struct BehaviourAttachment<'source> {
    pub(crate) uri: &'source str,
    pub(crate) pointer: String,
    pub(crate) node_id: &'source str,
    pub(crate) scope: Scope,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransformSource {
    pub position: Option<[f64; 3]>,
    pub scale: Option<[f64; 3]>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MeshSource {
    pub primitive: MeshPrimitive,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ColliderSource {
    pub shape: String,
}

/// Parses the authored world tree and validates the currently supported node data.
///
/// # Errors
/// Returns an error for invalid JSON, unsupported node types or primitives,
/// malformed transforms, or a mesh node without mesh information.
pub fn parse(source: &str) -> Result<WorldSource, serde_json::Error> {
    let world: WorldSource = serde_json::from_str(source)?;
    validate_node(&world.root)?;
    Ok(world)
}

fn validate_node(node: &NodeSource) -> Result<(), serde_json::Error> {
    if node.kind == NodeType::Mesh && node.mesh.is_none() {
        return Err(serde::de::Error::custom(format!(
            "Mesh node '{}' requires mesh information",
            node.id
        )));
    }
    for child in &node.children {
        validate_node(child)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../../fixtures/minimal-room/worlds/main.world");

    #[test]
    fn parses_fixture_and_preserves_node_data() {
        let world = parse(FIXTURE).unwrap();
        assert_eq!(world.root.kind, NodeType::Node3D);
        assert_eq!(world.root.children.len(), 5);
        let floor = &world.root.children[0];
        assert_eq!(floor.kind, NodeType::Mesh);
        assert_eq!(floor.mesh.as_ref().unwrap().primitive, MeshPrimitive::Cube);
        assert_eq!(
            floor.transform.as_ref().unwrap().position,
            Some([0.0, -0.5, 0.0])
        );
        assert_eq!(
            floor.transform.as_ref().unwrap().scale,
            Some([20.0, 1.0, 20.0])
        );
        assert_eq!(world.root.behaviours.len(), 2);
    }

    #[test]
    fn rejects_invalid_node_data_in_children() {
        let world: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
        for (field, value) in [
            ("type", serde_json::json!("Unknown")),
            ("mesh", serde_json::json!({"primitive": "sphere"})),
            ("mesh", serde_json::json!({"primitive": "box"})),
            ("mesh", serde_json::Value::Null),
            ("transform", serde_json::json!({"position": [0, 1]})),
            ("transform", serde_json::json!({"scale": [1, "bad", 1]})),
        ] {
            let mut invalid = world.clone();
            invalid["root"]["children"][0][field] = value;
            assert!(
                parse(&invalid.to_string()).is_err(),
                "invalid {field} accepted"
            );
        }
        assert!(parse("{").is_err());
    }

    #[test]
    fn accepts_optional_transform_fields() {
        let mut world: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
        world["root"]["children"][0]["transform"] = serde_json::json!({});
        let parsed = parse(&world.to_string()).unwrap();
        let transform = parsed.root.children[0].transform.as_ref().unwrap();
        assert_eq!(transform.position, None);
        assert_eq!(transform.scale, None);
    }
}
