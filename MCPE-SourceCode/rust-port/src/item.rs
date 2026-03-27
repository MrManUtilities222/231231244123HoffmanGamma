/// Tool tier definitions ported from Item.h Tier class
#[derive(Clone, Debug)]
pub struct Tier {
    pub level: i32,
    pub uses: i32,
    pub speed: f32,
    pub damage: i32,
}

impl Tier {
    pub fn new(level: i32, uses: i32, speed: f32, damage: i32) -> Self {
        Self { level, uses, speed, damage }
    }
}

lazy_static::lazy_static! {
    pub static ref TIER_WOOD: Tier = Tier::new(0, 59, 2.0, 0);
    pub static ref TIER_STONE: Tier = Tier::new(1, 131, 4.0, 1);
    pub static ref TIER_IRON: Tier = Tier::new(2, 250, 6.0, 2);
    pub static ref TIER_EMERALD: Tier = Tier::new(3, 1561, 8.0, 3);
    pub static ref TIER_GOLD: Tier = Tier::new(0, 32, 12.0, 0);
}

pub const MAX_STACK_SIZE: i32 = 64;
pub const MAX_ITEMS: usize = 512;

/// Item categories
pub mod category {
    pub const CONSTRUCTION: i32 = 1;
    pub const NATURE: i32 = 2;
    pub const EQUIPMENT: i32 = 3;
    pub const ITEMS: i32 = 4;
}

/// Item type determines behavior
#[derive(Clone, Debug, PartialEq)]
pub enum ItemKind {
    Generic,
    Sword { tier_damage: i32 },
    Pickaxe { tier_speed: f32 },
    Shovel { tier_speed: f32 },
    Hatchet { tier_speed: f32 },
    Hoe,
    Bow,
    Armor { slot: i32, defense: i32 },
    Food { nutrition: i32, saturation: f32 },
    Seed,
    Tool,
}

/// A single Item definition — ported from Item.h
#[derive(Clone, Debug)]
pub struct Item {
    pub id: i32,
    pub max_stack_size: i32,
    pub max_damage: i32,
    pub icon: i32,
    pub name: String,
    pub kind: ItemKind,
    pub hand_equipped: bool,
}

impl Item {
    pub fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            max_stack_size: MAX_STACK_SIZE,
            max_damage: 0,
            icon: 0,
            name: name.to_string(),
            kind: ItemKind::Generic,
            hand_equipped: false,
        }
    }

    pub fn with_stack(mut self, max: i32) -> Self { self.max_stack_size = max; self }
    pub fn with_damage(mut self, max: i32) -> Self { self.max_damage = max; self.max_stack_size = 1; self }
    pub fn with_icon(mut self, col: i32, row: i32) -> Self { self.icon = col + row * 16; self }
    pub fn with_kind(mut self, kind: ItemKind) -> Self { self.kind = kind; self }
    pub fn equipped(mut self) -> Self { self.hand_equipped = true; self }

    pub fn is_food(&self) -> bool { matches!(self.kind, ItemKind::Food { .. }) }
    pub fn is_armor(&self) -> bool { matches!(self.kind, ItemKind::Armor { .. }) }
    pub fn is_tool(&self) -> bool {
        matches!(self.kind, ItemKind::Sword { .. } | ItemKind::Pickaxe { .. } | ItemKind::Shovel { .. } | ItemKind::Hatchet { .. } | ItemKind::Hoe)
    }
    pub fn can_be_depleted(&self) -> bool { self.max_damage > 0 }

    pub fn get_attack_damage(&self) -> i32 {
        match &self.kind {
            ItemKind::Sword { tier_damage } => 4 + tier_damage,
            _ => 1,
        }
    }
}

// ======================== Full MCPE Item Registry ========================
// IDs match exactly: Item C++ constructor uses (id) where actual ID = 256 + id
lazy_static::lazy_static! {
    // ---- Tools: Iron ----
    pub static ref SHOVEL_IRON: Item = Item::new(256, "iron_shovel").with_damage(250).equipped().with_kind(ItemKind::Shovel { tier_speed: 6.0 });
    pub static ref PICKAXE_IRON: Item = Item::new(257, "iron_pickaxe").with_damage(250).equipped().with_kind(ItemKind::Pickaxe { tier_speed: 6.0 });
    pub static ref HATCHET_IRON: Item = Item::new(258, "iron_hatchet").with_damage(250).equipped().with_kind(ItemKind::Hatchet { tier_speed: 6.0 });
    pub static ref FLINT_AND_STEEL: Item = Item::new(259, "flint_and_steel").with_damage(64).with_stack(1);
    pub static ref APPLE: Item = Item::new(260, "apple").with_kind(ItemKind::Food { nutrition: 4, saturation: 2.4 });
    pub static ref BOW: Item = Item::new(261, "bow").with_damage(384).with_stack(1).with_kind(ItemKind::Bow);
    pub static ref ARROW: Item = Item::new(262, "arrow");
    pub static ref COAL: Item = Item::new(263, "coal");
    pub static ref DIAMOND: Item = Item::new(264, "diamond");
    pub static ref IRON_INGOT: Item = Item::new(265, "iron_ingot");
    pub static ref GOLD_INGOT: Item = Item::new(266, "gold_ingot");

    // ---- Tools: Iron Sword ----
    pub static ref SWORD_IRON: Item = Item::new(267, "iron_sword").with_damage(250).equipped().with_kind(ItemKind::Sword { tier_damage: 2 });

    // ---- Tools: Wood ----
    pub static ref SWORD_WOOD: Item = Item::new(268, "wood_sword").with_damage(59).equipped().with_kind(ItemKind::Sword { tier_damage: 0 });
    pub static ref SHOVEL_WOOD: Item = Item::new(269, "wood_shovel").with_damage(59).equipped().with_kind(ItemKind::Shovel { tier_speed: 2.0 });
    pub static ref PICKAXE_WOOD: Item = Item::new(270, "wood_pickaxe").with_damage(59).equipped().with_kind(ItemKind::Pickaxe { tier_speed: 2.0 });
    pub static ref HATCHET_WOOD: Item = Item::new(271, "wood_hatchet").with_damage(59).equipped().with_kind(ItemKind::Hatchet { tier_speed: 2.0 });

    // ---- Tools: Stone ----
    pub static ref SWORD_STONE: Item = Item::new(272, "stone_sword").with_damage(131).equipped().with_kind(ItemKind::Sword { tier_damage: 1 });
    pub static ref SHOVEL_STONE: Item = Item::new(273, "stone_shovel").with_damage(131).equipped().with_kind(ItemKind::Shovel { tier_speed: 4.0 });
    pub static ref PICKAXE_STONE: Item = Item::new(274, "stone_pickaxe").with_damage(131).equipped().with_kind(ItemKind::Pickaxe { tier_speed: 4.0 });
    pub static ref HATCHET_STONE: Item = Item::new(275, "stone_hatchet").with_damage(131).equipped().with_kind(ItemKind::Hatchet { tier_speed: 4.0 });

    // ---- Tools: Diamond ----
    pub static ref SWORD_DIAMOND: Item = Item::new(276, "diamond_sword").with_damage(1561).equipped().with_kind(ItemKind::Sword { tier_damage: 3 });
    pub static ref SHOVEL_DIAMOND: Item = Item::new(277, "diamond_shovel").with_damage(1561).equipped().with_kind(ItemKind::Shovel { tier_speed: 8.0 });
    pub static ref PICKAXE_DIAMOND: Item = Item::new(278, "diamond_pickaxe").with_damage(1561).equipped().with_kind(ItemKind::Pickaxe { tier_speed: 8.0 });
    pub static ref HATCHET_DIAMOND: Item = Item::new(279, "diamond_hatchet").with_damage(1561).equipped().with_kind(ItemKind::Hatchet { tier_speed: 8.0 });

    // ---- Materials & Misc ----
    pub static ref STICK: Item = Item::new(280, "stick");
    pub static ref BOWL: Item = Item::new(281, "bowl");
    pub static ref MUSHROOM_STEW: Item = Item::new(282, "mushroom_stew").with_stack(1).with_kind(ItemKind::Food { nutrition: 6, saturation: 7.2 });

    // ---- Tools: Gold ----
    pub static ref SWORD_GOLD: Item = Item::new(283, "gold_sword").with_damage(32).equipped().with_kind(ItemKind::Sword { tier_damage: 0 });
    pub static ref SHOVEL_GOLD: Item = Item::new(284, "gold_shovel").with_damage(32).equipped().with_kind(ItemKind::Shovel { tier_speed: 12.0 });
    pub static ref PICKAXE_GOLD: Item = Item::new(285, "gold_pickaxe").with_damage(32).equipped().with_kind(ItemKind::Pickaxe { tier_speed: 12.0 });
    pub static ref HATCHET_GOLD: Item = Item::new(286, "gold_hatchet").with_damage(32).equipped().with_kind(ItemKind::Hatchet { tier_speed: 12.0 });

    // ---- Mob drops ----
    pub static ref STRING: Item = Item::new(287, "string");
    pub static ref FEATHER: Item = Item::new(288, "feather");
    pub static ref SULPHUR: Item = Item::new(289, "sulphur"); // Gunpowder

    // ---- Hoes ----
    pub static ref HOE_WOOD: Item = Item::new(290, "wood_hoe").with_damage(59).with_kind(ItemKind::Hoe);
    pub static ref HOE_STONE: Item = Item::new(291, "stone_hoe").with_damage(131).with_kind(ItemKind::Hoe);
    pub static ref HOE_IRON: Item = Item::new(292, "iron_hoe").with_damage(250).with_kind(ItemKind::Hoe);
    pub static ref HOE_DIAMOND: Item = Item::new(293, "diamond_hoe").with_damage(1561).with_kind(ItemKind::Hoe);
    pub static ref HOE_GOLD: Item = Item::new(294, "gold_hoe").with_damage(32).with_kind(ItemKind::Hoe);

    // ---- Farming ----
    pub static ref SEEDS_WHEAT: Item = Item::new(295, "seeds_wheat").with_kind(ItemKind::Seed);
    pub static ref WHEAT: Item = Item::new(296, "wheat");
    pub static ref BREAD: Item = Item::new(297, "bread").with_kind(ItemKind::Food { nutrition: 5, saturation: 6.0 });

    // ---- Armor: Leather ----
    pub static ref HELMET_CLOTH: Item = Item::new(298, "leather_helmet").with_damage(55).with_stack(1).with_kind(ItemKind::Armor { slot: 0, defense: 1 });
    pub static ref CHESTPLATE_CLOTH: Item = Item::new(299, "leather_chestplate").with_damage(80).with_stack(1).with_kind(ItemKind::Armor { slot: 1, defense: 3 });
    pub static ref LEGGINGS_CLOTH: Item = Item::new(300, "leather_leggings").with_damage(75).with_stack(1).with_kind(ItemKind::Armor { slot: 2, defense: 2 });
    pub static ref BOOTS_CLOTH: Item = Item::new(301, "leather_boots").with_damage(65).with_stack(1).with_kind(ItemKind::Armor { slot: 3, defense: 1 });

    // ---- Armor: Chain ----
    pub static ref HELMET_CHAIN: Item = Item::new(302, "chain_helmet").with_damage(165).with_stack(1).with_kind(ItemKind::Armor { slot: 0, defense: 2 });
    pub static ref CHESTPLATE_CHAIN: Item = Item::new(303, "chain_chestplate").with_damage(240).with_stack(1).with_kind(ItemKind::Armor { slot: 1, defense: 5 });
    pub static ref LEGGINGS_CHAIN: Item = Item::new(304, "chain_leggings").with_damage(225).with_stack(1).with_kind(ItemKind::Armor { slot: 2, defense: 4 });
    pub static ref BOOTS_CHAIN: Item = Item::new(305, "chain_boots").with_damage(195).with_stack(1).with_kind(ItemKind::Armor { slot: 3, defense: 1 });

    // ---- Armor: Iron ----
    pub static ref HELMET_IRON: Item = Item::new(306, "iron_helmet").with_damage(165).with_stack(1).with_kind(ItemKind::Armor { slot: 0, defense: 2 });
    pub static ref CHESTPLATE_IRON: Item = Item::new(307, "iron_chestplate").with_damage(240).with_stack(1).with_kind(ItemKind::Armor { slot: 1, defense: 6 });
    pub static ref LEGGINGS_IRON: Item = Item::new(308, "iron_leggings").with_damage(225).with_stack(1).with_kind(ItemKind::Armor { slot: 2, defense: 5 });
    pub static ref BOOTS_IRON: Item = Item::new(309, "iron_boots").with_damage(195).with_stack(1).with_kind(ItemKind::Armor { slot: 3, defense: 2 });

    // ---- Armor: Diamond ----
    pub static ref HELMET_DIAMOND: Item = Item::new(310, "diamond_helmet").with_damage(363).with_stack(1).with_kind(ItemKind::Armor { slot: 0, defense: 3 });
    pub static ref CHESTPLATE_DIAMOND: Item = Item::new(311, "diamond_chestplate").with_damage(528).with_stack(1).with_kind(ItemKind::Armor { slot: 1, defense: 8 });
    pub static ref LEGGINGS_DIAMOND: Item = Item::new(312, "diamond_leggings").with_damage(495).with_stack(1).with_kind(ItemKind::Armor { slot: 2, defense: 6 });
    pub static ref BOOTS_DIAMOND: Item = Item::new(313, "diamond_boots").with_damage(429).with_stack(1).with_kind(ItemKind::Armor { slot: 3, defense: 3 });

    // ---- Armor: Gold ----
    pub static ref HELMET_GOLD: Item = Item::new(314, "gold_helmet").with_damage(77).with_stack(1).with_kind(ItemKind::Armor { slot: 0, defense: 2 });
    pub static ref CHESTPLATE_GOLD: Item = Item::new(315, "gold_chestplate").with_damage(112).with_stack(1).with_kind(ItemKind::Armor { slot: 1, defense: 5 });
    pub static ref LEGGINGS_GOLD: Item = Item::new(316, "gold_leggings").with_damage(105).with_stack(1).with_kind(ItemKind::Armor { slot: 2, defense: 3 });
    pub static ref BOOTS_GOLD: Item = Item::new(317, "gold_boots").with_damage(91).with_stack(1).with_kind(ItemKind::Armor { slot: 3, defense: 1 });

    // ---- Misc Items ----
    pub static ref FLINT: Item = Item::new(318, "flint");
    pub static ref PORKCHOP_RAW: Item = Item::new(319, "porkchop_raw").with_kind(ItemKind::Food { nutrition: 3, saturation: 1.8 });
    pub static ref PORKCHOP_COOKED: Item = Item::new(320, "porkchop_cooked").with_kind(ItemKind::Food { nutrition: 8, saturation: 12.8 });
    pub static ref PAINTING: Item = Item::new(321, "painting");
    pub static ref APPLE_GOLD: Item = Item::new(322, "golden_apple").with_kind(ItemKind::Food { nutrition: 4, saturation: 9.6 });
    pub static ref SIGN: Item = Item::new(323, "sign").with_stack(16);
    pub static ref DOOR_WOOD: Item = Item::new(324, "wooden_door").with_stack(1);
    pub static ref BUCKET_EMPTY: Item = Item::new(325, "bucket").with_stack(16);
    pub static ref BUCKET_WATER: Item = Item::new(326, "water_bucket").with_stack(1);
    pub static ref BUCKET_LAVA: Item = Item::new(327, "lava_bucket").with_stack(1);
    pub static ref SADDLE: Item = Item::new(329, "saddle").with_stack(1);
    pub static ref DOOR_IRON: Item = Item::new(330, "iron_door").with_stack(1);
    pub static ref REDSTONE: Item = Item::new(331, "redstone");
    pub static ref SNOWBALL: Item = Item::new(332, "snowball").with_stack(16);
    pub static ref LEATHER: Item = Item::new(334, "leather");
    pub static ref BRICK: Item = Item::new(336, "brick");
    pub static ref CLAY_BALL: Item = Item::new(337, "clay_ball");
    pub static ref REEDS: Item = Item::new(338, "reeds");
    pub static ref PAPER: Item = Item::new(339, "paper");
    pub static ref BOOK: Item = Item::new(340, "book");
    pub static ref SLIME_BALL: Item = Item::new(341, "slime_ball");
    pub static ref EGG: Item = Item::new(344, "egg").with_stack(16);
    pub static ref COMPASS: Item = Item::new(345, "compass");
    pub static ref CLOCK: Item = Item::new(347, "clock");
    pub static ref GLOWSTONE_DUST: Item = Item::new(348, "glowstone_dust");
    pub static ref FISH_RAW: Item = Item::new(349, "fish_raw").with_kind(ItemKind::Food { nutrition: 2, saturation: 0.4 });
    pub static ref FISH_COOKED: Item = Item::new(350, "fish_cooked").with_kind(ItemKind::Food { nutrition: 5, saturation: 6.0 });
    pub static ref DYE_POWDER: Item = Item::new(351, "dye_powder");
    pub static ref BONE: Item = Item::new(352, "bone");
    pub static ref SUGAR: Item = Item::new(353, "sugar");
    pub static ref CAKE: Item = Item::new(354, "cake").with_stack(1);
    pub static ref BED: Item = Item::new(355, "bed").with_stack(1);
    pub static ref SHEARS: Item = Item::new(359, "shears").with_damage(238);
    pub static ref MELON_SLICE: Item = Item::new(360, "melon").with_kind(ItemKind::Food { nutrition: 2, saturation: 1.2 });
    pub static ref SEEDS_MELON: Item = Item::new(362, "seeds_melon").with_kind(ItemKind::Seed);
    pub static ref BEEF_RAW: Item = Item::new(363, "beef_raw").with_kind(ItemKind::Food { nutrition: 3, saturation: 1.8 });
    pub static ref BEEF_COOKED: Item = Item::new(364, "beef_cooked").with_kind(ItemKind::Food { nutrition: 8, saturation: 12.8 });
    pub static ref CHICKEN_RAW: Item = Item::new(365, "chicken_raw").with_kind(ItemKind::Food { nutrition: 2, saturation: 1.2 });
    pub static ref CHICKEN_COOKED: Item = Item::new(366, "chicken_cooked").with_kind(ItemKind::Food { nutrition: 6, saturation: 7.2 });
    pub static ref ROTTEN_FLESH: Item = Item::new(367, "rotten_flesh").with_kind(ItemKind::Food { nutrition: 4, saturation: 0.8 });
    pub static ref GOLD_NUGGET: Item = Item::new(371, "gold_nugget");
    pub static ref NETHER_BRICK: Item = Item::new(405, "nether_brick");
    pub static ref NETHER_QUARTZ: Item = Item::new(406, "nether_quartz");
}

pub fn init_items() {
    // Force lazy_static initialization
    let _ = SHOVEL_IRON.id;
    let _ = ARROW.id;
}

pub fn get_max_stack_size(id: i32) -> i32 {
    // For now, if it's over 255 it's an item. If we want we could search the lazy statics,
    // but a dummy default works until a HashMap registry is added.
    if matches!(id, 323|325|332|344) { 16 } // signs, buckets, snowballs, eggs
    else if matches!(id, 256..=261|267..=279|283..=286|290..=294|298..=317|324|326..=330|354..=355|359) { 1 } // tools, armor, doors, beds, stews
    else { MAX_STACK_SIZE }
}