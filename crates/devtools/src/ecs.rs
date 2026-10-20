use engine::ecs::{
    bevy_ecs::{
        prelude::{Entity, World},
        resource::IsResource,
        world::EntityRef,
    },
    player::{LocalPlayer, Player},
    replication::{PeerOwner, Replicated},
    scene::{Camera, LocalTransform, Material, Mesh, Node, Transform},
};

#[derive(Default)]
pub(super) struct Inspector {
    search: String,
    selected: Option<Entity>,
}

impl Inspector {
    pub fn show(&mut self, ui: &mut egui::Ui, world: &World) {
        if self
            .selected
            .is_some_and(|id| world.get_entity(id).is_err())
        {
            self.selected = None;
        }
        ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search names or IDs"));
        let search = self.search.trim().to_lowercase();
        let mut entities: Vec<_> = world
            .iter_entities()
            .filter(|entity| !entity.contains::<IsResource>())
            .collect();
        entities.sort_unstable_by_key(EntityRef::id);
        let total = entities.len();
        let rows: Vec<_> = entities
            .iter()
            .filter_map(|entity| {
                let label = entity_label(entity);
                let network_id = entity
                    .get::<Replicated>()
                    .map(|replicated| replicated.entity_id);
                (label.to_lowercase().contains(&search)
                    || network_id.is_some_and(|id| id.to_string().contains(&search))
                    || entity
                        .get::<Node>()
                        .is_some_and(|node| node.id.to_lowercase().contains(&search)))
                .then_some((entity.id(), label))
            })
            .collect();
        ui.small(format!("{} / {total} entities", rows.len()));

        egui::Panel::top("ecs_entities")
            .default_size(ui.available_height() * 0.5)
            .min_size(60.0)
            .max_size((ui.available_height() - 60.0).max(60.0))
            .resizable(true)
            .show(ui, |ui| {
                let row_height = ui.text_style_height(&egui::TextStyle::Body) + 8.0;
                egui::ScrollArea::vertical()
                    .id_salt("entity_rows")
                    .auto_shrink([false, false])
                    .show_rows(ui, row_height, rows.len(), |ui, range| {
                        for index in range {
                            let (id, label) = &rows[index];
                            if ui
                                .add_sized(
                                    [ui.available_width(), row_height],
                                    egui::Button::selectable(self.selected == Some(*id), label)
                                        .truncate(),
                                )
                                .clicked()
                            {
                                self.selected = Some(*id);
                            }
                        }
                    });
                ui.take_available_space();
            });

        egui::ScrollArea::vertical()
            .id_salt("entity_details")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(entity) = self.selected.and_then(|id| world.get_entity(id).ok()) {
                    ui.push_id(entity.id(), |ui| show_entity(ui, &entity));
                } else {
                    ui.weak("Select an entity to inspect its components.");
                }
            });
    }
}

fn entity_label(entity: &EntityRef<'_>) -> String {
    let name = if let Some(node) = entity.get::<Node>().filter(|node| !node.name.is_empty()) {
        &node.name
    } else if entity.contains::<LocalPlayer>() {
        "Local player"
    } else if entity.contains::<Player>() {
        "Player"
    } else {
        "Entity"
    };
    format!("{name} · {}", entity.id())
}

fn show_entity(ui: &mut egui::Ui, entity: &EntityRef<'_>) {
    ui.label(entity_label(entity));
    ui.small(format!("Local ECS ID: {}", entity.id()));
    ui.separator();
    if let Some(node) = entity.get::<Node>() {
        component(
            ui,
            "Node",
            &[
                ("Source ID", node.id.clone()),
                ("Name", node.name.clone()),
                ("Type", format!("{:?}", node.kind)),
            ],
        );
    }
    if let Some(transform) = entity.get::<Transform>() {
        show_transform(ui, "Transform", transform);
    }
    if let Some(transform) = entity.get::<LocalTransform>() {
        show_transform(ui, "LocalTransform", &transform.0);
    }
    if let Some(Mesh::Primitive(primitive)) = entity.get::<Mesh>() {
        component(ui, "Mesh", &[("Primitive", format!("{primitive:?}"))]);
    }
    if let Some(material) = entity.get::<Material>() {
        component(
            ui,
            "Material",
            &[("Colour (RGB)", format!("{:?}", material.color))],
        );
    }
    if let Some(camera) = entity.get::<Camera>() {
        component(
            ui,
            "Camera",
            &[
                (
                    "Vertical FOV (degrees)",
                    camera.fov_y_radians.to_degrees().to_string(),
                ),
                ("Near", camera.near.to_string()),
                ("Far", camera.far.to_string()),
            ],
        );
    }
    if let Some(replicated) = entity.get::<Replicated>() {
        component(
            ui,
            "Replicated",
            &[("Network entity ID", replicated.entity_id.to_string())],
        );
    }
    if let Some(owner) = entity.get::<PeerOwner>() {
        component(ui, "PeerOwner", &[("Peer ID", owner.0.to_string())]);
    }
    if entity.contains::<Player>() {
        component(ui, "Player", &[]);
    }
    if entity.contains::<LocalPlayer>() {
        component(ui, "LocalPlayer", &[]);
    }
}

fn show_transform(ui: &mut egui::Ui, name: &str, transform: &Transform) {
    component(
        ui,
        name,
        &[
            ("Position (XYZ)", format!("{:?}", transform.translation)),
            (
                "Rotation (quaternion XYZW)",
                format!("{:?}", transform.rotation),
            ),
            ("Scale (XYZ)", format!("{:?}", transform.scale)),
        ],
    );
}

fn component(ui: &mut egui::Ui, name: &str, fields: &[(&str, String)]) {
    egui::CollapsingHeader::new(name)
        .default_open(true)
        .show(ui, |ui| {
            if fields.is_empty() {
                ui.weak("Marker present");
            }
            for (label, value) in fields {
                ui.label(*label);
                ui.add(egui::Label::new(egui::RichText::new(value).monospace()).selectable(true));
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::ecs::{Ecs, player::spawn_local_player, scene::spawn_cube};

    fn frame(
        inspector: &mut Inspector,
        world: &World,
        context: &egui::Context,
        events: Vec<egui::Event>,
    ) -> Vec<egui::epaint::TextShape> {
        let output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 1600.0),
                )),
                events,
                ..Default::default()
            },
            |ui| inspector.show(ui, world),
        );
        let text = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::epaint::Shape::Text(text) => Some(text.clone()),
                _ => None,
            })
            .collect();
        output.drop_without_applying_deltas();
        text
    }

    #[test]
    fn selection_shows_live_components_and_clears_after_despawn() {
        let mut world = World::new();
        let player = spawn_local_player(&mut world);
        world
            .entity_mut(player)
            .insert((Replicated { entity_id: 72 }, PeerOwner(3)));
        let mut inspector = Inspector::default();
        let context = egui::Context::default();
        frame(&mut inspector, &world, &context, vec![]);
        let text = frame(&mut inspector, &world, &context, vec![]);
        let pos = text
            .iter()
            .find(|text| text.galley.text().contains("Local player"))
            .expect("Player row is drawn")
            .pos
            + egui::vec2(5.0, 5.0);
        frame(
            &mut inspector,
            &world,
            &context,
            vec![egui::Event::PointerMoved(pos)],
        );
        for pressed in [true, false] {
            frame(
                &mut inspector,
                &world,
                &context,
                vec![egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                }],
            );
        }
        assert_eq!(inspector.selected, Some(player));
        world.get_mut::<Transform>(player).unwrap().translation = [12.0, 2.0, 3.0];
        spawn_cube(&mut world);
        frame(&mut inspector, &world, &context, vec![]);
        let text = frame(&mut inspector, &world, &context, vec![]);
        let contains = |value| text.iter().any(|text| text.galley.text() == value);
        for value in [
            "[12.0, 2.0, 3.0]",
            "Camera",
            "Player",
            "LocalPlayer",
            "Network entity ID",
            "72",
            "Peer ID",
            "3",
        ] {
            assert!(contains(value), "Missing component detail: {value}");
        }
        assert_eq!(inspector.selected, Some(player));
        world.despawn(player);
        let replacement = spawn_local_player(&mut world);
        assert_ne!(replacement, player);
        let text = frame(&mut inspector, &world, &context, vec![]);
        assert_eq!(inspector.selected, None);
        assert!(
            text.iter()
                .any(|text| text.galley.text().contains("Select an entity"))
        );
    }

    #[test]
    fn search_matches_names_and_ids_and_excludes_resources() {
        let mut ecs = Ecs::default();
        let player = spawn_local_player(&mut ecs.world);
        ecs.world
            .entity_mut(player)
            .insert(Replicated { entity_id: 987_654 });
        let cube = spawn_cube(&mut ecs.world);
        let mut inspector = Inspector::default();
        let context = egui::Context::default();
        let text = frame(&mut inspector, &ecs.world, &context, vec![]);
        assert!(
            text.iter()
                .any(|text| text.galley.text() == "2 / 2 entities")
        );
        for search in [
            "LOCAL PLAYER".to_owned(),
            player.to_string(),
            "987654".to_owned(),
        ] {
            inspector.search = search;
            let text = frame(&mut inspector, &ecs.world, &context, vec![]);
            assert!(
                text.iter()
                    .any(|text| text.galley.text().contains("Local player"))
            );
            assert!(
                !text
                    .iter()
                    .any(|text| text.galley.text() == format!("Entity · {cube}"))
            );
        }
        inspector.search = "no matching entity".into();
        let text = frame(&mut inspector, &ecs.world, &context, vec![]);
        assert!(
            text.iter()
                .any(|text| text.galley.text() == "0 / 2 entities")
        );
    }
}
