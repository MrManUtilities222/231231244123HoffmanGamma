use crate::font::Font;
use crate::tesselator::Tesselator;
use crate::item_instance::ItemInstance;

/// Game state controlling whether the player is actively playing or in a menu.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
    Playing,
    Paused,
    StartMenu,
    Inventory,
    Options,
}

/// Input event type
pub enum InputEvent {
    MouseClick { x: f32, y: f32, button: i32 },
    MouseRelease { x: f32, y: f32, button: i32 },
    MouseMove { x: f32, y: f32 },
    KeyPress { key: i32 },
}

/// A trait representing a UI screen overlay (pause menu, inventory, etc.)
pub trait Screen {
    fn init(&mut self, width: f32, height: f32);
    fn render(&mut self, font: &Font, width: f32, height: f32, mx: f32, my: f32) -> (Vec<f32>, Vec<f32>);
    fn handle_input(&mut self, event: InputEvent) -> Option<ScreenAction>;
}

/// Actions a screen can request the main loop to perform
#[derive(Clone, Debug)]
pub enum ScreenAction {
    CloseScreen,
    ChangeState(GameState),
    LoadWorld(String),
    CreateWorld(String),
}

// ==== UI Core Elements ====

#[derive(Clone, Debug)]
pub struct Button {
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub text: String,
    pub is_hovered: bool,
    pub is_down: bool,
    pub disabled: bool,
}

impl Button {
    pub fn new(id: i32, x: f32, y: f32, w: f32, h: f32, text: &str) -> Self {
        Self { id, x, y, w, h, text: text.to_string(), is_hovered: false, is_down: false, disabled: false }
    }

    pub fn update(&mut self, mx: f32, my: f32) {
        self.is_hovered = mx >= self.x && mx <= self.x + self.w && my >= self.y && my <= self.y + self.h;
        if !self.is_hovered {
            self.is_down = false;
        }
    }

    pub fn handle_input(&mut self, event: &InputEvent) -> bool {
        if self.disabled { return false; }
        match event {
            InputEvent::MouseClick { .. } => {
                if self.is_hovered {
                    self.is_down = true;
                    return true; // Click intercepted
                }
            },
            InputEvent::MouseRelease { .. } => {
                if self.is_down && self.is_hovered {
                    self.is_down = false;
                    return true; // Click completed Action
                }
                self.is_down = false;
            },
            _ => {}
        }
        false
    }
}

// ==== Standard Button Rendering Helper ====
pub fn draw_button(t: &mut Tesselator, btn: &Button) {
    let (u0, v0, u1, v1) = if btn.disabled {
        (0.0, 46.0/256.0, 200.0/256.0, 66.0/256.0)
    } else if btn.is_hovered {
        (0.0, 86.0/256.0, 200.0/256.0, 106.0/256.0)
    } else {
        (0.0, 66.0/256.0, 200.0/256.0, 86.0/256.0)
    };
    
    // Stretch the texture across the button rect
    t.color(1.0, 1.0, 1.0);
    t.vertex_uv(btn.x, btn.y + btn.h, 0.0, u0, v1);
    t.vertex_uv(btn.x + btn.w, btn.y + btn.h, 0.0, u1, v1);
    t.vertex_uv(btn.x + btn.w, btn.y, 0.0, u1, v0);

    t.vertex_uv(btn.x + btn.w, btn.y, 0.0, u1, v0);
    t.vertex_uv(btn.x, btn.y, 0.0, u0, v0);
    t.vertex_uv(btn.x, btn.y + btn.h, 0.0, u0, v1);
}

// ==== Screens ====

pub struct PauseScreen {
    pub width: f32,
    pub height: f32,
    pub buttons: Vec<Button>,
}

impl PauseScreen {
    pub fn new() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            buttons: Vec::new(),
        }
    }
}

impl Default for PauseScreen {
    fn default() -> Self { Self::new() }
}

impl Screen for PauseScreen {
    fn init(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.buttons.clear();
        self.buttons.push(Button::new(0, width / 2.0 - 100.0, height / 2.0 + 10.0, 200.0, 40.0, "Back to Game"));
        self.buttons.push(Button::new(1, width / 2.0 - 100.0, height / 2.0 + 60.0, 200.0, 40.0, "Options"));
        self.buttons.push(Button::new(2, width / 2.0 - 100.0, height / 2.0 + 110.0, 200.0, 40.0, "Quit to Title"));
    }

    fn render(&mut self, font: &Font, width: f32, height: f32, mx: f32, my: f32) -> (Vec<f32>, Vec<f32>) {
        let mut t = Tesselator::new();
        t.begin();

        // Dark overlay
        t.color(0.0, 0.0, 0.0);
        let u0 = 0.0/256.0; let v0 = 0.0/256.0; let u1 = 1.0/256.0; let v1 = 1.0/256.0;
        t.vertex_uv(0.0, height, 0.0, u0, v1);
        t.vertex_uv(width, height, 0.0, u1, v1);
        t.vertex_uv(width, 0.0, 0.0, u1, v0);
        t.vertex_uv(width, 0.0, 0.0, u1, v0);
        t.vertex_uv(0.0, 0.0, 0.0, u0, v0);
        t.vertex_uv(0.0, height, 0.0, u0, v1);

        for btn in &mut self.buttons {
            btn.update(mx, my);
            draw_button(&mut t, btn);
        }

        let gui_verts = t.end();
        let mut text_verts = Vec::new();

        // Title
        let title = "Game Paused";
        let text_w = font.width(title) as f32 * 3.0;
        let p = font.draw_shadow(title, 0.0, 0.0, 0xFFFFFF);
        for chunk in p.chunks(8) {
            text_verts.extend_from_slice(&[chunk[0]*3.0 + (width-text_w)/2.0, chunk[1]*3.0 + height/4.0, chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]);
        }

        // Button text
        for btn in &self.buttons {
            let tw = font.width(&btn.text) as f32 * 2.0;
            let p = font.draw_shadow(&btn.text, 0.0, 0.0, 0xFFFFFF);
            for chunk in p.chunks(8) {
                text_verts.extend_from_slice(&[chunk[0]*2.0 + btn.x + (btn.w-tw)/2.0, chunk[1]*2.0 + btn.y + (btn.h-16.0)/2.0, chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]);
            }
        }

        (gui_verts, text_verts)
    }

    fn handle_input(&mut self, event: InputEvent) -> Option<ScreenAction> {
        for btn in &mut self.buttons {
            if btn.handle_input(&event) {
                if let InputEvent::MouseRelease { .. } = event {
                    match btn.id {
                        0 => return Some(ScreenAction::CloseScreen),
                        1 => return Some(ScreenAction::ChangeState(GameState::Options)),
                        2 => return Some(ScreenAction::ChangeState(GameState::StartMenu)),
                        _ => {}
                    }
                }
            }
        }
        None
    }
}

pub struct StartMenuScreen {
    pub width: f32,
    pub height: f32,
    pub buttons: Vec<Button>,
}

impl StartMenuScreen {
    pub fn new() -> Self { Self { width: 800.0, height: 600.0, buttons: Vec::new() } }
}

impl Default for StartMenuScreen { fn default() -> Self { Self::new() } }

impl Screen for StartMenuScreen {
    fn init(&mut self, width: f32, height: f32) {
        self.width = width; self.height = height;
        self.buttons.clear();
        self.buttons.push(Button::new(0, width / 2.0 - 100.0, height / 2.0 - 20.0, 200.0, 40.0, "Play"));
        self.buttons.push(Button::new(1, width / 2.0 - 100.0, height / 2.0 + 30.0, 200.0, 40.0, "Options"));
    }

    fn render(&mut self, font: &Font, width: f32, height: f32, mx: f32, my: f32) -> (Vec<f32>, Vec<f32>) {
        let mut t = Tesselator::new();
        t.begin();
        for btn in &mut self.buttons {
            btn.update(mx, my);
            draw_button(&mut t, btn);
        }
        let gui_verts = t.end();

        let mut text_verts = Vec::new();
        // Title Dirt Background is usually rendered by the main rendering loop before the UI
        let title = "Minecraft: Rust Edition";
        let text_w = font.width(title) as f32 * 4.0;
        let p = font.draw_shadow(title, 0.0, 0.0, 0xFFFFFF);
        for chunk in p.chunks(8) {
            text_verts.extend_from_slice(&[chunk[0]*4.0 + (width-text_w)/2.0, chunk[1]*4.0 + height/4.0, chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]);
        }
        for btn in &self.buttons {
            let tw = font.width(&btn.text) as f32 * 2.0;
            let p = font.draw_shadow(&btn.text, 0.0, 0.0, 0xFFFFFF);
            for chunk in p.chunks(8) {
                text_verts.extend_from_slice(&[chunk[0]*2.0 + btn.x + (btn.w-tw)/2.0, chunk[1]*2.0 + btn.y + (btn.h-16.0)/2.0, chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]);
            }
        }
        (gui_verts, text_verts)
    }

    fn handle_input(&mut self, event: InputEvent) -> Option<ScreenAction> {
        for btn in &mut self.buttons {
            if btn.handle_input(&event) {
                if let InputEvent::MouseRelease { .. } = event {
                    match btn.id {
                        0 => return Some(ScreenAction::ChangeState(GameState::Playing)),
                        1 => return Some(ScreenAction::ChangeState(GameState::Options)),
                        _ => {}
                    }
                }
            }
        }
        None
    }
}

pub struct OptionsScreen {
    pub width: f32,
    pub height: f32,
    pub buttons: Vec<Button>,
}

impl OptionsScreen {
    pub fn new() -> Self { Self { width: 800.0, height: 600.0, buttons: Vec::new() } }
}

impl Default for OptionsScreen { fn default() -> Self { Self::new() } }

impl Screen for OptionsScreen {
    fn init(&mut self, width: f32, height: f32) {
        self.width = width; self.height = height;
        self.buttons.clear();
        self.buttons.push(Button::new(0, width / 2.0 - 100.0, height / 2.0 - 50.0, 200.0, 40.0, "Graphics: Fast"));
        self.buttons.push(Button::new(1, width / 2.0 - 100.0, height / 2.0, 200.0, 40.0, "Sound: On"));
        self.buttons.push(Button::new(2, width / 2.0 - 100.0, height / 2.0 + 50.0, 200.0, 40.0, "View Distance: Normal"));
        self.buttons.push(Button::new(3, width / 2.0 - 100.0, height / 2.0 + 120.0, 200.0, 40.0, "Done"));
    }

    fn render(&mut self, font: &Font, width: f32, height: f32, mx: f32, my: f32) -> (Vec<f32>, Vec<f32>) {
        let mut t = Tesselator::new();
        t.begin();

        t.color(0.0, 0.0, 0.0);
        let u0 = 0.0/256.0; let v0 = 0.0/256.0; let u1 = 1.0/256.0; let v1 = 1.0/256.0;
        t.vertex_uv(0.0, height, 0.0, u0, v1); t.vertex_uv(width, height, 0.0, u1, v1); t.vertex_uv(width, 0.0, 0.0, u1, v0);
        t.vertex_uv(width, 0.0, 0.0, u1, v0); t.vertex_uv(0.0, 0.0, 0.0, u0, v0); t.vertex_uv(0.0, height, 0.0, u0, v1);

        for btn in &mut self.buttons {
            btn.update(mx, my);
            draw_button(&mut t, btn);
        }
        let gui_verts = t.end();

        let mut text_verts = Vec::new();
        let title = "Options";
        let text_w = font.width(title) as f32 * 3.0;
        let p = font.draw_shadow(title, 0.0, 0.0, 0xFFFFFF);
        for chunk in p.chunks(8) {
            text_verts.extend_from_slice(&[chunk[0]*3.0 + (width-text_w)/2.0, chunk[1]*3.0 + height/4.0 - 20.0, chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]);
        }
        for btn in &self.buttons {
            let tw = font.width(&btn.text) as f32 * 2.0;
            let p = font.draw_shadow(&btn.text, 0.0, 0.0, 0xFFFFFF);
            for chunk in p.chunks(8) {
                text_verts.extend_from_slice(&[chunk[0]*2.0 + btn.x + (btn.w-tw)/2.0, chunk[1]*2.0 + btn.y + (btn.h-16.0)/2.0, chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]);
            }
        }
        (gui_verts, text_verts)
    }

    fn handle_input(&mut self, event: InputEvent) -> Option<ScreenAction> {
        for btn in &mut self.buttons {
            if btn.handle_input(&event) {
                if let InputEvent::MouseRelease { .. } = event {
                    match btn.id {
                        0 => { btn.text = if btn.text == "Graphics: Fast" { "Graphics: Fancy".to_string() } else { "Graphics: Fast".to_string() }; },
                        1 => { btn.text = if btn.text == "Sound: On" { "Sound: Off".to_string() } else { "Sound: On".to_string() }; },
                        2 => { btn.text = if btn.text == "View Distance: Normal" { "View Distance: Short".to_string() } else { "View Distance: Normal".to_string() }; },
                        3 => return Some(ScreenAction::ChangeState(GameState::Paused)), // Or back to StartMenu
                        _ => {}
                    }
                }
            }
        }
        None
    }
}
