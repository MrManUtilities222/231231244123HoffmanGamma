/// Mob type categories ported from EntityTypes.h and MobCategory.h
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MobType {
    Zombie,
    Skeleton,
    Spider,
    Creeper,
    PigZombie,
    Pig,
    Sheep,
    Cow,
    Chicken,
}

/// AI behavior state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AiBehavior {
    Idle,
    Wandering,
    Fleeing,
    Attacking,
}

/// A spawned mob entity in the world.
#[derive(Clone, Debug)]
pub struct MobEntity {
    pub mob_type: MobType,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub health: i32,
    pub max_health: i32,
    pub attack_damage: i32,
    pub armor: i32,
    pub behavior: AiBehavior,
    pub wander_timer: i32,
    pub dead: bool,
    pub age: i32,
    // Sounds
    pub ambient_sound: &'static str,
    pub hurt_sound: &'static str,
    pub death_sound: &'static str,
    pub death_loot_id: i32,
    // AI state
    pub no_action_time: i32,
    pub path: Option<crate::pathfinder::Path>,
    pub target_id: Option<MobId>,
    pub core: CorePos,
}

#[derive(Clone, Debug)]
pub struct CorePos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type MobId = i32;

impl MobEntity {
    pub fn new(mob_type: MobType, x: f32, y: f32, z: f32) -> Self {
        let (max_hp, dmg, armor, amb, hurt, death, loot) = match mob_type {
            MobType::Zombie => (20, 4, 2, "mob.zombie.say", "mob.zombie.hurt", "mob.zombie.death", 367),
            MobType::Skeleton => (20, 3, 1, "mob.skeleton.say", "mob.skeleton.hurt", "mob.skeleton.death", 262),
            MobType::Spider => (16, 2, 0, "mob.spider.say", "mob.spider.hurt", "mob.spider.death", 287),
            MobType::Creeper => (20, 0, 0, "mob.creeper.say", "mob.creeper.hurt", "mob.creeper.death", 289),
            MobType::PigZombie => (20, 5, 2, "mob.zombiepig.zpig", "mob.zombiepig.zpighurt", "mob.zombiepig.zpigdeath", 371),
            MobType::Pig => (10, 0, 0, "mob.pig.say", "mob.pig.hurt", "mob.pig.death", 319),
            MobType::Sheep => (8, 0, 0, "mob.sheep.say", "mob.sheep.hurt", "mob.sheep.death", 35),
            MobType::Cow => (10, 0, 0, "mob.cow.say", "mob.cow.hurt", "mob.cow.death", 334),
            MobType::Chicken => (4, 0, 0, "mob.chicken.say", "mob.chicken.hurt", "mob.chicken.death", 365),
        };

        Self {
            mob_type,
            x, y, z,
            vx: 0.0, vy: 0.0, vz: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            health: max_hp,
            max_health: max_hp,
            attack_damage: dmg,
            armor: armor,
            behavior: AiBehavior::Idle,
            wander_timer: 0,
            dead: false,
            age: 0,
            ambient_sound: amb,
            hurt_sound: hurt,
            death_sound: death,
            death_loot_id: loot,
            no_action_time: 0,
            path: None,
            target_id: None,
            core: CorePos { x, y, z },
        }
    }

    pub fn is_hostile(&self) -> bool {
        matches!(self.mob_type, MobType::Zombie | MobType::Skeleton | MobType::Spider | MobType::Creeper | MobType::PigZombie)
    }

    pub fn is_animal(&self) -> bool {
        matches!(self.mob_type, MobType::Pig | MobType::Sheep | MobType::Cow | MobType::Chicken)
    }

    /// Simple AI tick: wander randomly, apply gravity, age.
    pub fn ai_step(&mut self, level: &crate::level::Level) {
        use rand::Rng;
        self.age += 1;
        self.no_action_time += 1; // Idle timer

        if self.dead { return; }

        // Sync core with entity until fully refactored
        self.core.x = self.x;
        self.core.y = self.y;
        self.core.z = self.z;

        // Simple gravity
        let below = level.get_tile(self.x as i32, (self.y - 1.0) as i32, self.z as i32);
        if !crate::tile::is_solid(below) {
            self.vy -= 0.08;
        } else {
            if self.vy < 0.0 {
                self.vy = 0.0;
                self.y = (self.y as i32) as f32 + 1.0;
            }
        }

        // Handle path movement
        if let Some(ref mut path) = self.path {
            self.no_action_time = 0; // Reset idle timer while moving
            if !path.is_done() {
                if let Some(node) = path.current_node() {
                    let dx = node.x as f32 - self.x;
                    let dz = node.z as f32 - self.z;
                    if dx * dx + dz * dz < 0.5 {
                        path.advance();
                    } else {
                        // Move toward current node
                        let speed = 0.06;
                        self.yaw = dx.atan2(dz).to_degrees();
                        self.vx = dx.signum() * speed;
                        self.vz = dz.signum() * speed;
                    }
                }
            } else {
                self.path = None; // Reached goal
            }
        } else {
            // Apply normal wander
            self.wander_timer -= 1;
            if self.wander_timer <= 0 {
                let mut rng = rand::thread_rng();
                self.behavior = if rng.gen_range(0..3) == 0 {
                    AiBehavior::Wandering
                } else {
                    AiBehavior::Idle
                };
                self.wander_timer = rng.gen_range(40..200);

                if self.behavior == AiBehavior::Wandering {
                    self.yaw = rng.gen_range(0.0..360.0f32);
                    let speed = 0.06;
                    self.vx = -self.yaw.to_radians().sin() * speed;
                    self.vz = self.yaw.to_radians().cos() * speed;
                }
            }
        }

        // Apply velocity
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
        self.core.x = self.x;
        self.core.y = self.y;
        self.core.z = self.z;

        // Friction
        if self.path.is_none() {
            self.vx *= 0.8;
            self.vz *= 0.8;
        }
        self.vy *= 0.98;
    }

    pub fn hurt(&mut self, damage: i32) {
        let effective = (damage - self.armor).max(1);
        self.health -= effective;
        if self.health <= 0 {
            self.dead = true;
        }
    }

    pub fn get_entity_type_id(&self) -> i32 {
        match self.mob_type {
            MobType::Chicken => 10,
            MobType::Cow => 11,
            MobType::Pig => 12,
            MobType::Sheep => 13,
            MobType::Zombie => 32,
            MobType::Creeper => 33,
            MobType::Skeleton => 34,
            MobType::Spider => 35,
            MobType::PigZombie => 36,
        }
    }
}

/// Mob spawner logic: called during level tick to potentially spawn mobs.
pub fn try_spawn_mobs(level: &crate::level::Level, mobs: &mut Vec<MobEntity>, max_mobs: usize) {
    use rand::Rng;
    if mobs.len() >= max_mobs { return; }

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-32..=32) as f32;
    let z = rng.gen_range(-32..=32) as f32;

    // Find surface
    for y in (0..127).rev() {
        let t = level.get_tile(x as i32, y, z as i32);
        if t == crate::tile::GRASS.id || t == crate::tile::SAND.id {
            let mob_type = if rng.gen_range(0..2) == 0 {
                // Animals
                match rng.gen_range(0..4) {
                    0 => MobType::Pig,
                    1 => MobType::Sheep,
                    2 => MobType::Cow,
                    _ => MobType::Chicken,
                }
            } else {
                // Monsters
                match rng.gen_range(0..4) {
                    0 => MobType::Zombie,
                    1 => MobType::Skeleton,
                    2 => MobType::Spider,
                    _ => MobType::Creeper,
                }
            };

            let mob = MobEntity::new(mob_type, x, (y + 1) as f32, z);
            mobs.push(mob);
            break;
        } else if crate::tile::is_solid(t) {
            break;
        }
    }
}
