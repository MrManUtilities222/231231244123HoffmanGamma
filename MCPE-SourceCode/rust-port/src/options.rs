#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Option {
    Music,
    Sound,
    InvertMouse,
    Sensitivity,
    RenderDistance,
    ViewBobbing,
    Anaglyph,
    LimitFramerate,
    Difficulty,
    Graphics,
    AmbientOcclusion,
    GuiScale,
    ThirdPerson,
    HideGui,
    ServerVisible,
    LeftHanded,
    UseTouchscreen,
    UseTouchJoypad,
    DestroyVibration,
    FancyClouds,
    Sprinting,
    PixelsPerMillimeter,
}

impl Option {
    pub fn is_progress(&self) -> bool {
        matches!(self, 
            Option::Music | 
            Option::Sound | 
            Option::Sensitivity | 
            Option::RenderDistance | 
            Option::GuiScale | 
            Option::PixelsPerMillimeter
        )
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, 
            Option::InvertMouse | 
            Option::ViewBobbing | 
            Option::Anaglyph | 
            Option::LimitFramerate | 
            Option::ThirdPerson | 
            Option::HideGui | 
            Option::ServerVisible | 
            Option::LeftHanded | 
            Option::UseTouchscreen | 
            Option::UseTouchJoypad | 
            Option::DestroyVibration | 
            Option::FancyClouds | 
            Option::Sprinting
        )
    }

    pub fn is_int(&self) -> bool {
        !self.is_boolean() && !self.is_progress()
    }

    pub fn get_caption_id(&self) -> &'static str {
        match self {
            Option::Music => "options.music",
            Option::Sound => "options.sound",
            Option::InvertMouse => "options.invertMouse",
            Option::Sensitivity => "options.sensitivity",
            Option::RenderDistance => "options.renderDistance",
            Option::ViewBobbing => "options.viewBobbing",
            Option::Anaglyph => "options.anaglyph",
            Option::LimitFramerate => "options.limitFramerate",
            Option::Difficulty => "options.difficulty",
            Option::Graphics => "options.graphics",
            Option::AmbientOcclusion => "options.ambientOcclusion",
            Option::GuiScale => "options.guiScale",
            Option::ThirdPerson => "options.thirdPerson",
            Option::HideGui => "options.hideGui",
            Option::ServerVisible => "options.serverVisible",
            Option::LeftHanded => "options.leftHanded",
            Option::UseTouchscreen => "options.useTouchscreen",
            Option::UseTouchJoypad => "options.useTouchJoypad",
            Option::DestroyVibration => "options.destroyVibration",
            Option::FancyClouds => "options.fancyClouds",
            Option::Sprinting => "options.sprinting",
            Option::PixelsPerMillimeter => "options.pixelsPerMillimeter",
        }
    }
}

pub struct Options {
    // Placeholder, need to implement the actual options storage
    pub music: f32,
    pub sound: f32,
    pub invert_mouse: bool,
    pub sensitivity: f32,
    pub render_distance: i32,
    pub view_bobbing: bool,
    pub anaglyph: bool,
    pub limit_framerate: bool,
    pub difficulty: i32,
    pub graphics: i32,
    pub ambient_occlusion: i32,
    pub gui_scale: i32,
    pub third_person: bool,
    pub hide_gui: bool,
    pub server_visible: bool,
    pub left_handed: bool,
    pub use_touchscreen: bool,
    pub use_touch_joypad: bool,
    pub destroy_vibration: bool,
    pub fancy_clouds: bool,
    pub sprinting: bool,
    pub pixels_per_millimeter: f32,
}

impl Options {
    pub fn new() -> Self {
        Self {
            music: 1.0,
            sound: 1.0,
            invert_mouse: false,
            sensitivity: 0.5,
            render_distance: 10,
            view_bobbing: true,
            anaglyph: false,
            limit_framerate: false,
            difficulty: 1,
            graphics: 0,
            ambient_occlusion: 0,
            gui_scale: 0,
            third_person: false,
            hide_gui: false,
            server_visible: true,
            left_handed: false,
            use_touchscreen: false,
            use_touch_joypad: false,
            destroy_vibration: false,
            fancy_clouds: false,
            sprinting: true,
            pixels_per_millimeter: 1.0,
        }
    }

    // Add methods as needed
}