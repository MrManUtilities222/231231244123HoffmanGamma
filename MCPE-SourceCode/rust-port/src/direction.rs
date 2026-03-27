use crate::facing;

pub const UNDEFINED: i32 = -1;
pub const SOUTH: i32 = 0;
pub const WEST: i32 = 1;
pub const NORTH: i32 = 2;
pub const EAST: i32 = 3;

pub const DIRECTION_FACING: [i32; 4] = [facing::SOUTH, facing::WEST, facing::NORTH, facing::EAST];
pub const FACING_DIRECTION: [i32; 6] = [UNDEFINED, UNDEFINED, NORTH, SOUTH, WEST, EAST];
pub const DIRECTION_OPPOSITE: [i32; 4] = [NORTH, EAST, SOUTH, WEST];

pub const RELATIVE_DIRECTION_FACING: [[i32; 6]; 4] = [
    // south
    [
        facing::DOWN,
        facing::UP,
        facing::SOUTH,
        facing::NORTH,
        facing::EAST,
        facing::WEST,
    ],
    // west
    [
        facing::DOWN,
        facing::UP,
        facing::EAST,
        facing::WEST,
        facing::NORTH,
        facing::SOUTH,
    ],
    // north
    [
        facing::DOWN,
        facing::UP,
        facing::NORTH,
        facing::SOUTH,
        facing::WEST,
        facing::EAST,
    ],
    // east
    [
        facing::DOWN,
        facing::UP,
        facing::WEST,
        facing::EAST,
        facing::SOUTH,
        facing::NORTH,
    ],
];

