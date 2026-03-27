// Simple 2D Value Noise for terrain generation

pub struct Noise2D {
    seed: i32,
}

impl Noise2D {
    pub fn new(seed: i32) -> Self {
        Self { seed }
    }

    fn hash(&self, mut x: i32, mut y: i32) -> f32 {
        let mut n = x.wrapping_add(y.wrapping_mul(57)).wrapping_add(self.seed);
        n = (n << 13) ^ n;
        let p = (n.wrapping_mul(n.wrapping_mul(n).wrapping_mul(15731).wrapping_add(789221)).wrapping_add(1376312589)) & 0x7fffffff;
        1.0 - (p as f32 / 1073741824.0)
    }

    fn interpolate(&self, a: f32, b: f32, x: f32) -> f32 {
        let ft = x * std::f32::consts::PI;
        let f = (1.0 - ft.cos()) * 0.5;
        a * (1.0 - f) + b * f
    }

    pub fn get_noise(&self, x: f32, y: f32) -> f32 {
        let int_x = x.floor() as i32;
        let int_y = y.floor() as i32;
        let frac_x = x - int_x as f32;
        let frac_y = y - int_y as f32;

        let v1 = self.hash(int_x, int_y);
        let v2 = self.hash(int_x + 1, int_y);
        let v3 = self.hash(int_x, int_y + 1);
        let v4 = self.hash(int_x + 1, int_y + 1);

        let i1 = self.interpolate(v1, v2, frac_x);
        let i2 = self.interpolate(v3, v4, frac_x);

        self.interpolate(i1, i2, frac_y)
    }

    // Octave noise
    pub fn get_octave_noise(&self, x: f32, y: f32, octaves: i32) -> f32 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            total += self.get_noise(x * frequency, y * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        total / max_value
    }
}
