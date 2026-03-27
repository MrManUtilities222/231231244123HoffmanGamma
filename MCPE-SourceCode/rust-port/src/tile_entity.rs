/// Tile entity system ported from world/level/tile/entity/
/// TileEntity.h, ChestTileEntity.h, FurnaceTileEntity.h, SignTileEntity.h

use crate::item_instance::ItemInstance;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TileEntityType {
    Chest,
    Furnace,
    Sign,
    NetherReactor,
}

/// Base tile entity data
#[derive(Clone, Debug)]
pub struct TileEntity {
    pub entity_type: TileEntityType,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub data: TileEntityData,
}

#[derive(Clone, Debug)]
pub enum TileEntityData {
    Chest(ChestData),
    Furnace(FurnaceData),
    Sign(SignData),
    NetherReactor(NetherReactorData),
}

/// Chest: 27 inventory slots
#[derive(Clone, Debug)]
pub struct ChestData {
    pub items: [ItemInstance; 27],
}

impl ChestData {
    pub fn new() -> Self {
        Self {
            items: std::array::from_fn(|_| ItemInstance::empty()),
        }
    }

    pub fn get_item(&self, slot: usize) -> &ItemInstance {
        if slot < 27 { &self.items[slot] } else { &self.items[0] }
    }

    pub fn set_item(&mut self, slot: usize, item: ItemInstance) {
        if slot < 27 { self.items[slot] = item; }
    }
}

/// Furnace: input, fuel, output slots + smelting progress
#[derive(Clone, Debug)]
pub struct FurnaceData {
    pub input: ItemInstance,
    pub fuel: ItemInstance,
    pub output: ItemInstance,
    pub burn_time: i32,
    pub max_burn_time: i32,
    pub cook_time: i32,
}

impl FurnaceData {
    pub fn new() -> Self {
        Self {
            input: ItemInstance::empty(),
            fuel: ItemInstance::empty(),
            output: ItemInstance::empty(),
            burn_time: 0,
            max_burn_time: 0,
            cook_time: 0,
        }
    }

    pub fn is_burning(&self) -> bool { self.burn_time > 0 }

    pub fn tick(&mut self) {
        if self.burn_time > 0 {
            self.burn_time -= 1;
        }

        // If we have fuel and something to smelt
        if !self.input.is_null() && self.burn_time > 0 {
            self.cook_time += 1;
            if self.cook_time >= 200 {
                // Smelting complete
                self.cook_time = 0;
                // Output generation would go here
            }
        } else {
            self.cook_time = 0;
        }
    }

    /// Get fuel burn ticks for a given item
    pub fn get_burn_duration(item_id: i32) -> i32 {
        match item_id {
            263 => 1600,   // Coal
            5 => 300,      // Planks
            280 => 100,    // Stick
            17 => 300,     // Log
            6 => 100,      // Sapling
            10 | 11 => 20000, // Lava bucket
            _ => 0,
        }
    }

    /// Get smelting result for a raw item
    pub fn get_smelt_result(item_id: i32) -> Option<i32> {
        match item_id {
            14 => Some(266),   // Gold Ore -> Gold Ingot
            15 => Some(265),   // Iron Ore -> Iron Ingot
            16 => Some(263),   // Coal Ore -> Coal
            56 => Some(264),   // Diamond Ore -> Diamond
            21 => Some(351),   // Lapis Ore -> Lapis
            73 => Some(331),   // Redstone Ore -> Redstone
            1 => Some(4),      // Stone -> Cobblestone (reversed: cobble into stone)
            12 => Some(20),    // Sand -> Glass
            82 => Some(336),   // Clay -> Brick
            87 => Some(405),   // Netherrack -> Nether Brick item
            319 => Some(320),  // Raw Pork -> Cooked Pork
            363 => Some(364),  // Raw Beef -> Steak
            365 => Some(366),  // Raw Chicken -> Cooked Chicken
            349 => Some(350),  // Raw Fish -> Cooked Fish
            _ => None,
        }
    }
}

/// Sign: 4 lines of text
#[derive(Clone, Debug)]
pub struct SignData {
    pub lines: [String; 4],
}

impl SignData {
    pub fn new() -> Self {
        Self {
            lines: [String::new(), String::new(), String::new(), String::new()],
        }
    }

    pub fn set_line(&mut self, index: usize, text: &str) {
        if index < 4 {
            self.lines[index] = text.chars().take(15).collect();
        }
    }
}

/// Nether reactor data
#[derive(Clone, Debug)]
pub struct NetherReactorData {
    pub initialized: bool,
    pub progress: i32,
    pub max_progress: i32,
}

impl NetherReactorData {
    pub fn new() -> Self {
        Self { initialized: false, progress: 0, max_progress: 900 }
    }
}

impl TileEntity {
    pub fn new_chest(x: i32, y: i32, z: i32) -> Self {
        Self {
            entity_type: TileEntityType::Chest,
            x, y, z,
            data: TileEntityData::Chest(ChestData::new()),
        }
    }

    pub fn new_furnace(x: i32, y: i32, z: i32) -> Self {
        Self {
            entity_type: TileEntityType::Furnace,
            x, y, z,
            data: TileEntityData::Furnace(FurnaceData::new()),
        }
    }

    pub fn new_sign(x: i32, y: i32, z: i32) -> Self {
        Self {
            entity_type: TileEntityType::Sign,
            x, y, z,
            data: TileEntityData::Sign(SignData::new()),
        }
    }

    pub fn new_nether_reactor(x: i32, y: i32, z: i32) -> Self {
        Self {
            entity_type: TileEntityType::NetherReactor,
            x, y, z,
            data: TileEntityData::NetherReactor(NetherReactorData::new()),
        }
    }

    pub fn tick(&mut self) {
        match &mut self.data {
            TileEntityData::Furnace(f) => f.tick(),
            TileEntityData::NetherReactor(n) => {
                if n.initialized {
                    n.progress += 1;
                    if n.progress >= n.max_progress { n.initialized = false; }
                }
            }
            _ => {}
        }
    }
}
