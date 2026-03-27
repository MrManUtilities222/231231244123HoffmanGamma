#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataLayer {
    pub data: Vec<u8>,
    pub length: usize,
    pub slot_max: usize,
}

impl Default for DataLayer {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            length: 0,
            slot_max: 0,
        }
    }
}

impl DataLayer {
    pub fn new(length: usize) -> Self {
        let byte_len = length >> 1;
        let mut s = Self {
            data: vec![0; byte_len],
            length: byte_len,
            slot_max: byte_len,
        };
        s.set_all(0);
        s
    }

    pub fn from_data(data: Vec<u8>, length: usize) -> Self {
        let byte_len = length >> 1;
        Self {
            data,
            length: byte_len,
            slot_max: byte_len,
        }
    }

    pub fn get_xyz(&self, x: usize, y: usize, z: usize) -> u8 {
        self.get((x << 11) | (z << 7) | y)
    }

    pub fn set_xyz(&mut self, x: usize, y: usize, z: usize, val: u8) {
        self.set((x << 11) | (z << 7) | y, val);
    }

    pub fn get(&self, pos: usize) -> u8 {
        let slot = pos >> 1;
        let part = pos & 1;
        if slot >= self.data.len() {
            return 0;
        }
        if part == 0 {
            self.data[slot] & 0x0f
        } else {
            (self.data[slot] >> 4) & 0x0f
        }
    }

    pub fn set(&mut self, pos: usize, val: u8) {
        let slot = pos >> 1;
        let part = pos & 1;
        if slot >= self.data.len() {
            return;
        }
        if part == 0 {
            self.data[slot] = (self.data[slot] & 0xf0) | (val & 0x0f);
        } else {
            self.data[slot] = (self.data[slot] & 0x0f) | ((val & 0x0f) << 4);
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.data.is_empty()
    }

    pub fn set_all(&mut self, br: u8) {
        // Preserve source behavior exactly: (br & (br << 4))
        let val = br & (br << 4);
        self.data.fill(val);
    }
}

