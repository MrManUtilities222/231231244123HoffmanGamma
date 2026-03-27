#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl TilePos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn hash_code(self) -> i32 {
        self.x
            .wrapping_mul(8_976_890)
            .wrapping_add(self.y.wrapping_mul(981_131))
            .wrapping_add(self.z)
    }
}

