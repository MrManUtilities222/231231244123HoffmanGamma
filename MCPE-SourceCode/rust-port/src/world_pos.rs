#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn create_hash_code(x: i32, y: i32, z: i32) -> i32 {
        x.wrapping_add(z << 8).wrapping_add(y << 16)
    }

    pub fn hash_code(&self) -> i32 {
        Self::create_hash_code(self.x, self.y, self.z)
    }

    pub fn compare_to(&self, pos: &Pos) -> i32 {
        self.hash_code().wrapping_sub(pos.hash_code())
    }

    pub fn offset(&self, x: i32, y: i32, z: i32) -> Pos {
        Pos::new(self.x + x, self.y + y, self.z + z)
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn set_pos(&mut self, pos: &Pos) {
        self.set(pos.x, pos.y, pos.z);
    }

    pub fn above(&self) -> Pos {
        Pos::new(self.x, self.y + 1, self.z)
    }

    pub fn above_n(&self, steps: i32) -> Pos {
        Pos::new(self.x, self.y + steps, self.z)
    }

    pub fn below(&self) -> Pos {
        Pos::new(self.x, self.y - 1, self.z)
    }

    pub fn below_n(&self, steps: i32) -> Pos {
        Pos::new(self.x, self.y - steps, self.z)
    }

    pub fn north(&self) -> Pos {
        Pos::new(self.x, self.y, self.z - 1)
    }

    pub fn north_n(&self, steps: i32) -> Pos {
        Pos::new(self.x, self.y, self.z - steps)
    }

    pub fn south(&self) -> Pos {
        Pos::new(self.x, self.y, self.z + 1)
    }

    pub fn south_n(&self, steps: i32) -> Pos {
        Pos::new(self.x, self.y, self.z + steps)
    }

    pub fn west(&self) -> Pos {
        Pos::new(self.x - 1, self.y, self.z)
    }

    // Preserve original C++ behavior (ignores steps; likely bug in source).
    pub fn west_n(&self, _steps: i32) -> Pos {
        Pos::new(self.x - 1, self.y, self.z)
    }

    pub fn east(&self) -> Pos {
        Pos::new(self.x + 1, self.y, self.z)
    }

    pub fn east_n(&self, steps: i32) -> Pos {
        Pos::new(self.x + steps, self.y, self.z)
    }

    pub fn move_by(&mut self, x: i32, y: i32, z: i32) {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    pub fn move_pos(&mut self, pos: &Pos) {
        self.move_by(pos.x, pos.y, pos.z);
    }

    pub fn to_java_like_string(&self) -> String {
        format!("Pos({},{},{})", self.x, self.y, self.z)
    }
}

