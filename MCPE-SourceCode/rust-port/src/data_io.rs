pub trait DataOutput {
    fn write_string(&mut self, v: &str);
    fn write_float(&mut self, v: f32);
    fn write_double(&mut self, v: f64);
    fn write_byte(&mut self, v: i8);
    fn write_short(&mut self, v: i16);
    fn write_int(&mut self, v: i32);
    fn write_long_long(&mut self, v: i64);
    fn write_bytes(&mut self, data: &[u8]);
}

pub trait DataInput {
    fn read_string(&mut self) -> String;
    fn read_float(&mut self) -> f32;
    fn read_double(&mut self) -> f64;
    fn read_byte(&mut self) -> i8;
    fn read_short(&mut self) -> i16;
    fn read_int(&mut self) -> i32;
    fn read_long_long(&mut self) -> i64;
    fn read_bytes(&mut self, bytes: usize) -> Vec<u8>;
}

pub const MAX_STRING_LENGTH: usize = i16::MAX as usize;

#[derive(Default, Clone)]
pub struct MemoryDataOutput {
    pub bytes: Vec<u8>,
}

impl MemoryDataOutput {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

impl DataOutput for MemoryDataOutput {
    fn write_string(&mut self, v: &str) {
        let length = (v.len() & 0x7fff) as i16;
        self.write_short(length);
        self.write_bytes(&v.as_bytes()[..length as usize]);
    }

    fn write_float(&mut self, v: f32) {
        self.write_bytes(&v.to_le_bytes());
    }

    fn write_double(&mut self, v: f64) {
        self.write_bytes(&v.to_le_bytes());
    }

    fn write_byte(&mut self, v: i8) {
        self.write_bytes(&[v as u8]);
    }

    fn write_short(&mut self, v: i16) {
        self.write_bytes(&v.to_le_bytes());
    }

    fn write_int(&mut self, v: i32) {
        self.write_bytes(&v.to_le_bytes());
    }

    fn write_long_long(&mut self, v: i64) {
        self.write_bytes(&v.to_le_bytes());
    }

    fn write_bytes(&mut self, data: &[u8]) {
        self.bytes.extend_from_slice(data);
    }
}

#[derive(Clone)]
pub struct MemoryDataInput {
    bytes: Vec<u8>,
    index: usize,
}

impl MemoryDataInput {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, index: 0 }
    }
}

impl DataInput for MemoryDataInput {
    fn read_string(&mut self) -> String {
        let mut len = self.read_short() as usize;
        if len > MAX_STRING_LENGTH - 1 {
            len = MAX_STRING_LENGTH - 1;
        }
        let raw = self.read_bytes(len);
        String::from_utf8_lossy(&raw).to_string()
    }

    fn read_float(&mut self) -> f32 {
        let b = self.read_bytes(4);
        f32::from_le_bytes([b[0], b[1], b[2], b[3]])
    }

    fn read_double(&mut self) -> f64 {
        let b = self.read_bytes(8);
        f64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }

    fn read_byte(&mut self) -> i8 {
        self.read_bytes(1)[0] as i8
    }

    fn read_short(&mut self) -> i16 {
        let b = self.read_bytes(2);
        i16::from_le_bytes([b[0], b[1]])
    }

    fn read_int(&mut self) -> i32 {
        let b = self.read_bytes(4);
        i32::from_le_bytes([b[0], b[1], b[2], b[3]])
    }

    fn read_long_long(&mut self) -> i64 {
        let b = self.read_bytes(8);
        i64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }

    fn read_bytes(&mut self, bytes: usize) -> Vec<u8> {
        if self.index >= self.bytes.len() || bytes == 0 {
            return Vec::new();
        }
        let end = (self.index + bytes).min(self.bytes.len());
        let out = self.bytes[self.index..end].to_vec();
        self.index = end;
        out
    }
}

