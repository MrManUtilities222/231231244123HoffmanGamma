use crate::entity::Entity;
use crate::level_data::LevelData;
use crate::level_storage_source::LevelStorageSource;
use crate::chunk_pos::ChunkPos;
use crate::level_chunk::LevelChunk;
use std::collections::HashMap;

pub struct Level {
    level_data: LevelData,
    _level_storage: Box<dyn LevelStorageSource>,
    chunks: HashMap<ChunkPos, LevelChunk>,
    entities: Vec<Entity>,
}

impl Level {
    pub fn new(level_storage: Box<dyn LevelStorageSource>, level_name: &str, settings: (), generator_version: i32) -> Self {
        let mut level = Self {
            level_data: LevelData::new(level_name.to_string()),
            _level_storage: level_storage,
            chunks: HashMap::new(),
            entities: Vec::new(),
        };
        level.generate_terrain(1);
        level
    }

    fn get_chunk_mut(&mut self, chunk_x: i32, chunk_z: i32) -> &mut LevelChunk {
        self.chunks
            .entry(ChunkPos::new(chunk_x, chunk_z))
            .or_insert_with(|| LevelChunk::new(chunk_x, chunk_z))
    }

    pub fn generate_terrain(&mut self, radius: i32) {
        let terrain_noise = crate::noise::Noise2D::new(self.level_data.seed as i32);
        // Second noise map seeded slightly differently to simulate Temperature/Rainfall
        let biome_noise = crate::noise::Noise2D::new((self.level_data.seed ^ 10) as i32);
        
        for cx in -radius..=radius {
            for cz in -radius..=radius {
                let chunk = self.get_chunk_mut(cx, cz);
                for x in 0..16 {
                    for z in 0..16 {
                        let global_x = (cx * 16 + x as i32) as f32;
                        let global_z = (cz * 16 + z as i32) as f32;
                        
                        // Frequencies for a nicer looking hills terrain
                        let height_val = terrain_noise.get_octave_noise(global_x * 0.05, global_z * 0.05, 4);
                        // Map noise (-1.0 to 1.0) to height
                        let height = (64.0 + height_val * 16.0) as i32;
                        
                        // Biome map (very smooth)
                        let temp_val = biome_noise.get_octave_noise(global_x * 0.01, global_z * 0.01, 2);
                        let is_desert = temp_val > 0.1;
                        
                        for y in 0i32..128 {
                            let tile_id = if y == height {
                                if is_desert { crate::tile::SAND.id as u8 } else { crate::tile::GRASS.id as u8 }
                            } else if y < height && y > height - 4 {
                                if is_desert { crate::tile::SAND.id as u8 } else { crate::tile::DIRT.id as u8 }
                            } else if y <= height - 4 {
                                crate::tile::STONE.id as u8
                            } else {
                                crate::tile::AIR.id as u8
                            };
                            chunk.set_tile_raw(x, y as usize, z, tile_id);
                        }
                    }
                }
            }
        }
        
        self.populate_features(radius);
    }

    fn populate_features(&mut self, radius: i32) {
        use rand::Rng;
        // Simple random spawner seeded by thread_rng instead of level seed for rapid testing.
        let mut rng = rand::thread_rng();
        let tree_feature = crate::feature::TreeFeature::new();
        let cactus_feature = crate::feature::CactusFeature::new();

        for cx in -radius..=radius {
            for cz in -radius..=radius {
                // Try to spawn ~3 trees per chunk area
                for _ in 0..3 {
                    let rx = cx * 16 + rng.gen_range(0..=15);
                    let rz = cz * 16 + rng.gen_range(0..=15);

                    // Find surface block
                    for ry in (0..127).rev() {
                        let t = self.get_tile(rx, ry, rz);
                        if t == crate::tile::GRASS.id {
                            let _ = tree_feature.place(self, rx, ry + 1, rz, &mut rng);
                            break;
                        } else if t == crate::tile::SAND.id {
                            // If we hit sand, possibly spawn cactus
                            if rng.gen_range(0..2) == 0 {
                                let _ = cactus_feature.place(self, rx, ry + 1, rz, &mut rng);
                            }
                            break;
                        } else if t != crate::tile::AIR.id && t != crate::tile::LEAVES.id && t != crate::tile::CACTUS.id {
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn tick_entities(&mut self) {
        for entity in &mut self.entities {
            entity.tick(None);
        }
    }

    pub fn get_tile(&self, x: i32, y: i32, z: i32) -> i32 {
        if y < 0 || y >= 128 {
            return crate::tile::AIR.id;
        }

        let chunk_x = x.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        let block_x = x.rem_euclid(16) as usize;
        let block_z = z.rem_euclid(16) as usize;
        let chunk_pos = ChunkPos::new(chunk_x, chunk_z);

        if let Some(chunk) = self.chunks.get(&chunk_pos) {
            chunk.get_tile(block_x, y as usize, block_z) as i32
        } else {
            crate::tile::AIR.id
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, z: i32, tile: i32) -> bool {
        if y < 0 || y >= 128 {
            return false;
        }

        let chunk_x = x.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        let block_x = x.rem_euclid(16) as usize;
        let block_z = z.rem_euclid(16) as usize;

        let chunk = self.get_chunk_mut(chunk_x, chunk_z);
        chunk.set_tile_and_data(block_x, y as usize, block_z, tile as u8, 0)
    }

    pub fn has_chunk_at(&self, x: i32, _y: i32, z: i32) -> bool {
        let chunk_pos = ChunkPos::new(x >> 4, z >> 4);
        self.chunks.contains_key(&chunk_pos)
    }

    pub fn get_cubes(&self, aabb: &crate::aabb::Aabb) -> Vec<crate::aabb::Aabb> {
        let mut cubes = Vec::new();
        
        let x0 = aabb.x0.floor() as i32;
        let x1 = aabb.x1.ceil() as i32;
        let y0 = aabb.y0.floor() as i32;
        let y1 = aabb.y1.ceil() as i32;
        let z0 = aabb.z0.floor() as i32;
        let z1 = aabb.z1.ceil() as i32;
        
        for x in x0..x1 {
            for y in y0..y1 {
                for z in z0..z1 {
                    let tile_id = self.get_tile(x, y, z);
                    if tile_id > 0 && crate::tile::is_solid(tile_id) {
                        let block_aabb = crate::aabb::Aabb::new(
                            x as f32, y as f32, z as f32,
                            (x + 1) as f32, (y + 1) as f32, (z + 1) as f32
                        );
                        if aabb.intersects(block_aabb) {
                            cubes.push(block_aabb);
                        }
                    }
                }
            }
        }
        
        cubes
    }

    pub fn clip(&self, p1: crate::vec3::Vec3, p2: crate::vec3::Vec3) -> Option<crate::hit_result::HitResult> {
        let min_x = p1.x.min(p2.x) - 1.0;
        let min_y = p1.y.min(p2.y) - 1.0;
        let min_z = p1.z.min(p2.z) - 1.0;
        let max_x = p1.x.max(p2.x) + 1.0;
        let max_y = p1.y.max(p2.y) + 1.0;
        let max_z = p1.z.max(p2.z) + 1.0;
        
        let aabb = crate::aabb::Aabb::new(min_x, min_y, min_z, max_x, max_y, max_z);
        let cubes = self.get_cubes(&aabb);
        
        let mut nearest_hit = None;
        let mut nearest_dist = std::f32::MAX;
        
        for cube in cubes {
            let hit = cube.clip(p1, p2);
            if hit.is_hit() {
                let d = p1.distance_to_sqr(hit.pos);
                if d < nearest_dist {
                    nearest_dist = d;
                    nearest_hit = Some(hit);
                }
            }
        }
        
        nearest_hit
    }
}