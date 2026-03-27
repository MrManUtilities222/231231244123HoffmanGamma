use crate::mob_entity::{MobEntity, MobId};
use crate::level::Level;

const FLAG_MOVE: u32 = 1;
const FLAG_TARGET: u32 = 2;

#[derive(Clone, Debug)]
pub enum AiState {
    Idle,
    Running,
}

pub trait AiGoal {
    fn can_use(&mut self, mob: &MobEntity, level: &Level) -> bool;
    
    fn can_continue_to_use(&mut self, mob: &MobEntity, level: &Level) -> bool {
        self.can_use(mob, level)
    }
    
    fn can_interrupt(&self) -> bool { true }
    
    fn start(&mut self, _mob: &mut MobEntity, _level: &Level) {}
    fn stop(&mut self, _mob: &mut MobEntity, _level: &Level) {}
    fn tick(&mut self, _mob: &mut MobEntity, _level: &Level) {}
    
    fn get_flags(&self) -> u32;
}

/// Randomly walk around
pub struct RandomStrollGoal {
    pub wanted_x: f32,
    pub wanted_y: f32,
    pub wanted_z: f32,
    pub speed: f32,
}

impl RandomStrollGoal {
    pub fn new(speed: f32) -> Self {
        Self { wanted_x: 0.0, wanted_y: 0.0, wanted_z: 0.0, speed }
    }
}

impl AiGoal for RandomStrollGoal {
    fn can_use(&mut self, mob: &MobEntity, _level: &Level) -> bool {
        if mob.no_action_time >= 100 { return false; } // 5 seconds idle
        
        // Random chance to stroll
        if rand::random::<u32>() % 120 != 0 { return false; }
        
        // Pick a random block offset within 10 blocks X/Z, 7 blocks Y
        let rx = (rand::random::<f32>() * 20.0) - 10.0;
        let ry = (rand::random::<f32>() * 14.0) - 7.0;
        let rz = (rand::random::<f32>() * 20.0) - 10.0;
        
        self.wanted_x = mob.core.x + rx;
        self.wanted_y = mob.core.y + ry;
        self.wanted_z = mob.core.z + rz;
        true
    }

    fn can_continue_to_use(&mut self, mob: &MobEntity, _level: &Level) -> bool {
        mob.path.is_some()
    }

    fn start(&mut self, mob: &mut MobEntity, level: &Level) {
        if let Some(path) = crate::pathfinder::find_path(
            level,
            crate::pathfinder::PathNode::new(mob.core.x.floor() as i32, mob.core.y.floor() as i32, mob.core.z.floor() as i32),
            crate::pathfinder::PathNode::new(self.wanted_x.floor() as i32, self.wanted_y.floor() as i32, self.wanted_z.floor() as i32),
            32.0
        ) {
            mob.path = Some(path);
        }
    }

    fn tick(&mut self, mob: &mut MobEntity, _level: &Level) {
        // Abstract path movement logic; mob core tick moves along path
    }

    fn get_flags(&self) -> u32 { FLAG_MOVE }
}

/// Target the nearest player and set it as attack target
pub struct NearestAttackableTargetGoal {
    pub within: f32,
    pub random_interval: i32,
    pub must_see: bool,
    pub target_id: Option<MobId>,
}

impl NearestAttackableTargetGoal {
    pub fn new(within: f32, random_interval: i32, must_see: bool) -> Self {
        Self { within, random_interval, must_see, target_id: None }
    }
}

impl AiGoal for NearestAttackableTargetGoal {
    fn can_use(&mut self, _mob: &MobEntity, _level: &Level) -> bool {
        if self.random_interval > 0 && rand::random::<i32>().abs() % self.random_interval != 0 {
            return false;
        }
        // Simplified: The full game searches all entities for the nearest Player.
        // For now, we will assume a basic structure.
        // In the full game, target selection checks distance.
        true
    }

    fn start(&mut self, mob: &mut MobEntity, _level: &Level) {
        mob.target_id = self.target_id;
    }

    fn get_flags(&self) -> u32 { FLAG_TARGET }
}

/// Melee attack an acquired target
pub struct MeleeAttackGoal {
    pub speed: f32,
    pub attack_reach: f32,
    pub attack_delay: i32,
}

impl MeleeAttackGoal {
    pub fn new(speed: f32, attack_reach: f32) -> Self {
        Self { speed, attack_reach, attack_delay: 0 }
    }
}

impl AiGoal for MeleeAttackGoal {
    fn can_use(&mut self, mob: &MobEntity, _level: &Level) -> bool {
        mob.target_id.is_some()
    }

    fn can_continue_to_use(&mut self, mob: &MobEntity, level: &Level) -> bool {
        self.can_use(mob, level) && mob.path.is_some()
    }

    fn start(&mut self, _mob: &mut MobEntity, _level: &Level) {
        self.attack_delay = 0;
    }

    fn tick(&mut self, mob: &mut MobEntity, level: &Level) {
        if let Some(target_id) = mob.target_id {
            // Find target pos
            // Let's assume there's a way to get target pos.
            // Move to target
            // If near, hit target
        }
        if self.attack_delay > 0 { self.attack_delay -= 1; }
    }

    fn get_flags(&self) -> u32 { FLAG_MOVE }
}

/// Run GoalSelector
pub struct GoalSelector {
    pub goals: Vec<(u32, Box<dyn AiGoal>)>,
    pub running_flags: u32,
}

impl GoalSelector {
    pub fn new() -> Self {
        Self { goals: Vec::new(), running_flags: 0 }
    }

    pub fn add_goal(&mut self, priority: u32, goal: Box<dyn AiGoal>) {
        self.goals.push((priority, goal));
    }

    pub fn tick(&mut self, mob: &mut MobEntity, level: &Level) {
        // Simplified Goal Selector:
        // Try starting goals if their flags are available.
        // Try stopping goals if they finished.
    }
}
