use crate::app::{App, AppContext};
use crate::app_platform::AppPlatform;
use crate::options::Options;
use crate::timer::Timer;
use crate::level::Level;
use crate::memory_storage::MemoryLevelStorageSource;

pub struct Minecraft {
    _options: Options,
    timer: Timer,
    level: Option<Level>,
    inited: bool,
    finished: bool,
    context: AppContext,
}

impl Minecraft {
    pub fn new() -> Self {
        Self {
            _options: Options::new(),
            timer: Timer::new(20.0), // 20 ticks per second
            level: None,
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

    // Placeholder methods
    pub fn is_online(&self) -> bool { false }
    pub fn is_online_client(&self) -> bool { false }
    pub fn is_creative_mode(&self) -> bool { false }
    pub fn set_is_creative_mode(&mut self, _creative: bool) {}
    pub fn pause_game(&mut self, _back_paused: bool) {}
    pub fn game_lost_focus(&mut self) {}
    pub fn update(&mut self) {
        self.timer.advance_time();
        if let Some(level) = &mut self.level {
            level.tick_entities();
        }
    }
    pub fn tick(&mut self, _n_tick: i32, _max_tick: i32) {}
    pub fn tick_input(&mut self) {}
    pub fn select_level(&mut self, level_id: &str, level_name: &str, _settings: ()) {
        let source = MemoryLevelStorageSource::new();
        let mut level = Level::new(Box::new(source), level_name, (), 0);
        level.generate_terrain(2);
        self.set_level(level, "Level loaded", None);
        println!("Selected level {} (id={})", level_name, level_id);
    }

    pub fn set_level(&mut self, level: Level, _message: &str, _force_insert_player: Option<()>) {
        self.level = Some(level);
    }

    pub fn level_ref(&self) -> Option<&Level> {
        self.level.as_ref()
    }

    pub fn level_mut(&mut self) -> Option<&mut Level> {
        self.level.as_mut()
    }
    pub fn generate_level(&mut self, _message: &str, _level: ()) {}
    pub fn get_level_source(&self) -> () { () }
    pub fn prepare_level(&mut self, _message: &str) {}
    pub fn handle_build_action(&mut self, _action: ()) {}
    pub fn toggle_dimension(&mut self) {}
    pub fn set_screen(&mut self, _screen: ()) {}
    pub fn grab_mouse(&mut self) {}
    pub fn release_mouse(&mut self) {}
    pub fn support_non_touch_screen(&self) -> bool { true }
    pub fn use_touchscreen(&self) -> bool { false }
    pub fn reload_options(&mut self) {}
    pub fn set_size(&mut self, _width: i32, _height: i32) {}
}

impl App for Minecraft {
    fn init(&mut self) {
        crate::mth::init_mth();
        crate::material::init_materials();
        crate::tile::init_tiles();
        crate::item::init_items();

        // Create one default world at startup.
        self.select_level("world0", "New World", ());
    }

    fn is_inited(&self) -> bool { self.inited }

    fn platform(&self) -> Option<&dyn AppPlatform> {
        self.context.platform.as_ref().map(|p| p.as_ref())
    }

    fn on_graphics_reset(&mut self) {
        // TODO: Reinit GL states
    }

    fn update(&mut self) {
        self.update();
    }

    fn quit(&mut self) { self.finished = true; }

    fn want_to_quit(&self) -> bool { self.finished }

    fn handle_back(&mut self, _is_down: bool) -> bool {
        // TODO: Handle back button
        false
    }
}