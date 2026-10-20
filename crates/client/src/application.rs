use std::{sync::Arc, time::Duration};

use web_time::Instant;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

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
    #[cfg(not(target_arch = "wasm32"))]
    network_updates: Option<std::sync::mpsc::Receiver<crate::replication::Update>>,
    #[cfg(not(target_arch = "wasm32"))]
    replication: crate::replication::Replication,
}

impl State {
    /// # Errors
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let source = project::world::parse(include_str!(
            "../../../fixtures/minimal-room/worlds/main.world"
        ))?;
        let mut ecs = engine::ecs::Ecs::from_source(&source).map_err(anyhow::Error::msg)?;
        #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
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
            #[cfg(not(target_arch = "wasm32"))]
            network_updates: None,
            #[cfg(not(target_arch = "wasm32"))]
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
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut disconnected = false;
            if let Some(updates) = &self.network_updates {
                loop {
                    match updates.try_recv() {
                        Ok(update) => {
                            if let Err(error) = self.replication.apply(&mut self.ecs.world, update)
                            {
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
        }
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame).min(MAX_FRAME_TIME);
        self.last_frame = now;
        self.ecs.world.insert_resource(self.movement_keys.intent());
        self.ecs.tick(elapsed);
    }

    fn render(&mut self) -> anyhow::Result<renderer::RenderOutcome> {
        self.overlay.prepare(&self.window, self.devtools_open);
        self.renderer
            .render(&mut self.ecs.world, |frame| self.overlay.paint(frame))
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(Default))]
pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
    #[cfg(not(target_arch = "wasm32"))]
    network_updates: Option<std::sync::mpsc::Receiver<crate::replication::Update>>,
    #[cfg(not(target_arch = "wasm32"))]
    world_ready: Option<tokio::sync::oneshot::Sender<()>>,
}

impl App {
    #[must_use]
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            state: None,
            #[cfg(not(target_arch = "wasm32"))]
            network_updates: None,
            #[cfg(not(target_arch = "wasm32"))]
            world_ready: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
        }
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
        // The web state arrives asynchronously; don't create another window while waiting.
        #[cfg(target_arch = "wasm32")]
        if self.proxy.is_none() {
            return;
        }

        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to
            // await the window creation
            let state = pollster::block_on(State::new(window)).unwrap();
            self.user_event(event_loop, state);
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Run the future asynchronously and use the
            // proxy to send the results to the event loop
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(
                        proxy
                            .send_event(
                                State::new(window)
                                    .await
                                    .expect("Unable to create canvas!!!")
                            )
                            .is_ok()
                    )
                });
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        // This is where proxy.send_event() ends up
        let size = event.window.inner_size();
        event.resize(size.width, size.height);
        #[cfg(not(target_arch = "wasm32"))]
        {
            event.network_updates = self.network_updates.take();
            if let Some(ready) = self.world_ready.take() {
                let _ = ready.send(());
            }
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
pub fn run() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
    }

    let event_loop = EventLoop::with_user_event().build()?;
    #[cfg(not(target_arch = "wasm32"))]
    {
        let network_runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?;
        let (updates, receiver) = std::sync::mpsc::sync_channel(64);
        let (ready, world_ready) = tokio::sync::oneshot::channel();
        let network_task = network_runtime.spawn(async move {
            if let Err(error) = crate::connection::run(updates, world_ready).await {
                eprintln!("QUIC connection failed: {error:#}");
            }
        });
        let mut app = App::new();
        app.network_updates = Some(receiver);
        app.world_ready = Some(ready);
        let result = event_loop.run_app(&mut app);
        network_task.abort();
        result?;
    }
    #[cfg(target_arch = "wasm32")]
    {
        let app = App::new(&event_loop);
        event_loop.spawn_app(app);
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    run().unwrap_throw();

    Ok(())
}
