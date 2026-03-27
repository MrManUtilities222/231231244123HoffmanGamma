#[derive(Clone)]
pub struct Random {
    seed: i64,
    mt: [u32; Self::N],
    mti: usize,
    have_next_next_gaussian: bool,
    next_next_gaussian: f32,
}

impl Random {
    const N: usize = 624;
    const M: usize = 397;
    const MATRIX_A: u32 = 0x9908_b0df;
    const UPPER_MASK: u32 = 0x8000_0000;
    const LOWER_MASK: u32 = 0x7fff_ffff;

    pub fn new(seed: i64) -> Self {
        let mut out = Self {
            seed,
            mt: [0; Self::N],
            mti: Self::N + 1,
            have_next_next_gaussian: false,
            next_next_gaussian: 0.0,
        };
        out.set_seed(seed);
        out
    }

    pub fn set_seed(&mut self, seed: i64) {
        self.seed = seed;
        self.mti = Self::N + 1;
        self.have_next_next_gaussian = false;
        self.next_next_gaussian = 0.0;
        self.init_genrand(seed as u32);
    }

    pub fn seed(&self) -> i64 {
        self.seed
    }

    pub fn next_boolean(&mut self) -> bool {
        (self.genrand_int32() & 0x0800_0000) > 0
    }

    pub fn next_float(&mut self) -> f32 {
        self.genrand_real2() as f32
    }

    pub fn next_double(&mut self) -> f64 {
        self.genrand_real2()
    }

    pub fn next_int(&mut self) -> i32 {
        (self.genrand_int32() >> 1) as i32
    }

    pub fn next_int_n(&mut self, n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        (self.genrand_int32() % n as u32) as i32
    }

    pub fn next_long(&mut self) -> i32 {
        (self.genrand_int32() >> 1) as i32
    }

    pub fn next_long_n(&mut self, n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        (self.genrand_int32() % n as u32) as i32
    }

    pub fn next_gaussian(&mut self) -> f32 {
        if self.have_next_next_gaussian {
            self.have_next_next_gaussian = false;
            return self.next_next_gaussian;
        }

        let (v1, v2, s) = loop {
            let v1 = 2.0 * self.next_float() - 1.0;
            let v2 = 2.0 * self.next_float() - 1.0;
            let s = v1 * v1 + v2 * v2;
            if s < 1.0 && s != 0.0 {
                break (v1, v2, s);
            }
        };

        let multiplier = (-2.0 * s.ln() / s).sqrt();
        self.next_next_gaussian = v2 * multiplier;
        self.have_next_next_gaussian = true;
        v1 * multiplier
    }

    pub fn rr_diff_1(&mut self) -> f32 {
        let u = self.genrand_int32();
        let xx0 = (u & 0xffff) as f32 / 65536.0;
        let xx1 = ((u >> 16) & 0xffff) as f32 / 65536.0;
        xx0 - xx1
    }

    pub fn rr_diff_2(&mut self) -> (f32, f32) {
        let u = self.genrand_int32();
        let xx0 = (u & 0xff) as f32 / 256.0;
        let xx1 = ((u >> 8) & 0xff) as f32 / 256.0;
        let yy0 = ((u >> 16) & 0xff) as f32 / 256.0;
        let yy1 = ((u >> 24) & 0xff) as f32 / 256.0;
        (xx0 - xx1, yy0 - yy1)
    }

    pub fn rr_diff_3(&mut self) -> (f32, f32, f32) {
        let u = self.genrand_int32();
        let xx0 = (u & 0x1f) as f32 / 32.0;
        let xx1 = ((u >> 5) & 0x1f) as f32 / 32.0;
        let yy0 = ((u >> 10) & 0x1f) as f32 / 32.0;
        let yy1 = ((u >> 15) & 0x1f) as f32 / 32.0;
        let zz0 = ((u >> 20) & 0x1f) as f32 / 32.0;
        let zz1 = ((u >> 25) & 0x1f) as f32 / 32.0;
        (xx0 - xx1, yy0 - yy1, zz0 - zz1)
    }

    fn init_genrand(&mut self, s: u32) {
        self.mt[0] = s;
        for i in 1..Self::N {
            let prev = self.mt[i - 1];
            self.mt[i] = 1812433253u32
                .wrapping_mul(prev ^ (prev >> 30))
                .wrapping_add(i as u32);
        }
        self.mti = Self::N;
    }

    fn genrand_int32(&mut self) -> u32 {
        let mag01 = [0u32, Self::MATRIX_A];
        if self.mti >= Self::N {
            if self.mti == Self::N + 1 {
                self.init_genrand(5489);
            }

            for kk in 0..(Self::N - Self::M) {
                let y = (self.mt[kk] & Self::UPPER_MASK) | (self.mt[kk + 1] & Self::LOWER_MASK);
                self.mt[kk] = self.mt[kk + Self::M] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
            }
            for kk in (Self::N - Self::M)..(Self::N - 1) {
                let y = (self.mt[kk] & Self::UPPER_MASK) | (self.mt[kk + 1] & Self::LOWER_MASK);
                self.mt[kk] = self.mt[kk - (Self::N - Self::M)] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
            }
            let y = (self.mt[Self::N - 1] & Self::UPPER_MASK) | (self.mt[0] & Self::LOWER_MASK);
            self.mt[Self::N - 1] = self.mt[Self::M - 1] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
            self.mti = 0;
        }

        let mut y = self.mt[self.mti];
        self.mti += 1;

        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^= y >> 18;
        y
    }

    fn genrand_real2(&mut self) -> f64 {
        self.genrand_int32() as f64 * (1.0 / 4294967296.0)
    }
}

