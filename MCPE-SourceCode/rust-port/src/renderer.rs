use crate::tesselator::Tesselator;
use crate::item_instance::ItemInstance;
use crate::level::Level;
use crate::mob_entity::MobEntity;
use std::rc::Rc;
use glow::HasContext;

pub struct GameRenderer {
    pub gl: Rc<glow::Context>,
    pub program: glow::Program,
    pub vao: glow::VertexArray,
    pub vbo: glow::Buffer,
    pub terrain_texture: glow::NativeTexture,
    pub gui_texture: glow::NativeTexture,
    pub font_texture: glow::NativeTexture,
    pub index_count: i32,
    pub textures: crate::textures::Textures,
}

impl GameRenderer {
    pub fn new(gl_context: Option<Rc<glow::Context>>) -> Self {
        let gl = gl_context.expect("OpenGL Context required");
        let mut textures = crate::textures::Textures::new(gl.clone());
        let terrain_texture = textures.load_and_bind_texture("../data/images/terrain.png").unwrap();
        let gui_texture = textures.load_and_bind_texture("../data/images/gui/gui.png").unwrap();
        let font_texture = textures.load_and_bind_texture("../data/images/font/default8.png").unwrap();

        let (program, vao, vbo) = unsafe {
            let vs = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vs, r#"#version 330 core
                layout (location = 0) in vec3 aPos;
                layout (location = 1) in vec2 aTexCoord;
                out vec2 TexCoord;
                uniform mat4 mvp;
                void main() {
                    gl_Position = mvp * vec4(aPos, 1.0);
                    TexCoord = aTexCoord;
                }
            "#);
            gl.compile_shader(vs);

            let fs = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(fs, r#"#version 330 core
                in vec2 TexCoord;
                out vec4 FragColor;
                uniform sampler2D texture1;
                void main() {
                    vec4 texColor = texture(texture1, TexCoord);
                    if(texColor.a < 0.1) discard;
                    FragColor = texColor;
                }
            "#);
            gl.compile_shader(fs);

            let program = gl.create_program().unwrap();
            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);

            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let stride = 5 * std::mem::size_of::<f32>() as i32;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, stride, 3 * std::mem::size_of::<f32>() as i32);
            gl.enable_vertex_attrib_array(1);

            (program, vao, vbo)
        };

        Self {
            gl, program, vao, vbo, terrain_texture, gui_texture, font_texture, index_count: 0, textures
        }
    }

    pub fn upload_mesh(&mut self, vertices: &[f32]) {
        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            let bytes: &[u8] = std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<f32>(),
            );
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes, glow::DYNAMIC_DRAW);
            self.index_count = (vertices.len() / 5) as i32;
        }
    }

    pub fn render_scene(&self, proj: &[f32; 16], pos: [f32; 3], rot: [f32; 2]) {
        unsafe {
            self.gl.use_program(Some(self.program));
            let mvp_loc = self.gl.get_uniform_location(self.program, "mvp");

            // Simple MVP matrix using standard 3D math (transposed projection)
            let mut view = crate::mth::identity();
            crate::mth::rotate_x(&mut view, -rot[1]); // Pitch
            crate::mth::rotate_y(&mut view, -rot[0]); // Yaw
            crate::mth::translate(&mut view, -pos[0], -pos[1] - 1.62, -pos[2]); // Camera translation

            let mut mvp = [0.0; 16];
            crate::mth::multiply_matrix(&mut mvp, proj, &view);

            self.gl.uniform_matrix_4_f32_slice(mvp_loc.as_ref(), false, &mvp);
            
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.terrain_texture));
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.draw_arrays(glow::TRIANGLES, 0, self.index_count);
        }
    }

    pub fn render_gui(&self, ortho: &[f32; 16], vertices: &[f32], use_gui_png: bool) {
        if vertices.is_empty() { return; }
        unsafe {
            self.gl.use_program(Some(self.program));
            let mvp_loc = self.gl.get_uniform_location(self.program, "mvp");
            self.gl.uniform_matrix_4_f32_slice(mvp_loc.as_ref(), false, ortho);

            if use_gui_png {
                self.gl.bind_texture(glow::TEXTURE_2D, Some(self.gui_texture));
            } else {
                self.gl.bind_texture(glow::TEXTURE_2D, Some(self.terrain_texture));
            }

            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            let bytes: &[u8] = std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<f32>(),
            );
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes, glow::DYNAMIC_DRAW);
            
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.draw_arrays(glow::TRIANGLES, 0, (vertices.len() / 5) as i32);
        }
    }

    pub fn set_viewport(&self, width: i32, height: i32) {
        unsafe {
            self.gl.viewport(0, 0, width, height);
        }
    }

    pub fn render_gui_tex(&self, ortho: &[f32; 16], vertices: &[f32], tex: glow::NativeTexture) {
        if vertices.is_empty() { return; }
        unsafe {
            self.gl.use_program(Some(self.program));
            let mvp_loc = self.gl.get_uniform_location(self.program, "mvp");
            self.gl.uniform_matrix_4_f32_slice(mvp_loc.as_ref(), false, ortho);

            self.gl.bind_texture(glow::TEXTURE_2D, Some(tex));

            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            let bytes: &[u8] = std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<f32>(),
            );
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes, glow::DYNAMIC_DRAW);
            
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.draw_arrays(glow::TRIANGLES, 0, (vertices.len() / 5) as i32);
        }
    }
}

pub struct ItemInHandRenderer {
    pub height: f32,
    pub o_height: f32,
    pub last_slot: i32,
}

impl ItemInHandRenderer {
    pub fn new() -> Self {
        Self { height: 0.0, o_height: 0.0, last_slot: 0 }
    }

    pub fn tick(&mut self, is_holding_item: bool, slot: i32) {
        self.o_height = self.height;

        // Animate item lowering and rising when switching slots
        if self.last_slot != slot {
            self.last_slot = slot;
            // Force lower the item
            self.height = 0.0;
        }

        let mut target_height = 0.0;
        if is_holding_item { target_height = 1.0; }

        self.height += (target_height - self.height) * 0.4;
        if self.height < 0.1 { self.height = 0.1; }
        if self.height > 1.0 { self.height = 1.0; }
    }

    pub fn render_item(&self, t: &mut Tesselator, item: &ItemInstance, scale: f32, equip_progress: f32, swing_progress: f32) {
        if item.is_null() { return; }

        let is_block = item.id < 256;

        // Basic placeholder transform logic (actual math will apply matrices)
        // Since we are pushing vertices directly from CPU for this:
        t.color(1.0, 1.0, 1.0);
        
        let y_offset = -0.5 * (1.0 - equip_progress);

        if is_block {
            // Draw a tiny 3D block
            let s = 0.4 * scale;
            let cx = 0.5; let cy = -0.5 + y_offset; let cz = -0.8;
            
            // Map textures
            let mut tex_id = item.id;
            // Temporary mapping 
            let u0 = ((tex_id % 16) as f32) / 16.0;
            let v0 = ((tex_id / 16) as f32) / 16.0;
            let u1 = u0 + 1.0/16.0;
            let v1 = v0 + 1.0/16.0;

            t.vertex_uv(cx-s, cy-s, cz+s, u0, v1);
            t.vertex_uv(cx+s, cy-s, cz+s, u1, v1);
            t.vertex_uv(cx+s, cy+s, cz+s, u1, v0);
            
            t.vertex_uv(cx+s, cy+s, cz+s, u1, v0);
            t.vertex_uv(cx-s, cy+s, cz+s, u0, v0);
            t.vertex_uv(cx-s, cy-s, cz+s, u0, v1);
            
            // Other faces omitted for speed, full block renderer in next pass
        } else {
            // Draw 2D sprite
            let s = 0.5 * scale;
            let cx = 0.6; let cy = -0.4 + y_offset; let cz = -0.9;

            let mut tex_id = item.get_aux_value(); // usually mapped by Item definitions
            let u0 = ((tex_id % 16) as f32) / 16.0;
            let v0 = ((tex_id / 16) as f32) / 16.0;
            let u1 = u0 + 1.0/16.0;
            let v1 = v0 + 1.0/16.0;

            t.vertex_uv(cx-s, cy-s, cz, u0, v1);
            t.vertex_uv(cx+s, cy-s, cz, u1, v1);
            t.vertex_uv(cx+s, cy+s, cz, u1, v0);
            
            t.vertex_uv(cx+s, cy+s, cz, u1, v0);
            t.vertex_uv(cx-s, cy+s, cz, u0, v0);
            t.vertex_uv(cx-s, cy-s, cz, u0, v1);
        }
    }
}

pub struct LevelRenderer {
    pub view_distance: f32,
    pub clouds_enabled: bool,
    // Holds chunks to rerender
}

impl LevelRenderer {
    pub fn new() -> Self {
        Self {
            view_distance: 64.0,
            clouds_enabled: true,
        }
    }

    pub fn tick(&mut self, _level: &Level) {
        // Find dirty chunks, upload them
    }

    pub fn render_sky(&self, t: &mut Tesselator, _time: f32) {
        t.color(0.5, 0.7, 1.0); // Simple azure sky
        let s = 100.0;
        let y = 50.0;
        // Top cap
        t.vertex_uv(-s, y, -s, 0.0, 0.0);
        t.vertex_uv(s, y, -s, 1.0, 0.0);
        t.vertex_uv(s, y, s, 1.0, 1.0);

        t.vertex_uv(s, y, s, 1.0, 1.0);
        t.vertex_uv(-s, y, s, 0.0, 1.0);
        t.vertex_uv(-s, y, -s, 0.0, 0.0);
    }
    
    pub fn render_clouds(&self, t: &mut Tesselator, time: f32) {
        if !self.clouds_enabled { return; }
        t.color(1.0, 1.0, 1.0);
        
        let cx = (time * 0.1) % 2048.0;
        let s = 512.0;
        let y = 120.0;

        let u0 = cx / 2048.0;
        let v0 = 0.0;
        let u1 = (cx + s) / 2048.0;
        let v1 = s / 2048.0;

        t.vertex_uv(-s, y, -s, u0, v0);
        t.vertex_uv(s, y, -s, u1, v0);
        t.vertex_uv(s, y, s, u1, v1);

        t.vertex_uv(s, y, s, u1, v1);
        t.vertex_uv(-s, y, s, u0, v1);
        t.vertex_uv(-s, y, -s, u0, v0);
    }

    pub fn render_entities(&self, t: &mut Tesselator, mobs: &[MobEntity], partial_ticks: f32) {
        // The main model engine parses this
    }
}