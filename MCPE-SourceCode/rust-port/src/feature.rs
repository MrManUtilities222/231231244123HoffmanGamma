use crate::level::Level;
use crate::tile;
use rand::Rng;

pub struct TreeFeature;

impl Default for TreeFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeFeature {
    pub fn new() -> Self {
        Self {}
    }

    pub fn place(&self, level: &mut Level, x: i32, y: i32, z: i32, rng: &mut impl Rng) -> bool {
        let tree_height = rng.gen_range(4..=6);

        let mut free = true;
        if y < 1 || y + tree_height + 1 > 128 { // Max depth is 128
            return false;
        }

        // Verify that there is space for the tree
        for yy in y..=(y + 1 + tree_height) {
            let mut r = 1;
            if yy == y {
                r = 0;
            }
            if yy >= y + 1 + tree_height - 2 {
                r = 2;
            }
            for xx in (x - r)..=(x + r) {
                if !free { break; }
                for zz in (z - r)..=(z + r) {
                    if !free { break; }
                    if yy >= 0 && yy < 128 {
                        let tt = level.get_tile(xx, yy, zz);
                        if tt != tile::AIR.id && tt != tile::LEAVES.id {
                            free = false;
                        }
                    } else {
                        free = false;
                    }
                }
            }
        }

        if !free {
            return false;
        }

        // Must grow on grass or dirt
        let below_tile = level.get_tile(x, y - 1, z);
        if (below_tile != tile::GRASS.id && below_tile != tile::DIRT.id) || y >= 128 - tree_height - 1 {
            return false;
        }

        // Convert the grass block underneath into dirt
        level.set_tile(x, y - 1, z, tile::DIRT.id);

        // Generate the leaves canopy
        for yy in (y - 3 + tree_height)..=(y + tree_height) {
            let yo = yy - (y + tree_height);
            let offs = 1 - yo / 2;
            for xx in (x - offs)..=(x + offs) {
                let xo = xx - x;
                for zz in (z - offs)..=(z + offs) {
                    let zo = zz - z;
                    // Leave corners empty occasionally for organic shape
                    if xo.abs() == offs && zo.abs() == offs && (rng.gen_range(0..2) == 0 || yo == 0) {
                        continue;
                    }
                    
                    let t = level.get_tile(xx, yy, zz);
                    if t == tile::AIR.id || t == tile::LEAVES.id {
                        level.set_tile(xx, yy, zz, tile::LEAVES.id);
                    }
                }
            }
        }

        // Generate the log trunk
        for hh in 0..tree_height {
            let t = level.get_tile(x, y + hh, z);
            if t == tile::AIR.id || t == tile::LEAVES.id {
                level.set_tile(x, y + hh, z, tile::LOG.id);
            }
        }

        true
    }
}

pub struct CactusFeature;

impl Default for CactusFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl CactusFeature {
    pub fn new() -> Self {
        Self {}
    }

    pub fn place(&self, level: &mut Level, x: i32, y: i32, z: i32, rng: &mut impl Rng) -> bool {
        for _ in 0..10 {
            let x2 = x + rng.gen_range(0..=7) - rng.gen_range(0..=7);
            let y2 = y + rng.gen_range(0..=3) - rng.gen_range(0..=3);
            let z2 = z + rng.gen_range(0..=7) - rng.gen_range(0..=7);
            
            if level.get_tile(x2, y2, z2) == tile::AIR.id {
                let max_h = rng.gen_range(0..=2);
                let h = 1 + rng.gen_range(0..=max_h);
                for yy in 0..h {
                    // Check if cactus can survive
                    let below = level.get_tile(x2, y2 + yy - 1, z2);
                    if below == tile::SAND.id || below == tile::CACTUS.id {
                        // Cactus requires adjacent horizontal blocks to be AIR
                        let mut safe = true;
                        if level.get_tile(x2 - 1, y2 + yy, z2) != tile::AIR.id { safe = false; }
                        if level.get_tile(x2 + 1, y2 + yy, z2) != tile::AIR.id { safe = false; }
                        if level.get_tile(x2, y2 + yy, z2 - 1) != tile::AIR.id { safe = false; }
                        if level.get_tile(x2, y2 + yy, z2 + 1) != tile::AIR.id { safe = false; }
                        
                        if safe && y2 + yy < 128 {
                            level.set_tile(x2, y2 + yy, z2, tile::CACTUS.id);
                        }
                    }
                }
            }
        }
        
        true
    }
}
