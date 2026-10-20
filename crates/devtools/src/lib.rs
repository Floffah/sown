mod ecs;
mod network;

#[derive(Default, PartialEq, Eq)]
enum Tab {
    #[default]
    Network,
    Ecs,
}

/// Developer-tools panel state, retained when the overlay is hidden.
#[derive(Default)]
pub struct Devtools {
    selected_tab: Tab,
    network: network::Network,
    ecs: ecs::Inspector,
}

impl Devtools {
    pub fn set_capture(&mut self, capture: networking::inspection::CaptureReceiver) {
        self.network.set_capture(capture);
    }

    /// Drains capture records even when the panel is hidden.
    pub fn update(&mut self) {
        self.network.update();
    }

    /// Draws the tab bar and panel over the right side of the game view.
    pub fn show(&mut self, ui: &mut egui::Ui, world: &engine::ecs::bevy_ecs::world::World) {
        egui::Panel::right("sown_devtools")
            .default_size(320.0)
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.selectable_value(&mut self.selected_tab, Tab::Network, "Network");
                    ui.selectable_value(&mut self.selected_tab, Tab::Ecs, "ECS");
                });
                ui.separator();
                match self.selected_tab {
                    Tab::Network => self.network.show(ui),
                    Tab::Ecs => self.ecs.show(ui, world),
                }
                ui.take_available_space();
            });
    }
}

#[cfg(test)]
mod tests {
    use egui::{Context, Event, Modifiers, PointerButton, Pos2, RawInput, Rect, Vec2};

    fn frame(context: &Context, devtools: &mut super::Devtools, events: Vec<Event>) -> Rect {
        let mut remaining = Rect::NOTHING;
        let output = context.run_ui(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0))),
                events,
                ..Default::default()
            },
            |ui| {
                devtools.show(ui, &engine::ecs::bevy_ecs::world::World::new());
                remaining = ui.available_rect_before_wrap();
            },
        );
        output.drop_without_applying_deltas();
        remaining
    }

    #[test]
    fn resizing_changes_width_and_retains_it_after_release() {
        let context = Context::default();
        let mut devtools = super::Devtools::default();
        frame(&context, &mut devtools, vec![]);
        let before = frame(&context, &mut devtools, vec![]);
        let start = Pos2::new(before.right(), 300.0);
        let end = start - Vec2::new(150.0, 0.0);
        frame(&context, &mut devtools, vec![Event::PointerMoved(start)]);
        frame(
            &context,
            &mut devtools,
            vec![Event::PointerButton {
                pos: start,
                button: PointerButton::Primary,
                pressed: true,
                modifiers: Modifiers::NONE,
            }],
        );
        frame(&context, &mut devtools, vec![Event::PointerMoved(end)]);
        let dragging = frame(&context, &mut devtools, vec![]);
        frame(
            &context,
            &mut devtools,
            vec![Event::PointerButton {
                pos: end,
                button: PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::NONE,
            }],
        );
        let released = frame(&context, &mut devtools, vec![]);
        assert!((before.right() - dragging.right() - 150.0).abs() < 2.0);
        assert!((released.right() - dragging.right()).abs() < 2.0);
    }
}
