use renderer::{RenderFrame, wgpu};
use winit::{event::WindowEvent, window::Window};

pub(super) struct Overlay {
    context: egui::Context,
    devtools: devtools::Devtools,
    input: egui_winit::State,
    painter: egui_wgpu::Renderer,
    paint_jobs: Vec<egui::ClippedPrimitive>,
    textures: egui::TexturesDelta,
    pixels_per_point: f32,
}

impl Overlay {
    #[allow(clippy::cast_possible_truncation)] // Window scale factors fit in f32.
    pub fn new(window: &Window, renderer: &renderer::Renderer) -> Self {
        let context = egui::Context::default();
        context.set_visuals(egui::Visuals::dark());
        let input = egui_winit::State::new(
            context.clone(),
            egui::ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            Some(winit::window::Theme::Dark),
            Some(renderer.device().limits().max_texture_dimension_2d as usize),
        );
        let painter = egui_wgpu::Renderer::new(
            renderer.device(),
            renderer.surface_format(),
            egui_wgpu::RendererOptions::default(),
        );
        Self {
            context,
            devtools: devtools::Devtools::default(),
            input,
            painter,
            paint_jobs: Vec::new(),
            textures: egui::TexturesDelta::default(),
            pixels_per_point: 1.0,
        }
    }

    pub fn on_window_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        let response = self.input.on_window_event(window, event);
        if response.repaint {
            window.request_redraw();
        }
        response.consumed
    }

    pub fn wants_keyboard_input(&self) -> bool {
        self.context.egui_wants_keyboard_input()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_capture(&mut self, capture: networking::inspection::CaptureReceiver) {
        self.devtools.set_capture(capture);
    }

    pub fn prepare(
        &mut self,
        window: &Window,
        devtools_open: bool,
        world: &engine::ecs::bevy_ecs::world::World,
    ) {
        self.devtools.update();
        let input = self.input.take_egui_input(window);
        let output = self.context.run_ui(input, |ui| {
            if devtools_open {
                self.devtools.show(ui, world);
            }
        });
        self.input
            .handle_platform_output(window, output.platform_output);
        self.pixels_per_point = output.pixels_per_point;
        self.paint_jobs = self
            .context
            .tessellate(output.shapes, output.pixels_per_point);
        // Keep texture changes until a frame is painted, including after a surface retry.
        self.textures.append(output.textures_delta);
    }

    pub fn paint(&mut self, frame: RenderFrame<'_>) -> Vec<wgpu::CommandBuffer> {
        let RenderFrame {
            device,
            queue,
            encoder,
            color,
            size_in_pixels,
        } = frame;
        let mut textures = std::mem::take(&mut self.textures);
        for (id, deltas) in &textures.set {
            for delta in deltas {
                self.painter.update_texture(device, queue, *id, delta);
            }
        }
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels,
            pixels_per_point: self.pixels_per_point,
        };
        let commands =
            self.painter
                .update_buffers(device, queue, encoder, &self.paint_jobs, &screen);
        {
            let mut pass = encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("UI overlay"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: color,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                })
                .forget_lifetime();
            self.painter.render(&mut pass, &self.paint_jobs, &screen);
        }
        for id in &textures.free {
            self.painter.free_texture(id);
        }
        textures.clear();
        commands
    }
}

impl Drop for Overlay {
    fn drop(&mut self) {
        self.textures.clear();
    }
}
