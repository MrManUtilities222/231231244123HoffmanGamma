use std::sync::atomic::{AtomicI32, Ordering};

use crate::aabb::Aabb;
use crate::vec3::Vec3;

static ENTITY_COUNTER: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityRendererId {
    DefaultRenderer,
    QueryRenderer,
    TntRenderer,
    HumanoidRenderer,
    ItemRenderer,
    TripodcameraRenderer,
    ChickenRenderer,
    CowRenderer,
    PigRenderer,
    SheepRenderer,
    SheepFurRenderer,
    ZombieRenderer,
    SkeletonRenderer,
    SpiderRenderer,
    CreeperRenderer,
    ArrowRenderer,
    PlayerRenderer,
    ThrowneggRenderer,
    SnowballRenderer,
    PaintingRenderer,
    FallingtileRenderer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityCore {
    pub entity_id: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub xo: f32,
    pub yo: f32,
    pub zo: f32,
    pub xd: f32,
    pub yd: f32,
    pub zd: f32,
    pub y_rot: f32,
    pub x_rot: f32,
    pub y_rot_o: f32,
    pub x_rot_o: f32,
    pub x_old: f32,
    pub y_old: f32,
    pub z_old: f32,
    pub bb: Aabb,
    pub bb_width: f32,
    pub bb_height: f32,
    pub height_offset: f32,
    pub y_slide_offset: f32,
    pub pushthrough: f32,
    pub removed: bool,
    pub on_ground: bool,
    pub in_chunk: bool,
    pub x_chunk: i32,
    pub y_chunk: i32,
    pub z_chunk: i32,
    pub entity_renderer_id: EntityRendererId,
}

impl Default for EntityCore {
    fn default() -> Self {
        let id = ENTITY_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        let mut e = Self {
            entity_id: id,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            xo: 0.0,
            yo: 0.0,
            zo: 0.0,
            xd: 0.0,
            yd: 0.0,
            zd: 0.0,
            y_rot: 0.0,
            x_rot: 0.0,
            y_rot_o: 0.0,
            x_rot_o: 0.0,
            x_old: 0.0,
            y_old: 0.0,
            z_old: 0.0,
            bb: Aabb::default(),
            bb_width: 0.6,
            bb_height: 1.8,
            height_offset: 0.0,
            y_slide_offset: 0.0,
            pushthrough: 0.0,
            removed: false,
            on_ground: false,
            in_chunk: false,
            x_chunk: 0,
            y_chunk: 0,
            z_chunk: 0,
            entity_renderer_id: EntityRendererId::DefaultRenderer,
        };
        e.set_pos(0.0, 0.0, 0.0);
        e
    }
}

impl EntityCore {
    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        let w = self.bb_width / 2.0;
        let h = self.bb_height;
        self.bb = Aabb::new(
            x - w,
            y - self.height_offset + self.y_slide_offset,
            z - w,
            x + w,
            y - self.height_offset + self.y_slide_offset + h,
            z + w,
        );
    }

    pub fn set_rot(&mut self, y_rot: f32, x_rot: f32) {
        self.y_rot = y_rot;
        self.y_rot_o = y_rot;
        self.x_rot = x_rot;
        self.x_rot_o = x_rot;
    }

    pub fn move_to(&mut self, x: f32, y: f32, z: f32, y_rot: f32, x_rot: f32) {
        self.x_old = x;
        self.y_old = y + self.height_offset;
        self.z_old = z;
        self.xo = self.x_old;
        self.yo = self.y_old;
        self.zo = self.z_old;
        self.x = x;
        self.y = y + self.height_offset;
        self.z = z;
        self.y_rot = y_rot;
        self.y_rot_o = y_rot;
        self.x_rot = x_rot;
        self.x_rot_o = x_rot;
        self.set_pos(self.x, self.y, self.z);
    }

    pub fn push(&mut self, xa: f32, ya: f32, za: f32) {
        self.xd += xa;
        self.yd += ya;
        self.zd += za;
    }

    pub fn distance_to_entity(&self, other: &EntityCore) -> f32 {
        self.distance_to_xyz(other.x, other.y, other.z)
    }

    pub fn distance_to_xyz(&self, x2: f32, y2: f32, z2: f32) -> f32 {
        let xd = self.x - x2;
        let yd = self.y - y2;
        let zd = self.z - z2;
        (xd * xd + yd * yd + zd * zd).sqrt()
    }

    pub fn distance_to_sqr_xyz(&self, x2: f32, y2: f32, z2: f32) -> f32 {
        let xd = self.x - x2;
        let yd = self.y - y2;
        let zd = self.z - z2;
        xd * xd + yd * yd + zd * zd
    }

    pub fn remove(&mut self) {
        self.removed = true;
    }

    pub fn is_alive(&self) -> bool {
        !self.removed
    }

    pub fn should_render(&self, camera: Vec3) -> bool {
        let xd = self.x - camera.x;
        let yd = self.y - camera.y;
        let zd = self.z - camera.z;
        let distance = xd * xd + yd * yd + zd * zd;
        self.should_render_at_sqr_distance(distance)
    }

    pub fn should_render_at_sqr_distance(&self, distance: f32) -> bool {
        let mut size = ((self.bb.x1 - self.bb.x0) + (self.bb.y1 - self.bb.y0) + (self.bb.z1 - self.bb.z0)) / 3.0;
        size *= 64.0;
        distance < size * size
    }
}

