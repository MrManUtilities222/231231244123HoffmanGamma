use crate::chunk_codec::{CHUNK_BLOCK_COUNT, CHUNK_COLUMNS, CHUNK_DEPTH, CHUNK_WIDTH};
use crate::data_layer::DataLayer;
use crate::light_layer::LightLayer;
use std::sync::atomic::{AtomicBool, Ordering};

static TOUCHED_SKY: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelChunk {
    pub x: i32,
    pub z: i32,
    pub xt: i32,
    pub zt: i32,
    pub blocks: Vec<u8>,
    pub data: DataLayer,
    pub sky_light: DataLayer,
    pub block_light: DataLayer,
    pub heightmap: [u8; CHUNK_COLUMNS],
    pub update_map: [u8; CHUNK_COLUMNS],
    pub unsaved: bool,
    pub dont_save: bool,
    pub terrain_populated: bool,
    pub created_from_save: bool,
    entity_blocks: Vec<Vec<u32>>,
}

impl LevelChunk {
    pub const UPDATE_MAP_BIT_SHIFT: usize = 4;

    pub fn new(x: i32, z: i32) -> Self {
        Self::with_blocks(vec![0; CHUNK_BLOCK_COUNT], x, z)
    }

    pub fn with_blocks(blocks: Vec<u8>, x: i32, z: i32) -> Self {
        let mut out = Self {
            x,
            z,
            xt: x * CHUNK_WIDTH as i32,
            zt: z * CHUNK_DEPTH as i32,
            blocks,
            data: DataLayer::new(CHUNK_BLOCK_COUNT),
            sky_light: DataLayer::new(CHUNK_BLOCK_COUNT),
            block_light: DataLayer::new(CHUNK_BLOCK_COUNT),
            heightmap: [0; CHUNK_COLUMNS],
            update_map: [0; CHUNK_COLUMNS],
            unsaved: false,
            dont_save: false,
            terrain_populated: false,
            created_from_save: false,
            entity_blocks: vec![Vec::new(); 128 / 16],
        };
        out.init();
        out
    }

    pub fn init(&mut self) {
        self.terrain_populated = false;
        self.dont_save = false;
        self.unsaved = false;
        self.created_from_save = false;
        self.heightmap.fill(0);
        self.update_map.fill(0);
    }

    pub fn clear_update_map(&mut self) {
        self.update_map.fill(0);
        self.unsaved = false;
    }

    pub fn mark_unsaved(&mut self) {
        self.unsaved = true;
    }

    pub fn is_at(&self, x: i32, z: i32) -> bool {
        self.x == x && self.z == z
    }

    pub fn get_tile(&self, x: usize, y: usize, z: usize) -> u8 {
        self.blocks[(x << 11) | (z << 7) | y]
    }

    pub fn set_tile_raw(&mut self, x: usize, y: usize, z: usize, tile: u8) {
        self.blocks[(x << 11) | (z << 7) | y] = tile;
    }

    pub fn get_data(&self, x: usize, y: usize, z: usize) -> u8 {
        self.data.get_xyz(x, y, z)
    }

    pub fn set_data(&mut self, x: usize, y: usize, z: usize, val: u8) {
        self.data.set_xyz(x, y, z, val);
    }

    pub fn set_tile_and_data(&mut self, x: usize, y: usize, z: usize, tile: u8, data: u8) -> bool {
        let old = self.get_tile(x, y, z);
        let old_data = self.get_data(x, y, z);
        if old == tile && old_data == data {
            return false;
        }

        self.set_tile_raw(x, y, z, tile);
        self.set_data(x, y, z, data);
        self.unsaved = true;

        let col = x | (z << 4);
        self.update_map[col] |= 1 << (y >> Self::UPDATE_MAP_BIT_SHIFT);
        true
    }

    pub fn get_brightness(&self, layer: LightLayer, x: usize, y: usize, z: usize) -> u8 {
        match layer {
            LightLayer::Sky => self.sky_light.get_xyz(x, y, z),
            LightLayer::Block => self.block_light.get_xyz(x, y, z),
        }
    }

    pub fn set_brightness(&mut self, layer: LightLayer, x: usize, y: usize, z: usize, brightness: u8) {
        match layer {
            LightLayer::Sky => self.sky_light.set_xyz(x, y, z, brightness),
            LightLayer::Block => self.block_light.set_xyz(x, y, z, brightness),
        }
    }

    pub fn get_raw_brightness(&self, x: usize, y: usize, z: usize, sky_dampen: i32) -> i32 {
        let mut light = self.sky_light.get_xyz(x, y, z) as i32;
        if light > 0 {
            TOUCHED_SKY.store(true, Ordering::Relaxed);
        }
        light -= sky_dampen;
        let block = self.block_light.get_xyz(x, y, z) as i32;
        if block > light {
            light = block;
        }
        light
    }

    pub fn touched_sky() -> bool {
        TOUCHED_SKY.load(Ordering::Relaxed)
    }

    pub fn clear_touched_sky() {
        TOUCHED_SKY.store(false, Ordering::Relaxed);
    }

    pub fn add_entity_stub(&mut self, entity_id: u32, y: f32) -> usize {
        let mut yc = (y / 16.0).floor() as isize;
        if yc < 0 {
            yc = 0;
        }
        let max = self.entity_blocks.len() as isize - 1;
        if yc > max {
            yc = max;
        }
        let idx = yc as usize;
        self.entity_blocks[idx].push(entity_id);
        idx
    }

    pub fn remove_entity_stub(&mut self, entity_id: u32, yc: usize) {
        if yc >= self.entity_blocks.len() {
            return;
        }
        self.entity_blocks[yc].retain(|id| *id != entity_id);
    }

    pub fn count_entities(&self) -> usize {
        self.entity_blocks.iter().map(|v| v.len()).sum()
    }
}

