use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitResultType {
    Tile,
    Entity,
    NoHit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitResult {
    pub hit_type: HitResultType,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub f: i32,
    pub pos: Vec3,
    pub entity_id: Option<u32>,
    pub indirect_hit: bool,
}

impl Default for HitResult {
    fn default() -> Self {
        Self {
            hit_type: HitResultType::NoHit,
            x: 0,
            y: 0,
            z: 0,
            f: 0,
            pos: Vec3::default(),
            entity_id: None,
            indirect_hit: false,
        }
    }
}

impl HitResult {
    pub fn tile(x: i32, y: i32, z: i32, f: i32, pos: Vec3) -> Self {
        Self {
            hit_type: HitResultType::Tile,
            x,
            y,
            z,
            f,
            pos,
            entity_id: None,
            indirect_hit: false,
        }
    }

    pub fn entity(entity_id: u32, pos: Vec3) -> Self {
        Self {
            hit_type: HitResultType::Entity,
            x: 0,
            y: 0,
            z: 0,
            f: 0,
            pos,
            entity_id: Some(entity_id),
            indirect_hit: false,
        }
    }

    pub fn is_hit(self) -> bool {
        self.hit_type != HitResultType::NoHit
    }

    pub fn distance_to_xyz_sqr(self, entity_pos: Vec3) -> f32 {
        self.pos.distance_to_sqr(entity_pos)
    }
}

