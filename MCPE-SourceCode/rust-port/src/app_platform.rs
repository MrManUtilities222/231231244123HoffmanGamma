use std::path::Path;

pub struct TextureData {
    // Placeholder, need to define based on actual usage
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct BinaryBlob {
    pub data: Option<Vec<u8>>,
}

impl BinaryBlob {
    pub fn new() -> Self {
        Self { data: None }
    }

    pub fn with_data(data: Vec<u8>) -> Self {
        Self { data: Some(data) }
    }
}

pub trait AppPlatform {
    fn save_screenshot(&self, _filename: &str, _gl_width: i32, _gl_height: i32) {}
    fn load_texture(&self, filename: &str, texture_folder: bool) -> TextureData {
        // Default implementation, return empty
        TextureData {
            data: vec![],
            width: 0,
            height: 0,
        }
    }
    fn play_sound(&self, _filename: &str, _volume: f32, _pitch: f32) {}
    fn show_dialog(&self, _dialog_id: i32) {}
    fn create_user_input(&self) {}
    fn get_user_input_status(&self) -> i32 { 0 }
    fn get_user_input(&self) -> Vec<String> { vec![] }
    fn get_date_string(&self, _time: i64) -> String { String::new() }
    fn check_license(&self) -> i32 { 0 }
    fn has_buy_button_when_invalid_license(&self) -> bool { false }
    fn is_big_endian(&self) -> bool {
        // Check endianness
        let num: u32 = 0x01020304;
        let bytes = num.to_le_bytes();
        bytes[0] == 1
    }
}