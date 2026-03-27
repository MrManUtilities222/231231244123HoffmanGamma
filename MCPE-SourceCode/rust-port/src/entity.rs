use crate::entity_core::EntityCore;
use crate::level::Level;
use crate::vec3::Vec3;
use crate::aabb::Aabb;

pub struct Entity {
    pub core: EntityCore,
    // Add more fields as needed
}

impl Entity {
    pub fn new() -> Self {
        Self {
            core: EntityCore::default(),
        }
    }

    pub fn tick(&mut self, level: Option<&crate::level::Level>) {
        // Update position
        self.core.xo = self.core.x;
        self.core.yo = self.core.y;
        self.core.zo = self.core.z;
        
        // Gravity
        self.core.yd -= 0.08;
        
        let xd = self.core.xd;
        let yd = self.core.yd;
        let zd = self.core.zd;
        
        self.move_pos(xd, yd, zd, level);
        
        // Friction / drag
        self.core.xd *= 0.91;
        self.core.yd *= 0.98;
        self.core.zd *= 0.91;
    }

    pub fn move_pos(&mut self, mut xa: f32, mut ya: f32, mut za: f32, level: Option<&crate::level::Level>) {
        let xa_org = xa;
        let ya_org = ya;
        let za_org = za;

        if let Some(level) = level {
            let expanded = self.core.bb.expand(xa, ya, za);
            let cubes = level.get_cubes(&expanded);

            for c in &cubes {
                ya = c.clip_y_collide(self.core.bb, ya);
            }
            self.core.bb.move_by(0.0, ya, 0.0);

            for c in &cubes {
                xa = c.clip_x_collide(self.core.bb, xa);
            }
            self.core.bb.move_by(xa, 0.0, 0.0);

            for c in &cubes {
                za = c.clip_z_collide(self.core.bb, za);
            }
            self.core.bb.move_by(0.0, 0.0, za);
        } else {
            self.core.bb.move_by(xa, ya, za);
        }

        self.core.on_ground = ya_org != ya && ya_org < 0.0;

        if ya_org != ya {
            self.core.yd = 0.0;
        }
        if xa_org != xa {
            self.core.xd = 0.0;
        }
        if za_org != za {
            self.core.zd = 0.0;
        }

        self.core.x = (self.core.bb.x0 + self.core.bb.x1) / 2.0;
        self.core.y = self.core.bb.y0 + self.core.y_slide_offset - self.core.height_offset;
        self.core.z = (self.core.bb.z0 + self.core.bb.z1) / 2.0;
    }

    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.core.x = x;
        self.core.y = y;
        self.core.z = z;
    }

    pub fn move_to(&mut self, x: f32, y: f32, z: f32, y_rot: f32, x_rot: f32) {
        self.set_pos(x, y, z);
        self.core.y_rot = y_rot;
        self.core.x_rot = x_rot;
    }

    pub fn is_alive(&self) -> bool {
        true // Placeholder
    }

    pub fn is_player(&self) -> bool {
        false
    }

    // Add more methods
}