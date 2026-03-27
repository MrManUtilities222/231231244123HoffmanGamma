use crate::app_platform::AppPlatform;

pub struct AppContext {
    pub platform: Option<Box<dyn AppPlatform>>,
    pub do_render: bool,
    // For EGL, we might need to handle differently in Rust
    // For now, placeholder
}

pub trait App {
    fn init(&mut self);
    fn is_inited(&self) -> bool;
    fn platform(&self) -> Option<&dyn AppPlatform>;
    fn on_graphics_reset(&mut self);
    fn audio_engine_on(&mut self) {}
    fn audio_engine_off(&mut self) {}
    fn destroy(&mut self) {}
    fn load_state(&mut self, _state: &[u8]) {}
    fn save_state(&mut self) -> Option<Vec<u8>> { None }
    fn swap_buffers(&self) {
        // In Rust, handle with graphics library
    }
    fn draw(&mut self) {}
    fn update(&mut self);
    fn set_size(&mut self, _width: i32, _height: i32) {}
    fn quit(&mut self);
    fn want_to_quit(&self) -> bool;
    fn handle_back(&mut self, _is_down: bool) -> bool { false }
}

pub struct BaseApp {
    inited: bool,
    finished: bool,
    context: AppContext,
}

impl BaseApp {
    pub fn new() -> Self {
        Self {
            inited: false,
            finished: false,
            context: AppContext {
                platform: None,
                do_render: true,
            },
        }
    }

    pub fn init_with_context(&mut self, context: AppContext) {
        self.context = context;
        self.init();
        self.inited = true;
    }

    pub fn on_graphics_reset_with_context(&mut self, context: AppContext) {
        self.context = context;
        self.on_graphics_reset();
    }
}

impl App for BaseApp {
    fn init(&mut self) {}
    fn is_inited(&self) -> bool { self.inited }
    fn platform(&self) -> Option<&dyn AppPlatform> {
        self.context.platform.as_ref().map(|p| p.as_ref())
    }
    fn on_graphics_reset(&mut self) {}
    fn update(&mut self) {}
    fn quit(&mut self) { self.finished = true; }
    fn want_to_quit(&self) -> bool { self.finished }
}