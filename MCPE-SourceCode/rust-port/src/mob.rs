use crate::entity::Entity;
use crate::level::Level;

pub struct Mob {
    pub entity: Entity,
    pub health: i32,
    pub max_health: i32,
    // Placeholder for AI, etc.
}

impl Mob {
    pub fn new() -> Self {
        Self {
            entity: Entity::new(),
            health: 10,
            max_health: 10,
        }
    }

    pub fn tick(&mut self, level: Option<&crate::level::Level>) {
        self.entity.tick(level);
        // AI step
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn hurt(&mut self, damage: i32) {
        self.health -= damage;
        if self.health < 0 {
            self.health = 0;
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    // Add more methods
}