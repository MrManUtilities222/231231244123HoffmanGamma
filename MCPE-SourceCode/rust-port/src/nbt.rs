use std::collections::HashMap;
use std::io::{self, Read, Write, Cursor};

/// Backwards-compatible alias used by level_data.rs and storage_api.rs
pub type TagValue = NbtTag;

/// NBT Tag types matching the Minecraft spec.
#[derive(Clone, Debug, PartialEq)]
pub enum NbtTag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<NbtTag>),
    Compound(HashMap<String, NbtTag>),
}

impl NbtTag {
    pub fn type_id(&self) -> u8 {
        match self {
            NbtTag::End => 0,
            NbtTag::Byte(_) => 1,
            NbtTag::Short(_) => 2,
            NbtTag::Int(_) => 3,
            NbtTag::Long(_) => 4,
            NbtTag::Float(_) => 5,
            NbtTag::Double(_) => 6,
            NbtTag::ByteArray(_) => 7,
            NbtTag::String(_) => 8,
            NbtTag::List(_) => 9,
            NbtTag::Compound(_) => 10,
        }
    }

    pub fn as_byte(&self) -> Option<i8> {
        if let NbtTag::Byte(v) = self { Some(*v) } else { None }
    }

    pub fn as_short(&self) -> Option<i16> {
        if let NbtTag::Short(v) = self { Some(*v) } else { None }
    }

    pub fn as_int(&self) -> Option<i32> {
        if let NbtTag::Int(v) = self { Some(*v) } else { None }
    }

    pub fn as_long(&self) -> Option<i64> {
        if let NbtTag::Long(v) = self { Some(*v) } else { None }
    }

    pub fn as_float(&self) -> Option<f32> {
        if let NbtTag::Float(v) = self { Some(*v) } else { None }
    }

    pub fn as_double(&self) -> Option<f64> {
        if let NbtTag::Double(v) = self { Some(*v) } else { None }
    }

    pub fn as_string(&self) -> Option<&str> {
        if let NbtTag::String(v) = self { Some(v.as_str()) } else { None }
    }

    pub fn as_byte_array(&self) -> Option<&[u8]> {
        if let NbtTag::ByteArray(v) = self { Some(v.as_slice()) } else { None }
    }

    pub fn as_compound(&self) -> Option<&HashMap<String, NbtTag>> {
        if let NbtTag::Compound(v) = self { Some(v) } else { None }
    }

    pub fn as_list(&self) -> Option<&Vec<NbtTag>> {
        if let NbtTag::List(v) = self { Some(v) } else { None }
    }
}

/// A named compound tag at the root level.
#[derive(Clone, Debug)]
pub struct CompoundTag {
    pub name: String,
    pub tags: HashMap<String, NbtTag>,
}

impl CompoundTag {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tags: HashMap::new(),
        }
    }

    pub fn put_byte(&mut self, key: &str, val: i8) {
        self.tags.insert(key.to_string(), NbtTag::Byte(val));
    }

    pub fn put_short(&mut self, key: &str, val: i16) {
        self.tags.insert(key.to_string(), NbtTag::Short(val));
    }

    pub fn put_int(&mut self, key: &str, val: i32) {
        self.tags.insert(key.to_string(), NbtTag::Int(val));
    }

    pub fn put_long(&mut self, key: &str, val: i64) {
        self.tags.insert(key.to_string(), NbtTag::Long(val));
    }

    pub fn put_float(&mut self, key: &str, val: f32) {
        self.tags.insert(key.to_string(), NbtTag::Float(val));
    }

    pub fn put_double(&mut self, key: &str, val: f64) {
        self.tags.insert(key.to_string(), NbtTag::Double(val));
    }

    pub fn put_string(&mut self, key: &str, val: &str) {
        self.tags.insert(key.to_string(), NbtTag::String(val.to_string()));
    }

    pub fn put_byte_array(&mut self, key: &str, val: Vec<u8>) {
        self.tags.insert(key.to_string(), NbtTag::ByteArray(val));
    }

    pub fn put_boolean(&mut self, key: &str, val: bool) {
        self.put_byte(key, if val { 1 } else { 0 });
    }

    pub fn put_compound(&mut self, key: &str, val: CompoundTag) {
        self.tags.insert(key.to_string(), NbtTag::Compound(val.tags));
    }

    pub fn get(&self, key: &str) -> Option<&NbtTag> {
        self.tags.get(key)
    }

    pub fn get_int(&self, key: &str) -> i32 {
        self.tags.get(key).and_then(|t| t.as_int()).unwrap_or(0)
    }

    pub fn get_byte(&self, key: &str) -> i8 {
        self.tags.get(key).and_then(|t| t.as_byte()).unwrap_or(0)
    }

    pub fn get_short(&self, key: &str) -> i16 {
        self.tags.get(key).and_then(|t| t.as_short()).unwrap_or(0)
    }

    pub fn get_long(&self, key: &str) -> i64 {
        self.tags.get(key).and_then(|t| t.as_long()).unwrap_or(0)
    }

    pub fn get_float(&self, key: &str) -> f32 {
        self.tags.get(key).and_then(|t| t.as_float()).unwrap_or(0.0)
    }

    pub fn get_double(&self, key: &str) -> f64 {
        self.tags.get(key).and_then(|t| t.as_double()).unwrap_or(0.0)
    }

    pub fn get_string(&self, key: &str) -> &str {
        self.tags.get(key).and_then(|t| t.as_string()).unwrap_or("")
    }

    pub fn get_boolean(&self, key: &str) -> bool {
        self.get_byte(key) != 0
    }

    pub fn get_compound(&self, key: &str) -> Option<CompoundTag> {
        self.tags.get(key).and_then(|t| {
            if let NbtTag::Compound(m) = t {
                Some(CompoundTag { name: key.to_string(), tags: m.clone() })
            } else {
                None
            }
        })
    }

    pub fn contains(&self, key: &str) -> bool {
        self.tags.contains_key(key)
    }
}

/// NBT binary I/O utilities matching NbtIo.h
pub struct NbtIo;

impl NbtIo {
    fn write_string(out: &mut Vec<u8>, s: &str) {
        let bytes = s.as_bytes();
        out.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
        out.extend_from_slice(bytes);
    }

    fn write_tag(out: &mut Vec<u8>, tag: &NbtTag) {
        match tag {
            NbtTag::End => {},
            NbtTag::Byte(v) => out.push(*v as u8),
            NbtTag::Short(v) => out.extend_from_slice(&v.to_be_bytes()),
            NbtTag::Int(v) => out.extend_from_slice(&v.to_be_bytes()),
            NbtTag::Long(v) => out.extend_from_slice(&v.to_be_bytes()),
            NbtTag::Float(v) => out.extend_from_slice(&v.to_be_bytes()),
            NbtTag::Double(v) => out.extend_from_slice(&v.to_be_bytes()),
            NbtTag::ByteArray(v) => {
                out.extend_from_slice(&(v.len() as i32).to_be_bytes());
                out.extend_from_slice(v);
            },
            NbtTag::String(v) => Self::write_string(out, v),
            NbtTag::List(items) => {
                let list_type = items.first().map(|t| t.type_id()).unwrap_or(0);
                out.push(list_type);
                out.extend_from_slice(&(items.len() as i32).to_be_bytes());
                for item in items {
                    Self::write_tag(out, item);
                }
            },
            NbtTag::Compound(map) => {
                for (name, val) in map {
                    out.push(val.type_id());
                    Self::write_string(out, name);
                    Self::write_tag(out, val);
                }
                out.push(0); // End tag
            },
        }
    }

    /// Serialize a CompoundTag to binary NBT bytes.
    pub fn write(tag: &CompoundTag) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(10); // Compound tag type
        Self::write_string(&mut out, &tag.name);
        let compound = NbtTag::Compound(tag.tags.clone());
        Self::write_tag(&mut out, &compound);
        out
    }

    fn read_string(data: &[u8], pos: &mut usize) -> String {
        if *pos + 2 > data.len() { return String::new(); }
        let len = u16::from_be_bytes([data[*pos], data[*pos + 1]]) as usize;
        *pos += 2;
        if *pos + len > data.len() { return String::new(); }
        let s = String::from_utf8_lossy(&data[*pos..*pos + len]).to_string();
        *pos += len;
        s
    }

    fn read_tag(data: &[u8], pos: &mut usize, tag_type: u8) -> NbtTag {
        match tag_type {
            0 => NbtTag::End,
            1 => { let v = data[*pos] as i8; *pos += 1; NbtTag::Byte(v) },
            2 => { let v = i16::from_be_bytes([data[*pos], data[*pos+1]]); *pos += 2; NbtTag::Short(v) },
            3 => { let v = i32::from_be_bytes([data[*pos], data[*pos+1], data[*pos+2], data[*pos+3]]); *pos += 4; NbtTag::Int(v) },
            4 => { let mut b = [0u8; 8]; b.copy_from_slice(&data[*pos..*pos+8]); *pos += 8; NbtTag::Long(i64::from_be_bytes(b)) },
            5 => { let v = f32::from_be_bytes([data[*pos], data[*pos+1], data[*pos+2], data[*pos+3]]); *pos += 4; NbtTag::Float(v) },
            6 => { let mut b = [0u8; 8]; b.copy_from_slice(&data[*pos..*pos+8]); *pos += 8; NbtTag::Double(f64::from_be_bytes(b)) },
            7 => {
                let len = i32::from_be_bytes([data[*pos], data[*pos+1], data[*pos+2], data[*pos+3]]) as usize;
                *pos += 4;
                let arr = data[*pos..*pos+len].to_vec();
                *pos += len;
                NbtTag::ByteArray(arr)
            },
            8 => NbtTag::String(Self::read_string(data, pos)),
            9 => {
                let list_type = data[*pos]; *pos += 1;
                let count = i32::from_be_bytes([data[*pos], data[*pos+1], data[*pos+2], data[*pos+3]]) as usize;
                *pos += 4;
                let mut items = Vec::with_capacity(count);
                for _ in 0..count {
                    items.push(Self::read_tag(data, pos, list_type));
                }
                NbtTag::List(items)
            },
            10 => {
                let mut map = HashMap::new();
                loop {
                    if *pos >= data.len() { break; }
                    let child_type = data[*pos]; *pos += 1;
                    if child_type == 0 { break; }
                    let name = Self::read_string(data, pos);
                    let val = Self::read_tag(data, pos, child_type);
                    map.insert(name, val);
                }
                NbtTag::Compound(map)
            },
            _ => NbtTag::End,
        }
    }

    /// Deserialize binary NBT bytes into a CompoundTag.
    pub fn read(data: &[u8]) -> Option<CompoundTag> {
        if data.is_empty() { return None; }
        let mut pos = 0usize;
        let tag_type = data[pos]; pos += 1;
        if tag_type != 10 { return None; } // Must be compound
        let name = Self::read_string(data, &mut pos);
        if let NbtTag::Compound(tags) = Self::read_tag(data, &mut pos, 10) {
            Some(CompoundTag { name, tags })
        } else {
            None
        }
    }
}

// ---- Backwards-compatible free functions used by storage_api.rs ----

use crate::data_io::{DataOutput, DataInput, MemoryDataOutput, MemoryDataInput};
use std::collections::BTreeMap;

fn write_tag_le(out: &mut MemoryDataOutput, tag: &TagValue) {
    match tag {
        TagValue::End => {},
        TagValue::Byte(v) => out.write_byte(*v),
        TagValue::Short(v) => out.write_short(*v),
        TagValue::Int(v) => out.write_int(*v),
        TagValue::Long(v) => out.write_long_long(*v),
        TagValue::Float(v) => out.write_float(*v),
        TagValue::Double(v) => out.write_double(*v),
        TagValue::ByteArray(v) => {
            out.write_int(v.len() as i32);
            out.write_bytes(v);
        },
        TagValue::String(v) => out.write_string(v),
        TagValue::List(items) => {
            let list_type = items.first().map(|t| t.type_id()).unwrap_or(0);
            out.write_byte(list_type as i8);
            out.write_int(items.len() as i32);
            for item in items {
                write_tag_le(out, item);
            }
        },
        TagValue::Compound(map) => {
            for (name, val) in map {
                out.write_byte(val.type_id() as i8);
                out.write_string(name);
                write_tag_le(out, val);
            }
            out.write_byte(0);
        },
    }
}

/// Write a root compound tag using LE DataOutput (MCPE Bedrock format).
pub fn write_root_compound(out: &mut MemoryDataOutput, name: &str, tags: BTreeMap<String, TagValue>) {
    out.write_byte(10);
    out.write_string(name);
    let nbt_map: HashMap<String, NbtTag> = tags.into_iter().collect();
    let compound = TagValue::Compound(nbt_map);
    write_tag_le(out, &compound);
}

fn read_tag_le(inp: &mut MemoryDataInput, tag_type: u8) -> TagValue {
    match tag_type {
        0 => TagValue::End,
        1 => TagValue::Byte(inp.read_byte()),
        2 => TagValue::Short(inp.read_short()),
        3 => TagValue::Int(inp.read_int()),
        4 => TagValue::Long(inp.read_long_long()),
        5 => TagValue::Float(inp.read_float()),
        6 => TagValue::Double(inp.read_double()),
        7 => {
            let len = inp.read_int() as usize;
            TagValue::ByteArray(inp.read_bytes(len))
        },
        8 => TagValue::String(inp.read_string()),
        9 => {
            let list_type = inp.read_byte() as u8;
            let count = inp.read_int() as usize;
            let mut items = Vec::with_capacity(count);
            for _ in 0..count {
                items.push(read_tag_le(inp, list_type));
            }
            TagValue::List(items)
        },
        10 => {
            let mut map = HashMap::new();
            loop {
                let child_type = inp.read_byte() as u8;
                if child_type == 0 { break; }
                let name = inp.read_string();
                let val = read_tag_le(inp, child_type);
                map.insert(name, val);
            }
            TagValue::Compound(map)
        },
        _ => TagValue::End,
    }
}

/// Read a root compound tag from LE DataInput (MCPE Bedrock format).
pub fn read_root_compound(inp: &mut MemoryDataInput) -> Option<(String, BTreeMap<String, TagValue>)> {
    let tag_type = inp.read_byte() as u8;
    if tag_type != 10 { return None; }
    let name = inp.read_string();
    if let TagValue::Compound(hmap) = read_tag_le(inp, 10) {
        let btree: BTreeMap<String, TagValue> = hmap.into_iter().collect();
        Some((name, btree))
    } else {
        None
    }
}
