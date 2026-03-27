use crate::mob::Mob;
use crate::level::Level;
use crate::inventory::Inventory;

pub struct Player {
    pub mob: Mob,
    pub is_creative: bool,
    pub inventory: Inventory,
}

impl Player {
    pub fn new(is_creative: bool) -> Self {
        Self {
            mob: Mob::new(),
            is_creative,
            inventory: Inventory::new(is_creative),
        }
    }

    pub fn tick(&mut self, level: Option<&crate::level::Level>) {
        self.mob.tick(level);
        // Player specific tick
    }

    pub fn is_player(&self) -> bool {
        true
    }

    pub fn is_creative_mode_allowed(&self) -> bool {
        true
    }

    // Add more methods
}