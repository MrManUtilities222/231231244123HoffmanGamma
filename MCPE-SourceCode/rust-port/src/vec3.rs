#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(mut x: f32, mut y: f32, mut z: f32) -> Self {
        if x == -0.0 {
            x = 0.0;
        }
        if y == -0.0 {
            y = 0.0;
        }
        if z == -0.0 {
            z = 0.0;
        }
        Self { x, y, z }
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn normalized(self) -> Self {
        let dist = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if dist < 0.0001 {
            return Self::default();
        }
        Self::new(self.x / dist, self.y / dist, self.z / dist)
    }

    pub fn dot(self, p: Self) -> f32 {
        self.x * p.x + self.y * p.y + self.z * p.z
    }

    pub fn cross(self, p: Self) -> Self {
        Self::new(
            self.y * p.z - self.z * p.y,
            self.z * p.x - self.x * p.z,
            self.x * p.y - self.y * p.x,
        )
    }

    pub fn add(self, x: f32, y: f32, z: f32) -> Self {
        Self::new(self.x + x, self.y + y, self.z + z)
    }

    pub fn sub(self, x: f32, y: f32, z: f32) -> Self {
        Self::new(self.x - x, self.y - y, self.z - z)
    }

    pub fn distance_to(self, p: Self) -> f32 {
        let xd = p.x - self.x;
        let yd = p.y - self.y;
        let zd = p.z - self.z;
        (xd * xd + yd * yd + zd * zd).sqrt()
    }

    pub fn distance_to_sqr(self, p: Self) -> f32 {
        let xd = p.x - self.x;
        let yd = p.y - self.y;
        let zd = p.z - self.z;
        xd * xd + yd * yd + zd * zd
    }

    pub fn clip_x(self, b: Self, xt: f32) -> Option<Self> {
        let xd = b.x - self.x;
        let yd = b.y - self.y;
        let zd = b.z - self.z;
        if xd * xd < 0.0000001 {
            return None;
        }
        let d = (xt - self.x) / xd;
        if !(0.0..=1.0).contains(&d) {
            return None;
        }
        Some(Self::new(self.x + xd * d, self.y + yd * d, self.z + zd * d))
    }

    pub fn clip_y(self, b: Self, yt: f32) -> Option<Self> {
        let xd = b.x - self.x;
        let yd = b.y - self.y;
        let zd = b.z - self.z;
        if yd * yd < 0.0000001 {
            return None;
        }
        let d = (yt - self.y) / yd;
        if !(0.0..=1.0).contains(&d) {
            return None;
        }
        Some(Self::new(self.x + xd * d, self.y + yd * d, self.z + zd * d))
    }

    pub fn clip_z(self, b: Self, zt: f32) -> Option<Self> {
        let xd = b.x - self.x;
        let yd = b.y - self.y;
        let zd = b.z - self.z;
        if zd * zd < 0.0000001 {
            return None;
        }
        let d = (zt - self.z) / zd;
        if !(0.0..=1.0).contains(&d) {
            return None;
        }
        Some(Self::new(self.x + xd * d, self.y + yd * d, self.z + zd * d))
    }
}

