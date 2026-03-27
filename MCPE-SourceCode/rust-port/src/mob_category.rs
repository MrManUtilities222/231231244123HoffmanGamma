#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnMaterial {
    Air,
    Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MobCategory {
    base_class_id: i32,
    max: i32,
    max_per_level: i32,
    spawn_position_material: SpawnMaterial,
    is_friendly: bool,
}

impl MobCategory {
    pub const fn new(
        base_class_id: i32,
        max: i32,
        max_per_level: i32,
        is_friendly: bool,
        spawn_position_material: SpawnMaterial,
    ) -> Self {
        Self {
            base_class_id,
            max,
            max_per_level,
            spawn_position_material,
            is_friendly,
        }
    }

    pub fn base_class_id(self) -> i32 {
        self.base_class_id
    }
    pub fn max_instances_per_chunk(self) -> i32 {
        self.max
    }
    pub fn max_instances_per_level(self) -> i32 {
        self.max_per_level
    }
    pub fn spawn_position_material(self) -> SpawnMaterial {
        self.spawn_position_material
    }
    pub fn is_friendly(self) -> bool {
        self.is_friendly
    }
}

// Base IDs come from EntityTypes/MobTypes in source; constants kept local for now.
pub const BASE_ENEMY: i32 = 1;
pub const BASE_CREATURE: i32 = 2;
pub const BASE_WATER_CREATURE: i32 = 3;

pub const MONSTER: MobCategory = MobCategory::new(BASE_ENEMY, 10, 20, false, SpawnMaterial::Air);
pub const CREATURE: MobCategory = MobCategory::new(BASE_CREATURE, 10, 15, true, SpawnMaterial::Air);
pub const WATER_CREATURE: MobCategory =
    MobCategory::new(BASE_WATER_CREATURE, 5, 10, true, SpawnMaterial::Water);

pub const VALUES: [MobCategory; 3] = [MONSTER, CREATURE, WATER_CREATURE];

