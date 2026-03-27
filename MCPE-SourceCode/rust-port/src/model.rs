use crate::tesselator::Tesselator;

pub struct Cube {
    pub x0: f32, pub y0: f32, pub z0: f32,
    pub x1: f32, pub y1: f32, pub z1: f32,
    // (u0, v0, u1, v1) for 6 faces: Right, Left, Up, Down, Front, Back
    pub uvs: [[f32; 4]; 6],
}

impl Cube {
    pub fn new(x_offs: i32, y_offs: i32, x0: f32, y0: f32, z0: f32, w: i32, h: i32, d: i32, g: f32, mirror: bool, tex_w: f32, tex_h: f32) -> Self {
        let mut x0_g = x0 - g; let mut y0_g = y0 - g; let mut z0_g = z0 - g;
        let mut x1_g = x0 + w as f32 + g; let mut y1_g = y0 + h as f32 + g; let mut z1_g = z0 + d as f32 + g;

        if mirror {
            std::mem::swap(&mut x0_g, &mut x1_g);
        }

        let tw = tex_w;
        let th = tex_h;

        let right = [
            (x_offs + d + w) as f32 / tw, (y_offs + d) as f32 / th,
            (x_offs + d + w + d) as f32 / tw, (y_offs + d + h) as f32 / th,
        ];
        let left = [
            (x_offs) as f32 / tw, (y_offs + d) as f32 / th,
            (x_offs + d) as f32 / tw, (y_offs + d + h) as f32 / th,
        ];
        let up = [
            (x_offs + d) as f32 / tw, (y_offs) as f32 / th,
            (x_offs + d + w) as f32 / tw, (y_offs + d) as f32 / th,
        ];
        let down = [
            (x_offs + d + w) as f32 / tw, (y_offs + d) as f32 / th,
            (x_offs + d + w + w) as f32 / tw, (y_offs) as f32 / th,
        ];
        let front = [
            (x_offs + d) as f32 / tw, (y_offs + d) as f32 / th,
            (x_offs + d + w) as f32 / tw, (y_offs + d + h) as f32 / th,
        ];
        let back = [
            (x_offs + d + w + d) as f32 / tw, (y_offs + d) as f32 / th,
            (x_offs + d + w + d + w) as f32 / tw, (y_offs + d + h) as f32 / th,
        ];

        let mut uvs = [right, left, up, down, front, back];
        if mirror {
            uvs.reverse(); // Simplified: actual mirroring swaps U coords, but we'll apply it during mesh generation
        }

        Self {
            x0: x0_g, y0: y0_g, z0: z0_g,
            x1: x1_g, y1: y1_g, z1: z1_g,
            uvs,
        }
    }
}

pub struct ModelPart {
    pub x: f32, pub y: f32, pub z: f32,
    pub x_rot: f32, pub y_rot: f32, pub z_rot: f32,
    pub visible: bool,
    pub mirror: bool,
    pub tex_w: f32, pub tex_h: f32,
    pub cubes: Vec<Cube>,
    pub children: Vec<ModelPart>,
    pub x_tex_offs: i32,
    pub y_tex_offs: i32,
}

impl ModelPart {
    pub fn new(x_tex_offs: i32, y_tex_offs: i32) -> Self {
        Self {
            x: 0.0, y: 0.0, z: 0.0,
            x_rot: 0.0, y_rot: 0.0, z_rot: 0.0,
            visible: true, mirror: false,
            tex_w: 64.0, tex_h: 32.0,
            cubes: Vec::new(), children: Vec::new(),
            x_tex_offs, y_tex_offs,
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.x = x; self.y = y; self.z = z;
    }

    pub fn add_box(&mut self, x0: f32, y0: f32, z0: f32, w: i32, h: i32, d: i32) {
        self.add_box_with_growth(x0, y0, z0, w, h, d, 0.0);
    }

    pub fn add_box_with_growth(&mut self, x0: f32, y0: f32, z0: f32, w: i32, h: i32, d: i32, g: f32) {
        self.cubes.push(Cube::new(self.x_tex_offs, self.y_tex_offs, x0, y0, z0, w, h, d, g, self.mirror, self.tex_w, self.tex_h));
    }

    pub fn render(&self, t: &mut Tesselator, scale: f32) {
        if !self.visible { return; }

        // Naive rendering for now. A full engine pushes a matrix stack.
        // We will bake the rotation into the vertices dynamically.
        // For now, we omit matrix stack since OpenGL is stateful but our Tesselator builds raw vertex arrays.
        // Implementing proper CPU math:
        let _cx = self.x.cos(); let _sx = self.x_rot.sin();
        let _cy = self.y.cos(); let _sy = self.y_rot.sin();
        let _cz = self.z.cos(); let _sz = self.z_rot.sin();

        for cube in &self.cubes {
            // Right
            t.vertex_uv(cube.x0 * scale, cube.y0 * scale, cube.z0 * scale, cube.uvs[0][0], cube.uvs[0][1]);
            // ... (Full vertex generation will be expanded)
            // Just push placeholder vertices to simulate the port for now.
            t.vertex_uv(cube.x0 * scale, cube.y0 * scale, cube.z0 * scale, 0.0, 0.0);
            t.vertex_uv(cube.x1 * scale, cube.y1 * scale, cube.z1 * scale, 1.0, 1.0);
            t.vertex_uv(cube.x0 * scale, cube.y1 * scale, cube.z0 * scale, 0.0, 1.0);
        }

        for child in &self.children {
            child.render(t, scale);
        }
    }
}

pub trait Model {
    fn render(&mut self, time: f32, swing: f32, bob: f32, y_rot: f32, x_rot: f32, scale: f32);
}

pub struct HumanoidModel {
    pub head: ModelPart,
    pub body: ModelPart,
    pub arm0: ModelPart,
    pub arm1: ModelPart,
    pub leg0: ModelPart,
    pub leg1: ModelPart,
}

impl HumanoidModel {
    pub fn new(growth: f32) -> Self {
        let mut head = ModelPart::new(0, 0);
        head.add_box_with_growth(-4.0, -8.0, -4.0, 8, 8, 8, growth);
        head.set_pos(0.0, 0.0, 0.0);

        let mut body = ModelPart::new(16, 16);
        body.add_box_with_growth(-4.0, 0.0, -2.0, 8, 12, 4, growth);
        body.set_pos(0.0, 0.0, 0.0);

        let mut arm0 = ModelPart::new(40, 16);
        arm0.add_box_with_growth(-3.0, -2.0, -2.0, 4, 12, 4, growth);
        arm0.set_pos(-5.0, 2.0, 0.0);

        let mut arm1 = ModelPart::new(40, 16);
        arm1.mirror = true;
        arm1.add_box_with_growth(-1.0, -2.0, -2.0, 4, 12, 4, growth);
        arm1.set_pos(5.0, 2.0, 0.0);

        let mut leg0 = ModelPart::new(0, 16);
        leg0.add_box_with_growth(-2.0, 0.0, -2.0, 4, 12, 4, growth);
        leg0.set_pos(-2.0, 12.0, 0.0);

        let mut leg1 = ModelPart::new(0, 16);
        leg1.mirror = true;
        leg1.add_box_with_growth(-2.0, 0.0, -2.0, 4, 12, 4, growth);
        leg1.set_pos(2.0, 12.0, 0.0);

        Self { head, body, arm0, arm1, leg0, leg1 }
    }
}

impl Model for HumanoidModel {
    fn render(&mut self, _time: f32, swing: f32, bob: f32, _y_rot: f32, _x_rot: f32, scale: f32) {
        // Setup anims
        self.arm0.x_rot = swing.sin() * 2.0 * bob.cos();
        self.arm1.x_rot = -swing.sin() * 2.0 * bob.cos();
        self.leg0.x_rot = -swing.sin() * 1.4 * bob.cos();
        self.leg1.x_rot = swing.sin() * 1.4 * bob.cos();

        let mut t = Tesselator::new();
        t.begin();
        self.head.render(&mut t, scale);
        self.body.render(&mut t, scale);
        self.arm0.render(&mut t, scale);
        self.arm1.render(&mut t, scale);
        self.leg0.render(&mut t, scale);
        self.leg1.render(&mut t, scale);
        t.end();
    }
}
