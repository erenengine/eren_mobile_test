use std::sync::Arc;

use eren_render_shared::{adapter::Adapter, device::Device, instance::Instance, surface::Surface};
use eren_window::window::{WindowConfig, WindowEventHandler, WindowLifecycle};
use winit::window::Window;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

mod test_vertex_input {
    pub mod render_pass;
    pub mod renderer;
    pub mod ubo;
    pub mod vertex;
}

use crate::test_vertex_input::renderer::TestRenderer;

pub fn init_logger() {
    #[cfg(target_arch = "wasm32")]
    {
        use log::Level;

        console_error_panic_hook::set_once();
        console_log::init_with_level(Level::Debug).expect("Failed to init console_log");
    }

    #[cfg(target_os = "android")]
    {
        use log::LevelFilter;
        android_logger::init_once(
            android_logger::Config::default().with_max_level(LevelFilter::Trace),
        );
    }

    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    {
        env_logger::init();
    }
}

struct TestWindowEventHandler<'a> {
    window: Arc<Window>,
    _instance: Instance,
    surface: Surface<'a>,
    _adapter: Adapter,
    device: Device,
    renderer: TestRenderer,
}

impl<'a> WindowEventHandler for TestWindowEventHandler<'a> {
    async fn new(window: Arc<Window>) -> Self {
        log::debug!("Window created");

        let instance = Instance::new(window.clone()).await;
        let surface = Surface::new(&instance).unwrap();
        let adapter = Adapter::new(&instance, &surface).await.unwrap();

        let window_size = window.inner_size();
        let scale_factor = window.scale_factor();
        let device = Device::new(
            &adapter,
            &surface,
            window_size.width / scale_factor as u32,
            window_size.height / scale_factor as u32,
        )
        .await
        .unwrap();

        let renderer = TestRenderer::new(&device);

        log::debug!("Renderer created");

        Self {
            window,
            _instance: instance,
            surface,
            _adapter: adapter,
            device,
            renderer,
        }
    }

    fn on_resized(&mut self, width: u32, height: u32) {
        log::debug!("Window resized: {}x{}", width, height);

        let scale_factor = self.window.scale_factor();

        self.device.resize_surface(
            &self.surface,
            width / scale_factor as u32,
            height / scale_factor as u32,
        );
    }

    fn on_scale_factor_changed(&mut self, scale_factor: f64) {
        log::debug!("Scale factor changed: {}", scale_factor);

        let window_size = self.window.inner_size();

        self.device.resize_surface(
            &self.surface,
            window_size.width / scale_factor as u32,
            window_size.height / scale_factor as u32,
        );
    }

    fn on_redraw_requested(&mut self) {
        //log::debug!("Redraw requested");

        let window_size = self.window.inner_size();
        self.renderer
            .render(
                &self.surface,
                &self.device,
                window_size.width,
                window_size.height,
            )
            .unwrap();
    }
}

impl<'a> Drop for TestWindowEventHandler<'a> {
    fn drop(&mut self) {
        log::debug!("Window lost");
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    init_logger();

    match WindowLifecycle::<TestWindowEventHandler>::new(WindowConfig {
        width: 800,
        height: 600,
        title: "Test Window",
        canvas_id: None,
    })
    .start_event_loop(app)
    {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed to start event loop: {}", e);
        }
    }
}

#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn start_rust_app() {
    use std::env;
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }

    init_logger();

    match WindowLifecycle::<TestWindowEventHandler>::new(WindowConfig {
        width: 800,
        height: 600,
        title: "Test Window",
        canvas_id: None,
    })
    .start_event_loop()
    {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed to start event loop: {}", e);
        }
    }
}
