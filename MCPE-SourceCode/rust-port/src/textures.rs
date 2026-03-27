use glow::HasContext;
use std::rc::Rc;
use std::collections::HashMap;

pub struct Textures {
    gl: Rc<glow::Context>,
    textures: HashMap<String, glow::NativeTexture>,
}

impl Textures {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        Self {
            gl,
            textures: HashMap::new(),
        }
    }

    pub fn load_and_bind_texture(&mut self, path: &str) -> Option<glow::NativeTexture> {
        if let Some(&tex) = self.textures.get(path) {
            unsafe { self.gl.bind_texture(glow::TEXTURE_2D, Some(tex)); }
            return Some(tex);
        }

        let img = match image::open(path) {
            Ok(i) => i.into_rgba8(),
            Err(e) => {
                println!("Failed to load texture {}: {}", path, e);
                return None;
            }
        };

        let tex = unsafe {
            let texture = self.gl.create_texture().unwrap();
            self.gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            self.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&img.into_raw())
            );
            // Crisp, nearest-neighbor filtering for proper voxel look
            self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            self.gl.bind_texture(glow::TEXTURE_2D, None);
            texture
        };

        self.textures.insert(path.to_string(), tex);
        unsafe { self.gl.bind_texture(glow::TEXTURE_2D, Some(tex)); }
        Some(tex)
    }

    pub fn unbind(&self) {
        unsafe { self.gl.bind_texture(glow::TEXTURE_2D, None); }
    }
}
