use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use crate::data_io::{MemoryDataInput, MemoryDataOutput};
use crate::level_data::LevelData;
use crate::nbt;
use crate::region_file::RegionFile;

pub const STORAGE_VERSION_V2: i32 = 2;

pub fn save_level_data(level_dir: &Path, level_data: &LevelData) -> std::io::Result<()> {
    fs::create_dir_all(level_dir)?;
    let tmp = level_dir.join("level.dat_new");
    let dat = level_dir.join("level.dat");
    let old = level_dir.join("level.dat_old");

    let mut out = MemoryDataOutput::new();
    nbt::write_root_compound(&mut out, "", level_data.to_nbt());
    let payload = out.into_inner();

    let mut f = File::create(&tmp)?;
    f.write_all(&STORAGE_VERSION_V2.to_le_bytes())?;
    f.write_all(&(payload.len() as i32).to_le_bytes())?;
    f.write_all(&payload)?;
    f.flush()?;

    let _ = fs::remove_file(&old);
    if dat.exists() {
        let _ = fs::rename(&dat, &old);
    }
    fs::rename(&tmp, &dat)?;
    let _ = fs::remove_file(&tmp);
    Ok(())
}

pub fn load_level_data(level_dir: &Path) -> std::io::Result<Option<LevelData>> {
    let dat = level_dir.join("level.dat");
    let old = level_dir.join("level.dat_old");
    let source = if dat.exists() {
        dat
    } else if old.exists() {
        old
    } else {
        return Ok(None);
    };

    let mut f = File::open(source)?;
    let mut header = [0u8; 8];
    if f.read_exact(&mut header).is_err() {
        return Ok(None);
    }
    let _version = i32::from_le_bytes([header[0], header[1], header[2], header[3]]);
    let size = i32::from_le_bytes([header[4], header[5], header[6], header[7]]);
    if size <= 0 {
        return Ok(None);
    }

    let mut payload = vec![0u8; size as usize];
    f.read_exact(&mut payload)?;

    let mut inp = MemoryDataInput::new(payload);
    let Some((_name, root)) = nbt::read_root_compound(&mut inp) else {
        return Ok(None);
    };
    Ok(Some(LevelData::from_nbt(&root)))
}

pub fn save_chunk(level_dir: &Path, x: usize, z: usize, payload: &[u8]) -> std::io::Result<()> {
    fs::create_dir_all(level_dir)?;
    let mut region = RegionFile::new(level_dir);
    region.open()?;
    region.write_chunk(x, z, payload)?;
    region.close();
    Ok(())
}

pub fn load_chunk(level_dir: &Path, x: usize, z: usize) -> std::io::Result<Option<Vec<u8>>> {
    let mut region = RegionFile::new(level_dir);
    region.open()?;
    let out = region.read_chunk(x, z)?;
    region.close();
    Ok(out)
}

