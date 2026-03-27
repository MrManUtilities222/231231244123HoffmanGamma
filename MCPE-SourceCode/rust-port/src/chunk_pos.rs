#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn hash_code_xy(x: i32, z: i32) -> i32 {
        let xsign = if x < 0 { 0x8000_0000u32 } else { 0 };
        let zsign = if z < 0 { 0x0000_8000u32 } else { 0 };
        (xsign | (((x as u32) & 0x7fff) << 16) | zsign | ((z as u32) & 0x7fff)) as i32
    }

    pub fn hash_code(self) -> i32 {
        Self::hash_code_xy(self.x, self.z)
    }

    pub fn distance_to_sqr_entity_pos(self, ex: f32, ez: f32) -> f32 {
        let x_pos = (self.x * 16 + 8) as f32;
        let z_pos = (self.z * 16 + 8) as f32;
        let xd = x_pos - ex;
        let zd = z_pos - ez;
        xd * xd + zd * zd
    }
}

