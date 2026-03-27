use std::fs;
use std::path::{Path, PathBuf};

use crate::storage_api;

pub const TEMP_LEVEL_ID: &str = "_LastJoinedServer";

pub trait LevelStorageSource {
    fn get_name(&self) -> &'static str;
    fn get_full_path(&self, level_id: &str) -> PathBuf;
    fn get_level_list(&self) -> std::io::Result<Vec<LevelSummary>>;
    fn is_new_level_id_acceptable(&self, level_id: &str) -> bool;
    fn rename_level(&self, level_id: &str, new_title: &str) -> std::io::Result<String>;
    fn delete_level(&self, level_id: &str) -> std::io::Result<()>;
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelSummary {
    pub id: String,
    pub name: String,
    pub last_played: i64,
    pub game_type: i32,
    pub size_on_disk: u64,
}

pub struct ExternalFileLevelStorageSource {
    base_path: PathBuf,
    tmp_base_path: PathBuf,
    _has_temp_directory: bool,
}

impl ExternalFileLevelStorageSource {
    pub fn new(external_path: &Path, temporary_path: &Path, standalone_server: bool) -> std::io::Result<Self> {
        if standalone_server {
            fs::create_dir_all(external_path)?;
            fs::create_dir_all(temporary_path)?;
            return Ok(Self {
                base_path: external_path.to_path_buf(),
                tmp_base_path: temporary_path.to_path_buf(),
                _has_temp_directory: temporary_path != external_path,
            });
        }

        let tree = ["games", "com.mojang", "minecraftWorlds"];
        let mut base = external_path.to_path_buf();
        let mut tmp = temporary_path.to_path_buf();
        for seg in tree {
            base.push(seg);
            tmp.push(seg);
        }
        fs::create_dir_all(&base)?;
        fs::create_dir_all(&tmp)?;

        Ok(Self {
            base_path: base,
            tmp_base_path: tmp,
            _has_temp_directory: temporary_path != external_path,
        })
    }
}

impl LevelStorageSource for ExternalFileLevelStorageSource {
    fn get_name(&self) -> &'static str {
        "External File Level Storage"
    }

    fn get_full_path(&self, level_id: &str) -> PathBuf {
        if level_id == TEMP_LEVEL_ID {
            self.tmp_base_path.join(level_id)
        } else {
            self.base_path.join(level_id)
        }
    }

    fn get_level_list(&self) -> std::io::Result<Vec<LevelSummary>> {
        let mut out = Vec::new();
        if !self.base_path.exists() {
            return Ok(out);
        }

        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let md = entry.metadata()?;
            if !md.is_dir() {
                continue;
            }
            let id = entry.file_name().to_string_lossy().to_string();
            let dir = self.base_path.join(&id);
            if let Some(level_data) = storage_api::load_level_data(&dir)? {
                out.push(LevelSummary {
                    id,
                    name: level_data.level_name,
                    last_played: level_data.last_played,
                    game_type: level_data.game_type,
                    size_on_disk: level_data.size_on_disk as u64,
                });
            }
        }

        out.sort_by(|a, b| b.last_played.cmp(&a.last_played));
        Ok(out)
    }

    fn is_new_level_id_acceptable(&self, level_id: &str) -> bool {
        if level_id.is_empty() {
            return false;
        }
        let illegal = ['/', '\n', '\r', '\t', '\0', '\u{000C}', '`', '?', '*', '\\', '<', '>', '|', '"', ':'];
        !level_id.chars().any(|c| illegal.contains(&c))
    }

    fn rename_level(&self, level_id: &str, new_title: &str) -> std::io::Result<String> {
        // Sanitize illegal characters while preserving readable name.
        let sanitized: String = new_title
            .chars()
            .filter(|c| self.is_new_level_id_acceptable(&c.to_string()))
            .collect();

        if sanitized.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid level title"));
        }

        let target = self.base_path.join(&sanitized);
        let src_path = self.base_path.join(level_id);
        fs::rename(&src_path, &target)?;
        Ok(sanitized)
    }

    fn delete_level(&self, level_id: &str) -> std::io::Result<()> {
        let path = self.base_path.join(level_id);
        if path.exists() {
            fs::remove_dir_all(path)
        } else {
            Ok(())
        }
    }
}


