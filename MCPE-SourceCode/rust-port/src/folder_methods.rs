use std::fs;
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};

pub fn exists(path: &Path) -> bool {
    path.exists()
}

pub fn create_folder_if_not_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

pub fn create_tree(base: &Path, tree: &[&str]) -> std::io::Result<PathBuf> {
    create_folder_if_not_exists(base)?;
    let mut current = PathBuf::from(base);
    for segment in tree {
        let trimmed = segment.trim_start_matches('/');
        current.push(trimmed);
        if !current.exists() {
            fs::create_dir(&current)?;
        }
    }
    Ok(current)
}

pub fn get_file_size(path: &Path) -> std::io::Result<u64> {
    Ok(fs::metadata(path)?.len())
}

pub fn get_remaining_file_size(file: &mut fs::File) -> std::io::Result<u64> {
    let cur = file.stream_position()?;
    let end = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(cur))?;
    Ok(end.saturating_sub(cur))
}

