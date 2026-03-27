/// Game mode system ported from client/gamemode/
/// GameMode.h, SurvivalMode.h, CreativeMode.h, CreatorMode.h

use crate::level::Level;
use crate::item_instance::ItemInstance;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameModeType {
    Survival,
    Creative,
    Creator,
}

pub struct GameMode {
    pub mode_type: GameModeType,
    pub destroy_progress: f32,
    pub o_destroy_progress: f32,
    pub destroy_ticks: i32,
    pub destroy_delay: i32,
    pub destroy_block_pos: Option<(i32, i32, i32)>,
}

impl GameMode {
    pub fn new(mode_type: GameModeType) -> Self {
        Self {
            mode_type,
            destroy_progress: 0.0,
            o_destroy_progress: 0.0,
            destroy_ticks: 0,
            destroy_delay: 0,
            destroy_block_pos: None,
        }
    }

    pub fn survival() -> Self { Self::new(GameModeType::Survival) }
    pub fn creative() -> Self { Self::new(GameModeType::Creative) }
    pub fn creator() -> Self { Self::new(GameModeType::Creator) }

    pub fn is_creative(&self) -> bool { self.mode_type == GameModeType::Creative }
    pub fn is_survival(&self) -> bool { self.mode_type == GameModeType::Survival }
    pub fn can_hurt_player(&self) -> bool { self.mode_type == GameModeType::Survival }

    pub fn get_pick_range(&self) -> f32 {
        match self.mode_type {
            GameModeType::Creative | GameModeType::Creator => 5.0,
            GameModeType::Survival => 4.0,
        }
    }

    pub fn start_destroy_block(&mut self, x: i32, y: i32, z: i32, _face: i32) {
        if self.mode_type == GameModeType::Creative {
            // Instant break in creative
            self.destroy_block_pos = Some((x, y, z));
            self.destroy_progress = 1.0;
            return;
        }
        self.destroy_block_pos = Some((x, y, z));
        self.destroy_ticks = 0;
        self.destroy_progress = 0.0;
    }

    pub fn continue_destroy_block(&mut self, x: i32, y: i32, z: i32, _face: i32, level: &Level) {
        if self.mode_type == GameModeType::Creative { return; }
        
        if let Some((bx, by, bz)) = self.destroy_block_pos {
            if bx != x || by != y || bz != z {
                self.start_destroy_block(x, y, z, _face);
                return;
            }
        }

        self.destroy_ticks += 1;
        let tile_id = level.get_tile(x, y, z);
        let hardness = crate::tile::get_destroy_time(tile_id);

        if hardness <= 0.0 {
            self.destroy_progress = 1.0;
        } else {
            self.destroy_progress += 1.0 / hardness;
        }
    }

    pub fn stop_destroy_block(&mut self) {
        self.destroy_progress = 0.0;
        self.o_destroy_progress = 0.0;
        self.destroy_ticks = 0;
        self.destroy_block_pos = None;
    }

    pub fn should_destroy(&self) -> bool {
        self.destroy_progress >= 1.0
    }

    pub fn tick(&mut self) {
        self.o_destroy_progress = self.destroy_progress;
        if self.destroy_delay > 0 { self.destroy_delay -= 1; }
    }

    /// Use an item (e.g., eat food, shoot bow)
    pub fn use_item(&self, _item: &ItemInstance) -> bool {
        if _item.is_null() { return false; }
        // Delegate to item behavior (food, bow, etc.)
        true
    }
}

/// Player abilities flags, ported from Abilities.h
#[derive(Clone, Debug)]
pub struct Abilities {
    pub invulnerable: bool,
    pub flying: bool,
    pub may_fly: bool,
    pub instabuild: bool,
}

impl Abilities {
    pub fn new() -> Self {
        Self {
            invulnerable: false,
            flying: false,
            may_fly: false,
            instabuild: false,
        }
    }

    pub fn init_for_mode(&mut self, mode: &GameMode) {
        match mode.mode_type {
            GameModeType::Creative => {
                self.invulnerable = true;
                self.may_fly = true;
                self.instabuild = true;
            }
            GameModeType::Creator => {
                self.invulnerable = true;
                self.may_fly = true;
                self.instabuild = true;
            }
            GameModeType::Survival => {
                self.invulnerable = false;
                self.may_fly = false;
                self.instabuild = false;
            }
        }
    }
}
