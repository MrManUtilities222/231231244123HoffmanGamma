use crate::player::Player;
use crate::level::Level;

pub struct LocalPlayer {
    pub player: Player,
    // Client-specific fields
}

impl LocalPlayer {
    pub fn new(is_creative: bool) -> Self {
        Self {
            player: Player::new(is_creative),
        }
    }

    pub fn tick(&mut self, level: Option<&crate::level::Level>) {
        self.player.tick(level);
        // Client tick
    }

    pub fn move_relative(&mut self, forward: f32, strafe: f32, friction: f32) {
        let dist = forward * forward + strafe * strafe;
        if dist >= 0.01 {
            let dist = friction / dist.sqrt().max(1.0);
            let forward = forward * dist;
            let strafe = strafe * dist;
            let sin = (self.player.mob.entity.core.y_rot * std::f32::consts::PI / 180.0).sin();
            let cos = (self.player.mob.entity.core.y_rot * std::f32::consts::PI / 180.0).cos();
            self.player.mob.entity.core.xd += strafe * cos - forward * sin;
            self.player.mob.entity.core.zd += forward * cos + strafe * sin;
        }
    }

    pub fn jump(&mut self) {
        if self.player.mob.entity.core.on_ground {
            self.player.mob.entity.core.yd = 0.42;
        }
    }
}