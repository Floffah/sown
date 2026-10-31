use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::input::MovementKeys;

// Limit catch-up after a stalled frame so held input cannot cause a large jump.
const MAX_FRAME_TIME: Duration = Duration::from_millis(100);

pub struct State {
    window: Arc<Window>,
    renderer: renderer::Renderer,
    overlay: crate::overlay::Overlay,
    devtools_open: bool,
    ecs: engine::ecs::Ecs,
    movement_keys: MovementKeys,
    last_frame: Instant,
    network_updates: Option<std::sync::mpsc::Receiver<crate::replication::Update>>,
    replication: crate::replication::Replication,
}

impl State {
    /// # Errors
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let source = project::world::parse(include_str!(
            "../../../fixtures/minimal-room/worlds/main.world"
        ))?;
        let mut ecs = engine::ecs::Ecs::from_source(&source).map_err(anyhow::Error::msg)?;
        let local_player = engine::ecs::player::spawn_local_player(&mut ecs.world);
        let size = window.inner_size();
        let renderer = renderer::Renderer::new(window.clone(), size.width, size.height).await?;
        let overlay = crate::overlay::Overlay::new(&window, &renderer);
        Ok(Self {
            window,
            renderer,
            overlay,
            devtools_open: false,
            ecs,
            movement_keys: MovementKeys::default(),
            last_frame: Instant::now(),
            network_updates: None,
            replication: crate::replication::Replication::new(local_player),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
        if width > 0 && height > 0 {
            self.window.request_redraw();
        }
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        self.movement_keys.set(code, is_pressed);
        if let (KeyCode::Escape, true) = (code, is_pressed) {
            event_loop.exit();
        }
    }

    fn reset_input(&mut self) {
        self.movement_keys.clear();
        self.ecs.world.insert_resource(self.movement_keys.intent());
        self.last_frame = Instant::now();
    }

    fn update(&mut self) {
        if self.devtools_open && self.overlay.wants_keyboard_input() {
            self.movement_keys.clear();
        }
        let mut disconnected = false;
        if let Some(updates) = &self.network_updates {
            loop {
                match updates.try_recv() {
                    Ok(update) => {
                        if let Err(error) = self.replication.apply(&mut self.ecs.world, update) {
                            eprintln!("Invalid server state: {error:#}");
                            self.replication.reject(&mut self.ecs.world);
                            disconnected = true;
                            break;
                        }
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => break,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        disconnected = true;
                        break;
                    }
                }
            }
        }
        if disconnected {
            self.network_updates = None;
            self.replication.clear(&mut self.ecs.world);
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame).min(MAX_FRAME_TIME);
        self.last_frame = now;
        self.ecs.world.insert_resource(self.movement_keys.intent());
        self.ecs.tick(elapsed);
    }

    fn render(&mut self) -> anyhow::Result<renderer::RenderOutcome> {
        self.overlay
            .prepare(&self.window, self.devtools_open, &self.ecs.world);
        self.renderer
            .render(&mut self.ecs.world, |frame| self.overlay.paint(frame))
    }
}

#[derive(Default)]
pub struct App {
    state: Option<State>,
    network_updates: Option<std::sync::mpsc::Receiver<crate::replication::Update>>,
    world_ready: Option<tokio::sync::oneshot::Sender<()>>,
    packet_capture: Option<networking::inspection::CaptureReceiver>,
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.reset_input();
            let size = state.window.inner_size();
            state.resize(size.width, size.height);
            return;
        }
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let state = pollster::block_on(State::new(window)).unwrap();
        self.user_event(event_loop, state);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        let size = event.window.inner_size();
        event.resize(size.width, size.height);
        event.network_updates = self.network_updates.take();
        if let Some(capture) = self.packet_capture.take() {
            event.overlay.set_capture(capture);
        }
        if let Some(ready) = self.world_ready.take() {
            let _ = ready.send(());
        }

        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = &mut self.state else { return };
        let ui_consumed = state.overlay.on_window_event(&state.window, &event);

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::Occluded(false) => state.window.request_redraw(),
            WindowEvent::Focused(false) | WindowEvent::Occluded(true) => state.reset_input(),
            WindowEvent::Focused(true) => {
                state.last_frame = Instant::now();
                state.window.request_redraw();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        repeat,
                        ..
                    },
                ..
            } => {
                if code == KeyCode::F12 {
                    if key_state.is_pressed() && !repeat {
                        state.devtools_open = !state.devtools_open;
                        state.reset_input();
                        state.window.request_redraw();
                    }
                } else if !state.devtools_open || !ui_consumed || !key_state.is_pressed() {
                    state.handle_key(event_loop, code, key_state.is_pressed());
                }
            }
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(renderer::RenderOutcome::Presented | renderer::RenderOutcome::Retry) => {
                        state.window.request_redraw();
                    }
                    Ok(renderer::RenderOutcome::Skipped) => {}
                    Err(e) => {
                        // Log the error and exit gracefully
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.reset_input();
        }
    }
}

/// # Errors
pub fn run(server_address: Option<std::net::SocketAddr>) -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let network_runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()?;
    let (local_server, address, certificate) = if let Some(address) = server_address {
        use anyhow::Context;
        use quinn::rustls::pki_types::{CertificateDer, pem::PemObject};
        let certificate = CertificateDer::from_pem_file(".sown/server-tls.pem")
                    .context("Failed to load .sown/server-tls.pem: start the standalone server from the same working directory first")?;
        (None, address, certificate)
    } else {
        let server = server::EmbeddedServer::start(
            include_str!("../../../fixtures/minimal-room/worlds/main.world"),
            std::path::Path::new(".sown"),
        )?;
        let address = server.local_addr()?;
        let certificate = server.certificate();
        (Some(server), address, certificate)
    };
    let (updates, receiver) = std::sync::mpsc::sync_channel(64);
    let (ready, world_ready) = tokio::sync::oneshot::channel();
    let (capture, packets) = networking::inspection::capture_channel();
    let network_task = network_runtime.spawn(async move {
        if let Err(error) =
            crate::connection::run(address, certificate, updates, capture, world_ready).await
        {
            eprintln!("QUIC connection failed: {error:#}");
        }
    });
    let mut app = App::new();
    app.network_updates = Some(receiver);
    app.world_ready = Some(ready);
    app.packet_capture = Some(packets);
    let result = event_loop.run_app(&mut app);
    network_task.abort();
    network_runtime.block_on(async {
        let _ = network_task.await;
    });
    drop(app);
    let shutdown = local_server.map_or(Ok(()), server::EmbeddedServer::shutdown);
    result?;
    shutdown?;

    Ok(())
}
