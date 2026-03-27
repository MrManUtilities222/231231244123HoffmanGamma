pub struct Tesselator {
    vertices: Vec<f32>,
    current_color: [f32; 3],
    current_uv: [f32; 2],
    offset: [f32; 3],
}

impl Default for Tesselator {
    fn default() -> Self {
        Self::new()
    }
}

impl Tesselator {
    pub fn new() -> Self {
        Self {
            vertices: Vec::with_capacity(4096),
            current_color: [1.0, 1.0, 1.0],
            current_uv: [0.0, 0.0],
            offset: [0.0, 0.0, 0.0],
        }
    }

    pub fn begin(&mut self) {
        self.vertices.clear();
    }

    pub fn color(&mut self, r: f32, g: f32, b: f32) {
        self.current_color = [r, g, b];
    }

    pub fn tex(&mut self, u: f32, v: f32) {
        self.current_uv = [u, v];
    }

    pub fn offset(&mut self, x: f32, y: f32, z: f32) {
        self.offset = [x, y, z];
    }

    pub fn add_offset(&mut self, x: f32, y: f32, z: f32) {
        self.offset[0] += x;
        self.offset[1] += y;
        self.offset[2] += z;
    }

    pub fn vertex(&mut self, x: f32, y: f32, z: f32) {
        // Position
        self.vertices.push(x + self.offset[0]);
        self.vertices.push(y + self.offset[1]);
        self.vertices.push(z + self.offset[2]);
        
        // Color
        self.vertices.push(self.current_color[0]);
        self.vertices.push(self.current_color[1]);
        self.vertices.push(self.current_color[2]);
        
        // UV
        self.vertices.push(self.current_uv[0]);
        self.vertices.push(self.current_uv[1]);
    }

    pub fn vertex_uv(&mut self, x: f32, y: f32, z: f32, u: f32, v: f32) {
        self.tex(u, v);
        self.vertex(x, y, z);
    }

    pub fn end(&mut self) -> Vec<f32> {
        let mut result = Vec::with_capacity(self.vertices.len());
        std::mem::swap(&mut result, &mut self.vertices);
        result
    }
}
