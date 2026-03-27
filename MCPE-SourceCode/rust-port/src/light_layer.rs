#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightLayer {
    Sky,
    Block,
}

impl LightLayer {
    pub fn surrounding(self) -> i32 {
        match self {
            LightLayer::Sky => 15,
            LightLayer::Block => 0,
        }
    }
}

