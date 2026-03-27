use crate::material::Material;
use crate::aabb::Aabb;

#[derive(Clone, Debug)]
pub struct SoundType {
    pub volume: f32,
    pub pitch: f32,
    pub break_sound: String,
    pub step_sound: String,
}

impl SoundType {
    pub fn new(name: &str, volume: f32, pitch: f32) -> Self {
        Self {
            volume,
            pitch,
            break_sound: format!("step.{}", name),
            step_sound: format!("step.{}", name),
        }
    }

    pub fn with_break_sound(name: &str, break_sound: &str, volume: f32, pitch: f32) -> Self {
        Self {
            volume,
            pitch,
            step_sound: format!("step.{}", name),
            break_sound: break_sound.to_string(),
        }
    }
}

// Static sound types
lazy_static::lazy_static! {
    pub static ref SOUND_NORMAL: SoundType = SoundType::new("stone", 1.0, 1.0);
    pub static ref SOUND_WOOD: SoundType = SoundType::new("wood", 1.0, 1.0);
    pub static ref SOUND_GRAVEL: SoundType = SoundType::new("gravel", 1.0, 1.0);
    pub static ref SOUND_GRASS: SoundType = SoundType::new("grass", 1.0, 1.0);
    pub static ref SOUND_STONE: SoundType = SoundType::new("stone", 1.0, 1.0);
    pub static ref SOUND_METAL: SoundType = SoundType::new("metal", 1.0, 1.0);
    pub static ref SOUND_GLASS: SoundType = SoundType::new("glass", 1.0, 1.0);
    pub static ref SOUND_CLOTH: SoundType = SoundType::new("cloth", 1.0, 1.0);
    pub static ref SOUND_SAND: SoundType = SoundType::new("sand", 1.0, 1.0);
    pub static ref SOUND_SILENT: SoundType = SoundType::new("silent", 0.0, 1.0);
}

pub const SHAPE_INVISIBLE: i32 = -1;
pub const SHAPE_BLOCK: i32 = 0;
// Add more shapes as needed

#[derive(Clone, Debug)]
pub struct Tile {
    pub id: i32,
    pub material: Material,
    pub sound_type: SoundType,
    pub shape: i32,
    pub texture: i32,
    pub light_emission: i32,
    pub light_block: i32,
    pub hardness: f32,
    pub blast_resistance: f32,
    pub name: String,
    pub xx0: f32,
    pub yy0: f32,
    pub zz0: f32,
    pub xx1: f32,
    pub yy1: f32,
    pub zz1: f32,
    pub friction: f32,
    pub gravity: f32,
    pub category: i32,
}

impl Tile {
    pub fn new(id: i32, material: Material, sound_type: SoundType, name: &str) -> Self {
        Self {
            id,
            material,
            sound_type,
            shape: SHAPE_BLOCK,
            texture: 0,
            light_emission: 0,
            light_block: 15,
            hardness: 1.0,
            blast_resistance: 1.0,
            name: name.to_string(),
            xx0: 0.0,
            yy0: 0.0,
            zz0: 0.0,
            xx1: 1.0,
            yy1: 1.0,
            zz1: 1.0,
            friction: 0.6,
            gravity: 1.0,
            category: 0,
        }
    }

    pub fn set_shape(mut self, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> Self {
        self.xx0 = x0;
        self.yy0 = y0;
        self.zz0 = z0;
        self.xx1 = x1;
        self.yy1 = y1;
        self.zz1 = z1;
        self
    }

    pub fn with_hardness(mut self, hardness: f32) -> Self {
        self.hardness = hardness;
        // Basic derivation of blast resistance from hardness if not explicitly set
        self.blast_resistance = hardness * 5.0; 
        self
    }

    pub fn with_blast_resistance(mut self, res: f32) -> Self {
        self.blast_resistance = res;
        self
    }

    pub fn get_aabb(&self, x: i32, y: i32, z: i32) -> Aabb {
        Aabb::new(
            x as f32 + self.xx0,
            y as f32 + self.yy0,
            z as f32 + self.zz0,
            x as f32 + self.xx1,
            y as f32 + self.yy1,
            z as f32 + self.zz1,
        )
    }

    // Add more methods
}

// Static tiles — Full MCPE Block Registry ported from Tile.h / initTiles()
lazy_static::lazy_static! {
    pub static ref AIR: Tile = Tile::new(0, *crate::material::AIR, SOUND_NORMAL.clone(), "air").set_shape(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    pub static ref STONE: Tile = Tile::new(1, *crate::material::STONE, SOUND_STONE.clone(), "stone").with_hardness(1.5).with_blast_resistance(10.0);
    pub static ref GRASS: Tile = Tile::new(2, *crate::material::DIRT, SOUND_GRASS.clone(), "grass").with_hardness(0.6);
    pub static ref DIRT: Tile = Tile::new(3, *crate::material::DIRT, SOUND_GRAVEL.clone(), "dirt").with_hardness(0.5);
    pub static ref STONE_BRICK: Tile = Tile::new(4, *crate::material::STONE, SOUND_STONE.clone(), "stoneBrick").with_hardness(2.0).with_blast_resistance(10.0);
    pub static ref WOOD: Tile = Tile::new(5, *crate::material::WOOD, SOUND_WOOD.clone(), "wood").with_hardness(2.0).with_blast_resistance(5.0);
    pub static ref SAPLING: Tile = Tile::new(6, *crate::material::PLANT, SOUND_GRASS.clone(), "sapling").with_hardness(0.0);
    pub static ref BEDROCK: Tile = Tile::new(7, *crate::material::STONE, SOUND_STONE.clone(), "bedrock").with_hardness(-1.0).with_blast_resistance(6000000.0);
    pub static ref WATER: Tile = Tile::new(8, *crate::material::WATER, SOUND_NORMAL.clone(), "water").with_hardness(100.0);
    pub static ref CALM_WATER: Tile = Tile::new(9, *crate::material::WATER, SOUND_NORMAL.clone(), "calmWater").with_hardness(100.0);
    pub static ref LAVA: Tile = Tile::new(10, *crate::material::LAVA, SOUND_NORMAL.clone(), "lava").with_hardness(100.0);
    pub static ref CALM_LAVA: Tile = Tile::new(11, *crate::material::LAVA, SOUND_NORMAL.clone(), "calmLava").with_hardness(100.0);
    pub static ref SAND: Tile = Tile::new(12, *crate::material::SAND, SOUND_SAND.clone(), "sand").with_hardness(0.5);
    pub static ref GRAVEL: Tile = Tile::new(13, *crate::material::SAND, SOUND_GRAVEL.clone(), "gravel").with_hardness(0.6);
    pub static ref GOLD_ORE: Tile = Tile::new(14, *crate::material::STONE, SOUND_STONE.clone(), "goldOre").with_hardness(3.0).with_blast_resistance(5.0);
    pub static ref IRON_ORE: Tile = Tile::new(15, *crate::material::STONE, SOUND_STONE.clone(), "ironOre").with_hardness(3.0).with_blast_resistance(5.0);
    pub static ref COAL_ORE: Tile = Tile::new(16, *crate::material::STONE, SOUND_STONE.clone(), "coalOre").with_hardness(3.0).with_blast_resistance(5.0);
    pub static ref LOG: Tile = Tile::new(17, *crate::material::WOOD, SOUND_WOOD.clone(), "log").with_hardness(2.0);
    pub static ref LEAVES: Tile = Tile::new(18, *crate::material::LEAVES, SOUND_GRASS.clone(), "leaves").with_hardness(0.2);
    pub static ref SPONGE: Tile = Tile::new(19, *crate::material::SPONGE, SOUND_GRASS.clone(), "sponge").with_hardness(0.6);
    pub static ref GLASS: Tile = Tile::new(20, *crate::material::GLASS, SOUND_GLASS.clone(), "glass").with_hardness(0.3);
    pub static ref LAPIS_ORE: Tile = Tile::new(21, *crate::material::STONE, SOUND_STONE.clone(), "lapisOre").with_hardness(3.0).with_blast_resistance(5.0);
    pub static ref LAPIS_BLOCK: Tile = Tile::new(22, *crate::material::METAL, SOUND_STONE.clone(), "lapisBlock").with_hardness(3.0).with_blast_resistance(5.0);
    pub static ref SANDSTONE: Tile = Tile::new(24, *crate::material::STONE, SOUND_STONE.clone(), "sandStone").with_hardness(0.8);
    pub static ref BED: Tile = Tile::new(26, *crate::material::CLOTH, SOUND_WOOD.clone(), "bed").with_hardness(0.2);
    pub static ref TALLGRASS: Tile = Tile::new(31, *crate::material::PLANT, SOUND_GRASS.clone(), "tallgrass").with_hardness(0.0);
    pub static ref CLOTH: Tile = Tile::new(35, *crate::material::CLOTH, SOUND_CLOTH.clone(), "cloth").with_hardness(0.8);
    pub static ref FLOWER: Tile = Tile::new(37, *crate::material::PLANT, SOUND_GRASS.clone(), "flower").with_hardness(0.0);
    pub static ref ROSE: Tile = Tile::new(38, *crate::material::PLANT, SOUND_GRASS.clone(), "rose").with_hardness(0.0);
    pub static ref MUSHROOM1: Tile = Tile::new(39, *crate::material::PLANT, SOUND_GRASS.clone(), "mushroom1").with_hardness(0.0);
    pub static ref MUSHROOM2: Tile = Tile::new(40, *crate::material::PLANT, SOUND_GRASS.clone(), "mushroom2").with_hardness(0.0);
    pub static ref GOLD_BLOCK: Tile = Tile::new(41, *crate::material::METAL, SOUND_METAL.clone(), "goldBlock").with_hardness(3.0).with_blast_resistance(10.0);
    pub static ref IRON_BLOCK: Tile = Tile::new(42, *crate::material::METAL, SOUND_METAL.clone(), "ironBlock").with_hardness(5.0).with_blast_resistance(10.0);
    pub static ref STONE_SLAB: Tile = Tile::new(43, *crate::material::STONE, SOUND_STONE.clone(), "stoneSlab").with_hardness(2.0).with_blast_resistance(10.0);
    pub static ref STONE_SLAB_HALF: Tile = Tile::new(44, *crate::material::STONE, SOUND_STONE.clone(), "stoneSlabHalf").with_hardness(2.0).with_blast_resistance(10.0);
    pub static ref RED_BRICK: Tile = Tile::new(45, *crate::material::STONE, SOUND_STONE.clone(), "redBrick").with_hardness(2.0).with_blast_resistance(10.0);
    pub static ref TNT: Tile = Tile::new(46, *crate::material::EXPLOSIVE, SOUND_GRASS.clone(), "tnt").with_hardness(0.0);
    pub static ref BOOKSHELF: Tile = Tile::new(47, *crate::material::WOOD, SOUND_WOOD.clone(), "bookshelf").with_hardness(1.5);
    pub static ref MOSS_STONE: Tile = Tile::new(48, *crate::material::STONE, SOUND_STONE.clone(), "mossStone").with_hardness(2.0).with_blast_resistance(10.0);
    pub static ref OBSIDIAN: Tile = Tile::new(49, *crate::material::STONE, SOUND_STONE.clone(), "obsidian").with_hardness(50.0).with_blast_resistance(2000.0);
    pub static ref TORCH: Tile = Tile::new(50, *crate::material::DECORATION, SOUND_WOOD.clone(), "torch").with_hardness(0.0);
    pub static ref STAIRS_WOOD: Tile = Tile::new(53, *crate::material::WOOD, SOUND_WOOD.clone(), "stairsWood").with_hardness(2.0);
    pub static ref CHEST: Tile = Tile::new(54, *crate::material::WOOD, SOUND_WOOD.clone(), "chest").with_hardness(2.5);
    pub static ref EMERALD_ORE: Tile = Tile::new(56, *crate::material::STONE, SOUND_STONE.clone(), "emeraldOre").with_hardness(3.0).with_blast_resistance(5.0);
    pub static ref EMERALD_BLOCK: Tile = Tile::new(57, *crate::material::METAL, SOUND_METAL.clone(), "emeraldBlock").with_hardness(5.0).with_blast_resistance(10.0);
    pub static ref WORKBENCH: Tile = Tile::new(58, *crate::material::WOOD, SOUND_WOOD.clone(), "workBench").with_hardness(2.5);
    pub static ref CROPS: Tile = Tile::new(59, *crate::material::PLANT, SOUND_GRASS.clone(), "crops").with_hardness(0.0);
    pub static ref FARMLAND: Tile = Tile::new(60, *crate::material::DIRT, SOUND_GRAVEL.clone(), "farmland").with_hardness(0.6);
    pub static ref FURNACE: Tile = Tile::new(61, *crate::material::STONE, SOUND_STONE.clone(), "furnace").with_hardness(3.5);
    pub static ref FURNACE_LIT: Tile = Tile::new(62, *crate::material::STONE, SOUND_STONE.clone(), "furnaceLit").with_hardness(3.5);
    pub static ref SIGN: Tile = Tile::new(63, *crate::material::WOOD, SOUND_WOOD.clone(), "sign").with_hardness(1.0);
    pub static ref DOOR_WOOD: Tile = Tile::new(64, *crate::material::WOOD, SOUND_WOOD.clone(), "doorWood").with_hardness(3.0);
    pub static ref LADDER: Tile = Tile::new(65, *crate::material::DECORATION, SOUND_WOOD.clone(), "ladder").with_hardness(0.4);
    pub static ref STAIRS_STONE: Tile = Tile::new(67, *crate::material::STONE, SOUND_STONE.clone(), "stairsStone").with_hardness(2.0);
    pub static ref DOOR_IRON: Tile = Tile::new(71, *crate::material::METAL, SOUND_METAL.clone(), "doorIron").with_hardness(5.0);
    pub static ref REDSTONE_ORE: Tile = Tile::new(73, *crate::material::STONE, SOUND_STONE.clone(), "redStoneOre").with_hardness(3.0).with_blast_resistance(5.0);
    pub static ref TOP_SNOW: Tile = Tile::new(78, *crate::material::TOP_SNOW, SOUND_CLOTH.clone(), "topSnow").with_hardness(0.1);
    pub static ref ICE: Tile = Tile::new(79, *crate::material::ICE, SOUND_GLASS.clone(), "ice").with_hardness(0.5);
    pub static ref SNOW: Tile = Tile::new(80, *crate::material::SNOW, SOUND_CLOTH.clone(), "snow").with_hardness(0.2);
    pub static ref CACTUS: Tile = Tile::new(81, *crate::material::CACTUS, SOUND_CLOTH.clone(), "cactus").with_hardness(0.4);
    pub static ref CLAY: Tile = Tile::new(82, *crate::material::CLAY, SOUND_GRAVEL.clone(), "clay").with_hardness(0.6);
    pub static ref REEDS: Tile = Tile::new(83, *crate::material::PLANT, SOUND_GRASS.clone(), "reeds").with_hardness(0.0);
    pub static ref FENCE: Tile = Tile::new(85, *crate::material::WOOD, SOUND_WOOD.clone(), "fence").with_hardness(2.0).with_blast_resistance(5.0);
    pub static ref PUMPKIN: Tile = Tile::new(86, *crate::material::VEGETABLE, SOUND_WOOD.clone(), "pumpkin").with_hardness(1.0);
    pub static ref NETHERRACK: Tile = Tile::new(87, *crate::material::STONE, SOUND_STONE.clone(), "netherrack").with_hardness(0.4);
    pub static ref SOUL_SAND: Tile = Tile::new(88, *crate::material::SAND, SOUND_SAND.clone(), "soulSand").with_hardness(0.5);
    pub static ref GLOWSTONE: Tile = Tile::new(89, *crate::material::GLASS, SOUND_GLASS.clone(), "glowstone").with_hardness(0.3);
    pub static ref LIT_PUMPKIN: Tile = Tile::new(91, *crate::material::VEGETABLE, SOUND_WOOD.clone(), "litPumpkin").with_hardness(1.0);
    pub static ref CAKE: Tile = Tile::new(92, *crate::material::CAKE, SOUND_CLOTH.clone(), "cake").with_hardness(0.5);
    pub static ref TRAPDOOR: Tile = Tile::new(96, *crate::material::WOOD, SOUND_WOOD.clone(), "trapdoor").with_hardness(3.0);
    pub static ref STONE_BRICK_SMOOTH: Tile = Tile::new(98, *crate::material::STONE, SOUND_STONE.clone(), "stoneBrickSmooth").with_hardness(1.5).with_blast_resistance(10.0);
    pub static ref MELON: Tile = Tile::new(103, *crate::material::VEGETABLE, SOUND_WOOD.clone(), "melon").with_hardness(1.0);
    pub static ref FENCE_GATE: Tile = Tile::new(107, *crate::material::WOOD, SOUND_WOOD.clone(), "fenceGate").with_hardness(2.0).with_blast_resistance(5.0);
    pub static ref STAIRS_BRICK: Tile = Tile::new(108, *crate::material::STONE, SOUND_STONE.clone(), "stairsBrick").with_hardness(2.0).with_blast_resistance(10.0);
    pub static ref STAIRS_STONE_BRICK: Tile = Tile::new(109, *crate::material::STONE, SOUND_STONE.clone(), "stairsStoneBrick").with_hardness(1.5).with_blast_resistance(10.0);
    pub static ref NETHER_BRICK: Tile = Tile::new(112, *crate::material::STONE, SOUND_STONE.clone(), "netherBrick").with_hardness(2.0).with_blast_resistance(10.0);
    pub static ref STAIRS_NETHER_BRICK: Tile = Tile::new(114, *crate::material::STONE, SOUND_STONE.clone(), "stairsNetherBrick").with_hardness(2.0).with_blast_resistance(10.0);
    pub static ref STAIRS_SANDSTONE: Tile = Tile::new(128, *crate::material::STONE, SOUND_STONE.clone(), "stairsSandstone").with_hardness(0.8);
    pub static ref QUARTZ_BLOCK: Tile = Tile::new(155, *crate::material::STONE, SOUND_STONE.clone(), "quartzBlock").with_hardness(0.8);
    pub static ref STAIRS_QUARTZ: Tile = Tile::new(156, *crate::material::STONE, SOUND_STONE.clone(), "stairsQuartz").with_hardness(0.8);
    pub static ref INVISIBLE_BEDROCK: Tile = Tile::new(95, *crate::material::STONE, SOUND_STONE.clone(), "invisibleBedrock").with_hardness(-1.0).with_blast_resistance(6000000.0);
    pub static ref GLOWING_OBSIDIAN: Tile = Tile::new(246, *crate::material::STONE, SOUND_STONE.clone(), "glowingObsidian").with_hardness(50.0).with_blast_resistance(2000.0);
    pub static ref NETHER_REACTOR: Tile = Tile::new(247, *crate::material::METAL, SOUND_METAL.clone(), "netherReactor").with_hardness(3.0).with_blast_resistance(10.0);
}

/// Whether a block ID is considered solid for collision purposes.
pub fn is_solid(id: i32) -> bool {
    matches!(id,
        1..=5 | 7 | 12..=22 | 24 | 35 | 41..=49 | 53..=54 | 56..=62 | 67 | 71 | 73 | 79..=82 | 85..=89 | 91..=92 | 96 | 98 | 103 | 107..=109 | 112 | 114 | 128 | 155..=156 | 246..=247
    )
}

/// Whether a block ID is transparent (should render faces behind it).
pub fn is_transparent(id: i32) -> bool {
    matches!(id, 0 | 6 | 8..=11 | 18 | 20 | 26 | 31 | 37..=40 | 50 | 59 | 63..=66 | 78 | 81 | 83)
}

pub fn init_tiles() {
    // Force lazy_static initialization
    let _ = AIR.id;
    let _ = STONE.id;
}