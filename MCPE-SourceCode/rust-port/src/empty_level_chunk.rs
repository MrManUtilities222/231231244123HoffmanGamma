use crate::level_chunk::LevelChunk;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyLevelChunk {
    pub inner: LevelChunk,
    pub empty_tile_id: u8,
}

impl EmptyLevelChunk {
    pub fn new(x: i32, z: i32, empty_tile_id: u8) -> Self {
        let mut inner = LevelChunk::new(x, z);
        inner.dont_save = true;
        Self { inner, empty_tile_id }
    }

    pub fn get_tile(&self, _x: usize, _y: usize, _z: usize) -> u8 {
        self.empty_tile_id
    }

    pub fn is_empty(&self) -> bool {
        true
    }
}

