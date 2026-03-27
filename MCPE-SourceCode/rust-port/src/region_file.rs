use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const SECTOR_BYTES: usize = 4096;
const SECTOR_INTS: usize = SECTOR_BYTES / 4;
const SECTOR_COLS: usize = 32;
const REGION_DAT_NAME: &str = "chunks.dat";

pub struct RegionFile {
    file: Option<File>,
    filename: PathBuf,
    offsets: [i32; SECTOR_INTS],
    sector_free: BTreeMap<usize, bool>,
}

impl RegionFile {
    pub fn new(base_path: &Path) -> Self {
        Self {
            file: None,
            filename: base_path.join(REGION_DAT_NAME),
            offsets: [0; SECTOR_INTS],
            sector_free: BTreeMap::new(),
        }
    }

    pub fn open(&mut self) -> std::io::Result<()> {
        self.close();
        self.offsets = [0; SECTOR_INTS];
        self.sector_free.clear();

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&self.filename)?;
        self.file = Some(file);

        let f = self.file_mut()?;
        let len = f.metadata()?.len() as usize;
        if len < SECTOR_BYTES {
            f.seek(SeekFrom::Start(0))?;
            f.write_all(&vec![0u8; SECTOR_BYTES])?;
            f.flush()?;
        } else {
            f.seek(SeekFrom::Start(0))?;
            let mut header = [0u8; SECTOR_BYTES];
            f.read_exact(&mut header)?;
            for i in 0..SECTOR_INTS {
                let p = i * 4;
                self.offsets[i] =
                    i32::from_le_bytes([header[p], header[p + 1], header[p + 2], header[p + 3]]);
            }
        }

        self.sector_free.insert(0, false);
        for sector in 0..SECTOR_INTS {
            let offset = self.offsets[sector];
            if offset != 0 {
                let base = (offset >> 8) as usize;
                let count = (offset & 0xff) as usize;
                for i in 0..count {
                    self.sector_free.insert(base + i, false);
                }
            }
        }
        Ok(())
    }

    pub fn read_chunk(&mut self, x: usize, z: usize) -> std::io::Result<Option<Vec<u8>>> {
        let idx = x + z * SECTOR_COLS;
        let offset = self.offsets[idx];
        if offset == 0 {
            return Ok(None);
        }
        let sector_num = (offset >> 8) as u64;
        let sector_count = (offset & 0xff) as usize;

        let f = self.file_mut()?;
        f.seek(SeekFrom::Start(sector_num * SECTOR_BYTES as u64))?;
        let mut len_buf = [0u8; 4];
        f.read_exact(&mut len_buf)?;
        let length_with_header = i32::from_le_bytes(len_buf).max(0) as usize;
        if length_with_header < 4 {
            return Ok(Some(Vec::new()));
        }
        let payload_len = length_with_header - 4;
        if payload_len > sector_count * SECTOR_BYTES {
            return Ok(None);
        }

        let mut data = vec![0u8; payload_len];
        f.read_exact(&mut data)?;
        Ok(Some(data))
    }

    pub fn write_chunk(&mut self, x: usize, z: usize, chunk_data: &[u8]) -> std::io::Result<()> {
        let idx = x + z * SECTOR_COLS;
        let size = chunk_data.len() + 4;
        let sectors_needed = (size / SECTOR_BYTES) + 1;
        if sectors_needed > 256 {
            return Err(std::io::Error::other("chunk is too big"));
        }

        let offset = self.offsets[idx];
        let sector_num = (offset >> 8) as usize;
        let sector_count = (offset & 0xff) as usize;

        if sector_num != 0 && sector_count == sectors_needed {
            self.write_sector(sector_num, chunk_data)?;
            return Ok(());
        }

        for i in 0..sector_count {
            self.sector_free.insert(sector_num + i, true);
        }

        let mut slot = 0usize;
        let mut run_length = 0usize;
        let mut extend_file = false;

        while run_length < sectors_needed {
            match self.sector_free.get(&(slot + run_length)) {
                Some(true) => run_length += 1,
                Some(false) => {
                    slot = slot + run_length + 1;
                    run_length = 0;
                }
                None => {
                    extend_file = true;
                    break;
                }
            }
        }

        if extend_file {
            {
                let f = self.file_mut()?;
                f.seek(SeekFrom::End(0))?;
                for _ in 0..(sectors_needed - run_length) {
                    f.write_all(&vec![0u8; SECTOR_BYTES])?;
                }
            }
            for i in 0..(sectors_needed - run_length) {
                self.sector_free.insert(slot + i, true);
            }
        }

        self.offsets[idx] = ((slot as i32) << 8) | sectors_needed as i32;
        for i in 0..sectors_needed {
            self.sector_free.insert(slot + i, false);
        }
        self.write_sector(slot, chunk_data)?;

        let header_value = self.offsets[idx].to_le_bytes();
        {
            let f = self.file_mut()?;
            f.seek(SeekFrom::Start((idx * 4) as u64))?;
            f.write_all(&header_value)?;
            f.flush()?;
        }
        Ok(())
    }

    pub fn close(&mut self) {
        self.file = None;
    }

    fn write_sector(&mut self, sector: usize, chunk_data: &[u8]) -> std::io::Result<()> {
        let f = self.file_mut()?;
        f.seek(SeekFrom::Start((sector * SECTOR_BYTES) as u64))?;
        let size = (chunk_data.len() + 4) as i32;
        f.write_all(&size.to_le_bytes())?;
        f.write_all(chunk_data)?;
        Ok(())
    }

    fn file_mut(&mut self) -> std::io::Result<&mut File> {
        self.file
            .as_mut()
            .ok_or_else(|| std::io::Error::other("region file is not open"))
    }
}

