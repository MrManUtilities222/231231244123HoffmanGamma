/// Projectile entity system ported from Arrow.h, Snowball.h, ThrownEgg.h, Throwable.h
/// Also covers FallingTile.h, PrimedTnt.h, ItemEntity.h

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectileKind {
    Arrow,
    Snowball,
    Egg,
    FallingTile { tile_id: i32 },
    PrimedTnt { fuse: i32 },
    DroppedItem { item_id: i32, count: i32 },
}

#[derive(Clone, Debug)]
pub struct Projectile {
    pub kind: ProjectileKind,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub age: i32,
    pub dead: bool,
    pub on_ground: bool,
    pub damage: f32,
    pub pickup: bool,
    pub owner_id: Option<u64>,
}

impl Projectile {
    pub fn new(kind: ProjectileKind, x: f32, y: f32, z: f32) -> Self {
        let damage = match &kind {
            ProjectileKind::Arrow => 2.0,
            ProjectileKind::Snowball => 0.0,
            ProjectileKind::Egg => 0.0,
            ProjectileKind::FallingTile { .. } => 0.0,
            ProjectileKind::PrimedTnt { .. } => 0.0,
            ProjectileKind::DroppedItem { .. } => 0.0,
        };
        Self {
            kind, x, y, z,
            vx: 0.0, vy: 0.0, vz: 0.0,
            age: 0, dead: false, on_ground: false,
            damage, pickup: true, owner_id: None,
        }
    }

    /// Shoot a projectile in a direction with speed
    pub fn shoot(&mut self, dir_x: f32, dir_y: f32, dir_z: f32, speed: f32) {
        let len = (dir_x * dir_x + dir_y * dir_y + dir_z * dir_z).sqrt();
        if len > 0.0 {
            self.vx = dir_x / len * speed;
            self.vy = dir_y / len * speed;
            self.vz = dir_z / len * speed;
        }
    }

    /// Tick the projectile physics
    pub fn tick(&mut self, level: &crate::level::Level) {
        self.age += 1;
        if self.dead { return; }

        // Max lifetime
        if self.age > 1200 {
            self.dead = true;
            return;
        }

        // Apply gravity
        match &self.kind {
            ProjectileKind::PrimedTnt { .. } => {
                self.vy -= 0.04;
            },
            ProjectileKind::FallingTile { .. } => {
                self.vy -= 0.04;
            },
            ProjectileKind::DroppedItem { .. } => {
                self.vy -= 0.04;
            },
            _ => {
                self.vy -= 0.05;
            }
        }

        // Move
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;

        // Air drag
        self.vx *= 0.99;
        self.vy *= 0.99;
        self.vz *= 0.99;

        // Ground collision
        let tile = level.get_tile(self.x as i32, self.y as i32, self.z as i32);
        if crate::tile::is_solid(tile) {
            self.on_ground = true;
            self.vx = 0.0;
            self.vy = 0.0;
            self.vz = 0.0;
            
            match &self.kind {
                ProjectileKind::Snowball | ProjectileKind::Egg => {
                    self.dead = true;
                },
                ProjectileKind::PrimedTnt { fuse } => {
                    if self.age >= *fuse {
                        self.dead = true;
                        // Explosion would happen here
                    }
                },
                ProjectileKind::FallingTile { tile_id } => {
                    // Place the tile at the landing position
                    self.dead = true;
                },
                ProjectileKind::Arrow => {
                    // Arrow sticks in the ground, can be picked up
                },
                ProjectileKind::DroppedItem { .. } => {
                    // Item sits on ground waiting for pickup
                },
            }
        }
    }
}

/// Create an arrow projectile shot from a position in a direction
pub fn shoot_arrow(x: f32, y: f32, z: f32, yaw: f32, pitch: f32) -> Projectile {
    let mut p = Projectile::new(ProjectileKind::Arrow, x, y, z);
    let yaw_rad = yaw.to_radians();
    let pitch_rad = pitch.to_radians();
    p.shoot(
        -yaw_rad.sin() * pitch_rad.cos(),
        -pitch_rad.sin(),
        yaw_rad.cos() * pitch_rad.cos(),
        1.5,
    );
    p
}

/// Create a falling tile entity (sand, gravel)
pub fn falling_tile(x: f32, y: f32, z: f32, tile_id: i32) -> Projectile {
    Projectile::new(ProjectileKind::FallingTile { tile_id }, x, y, z)
}

/// Create a dropped item entity
pub fn dropped_item(x: f32, y: f32, z: f32, item_id: i32, count: i32) -> Projectile {
    use rand::Rng;
    let mut p = Projectile::new(ProjectileKind::DroppedItem { item_id, count }, x, y, z);
    let mut rng = rand::thread_rng();
    p.vx = rng.gen_range(-0.1..0.1);
    p.vy = 0.2;
    p.vz = rng.gen_range(-0.1..0.1);
    p
}

/// Create primed TNT
pub fn primed_tnt(x: f32, y: f32, z: f32) -> Projectile {
    Projectile::new(ProjectileKind::PrimedTnt { fuse: 80 }, x, y, z)
}
