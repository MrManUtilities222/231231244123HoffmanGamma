/// Entity factory and extended entity types
/// Ported from world/entity/EntityFactory.h, Entity.h hierarchy,
/// animal/, monster/, player/ subdirectories

use crate::mob_entity::{MobEntity, MobType};

/// Entity type IDs matching MCPE's EntityFactory
pub const ENTITY_CHICKEN: i32 = 10;
pub const ENTITY_COW: i32 = 11;
pub const ENTITY_PIG: i32 = 12;
pub const ENTITY_SHEEP: i32 = 13;
pub const ENTITY_ZOMBIE: i32 = 32;
pub const ENTITY_CREEPER: i32 = 33;
pub const ENTITY_SKELETON: i32 = 34;
pub const ENTITY_SPIDER: i32 = 35;
pub const ENTITY_PIG_ZOMBIE: i32 = 36;
pub const ENTITY_ITEM: i32 = 64;
pub const ENTITY_PRIMED_TNT: i32 = 65;
pub const ENTITY_FALLING_TILE: i32 = 66;
pub const ENTITY_ARROW: i32 = 80;
pub const ENTITY_SNOWBALL: i32 = 81;
pub const ENTITY_EGG: i32 = 82;
pub const ENTITY_PAINTING: i32 = 83;

/// SynchedEntityData — data watcher ported from SynchedEntityData.h
/// In MCPE this is a key-value store for entity metadata sent over the network.
#[derive(Clone, Debug, Default)]
pub struct SynchedEntityData {
    data: std::collections::HashMap<u8, EntityDataValue>,
}

#[derive(Clone, Debug)]
pub enum EntityDataValue {
    Byte(u8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
}

impl SynchedEntityData {
    pub fn new() -> Self { Self::default() }

    pub fn define(&mut self, key: u8, value: EntityDataValue) {
        self.data.insert(key, value);
    }

    pub fn set(&mut self, key: u8, value: EntityDataValue) {
        self.data.insert(key, value);
    }

    pub fn get_byte(&self, key: u8) -> u8 {
        match self.data.get(&key) {
            Some(EntityDataValue::Byte(v)) => *v,
            _ => 0,
        }
    }

    pub fn get_short(&self, key: u8) -> i16 {
        match self.data.get(&key) {
            Some(EntityDataValue::Short(v)) => *v,
            _ => 0,
        }
    }

    pub fn get_int(&self, key: u8) -> i32 {
        match self.data.get(&key) {
            Some(EntityDataValue::Int(v)) => *v,
            _ => 0,
        }
    }

    pub fn get_float(&self, key: u8) -> f32 {
        match self.data.get(&key) {
            Some(EntityDataValue::Float(v)) => *v,
            _ => 0.0,
        }
    }
}

/// Painting motives — ported from Motive.h
#[derive(Clone, Debug)]
pub struct PaintingMotive {
    pub name: &'static str,
    pub width: i32,
    pub height: i32,
    pub tex_x: i32,
    pub tex_y: i32,
}

pub const PAINTING_MOTIVES: &[PaintingMotive] = &[
    PaintingMotive { name: "Kebab",       width: 16, height: 16, tex_x: 0,    tex_y: 0 },
    PaintingMotive { name: "Aztec",       width: 16, height: 16, tex_x: 16,   tex_y: 0 },
    PaintingMotive { name: "Alban",       width: 16, height: 16, tex_x: 32,   tex_y: 0 },
    PaintingMotive { name: "Aztec2",      width: 16, height: 16, tex_x: 48,   tex_y: 0 },
    PaintingMotive { name: "Bomb",        width: 16, height: 16, tex_x: 64,   tex_y: 0 },
    PaintingMotive { name: "Plant",       width: 16, height: 16, tex_x: 80,   tex_y: 0 },
    PaintingMotive { name: "Wasteland",   width: 16, height: 16, tex_x: 96,   tex_y: 0 },
    PaintingMotive { name: "Pool",        width: 32, height: 16, tex_x: 0,    tex_y: 32 },
    PaintingMotive { name: "Courbet",     width: 32, height: 16, tex_x: 32,   tex_y: 32 },
    PaintingMotive { name: "Sea",         width: 32, height: 16, tex_x: 64,   tex_y: 32 },
    PaintingMotive { name: "Sunset",      width: 32, height: 16, tex_x: 96,   tex_y: 32 },
    PaintingMotive { name: "Creebet",     width: 32, height: 16, tex_x: 128,  tex_y: 32 },
    PaintingMotive { name: "Wanderer",    width: 16, height: 32, tex_x: 0,    tex_y: 64 },
    PaintingMotive { name: "Graham",      width: 16, height: 32, tex_x: 16,   tex_y: 64 },
    PaintingMotive { name: "Match",       width: 32, height: 32, tex_x: 0,    tex_y: 128 },
    PaintingMotive { name: "Bust",        width: 32, height: 32, tex_x: 32,   tex_y: 128 },
    PaintingMotive { name: "Stage",       width: 32, height: 32, tex_x: 64,   tex_y: 128 },
    PaintingMotive { name: "Void",        width: 32, height: 32, tex_x: 96,   tex_y: 128 },
    PaintingMotive { name: "SkullAndRoses", width: 32, height: 32, tex_x: 128, tex_y: 128 },
    PaintingMotive { name: "Fighters",    width: 64, height: 32, tex_x: 0,    tex_y: 96 },
    PaintingMotive { name: "Skeleton",    width: 64, height: 48, tex_x: 192,  tex_y: 64 },
    PaintingMotive { name: "DonkeyKong",  width: 64, height: 48, tex_x: 192,  tex_y: 112 },
    PaintingMotive { name: "Pointer",     width: 64, height: 64, tex_x: 0,    tex_y: 192 },
    PaintingMotive { name: "Pigscene",    width: 64, height: 64, tex_x: 64,   tex_y: 192 },
    PaintingMotive { name: "Flaming Skull", width: 64, height: 64, tex_x: 128, tex_y: 192 },
];

/// HangingEntity / Painting — ported from HangingEntity.h, Painting.h
#[derive(Clone, Debug)]
pub struct Painting {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub direction: i32,
    pub motive_index: usize,
}

impl Painting {
    pub fn new(x: f32, y: f32, z: f32, direction: i32) -> Self {
        let motive_index = rand::random::<usize>() % PAINTING_MOTIVES.len();
        Self { x, y, z, direction, motive_index }
    }

    pub fn get_motive(&self) -> &PaintingMotive {
        &PAINTING_MOTIVES[self.motive_index]
    }
}

/// AgableMob — mobs that can be bred/age; ported from AgableMob.h
#[derive(Clone, Debug)]
pub struct AgableMobData {
    pub age: i32,
    pub is_baby: bool,
    pub love_time: i32,
}

impl AgableMobData {
    pub fn new(is_baby: bool) -> Self {
        Self {
            age: if is_baby { -24000 } else { 0 },
            is_baby,
            love_time: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.age < 0 {
            self.age += 1;
            if self.age >= 0 { self.is_baby = false; }
        }
        if self.love_time > 0 { self.love_time -= 1; }
    }
}

/// FlyingMob — ported from FlyingMob.h (Ghasts in Java, unused in PE Alpha)
pub fn flying_mob_ai_step(mob: &mut MobEntity, _level: &crate::level::Level) {
    // Simplified: flying mobs don't take fall damage, float in air
    mob.vy *= 0.6;
    mob.vy += 0.04; // counteract gravity
}

/// Entity factory — create mob by type ID
pub fn create_mob_by_id(type_id: i32, x: f32, y: f32, z: f32) -> Option<MobEntity> {
    let mob_type = match type_id {
        ENTITY_CHICKEN => MobType::Chicken,
        ENTITY_COW => MobType::Cow,
        ENTITY_PIG => MobType::Pig,
        ENTITY_SHEEP => MobType::Sheep,
        ENTITY_ZOMBIE => MobType::Zombie,
        ENTITY_CREEPER => MobType::Creeper,
        ENTITY_SKELETON => MobType::Skeleton,
        ENTITY_SPIDER => MobType::Spider,
        ENTITY_PIG_ZOMBIE => MobType::PigZombie,
        _ => return None,
    };
    Some(MobEntity::new(mob_type, x, y, z))
}
