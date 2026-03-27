/// Food/Hunger system ported from world/food/FoodData.h
/// Controls player hunger, saturation, and food-related effects

#[derive(Clone, Debug)]
pub struct FoodData {
    pub food_level: i32,
    pub saturation_level: f32,
    pub exhaustion: f32,
    pub food_tick_timer: i32,
}

impl Default for FoodData {
    fn default() -> Self {
        Self {
            food_level: 20,
            saturation_level: 5.0,
            exhaustion: 0.0,
            food_tick_timer: 0,
        }
    }
}

impl FoodData {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add food from eating an item
    pub fn eat(&mut self, nutrition: i32, saturation: f32) {
        self.food_level = (self.food_level + nutrition).min(20);
        self.saturation_level = (self.saturation_level + saturation).min(self.food_level as f32);
    }

    /// Tick the hunger system (called every game tick)
    pub fn tick(&mut self) -> FoodTickResult {
        if self.exhaustion > 4.0 {
            self.exhaustion -= 4.0;
            if self.saturation_level > 0.0 {
                self.saturation_level = (self.saturation_level - 1.0).max(0.0);
            } else {
                self.food_level = (self.food_level - 1).max(0);
            }
        }

        self.food_tick_timer += 1;

        // Natural regeneration when food is high
        if self.food_level >= 18 && self.food_tick_timer >= 80 {
            self.food_tick_timer = 0;
            return FoodTickResult::Heal;
        }

        // Starvation damage when food is empty
        if self.food_level <= 0 && self.food_tick_timer >= 80 {
            self.food_tick_timer = 0;
            return FoodTickResult::Starve;
        }

        FoodTickResult::None
    }

    /// Add exhaustion from an activity
    pub fn add_exhaustion(&mut self, amount: f32) {
        self.exhaustion += amount;
    }

    pub fn needs_food(&self) -> bool {
        self.food_level < 20
    }

    pub fn is_hungry(&self) -> bool {
        self.food_level <= 6
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FoodTickResult {
    None,
    Heal,
    Starve,
}
