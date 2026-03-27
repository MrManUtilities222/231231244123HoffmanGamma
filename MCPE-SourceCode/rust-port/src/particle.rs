/// Particle system ported from client/particle/
/// Covers: Particle.h, TerrainParticle.h, SmokeParticle.h, FlameParticle.h,
///         LavaParticle.h, BubbleParticle.h, RainSplashParticle.h

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParticleKind {
    BlockBreak { tile_id: i32 },
    Smoke,
    Flame,
    Lava,
    Bubble,
    RainSplash,
    Explosion,
    Heart,
    Critical,
    RedDust,
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub kind: ParticleKind,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub age: i32,
    pub max_age: i32,
    pub size: f32,
    pub gravity: f32,
    pub alpha: f32,
    pub dead: bool,
}

impl Particle {
    pub fn new(kind: ParticleKind, x: f32, y: f32, z: f32) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let (max_age, size, gravity) = match kind {
            ParticleKind::BlockBreak { .. } => (
                rng.gen_range(3..8),
                0.1 + rng.gen_range(0.0..0.05f32),
                1.0,
            ),
            ParticleKind::Smoke => (
                rng.gen_range(8..16),
                0.06,
                0.0,
            ),
            ParticleKind::Flame => (
                rng.gen_range(6..12),
                0.05,
                -0.06, // floats upward
            ),
            ParticleKind::Lava => (
                rng.gen_range(40..80),
                0.15,
                0.8,
            ),
            ParticleKind::Bubble => (
                rng.gen_range(4..12),
                0.04,
                -0.5, // rises in water
            ),
            ParticleKind::RainSplash => (
                rng.gen_range(2..6),
                0.07,
                0.5,
            ),
            ParticleKind::Explosion => (
                rng.gen_range(6..12),
                0.2,
                0.8,
            ),
            ParticleKind::Heart => (
                rng.gen_range(8..16),
                0.1,
                -0.04,
            ),
            ParticleKind::Critical => (
                rng.gen_range(4..8),
                0.06,
                0.6,
            ),
            ParticleKind::RedDust => (
                rng.gen_range(6..12),
                0.04,
                0.1,
            ),
        };

        Self {
            kind, x, y, z,
            vx: rng.gen_range(-0.04..0.04),
            vy: rng.gen_range(0.0..0.1),
            vz: rng.gen_range(-0.04..0.04),
            age: 0,
            max_age,
            size,
            gravity,
            alpha: 1.0,
            dead: false,
        }
    }

    pub fn tick(&mut self) {
        self.age += 1;
        if self.age >= self.max_age {
            self.dead = true;
            return;
        }

        // Fade out
        self.alpha = 1.0 - (self.age as f32 / self.max_age as f32);

        // Physics
        self.vy -= self.gravity * 0.04;
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;

        // Drag
        self.vx *= 0.98;
        self.vy *= 0.98;
        self.vz *= 0.98;
    }
}

/// Particle emitter for managing particle lists
pub struct ParticleEngine {
    pub particles: Vec<Particle>,
    pub max_particles: usize,
}

impl ParticleEngine {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            max_particles: 4000,
        }
    }

    pub fn add(&mut self, particle: Particle) {
        if self.particles.len() >= self.max_particles {
            self.particles.remove(0);
        }
        self.particles.push(particle);
    }

    /// Emit block-break particles at a position
    pub fn break_block(&mut self, x: i32, y: i32, z: i32, tile_id: i32) {
        for _ in 0..8 {
            let p = Particle::new(
                ParticleKind::BlockBreak { tile_id },
                x as f32 + 0.5,
                y as f32 + 0.5,
                z as f32 + 0.5,
            );
            self.add(p);
        }
    }

    pub fn tick_all(&mut self) {
        for p in &mut self.particles {
            p.tick();
        }
        self.particles.retain(|p| !p.dead);
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }
}

impl Default for ParticleEngine {
    fn default() -> Self {
        Self::new()
    }
}
