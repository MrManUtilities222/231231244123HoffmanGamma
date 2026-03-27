use crate::item_instance::ItemInstance;
use std::collections::HashMap;

/// A single crafting recipe (shaped grid).
#[derive(Clone, Debug)]
pub struct Recipe {
    pub width: i32,
    pub height: i32,
    pub ingredients: Vec<i32>, // item/tile IDs in row-major order (0 = empty)
    pub result: ItemInstance,
}

impl Recipe {
    pub fn new(width: i32, height: i32, ingredients: Vec<i32>, result: ItemInstance) -> Self {
        Self { width, height, ingredients, result }
    }

    pub fn matches(&self, grid: &[i32], grid_w: i32, grid_h: i32) -> bool {
        if self.width > grid_w || self.height > grid_h { return false; }
        
        // Try every possible offset in the grid
        for ox in 0..=(grid_w - self.width) {
            for oy in 0..=(grid_h - self.height) {
                let mut ok = true;
                for rx in 0..self.width {
                    for ry in 0..self.height {
                        let ri = (ry * self.width + rx) as usize;
                        let gi = ((oy + ry) * grid_w + (ox + rx)) as usize;
                        if self.ingredients[ri] != grid[gi] {
                            ok = false;
                            break;
                        }
                    }
                    if !ok { break; }
                }
                if ok { return true; }
            }
        }
        false
    }
}

/// Furnace smelting recipe.
#[derive(Clone, Debug)]
pub struct FurnaceRecipe {
    pub input_id: i32,
    pub result: ItemInstance,
}

/// The global recipe registry.
pub struct Recipes {
    pub crafting: Vec<Recipe>,
    pub furnace: Vec<FurnaceRecipe>,
}

impl Default for Recipes {
    fn default() -> Self {
        Self::new()
    }
}

impl Recipes {
    pub fn new() -> Self {
        let mut r = Self {
            crafting: Vec::new(),
            furnace: Vec::new(),
        };
        r.init_recipes();
        r
    }

    fn add_shaped(&mut self, w: i32, h: i32, ing: Vec<i32>, result_id: i32, result_count: i32, result_aux: i32) {
        self.crafting.push(Recipe::new(w, h, ing, ItemInstance::from_id(result_id, result_count, result_aux)));
    }

    fn add_smelt(&mut self, input: i32, output_id: i32) {
        self.furnace.push(FurnaceRecipe {
            input_id: input,
            result: ItemInstance::from_id(output_id, 1, 0),
        });
    }

    fn init_recipes(&mut self) {
        use crate::tile;

        // ====== Structure Recipes ======
        // Planks: 1 Log -> 4 Planks
        self.add_shaped(1, 1, vec![tile::LOG.id], tile::WOOD.id, 4, 0);

        // Sticks: 2 Planks -> 4 Sticks (Item ID 280)
        self.add_shaped(1, 2, vec![tile::WOOD.id, tile::WOOD.id], 280, 4, 0);

        // Workbench: 4 Planks
        self.add_shaped(2, 2, vec![tile::WOOD.id, tile::WOOD.id, tile::WOOD.id, tile::WOOD.id], tile::WORKBENCH.id, 1, 0);

        // Furnace: 8 Cobblestones
        self.add_shaped(3, 3, vec![
            tile::STONE_BRICK.id, tile::STONE_BRICK.id, tile::STONE_BRICK.id,
            tile::STONE_BRICK.id, 0, tile::STONE_BRICK.id,
            tile::STONE_BRICK.id, tile::STONE_BRICK.id, tile::STONE_BRICK.id
        ], tile::FURNACE.id, 1, 0);

        // Chest: 8 Planks
        self.add_shaped(3, 3, vec![
            tile::WOOD.id, tile::WOOD.id, tile::WOOD.id,
            tile::WOOD.id, 0, tile::WOOD.id,
            tile::WOOD.id, tile::WOOD.id, tile::WOOD.id
        ], tile::CHEST.id, 1, 0);

        // Fence: 6 Sticks
        self.add_shaped(3, 2, vec![280, 280, 280, 280, 280, 280], tile::FENCE.id, 2, 0);

        // Ladder: 7 Sticks
        self.add_shaped(3, 3, vec![
            280, 0, 280,
            280, 280, 280,
            280, 0, 280
        ], tile::LADDER.id, 3, 0);

        // ====== Tool Recipes ======
        // Wooden Pickaxe (Item 270)
        self.add_shaped(3, 3, vec![
            tile::WOOD.id, tile::WOOD.id, tile::WOOD.id,
            0, 280, 0,
            0, 280, 0
        ], 270, 1, 0);

        // Stone Pickaxe (Item 274)
        self.add_shaped(3, 3, vec![
            tile::STONE_BRICK.id, tile::STONE_BRICK.id, tile::STONE_BRICK.id,
            0, 280, 0,
            0, 280, 0
        ], 274, 1, 0);

        // Wooden Sword (Item 268)
        self.add_shaped(1, 3, vec![tile::WOOD.id, tile::WOOD.id, 280], 268, 1, 0);

        // Stone Sword (Item 272)
        self.add_shaped(1, 3, vec![tile::STONE_BRICK.id, tile::STONE_BRICK.id, 280], 272, 1, 0);

        // ====== Furnace Recipes ======
        self.add_smelt(tile::IRON_ORE.id, 265);   // Iron Ingot
        self.add_smelt(tile::GOLD_ORE.id, 266);   // Gold Ingot
        self.add_smelt(tile::SAND.id, tile::GLASS.id);
        self.add_smelt(tile::COAL_ORE.id, 263);   // Coal
        self.add_smelt(tile::LOG.id, 263);         // Charcoal
        self.add_smelt(319, 320);                  // Raw Pork -> Cooked Pork
        self.add_smelt(363, 364);                  // Raw Beef -> Steak
        self.add_smelt(365, 366);                  // Raw Chicken -> Cooked Chicken
    }

    pub fn find_crafting_match(&self, grid: &[i32], w: i32, h: i32) -> Option<&ItemInstance> {
        for recipe in &self.crafting {
            if recipe.matches(grid, w, h) {
                return Some(&recipe.result);
            }
        }
        None
    }

    pub fn find_furnace_result(&self, input_id: i32) -> Option<&ItemInstance> {
        for recipe in &self.furnace {
            if recipe.input_id == input_id {
                return Some(&recipe.result);
            }
        }
        None
    }
}
