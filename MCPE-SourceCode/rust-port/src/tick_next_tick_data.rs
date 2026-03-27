use std::cmp::Ordering;
use std::sync::atomic::{AtomicI64, Ordering as AtomicOrdering};

static C: AtomicI64 = AtomicI64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickNextTickData {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub tile_id: i32,
    pub delay: i64,
    c: i64,
}

impl TickNextTickData {
    pub fn new(x: i32, y: i32, z: i32, tile_id: i32) -> Self {
        let c = C.fetch_add(1, AtomicOrdering::Relaxed) + 1;
        Self {
            x,
            y,
            z,
            tile_id,
            delay: 0,
            c,
        }
    }

    pub fn hash_code(self) -> i32 {
        (((self.x * 128 * 1024) + (self.z * 128) + self.y) * 256) + self.tile_id
    }

    pub fn set_delay(mut self, d: i64) -> Self {
        self.delay = d;
        self
    }
}

impl Ord for TickNextTickData {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.delay < other.delay {
            Ordering::Less
        } else if self.delay > other.delay {
            Ordering::Greater
        } else if self.c < other.c {
            Ordering::Less
        } else if self.c > other.c {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for TickNextTickData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

