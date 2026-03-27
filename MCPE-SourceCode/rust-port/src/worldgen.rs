/// World generation system ported from world/level/levelgen/
/// RandomLevelSource.h/cpp, CanyonFeature.h, DungeonFeature.h,
/// LargeCaveFeature.h, TownFeature.h, synth/PerlinNoise.h

use crate::level::Level;
use crate::noise::PerlinNoise;

/// Ore generation parameters
struct OreConfig {
    tile_id: u8,
    count: i32,           // veins per chunk
    size: i32,            // blocks per vein
    min_height: i32,
    max_height: i32,
}

const ORE_CONFIGS: &[OreConfig] = &[
    OreConfig { tile_id: 16,  count: 20, size: 16, min_height: 0,  max_height: 128 }, // Coal
    OreConfig { tile_id: 15,  count: 20, size: 8,  min_height: 0,  max_height: 64 },  // Iron
    OreConfig { tile_id: 14,  count: 2,  size: 8,  min_height: 0,  max_height: 32 },  // Gold
    OreConfig { tile_id: 56,  count: 1,  size: 7,  min_height: 0,  max_height: 16 },  // Diamond
    OreConfig { tile_id: 73,  count: 8,  size: 7,  min_height: 0,  max_height: 16 },  // Redstone
    OreConfig { tile_id: 21,  count: 1,  size: 6,  min_height: 0,  max_height: 32 },  // Lapis
];

/// Cave carver — simplified version of LargeCaveFeature
pub fn carve_caves(level: &mut Level, chunk_x: i32, chunk_z: i32, seed: u64) {
    let mut rng = seed.wrapping_add((chunk_x as u64) * 341873128712 + (chunk_z as u64) * 132897987541);

    let cave_count = ((rng % 15) as i32).max(0);
    rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);

    for _ in 0..cave_count {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let cx = chunk_x * 16 + (rng % 16) as i32;
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let cy = (rng % 60) as i32 + 10;
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let cz = chunk_z * 16 + (rng % 16) as i32;

        let radius = ((rng % 3) as f32) + 2.0;
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);

        let ri = radius as i32;
        for dx in -ri..=ri {
            for dy in -ri..=ri {
                for dz in -ri..=ri {
                    if (dx*dx + dy*dy + dz*dz) as f32 <= radius * radius {
                        let bx = cx + dx;
                        let by = cy + dy;
                        let bz = cz + dz;
                        if by > 1 && by < 126 {
                            let tile = level.get_tile(bx, by, bz);
                            if tile == 1 || tile == 3 || tile == 2 { // Stone, dirt, grass
                                level.set_tile(bx, by, bz, 0); // Air
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Place ores in a chunk
pub fn generate_ores(level: &mut Level, chunk_x: i32, chunk_z: i32, seed: u64) {
    let mut rng = seed.wrapping_add((chunk_x as u64) * 498293748 + (chunk_z as u64) * 8923741);

    for ore in ORE_CONFIGS {
        for _ in 0..ore.count {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let x = chunk_x * 16 + (rng % 16) as i32;
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let y = (rng % (ore.max_height as u64 - ore.min_height as u64)) as i32 + ore.min_height;
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let z = chunk_z * 16 + (rng % 16) as i32;

            // Place a small blob
            let half = (ore.size as f32).sqrt() as i32;
            for dx in -half..=half {
                for dy in -half..=half {
                    for dz in -half..=half {
                        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                        if (rng % 3) == 0 { continue; } // randomize blob shape
                        let bx = x + dx;
                        let by = y + dy;
                        let bz = z + dz;
                        if by >= ore.min_height && by <= ore.max_height {
                            if level.get_tile(bx, by, bz) == 1 { // Only replace stone
                                level.set_tile(bx, by, bz, ore.tile_id);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generate a dungeon (mob spawner + chests)
pub fn generate_dungeon(level: &mut Level, x: i32, y: i32, z: i32, seed: u64) {
    let mut rng = seed.wrapping_add(x as u64 * 3 + z as u64 * 7 + y as u64 * 13);

    let sx = 3;
    let sy = 3;
    let sz = 3;

    // Hollow out area
    for dx in -sx..=sx {
        for dy in 0..sy {
            for dz in -sz..=sz {
                let bx = x + dx;
                let by = y + dy;
                let bz = z + dz;
                if dx == -sx || dx == sx || dz == -sz || dz == sz || dy == 0 || dy == sy - 1 {
                    level.set_tile(bx, by, bz, 48); // Mossy cobblestone
                } else {
                    level.set_tile(bx, by, bz, 0); // Air
                }
            }
        }
    }

    // Place spawner at center
    level.set_tile(x, y + 1, z, 52); // Mob spawner tile
}

/// Place trees on the surface
pub fn generate_trees(level: &mut Level, chunk_x: i32, chunk_z: i32, count: i32, seed: u64) {
    let mut rng = seed.wrapping_add((chunk_x as u64) * 123456 + (chunk_z as u64) * 654321);

    for _ in 0..count {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let tx = chunk_x * 16 + (rng % 16) as i32;
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let tz = chunk_z * 16 + (rng % 16) as i32;

        // Find surface
        for y in (1..126).rev() {
            let tile = level.get_tile(tx, y, tz);
            if tile == 2 || tile == 3 { // Grass or dirt
                let tree_h = (rng % 3) as i32 + 4;
                rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);

                // Place trunk
                for ty in 1..=tree_h {
                    level.set_tile(tx, y + ty, tz, 17); // Log
                }

                // Place leaves
                for ly in (tree_h - 2)..=(tree_h + 1) {
                    let radius = if ly >= tree_h { 1 } else { 2 };
                    for lx in -radius..=radius {
                        for lz in -radius..=radius {
                            if lx == 0 && lz == 0 && ly <= tree_h { continue; } // trunk pos
                            if level.get_tile(tx + lx, y + ly, tz + lz) == 0 {
                                level.set_tile(tx + lx, y + ly, tz + lz, 18); // Leaves
                            }
                        }
                    }
                }
                break;
            } else if tile != 0 {
                break; // Hit non-air/non-surface
            }
        }
    }
}

/// Post-process a generated chunk: caves, ores, trees, dungeons
pub fn post_process_chunk(level: &mut Level, chunk_x: i32, chunk_z: i32, seed: u64) {
    carve_caves(level, chunk_x, chunk_z, seed);
    generate_ores(level, chunk_x, chunk_z, seed);
    generate_trees(level, chunk_x, chunk_z, 3, seed);

    // Small chance for dungeon
    let dungeon_rng = seed.wrapping_add((chunk_x as u64) * 99 + (chunk_z as u64) * 77);
    if dungeon_rng % 50 == 0 {
        let dx = chunk_x * 16 + (dungeon_rng % 16) as i32;
        let dz = chunk_z * 16 + ((dungeon_rng >> 8) % 16) as i32;
        let dy = ((dungeon_rng >> 16) % 40) as i32 + 10;
        generate_dungeon(level, dx, dy, dz, seed);
    }
}
