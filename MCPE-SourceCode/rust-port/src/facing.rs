pub const DOWN: i32 = 0;
pub const UP: i32 = 1;
pub const NORTH: i32 = 2;
pub const SOUTH: i32 = 3;
pub const WEST: i32 = 4;
pub const EAST: i32 = 5;

pub const OPPOSITE_FACING: [i32; 6] = [UP, DOWN, SOUTH, NORTH, EAST, WEST];
pub const STEP_X: [i32; 6] = [0, 0, 0, 0, -1, 1];
pub const STEP_Y: [i32; 6] = [-1, 1, 0, 0, 0, 0];
pub const STEP_Z: [i32; 6] = [0, 0, -1, 1, 0, 0];

pub fn to_string(face: i32) -> &'static str {
    match face {
        DOWN => "Down",
        UP => "Up",
        NORTH => "North",
        SOUTH => "South",
        WEST => "West",
        EAST => "East",
        _ => "Unknown facing",
    }
}

