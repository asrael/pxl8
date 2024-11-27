#[derive(Clone, Copy, Debug, Default)]
pub struct UVec2 {
    pub x: u16,
    pub y: u16,
}

impl UVec2 {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::default()
    }
}
