use crate::tesselator::Tesselator;

/// Bitmap font renderer ported from MCPE's Font.cpp.
/// Loads a 128x128 PNG glyph sheet (16x16 grid of 8x8 chars)
/// and produces textured quads for each character.
pub struct Font {
    /// Per-character pixel widths, computed by scanning the glyph sheet alpha.
    pub char_widths: [i32; 256],
}

impl Font {
    /// Create a new Font by scanning `default8.png` pixel data to determine
    /// each glyph's actual width (the rightmost non-transparent column + 2).
    pub fn new(font_path: &str) -> Self {
        let mut char_widths = [8i32; 256];

        // Attempt to load the glyph sheet and compute widths
        if let Ok(img) = image::open(font_path) {
            let rgba = img.to_rgba8();
            let (img_w, _img_h) = rgba.dimensions();

            let cols = 16u32;
            let _rows = 16u32;

            for i in 0..256usize {
                let xt = (i as u32) % cols;
                let yt = (i as u32) / cols;

                let mut last_col = 0i32;
                for x in (0..8).rev() {
                    let px = xt * 8 + x;
                    let mut empty = true;
                    for y in 0..8u32 {
                        let py = yt * 8 + y;
                        if px < img_w && py < rgba.height() {
                            let pixel = rgba.get_pixel(px, py);
                            if pixel[3] > 0 {
                                empty = false;
                                break;
                            }
                        }
                    }
                    if !empty {
                        last_col = x as i32;
                        break;
                    }
                }

                // Space gets a fixed width
                if i == b' ' as usize {
                    last_col = 2;
                }

                char_widths[i] = last_col + 2;
            }
        }

        Self { char_widths }
    }

    /// Measure the pixel width of a string.
    pub fn width(&self, text: &str) -> i32 {
        let mut w = 0i32;
        for ch in text.bytes() {
            w += self.char_widths[ch as usize];
        }
        w
    }

    /// Build textured quads for `text` at position `(x, y)` with a given
    /// color (0xAARRGGBB). Returns the raw vertex data compatible with the
    /// GUI shader (stride = 8 floats: xyz rgb uv).
    pub fn draw(&self, text: &str, x: f32, y: f32, color: u32) -> Vec<f32> {
        let mut t = Tesselator::new();
        t.begin();

        let r = ((color >> 16) & 0xFF) as f32 / 255.0;
        let g = ((color >> 8) & 0xFF) as f32 / 255.0;
        let b = (color & 0xFF) as f32 / 255.0;
        t.color(r, g, b);

        let mut cursor_x = x;
        let cursor_y = y;
        let scale = 1.0f32; // 1:1 pixel mapping

        for ch in text.bytes() {
            let ix = ((ch & 15) as f32) * 8.0;
            let iy = ((ch >> 4) as f32) * 8.0;
            let s = 7.99f32;

            let u0 = ix / 128.0;
            let v0 = iy / 128.0;
            let u1 = (ix + s) / 128.0;
            let v1 = (iy + s) / 128.0;

            let x0 = cursor_x;
            let y0 = cursor_y;
            let x1 = cursor_x + s * scale;
            let y1 = cursor_y + s * scale;

            // Triangle 1
            t.vertex_uv(x0, y1, 0.0, u0, v1);
            t.vertex_uv(x1, y1, 0.0, u1, v1);
            t.vertex_uv(x1, y0, 0.0, u1, v0);

            // Triangle 2
            t.vertex_uv(x1, y0, 0.0, u1, v0);
            t.vertex_uv(x0, y0, 0.0, u0, v0);
            t.vertex_uv(x0, y1, 0.0, u0, v1);

            cursor_x += self.char_widths[ch as usize] as f32 * scale;
        }

        t.end()
    }

    /// Draw text with a dark shadow offset by (1,1) behind it.
    pub fn draw_shadow(&self, text: &str, x: f32, y: f32, color: u32) -> Vec<f32> {
        // Shadow color: quarter brightness
        let sr = (((color >> 16) & 0xFF) / 4) as u32;
        let sg = (((color >> 8) & 0xFF) / 4) as u32;
        let sb = ((color & 0xFF) / 4) as u32;
        let shadow_color = (sr << 16) | (sg << 8) | sb;

        let mut verts = self.draw(text, x + 1.0, y + 1.0, shadow_color);
        let front = self.draw(text, x, y, color);
        verts.extend_from_slice(&front);
        verts
    }
}
