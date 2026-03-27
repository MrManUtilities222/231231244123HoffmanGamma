#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Motive {
    pub name: &'static str,
    pub w: i32,
    pub h: i32,
    pub uo: i32,
    pub vo: i32,
    pub is_public: bool,
}

impl Motive {
    pub const MAX_MOTIVE_NAME_LENGTH: usize = 13;
    pub const fn new(name: &'static str, w: i32, h: i32, uo: i32, vo: i32, is_public: bool) -> Self {
        Self {
            name,
            w,
            h,
            uo,
            vo,
            is_public,
        }
    }
}

pub const KEBAB: Motive = Motive::new("Kebab", 16, 16, 0 * 16, 0 * 16, true);
pub const AZTEC: Motive = Motive::new("Aztec", 16, 16, 1 * 16, 0 * 16, true);
pub const ALBAN: Motive = Motive::new("Alban", 16, 16, 2 * 16, 0 * 16, true);
pub const AZTEC2: Motive = Motive::new("Aztec2", 16, 16, 3 * 16, 0 * 16, true);
pub const BOMB: Motive = Motive::new("Bomb", 16, 16, 4 * 16, 0 * 16, true);
pub const PLANT: Motive = Motive::new("Plant", 16, 16, 5 * 16, 0 * 16, true);
pub const WASTELAND: Motive = Motive::new("Wasteland", 16, 16, 6 * 16, 0 * 16, true);
pub const POOL: Motive = Motive::new("Pool", 32, 16, 0 * 16, 2 * 16, true);
pub const COURBET: Motive = Motive::new("Courbet", 32, 16, 2 * 16, 2 * 16, true);
pub const SEA: Motive = Motive::new("Sea", 32, 16, 4 * 16, 2 * 16, true);
pub const SUNSET: Motive = Motive::new("Sunset", 32, 16, 6 * 16, 2 * 16, true);
pub const CREEBET: Motive = Motive::new("Creebet", 32, 16, 8 * 16, 2 * 16, true);
pub const WANDERER: Motive = Motive::new("Wanderer", 16, 32, 0 * 16, 4 * 16, true);
pub const GRAHAM: Motive = Motive::new("Graham", 16, 32, 1 * 16, 4 * 16, true);
pub const MATCH: Motive = Motive::new("Match", 32, 32, 0 * 16, 8 * 16, true);
pub const BUST: Motive = Motive::new("Bust", 32, 32, 2 * 16, 8 * 16, true);
pub const STAGE: Motive = Motive::new("Stage", 32, 32, 4 * 16, 8 * 16, true);
pub const VOID: Motive = Motive::new("Void", 32, 32, 6 * 16, 8 * 16, true);
pub const SKULL_AND_ROSES: Motive = Motive::new("SkullAndRoses", 32, 32, 8 * 16, 8 * 16, true);
pub const FIGHTERS: Motive = Motive::new("Fighters", 64, 32, 0 * 16, 6 * 16, true);
pub const POINTER: Motive = Motive::new("Pointer", 64, 64, 0 * 16, 12 * 16, true);
pub const PIGSCENE: Motive = Motive::new("Pigscene", 64, 64, 4 * 16, 12 * 16, true);
pub const BURNING_SKULL: Motive = Motive::new("BurningSkull", 64, 64, 8 * 16, 12 * 16, true);
pub const SKELETON: Motive = Motive::new("Skeleton", 64, 48, 12 * 16, 4 * 16, true);
pub const DONKEY_KONG: Motive = Motive::new("DonkeyKong", 64, 48, 12 * 16, 7 * 16, true);
pub const EARTH: Motive = Motive::new("Earth", 32, 32, 0 * 16, 10 * 16, false);
pub const WIND: Motive = Motive::new("Wind", 32, 32, 2 * 16, 10 * 16, false);
pub const FIRE: Motive = Motive::new("Fire", 32, 32, 4 * 16, 10 * 16, false);
pub const WATER: Motive = Motive::new("Water", 32, 32, 6 * 16, 10 * 16, false);

pub const DEFAULT_IMAGE: Motive = KEBAB;

pub fn get_all_motives() -> Vec<Motive> {
    vec![
        KEBAB, AZTEC2, ALBAN, BOMB, PLANT, WASTELAND, POOL, COURBET, SEA, SUNSET, CREEBET, WANDERER, GRAHAM,
        MATCH, BUST, STAGE, VOID, SKULL_AND_ROSES, FIGHTERS, POINTER, PIGSCENE, BURNING_SKULL, SKELETON,
        DONKEY_KONG, EARTH, WIND, FIRE, WATER,
    ]
}

pub fn get_motive_by_name(name: &str) -> Motive {
    for m in get_all_motives() {
        if m.name == name {
            return m;
        }
    }
    DEFAULT_IMAGE
}

