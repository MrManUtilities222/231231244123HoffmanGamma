use crate::hit_result::HitResult;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            x0: 0.0,
            y0: 0.0,
            z0: 0.0,
            x1: 1.0,
            y1: 1.0,
            z1: 1.0,
        }
    }
}

impl Aabb {
    pub fn new(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> Self {
        Self { x0, y0, z0, x1, y1, z1 }
    }

    pub fn intersects(self, c: Self) -> bool {
        !(c.x1 <= self.x0
            || c.x0 >= self.x1
            || c.y1 <= self.y0
            || c.y0 >= self.y1
            || c.z1 <= self.z0
            || c.z0 >= self.z1)
    }

    pub fn intersects_inner(self, c: Self) -> bool {
        !(c.x1 < self.x0
            || c.x0 > self.x1
            || c.y1 < self.y0
            || c.y0 > self.y1
            || c.z1 < self.z0
            || c.z0 > self.z1)
    }

    pub fn intersects_vol(self, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> bool {
        !(x1 <= self.x0
            || x0 >= self.x1
            || y1 <= self.y0
            || y0 >= self.y1
            || z1 <= self.z0
            || z0 >= self.z1)
    }

    pub fn expand(self, xa: f32, ya: f32, za: f32) -> Self {
        let mut x0 = self.x0;
        let mut y0 = self.y0;
        let mut z0 = self.z0;
        let mut x1 = self.x1;
        let mut y1 = self.y1;
        let mut z1 = self.z1;

        if xa < 0.0 { x0 += xa; }
        if xa > 0.0 { x1 += xa; }
        if ya < 0.0 { y0 += ya; }
        if ya > 0.0 { y1 += ya; }
        if za < 0.0 { z0 += za; }
        if za > 0.0 { z1 += za; }

        Self { x0, y0, z0, x1, y1, z1 }
    }

    pub fn grow(self, xa: f32, ya: f32, za: f32) -> Self {
        Self {
            x0: self.x0 - xa,
            y0: self.y0 - ya,
            z0: self.z0 - za,
            x1: self.x1 + xa,
            y1: self.y1 + ya,
            z1: self.z1 + za,
        }
    }

    pub fn clone_move(self, xa: f32, ya: f32, za: f32) -> Self {
        Self {
            x0: self.x0 + xa,
            y0: self.y0 + ya,
            z0: self.z0 + za,
            x1: self.x1 + xa,
            y1: self.y1 + ya,
            z1: self.z1 + za,
        }
    }

    pub fn shrink(self, xa: f32, ya: f32, za: f32) -> Self {
        Self {
            x0: self.x0 + xa,
            y0: self.y0 + ya,
            z0: self.z0 + za,
            x1: self.x1 - xa,
            y1: self.y1 - ya,
            z1: self.z1 - za,
        }
    }

    pub fn move_by(&mut self, xa: f32, ya: f32, za: f32) {
        self.x0 += xa;
        self.y0 += ya;
        self.z0 += za;
        self.x1 += xa;
        self.y1 += ya;
        self.z1 += za;
    }

    pub fn get_size(self) -> f32 {
        let xs = self.x1 - self.x0;
        let ys = self.y1 - self.y0;
        let zs = self.z1 - self.z0;
        (xs + ys + zs) / 3.0
    }

    pub fn contains(self, p: Vec3) -> bool {
        if p.x <= self.x0 || p.x >= self.x1 { return false; }
        if p.y <= self.y0 || p.y >= self.y1 { return false; }
        if p.z <= self.z0 || p.z >= self.z1 { return false; }
        true
    }

    pub fn clip_x_collide(self, c: Self, mut xa: f32) -> f32 {
        if c.y1 <= self.y0 || c.y0 >= self.y1 || c.z1 <= self.z0 || c.z0 >= self.z1 {
            return xa;
        }
        if xa > 0.0 && c.x1 <= self.x0 {
            let max = self.x0 - c.x1;
            if max < xa {
                xa = max;
            }
        }
        if xa < 0.0 && c.x0 >= self.x1 {
            let max = self.x1 - c.x0;
            if max > xa {
                xa = max;
            }
        }
        xa
    }

    pub fn clip_y_collide(self, c: Self, mut ya: f32) -> f32 {
        if c.x1 <= self.x0 || c.x0 >= self.x1 || c.z1 <= self.z0 || c.z0 >= self.z1 {
            return ya;
        }
        if ya > 0.0 && c.y1 <= self.y0 {
            let max = self.y0 - c.y1;
            if max < ya {
                ya = max;
            }
        }
        if ya < 0.0 && c.y0 >= self.y1 {
            let max = self.y1 - c.y0;
            if max > ya {
                ya = max;
            }
        }
        ya
    }

    pub fn clip_z_collide(self, c: Self, mut za: f32) -> f32 {
        if c.x1 <= self.x0 || c.x0 >= self.x1 || c.y1 <= self.y0 || c.y0 >= self.y1 {
            return za;
        }
        if za > 0.0 && c.z1 <= self.z0 {
            let max = self.z0 - c.z1;
            if max < za {
                za = max;
            }
        }
        if za < 0.0 && c.z0 >= self.z1 {
            let max = self.z1 - c.z0;
            if max > za {
                za = max;
            }
        }
        za
    }

    pub fn clip(self, a: Vec3, b: Vec3) -> HitResult {
        let mut candidates: Vec<(Vec3, i32)> = Vec::new();

        if let Some(v) = a.clip_x(b, self.x0).filter(|v| self.contains_x(*v)) {
            candidates.push((v, 4));
        }
        if let Some(v) = a.clip_x(b, self.x1).filter(|v| self.contains_x(*v)) {
            candidates.push((v, 5));
        }
        if let Some(v) = a.clip_y(b, self.y0).filter(|v| self.contains_y(*v)) {
            candidates.push((v, 0));
        }
        if let Some(v) = a.clip_y(b, self.y1).filter(|v| self.contains_y(*v)) {
            candidates.push((v, 1));
        }
        if let Some(v) = a.clip_z(b, self.z0).filter(|v| self.contains_z(*v)) {
            candidates.push((v, 2));
        }
        if let Some(v) = a.clip_z(b, self.z1).filter(|v| self.contains_z(*v)) {
            candidates.push((v, 3));
        }

        if candidates.is_empty() {
            return HitResult::default();
        }

        let (pos, face) = candidates
            .into_iter()
            .min_by(|(p1, _), (p2, _)| {
                a.distance_to_sqr(*p1)
                    .partial_cmp(&a.distance_to_sqr(*p2))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .expect("non-empty");

        HitResult::tile(0, 0, 0, face, pos)
    }

    fn contains_x(self, v: Vec3) -> bool {
        v.y >= self.y0 && v.y <= self.y1 && v.z >= self.z0 && v.z <= self.z1
    }
    fn contains_y(self, v: Vec3) -> bool {
        v.x >= self.x0 && v.x <= self.x1 && v.z >= self.z0 && v.z <= self.z1
    }
    fn contains_z(self, v: Vec3) -> bool {
        v.x >= self.x0 && v.x <= self.x1 && v.y >= self.y0 && v.y <= self.y1
    }
}

