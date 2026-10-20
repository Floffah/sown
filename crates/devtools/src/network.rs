use std::collections::VecDeque;

use egui::{Color32, RichText};
use networking::inspection::{CaptureReceiver, Direction, Packet};

const MAX_PACKETS: usize = 1000;
const MAX_HISTORY_BYTES: usize = 4 * 1024 * 1024;
const CAPTURES_PER_FRAME: usize = 32;

struct Entry {
    id: u64,
    packet: Packet,
    preview: String,
    pretty: String,
}

impl Entry {
    fn new(id: u64, packet: Packet) -> Self {
        let json = packet
            .json()
            .unwrap_or_else(|error| serde_json::json!({"captureError": error.to_string()}));
        let preview = format!("{} {}", packet.message_type, json);
        let pretty = serde_json::to_string_pretty(&json).expect("JSON values serialize to JSON");
        Self {
            id,
            packet,
            preview,
            pretty,
        }
    }

    fn size(&self) -> usize {
        self.packet.body.len() + self.preview.len() + self.pretty.len()
    }
}

#[derive(Default)]
pub(super) struct Network {
    capture: Option<CaptureReceiver>,
    entries: VecDeque<Entry>,
    selected: Option<u64>,
    next_id: u64,
    history_bytes: usize,
}

impl Network {
    pub fn set_capture(&mut self, capture: CaptureReceiver) {
        self.capture = Some(capture);
    }

    pub fn update(&mut self) {
        for _ in 0..CAPTURES_PER_FRAME {
            let Some(packet) = self.capture.as_ref().and_then(CaptureReceiver::try_recv) else {
                break;
            };
            self.push(packet);
        }
    }

    fn push(&mut self, packet: Packet) {
        let entry = Entry::new(self.next_id, packet);
        self.next_id += 1;
        self.history_bytes += entry.size();
        self.entries.push_back(entry);
        while self.entries.len() > MAX_PACKETS || self.history_bytes > MAX_HISTORY_BYTES {
            let removed = self.entries.pop_front().unwrap();
            self.history_bytes -= removed.size();
            if self.selected == Some(removed.id) {
                self.selected = None;
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let dropped = self.capture.as_ref().map_or(0, CaptureReceiver::dropped);
        ui.small(format!(
            "{} packets · {dropped} dropped",
            self.entries.len()
        ));

        egui::Panel::top("packet_list")
            .default_size(ui.available_height() * 0.5)
            .min_size(60.0)
            .max_size((ui.available_height() - 60.0).max(60.0))
            .resizable(true)
            .show(ui, |ui| {
                let row_height = ui.text_style_height(&egui::TextStyle::Body) + 8.0;
                egui::ScrollArea::vertical()
                    .id_salt("packet_rows")
                    .auto_shrink([false, false])
                    .show_rows(ui, row_height, self.entries.len(), |ui, rows| {
                        for index in rows {
                            let entry = &self.entries[index];
                            ui.horizontal(|ui| {
                                let badge = direction_badge(ui, entry.packet.direction);
                                let response = ui.add_sized(
                                    [ui.available_width(), row_height],
                                    egui::Button::selectable(
                                        self.selected == Some(entry.id),
                                        RichText::new(&entry.preview).monospace(),
                                    )
                                    .truncate(),
                                );
                                if response.clicked() || badge.clicked() {
                                    self.selected = Some(entry.id);
                                }
                                response.on_hover_text(format!(
                                    "{:.3}s · connection {} · {:?} · {} bytes",
                                    entry.packet.elapsed.as_secs_f64(),
                                    entry.packet.connection_id,
                                    entry.packet.channel,
                                    entry.packet.wire_size,
                                ));
                            });
                        }
                    });
                ui.take_available_space();
            });

        egui::ScrollArea::both()
            .id_salt("packet_details")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(entry) = self
                    .entries
                    .iter()
                    .find(|entry| Some(entry.id) == self.selected)
                {
                    ui.label(format!(
                        "{} · {:?} · {} bytes",
                        entry.packet.message_type, entry.packet.channel, entry.packet.wire_size
                    ));
                    ui.separator();
                    ui.add(
                        egui::Label::new(RichText::new(&entry.pretty).monospace()).selectable(true),
                    );
                } else {
                    ui.weak("Select a packet to inspect its JSON.");
                }
            });
    }
}

fn direction_badge(ui: &mut egui::Ui, direction: Direction) -> egui::Response {
    let (label, color, sign) = match direction {
        Direction::Received => ("Received", Color32::from_rgb(30, 115, 65), 1.0),
        Direction::Sent => ("Sent", Color32::from_rgb(160, 45, 45), -1.0),
    };
    let (rect, response) = ui.allocate_exact_size(egui::vec2(24.0, 22.0), egui::Sense::click());
    ui.painter().rect_filled(rect, 3.0, color);
    let arrow = egui::vec2(14.0 * sign, 0.0);
    ui.painter().arrow(
        rect.center() - arrow * 0.5,
        arrow,
        egui::Stroke::new(1.5, Color32::WHITE),
    );
    response.on_hover_text(label)
}

#[cfg(test)]
mod tests {
    use super::*;
    use networking::inspection::{Channel, capture_channel};
    use protocol::{Message, v1::ControlClientHello};

    fn packet(username: &str) -> Packet {
        let (capture, receiver) = capture_channel();
        let body = ControlClientHello {
            protocol_version: 1,
            username: Some(username.into()),
        }
        .encode_to_vec();
        capture
            .context(1, Channel::Control)
            .record::<ControlClientHello>(Direction::Sent, &body, body.len() + 1);
        receiver.try_recv().unwrap()
    }

    fn frame(
        network: &mut Network,
        context: &egui::Context,
        events: Vec<egui::Event>,
    ) -> Vec<egui::epaint::ClippedShape> {
        let output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events,
                ..Default::default()
            },
            |ui| network.show(ui),
        );
        let shapes = output.shapes.clone();
        output.drop_without_applying_deltas();
        shapes
    }

    #[test]
    fn clicking_a_packet_displays_its_formatted_json() {
        let mut network = Network::default();
        network.push(packet("Ada"));
        let context = egui::Context::default();
        frame(&mut network, &context, vec![]);
        let shapes = frame(&mut network, &context, vec![]);
        let pos = shapes
            .iter()
            .find_map(|shape| {
                if let egui::epaint::Shape::Text(text) = &shape.shape
                    && text.galley.text().contains("ControlClientHello")
                {
                    Some(text.pos + egui::vec2(5.0, 5.0))
                } else {
                    None
                }
            })
            .expect("Packet preview is drawn");
        frame(&mut network, &context, vec![egui::Event::PointerMoved(pos)]);
        for pressed in [true, false] {
            frame(
                &mut network,
                &context,
                vec![egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                }],
            );
        }
        assert_eq!(network.selected, Some(0));
        let shapes = frame(&mut network, &context, vec![]);
        assert!(shapes.iter().any(|shape| matches!(&shape.shape,
            egui::epaint::Shape::Text(text) if text.galley.text().contains("\"username\": \"Ada\"")
                && text.galley.text().contains('\n'))));
    }

    #[test]
    fn history_limits_records_and_clears_evicted_selection() {
        let mut network = Network::default();
        network.push(packet("Ada"));
        network.selected = Some(0);
        assert!(network.entries[0].preview.contains("Ada"));
        assert!(network.entries[0].pretty.contains('\n'));
        for _ in 0..MAX_PACKETS {
            network.push(packet("Ada"));
        }
        assert_eq!(network.entries.len(), MAX_PACKETS);
        assert!(network.selected.is_none());
        assert_eq!(network.entries.front().unwrap().id, 1);
    }

    #[test]
    fn history_limits_bytes_and_retains_packets_while_hidden() {
        let mut network = Network::default();
        let (capture, receiver) = capture_channel();
        network.set_capture(receiver);
        let message = ControlClientHello {
            protocol_version: 1,
            username: None,
        };
        let body = message.encode_to_vec();
        capture
            .context(1, Channel::Control)
            .record::<ControlClientHello>(Direction::Sent, &body, body.len() + 1);
        network.update();
        assert_eq!(network.entries.len(), 1);
        let username = "x".repeat(8192);
        for _ in 0..200 {
            network.push(packet(&username));
        }
        assert!(network.history_bytes <= MAX_HISTORY_BYTES);
        assert!(network.entries.len() < 200);
    }
}
