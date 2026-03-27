/// Biome system ported from world/level/biome/
/// Biome.h, BiomeSource.h, ForestBiome.h, SwampBiome.h, etc.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BiomeType {
    RainForest,
    Swampland,
    SeasonalForest,
    Forest,
    Savanna,
    Shrubland,
    Taiga,
    Desert,
    Plains,
    IceDesert,
    Tundra,
}

#[derive(Clone, Debug)]
pub struct Biome {
    pub biome_type: BiomeType,
    pub name: &'static str,
    pub color: u32,
    pub top_material: u8,   // tile id for surface
    pub material: u8,       // tile id for subsurface
    pub leaf_color: u32,
    pub snow_covered: bool,
    pub temperature: f32,
    pub downfall: f32,
}

impl Biome {
    pub const fn new(
        biome_type: BiomeType, name: &'static str, color: u32,
        top: u8, mat: u8, leaf: u32, snow: bool, temp: f32, downfall: f32,
    ) -> Self {
        Self {
            biome_type, name, color,
            top_material: top, material: mat,
            leaf_color: leaf, snow_covered: snow,
            temperature: temp, downfall: downfall,
        }
    }
}

// Tile IDs from our tile.rs
const GRASS_ID: u8 = 2;
const DIRT_ID: u8 = 3;
const SAND_ID: u8 = 12;
const SNOW_ID: u8 = 80;

pub static BIOMES: &[Biome] = &[
    Biome::new(BiomeType::RainForest,     "Rainforest",      0x2C4205, GRASS_ID, DIRT_ID, 0x4E6E58, false, 1.0,  1.0),
    Biome::new(BiomeType::Swampland,      "Swampland",       0x2C4205, GRASS_ID, DIRT_ID, 0x4E6E58, false, 0.8,  0.9),
    Biome::new(BiomeType::SeasonalForest, "Seasonal Forest", 0x4E6E58, GRASS_ID, DIRT_ID, 0x4E6E58, false, 0.7,  0.8),
    Biome::new(BiomeType::Forest,         "Forest",          0x056621, GRASS_ID, DIRT_ID, 0x4E6E58, false, 0.7,  0.8),
    Biome::new(BiomeType::Savanna,        "Savanna",         0x79A637, GRASS_ID, DIRT_ID, 0xAEA42A, false, 0.9,  0.2),
    Biome::new(BiomeType::Shrubland,      "Shrubland",       0x79A637, GRASS_ID, DIRT_ID, 0xAEA42A, false, 0.6,  0.3),
    Biome::new(BiomeType::Taiga,          "Taiga",           0x0B6659, GRASS_ID, DIRT_ID, 0x68A464, true,  0.05, 0.8),
    Biome::new(BiomeType::Desert,         "Desert",          0xD2B482, SAND_ID,  SAND_ID, 0xAEA42A, false, 1.0,  0.0),
    Biome::new(BiomeType::Plains,         "Plains",          0xA0A037, GRASS_ID, DIRT_ID, 0xAEA42A, false, 0.5,  0.5),
    Biome::new(BiomeType::IceDesert,      "Ice Desert",      0xFFFFFF, SNOW_ID,  SAND_ID, 0xAEA42A, true,  0.0,  0.0),
    Biome::new(BiomeType::Tundra,         "Tundra",          0xC0D8C0, GRASS_ID, DIRT_ID, 0xAEA42A, true,  0.0,  0.5),
];

/// Get biome from temperature and downfall (Whittaker diagram), matching MCPE's Biome::getBiome
pub fn get_biome(temperature: f32, downfall: f32) -> &'static Biome {
    let temp = temperature.clamp(0.0, 1.0);
    let down = (downfall * temp).clamp(0.0, 1.0);

    if temp < 0.1 {
        if down < 0.2 { return &BIOMES[9]; } // IceDesert
        return &BIOMES[10]; // Tundra
    }
    if temp < 0.2 {
        return &BIOMES[6]; // Taiga
    }
    if down < 0.1 {
        return &BIOMES[7]; // Desert
    }
    if down < 0.3 {
        if temp < 0.5 { return &BIOMES[5]; } // Shrubland
        return &BIOMES[4]; // Savanna
    }
    if temp < 0.5 {
        return &BIOMES[8]; // Plains
    }
    if down < 0.6 {
        return &BIOMES[2]; // SeasonalForest
    }
    if temp < 0.8 {
        return &BIOMES[3]; // Forest
    }
    if down < 0.8 {
        return &BIOMES[1]; // Swampland
    }
    &BIOMES[0] // RainForest
}

/// Biome source: generates biome data for a region of chunks
pub struct BiomeSource {
    pub seed: i64,
    pub temperature_noise: crate::noise::PerlinNoise,
    pub humidity_noise: crate::noise::PerlinNoise,
}

impl BiomeSource {
    pub fn new(seed: i64) -> Self {
        Self {
            seed,
            temperature_noise: crate::noise::PerlinNoise::new(seed as u64),
            humidity_noise: crate::noise::PerlinNoise::new((seed + 1) as u64),
        }
    }

    pub fn get_biome_at(&self, x: i32, z: i32) -> &'static Biome {
        let temp = self.temperature_noise.get_value(x as f64 / 100.0, z as f64 / 100.0) as f32 * 0.5 + 0.5;
        let humid = self.humidity_noise.get_value(x as f64 / 100.0, z as f64 / 100.0) as f32 * 0.5 + 0.5;
        get_biome(temp, humid)
    }
}

/// Dimension system ported from world/level/dimension/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimensionType {
    Overworld,
    Nether,
}

pub struct Dimension {
    pub dimension_type: DimensionType,
    pub has_ceiling: bool,
    pub time_of_day: f32,
}

impl Dimension {
    pub fn overworld() -> Self {
        Self { dimension_type: DimensionType::Overworld, has_ceiling: false, time_of_day: 0.0 }
    }

    pub fn nether() -> Self {
        Self { dimension_type: DimensionType::Nether, has_ceiling: true, time_of_day: 0.5 }
    }

    pub fn tick(&mut self) {
        if self.dimension_type == DimensionType::Overworld {
            self.time_of_day += 1.0 / 24000.0;
            if self.time_of_day > 1.0 { self.time_of_day -= 1.0; }
        }
    }

    pub fn get_sky_color(&self) -> (f32, f32, f32) {
        match self.dimension_type {
            DimensionType::Overworld => {
                let t = self.time_of_day;
                if t < 0.25 || t > 0.75 {
                    // Day
                    (0.5, 0.7, 1.0)
                } else if t < 0.3 || t > 0.7 {
                    // Sunset/sunrise
                    (0.8, 0.5, 0.3)
                } else {
                    // Night
                    (0.02, 0.02, 0.08)
                }
            }
            DimensionType::Nether => (0.15, 0.05, 0.05),
        }
    }
}
