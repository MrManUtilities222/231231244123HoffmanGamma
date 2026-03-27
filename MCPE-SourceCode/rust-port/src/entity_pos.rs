#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityPos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub y_rot: f32,
    pub x_rot: f32,
    pub rot: bool,
    pub mov: bool,
}

impl EntityPos {
    pub fn with_move_and_rot(x: f32, y: f32, z: f32, y_rot: f32, x_rot: f32) -> Self {
        Self {
            x,
            y,
            z,
            y_rot,
            x_rot,
            rot: true,
            mov: true,
        }
    }

    pub fn with_move(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            y_rot: 0.0,
            x_rot: 0.0,
            rot: false,
            mov: true,
        }
    }

    pub fn with_rot(y_rot: f32, x_rot: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            y_rot,
            x_rot,
            rot: true,
            mov: false,
        }
    }
}

