use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::nbt::TagValue;

pub const DIMENSION_NORMAL: i32 = 0;
pub const GAME_TYPE_SURVIVAL: i32 = 0;
pub const GAME_TYPE_DEFAULT: i32 = GAME_TYPE_SURVIVAL;

#[derive(Debug, Clone, PartialEq)]
pub struct LevelData {
    pub seed: i64,
    pub game_type: i32,
    pub level_name: String,
    pub x_spawn: i32,
    pub y_spawn: i32,
    pub z_spawn: i32,
    pub time: i64,
    pub last_played: i64,
    pub size_on_disk: i64,
    pub storage_version: i32,
    pub dimension: i32,
    pub spawn_mobs: bool,
    pub generator_version: i32,
    pub loaded_player_tag: Option<BTreeMap<String, TagValue>>,
}

impl Default for LevelData {
    fn default() -> Self {
        Self {
            seed: 0,
            game_type: GAME_TYPE_DEFAULT,
            level_name: String::new(),
            x_spawn: 128,
            y_spawn: 64,
            z_spawn: 128,
            time: 0,
            last_played: 0,
            size_on_disk: 0,
            storage_version: 0,
            dimension: DIMENSION_NORMAL,
            spawn_mobs: true,
            generator_version: 0,
            loaded_player_tag: None,
        }
    }
}

impl LevelData {
    pub fn new(level_name: String) -> Self {
        Self {
            level_name,
            ..Default::default()
        }
    }

    pub fn from_nbt(tag: &BTreeMap<String, TagValue>) -> Self {
        let game_type = get_int(tag, "GameType", GAME_TYPE_DEFAULT);
        Self {
            seed: get_long(tag, "RandomSeed", 0),
            game_type,
            level_name: get_string(tag, "LevelName", ""),
            x_spawn: get_int(tag, "SpawnX", 128),
            y_spawn: get_int(tag, "SpawnY", 64),
            z_spawn: get_int(tag, "SpawnZ", 128),
            time: get_long(tag, "Time", 0),
            last_played: get_long(tag, "LastPlayed", 0),
            size_on_disk: get_long(tag, "SizeOnDisk", 0),
            storage_version: get_int(tag, "StorageVersion", 0),
            dimension: DIMENSION_NORMAL,
            spawn_mobs: game_type == GAME_TYPE_SURVIVAL,
            generator_version: 0,
            loaded_player_tag: get_compound(tag, "Player"),
        }
    }

    pub fn to_nbt(&self) -> BTreeMap<String, TagValue> {
        self.to_nbt_with_player(self.loaded_player_tag.clone())
    }

    pub fn to_nbt_with_player(
        &self,
        player_tag: Option<BTreeMap<String, TagValue>>,
    ) -> BTreeMap<String, TagValue> {
        let mut out = BTreeMap::new();
        out.insert("RandomSeed".to_string(), TagValue::Long(self.seed));
        out.insert("GameType".to_string(), TagValue::Int(self.game_type));
        out.insert("SpawnX".to_string(), TagValue::Int(self.x_spawn));
        out.insert("SpawnY".to_string(), TagValue::Int(self.y_spawn));
        out.insert("SpawnZ".to_string(), TagValue::Int(self.z_spawn));
        out.insert("Time".to_string(), TagValue::Long(self.time));
        out.insert("SizeOnDisk".to_string(), TagValue::Long(self.size_on_disk));
        out.insert(
            "LastPlayed".to_string(),
            TagValue::Long(epoch_time_s() as i64),
        );
        out.insert("LevelName".to_string(), TagValue::String(self.level_name.clone()));
        out.insert("StorageVersion".to_string(), TagValue::Int(self.storage_version));
        out.insert("Platform".to_string(), TagValue::Int(2));
        if let Some(player) = player_tag {
            let hmap: std::collections::HashMap<String, TagValue> = player.into_iter().collect();
            out.insert("Player".to_string(), TagValue::Compound(hmap));
        }
        out
    }
}

fn epoch_time_s() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn get_int(tag: &BTreeMap<String, TagValue>, key: &str, default: i32) -> i32 {
    match tag.get(key) {
        Some(TagValue::Int(v)) => *v,
        _ => default,
    }
}

fn get_long(tag: &BTreeMap<String, TagValue>, key: &str, default: i64) -> i64 {
    match tag.get(key) {
        Some(TagValue::Long(v)) => *v,
        _ => default,
    }
}

fn get_string(tag: &BTreeMap<String, TagValue>, key: &str, default: &str) -> String {
    match tag.get(key) {
        Some(TagValue::String(v)) => v.clone(),
        _ => default.to_string(),
    }
}

fn get_compound(
    tag: &BTreeMap<String, TagValue>,
    key: &str,
) -> Option<BTreeMap<String, TagValue>> {
    match tag.get(key) {
        Some(TagValue::Compound(v)) => {
            let btree: BTreeMap<String, TagValue> = v.iter().map(|(k,v)| (k.clone(), v.clone())).collect();
            Some(btree)
        },
        _ => None,
    }
}

