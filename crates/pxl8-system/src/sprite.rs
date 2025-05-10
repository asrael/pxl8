#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub x: u32,
    pub y: u32,
    pub u: f32,
    pub v: f32,
    pub width: u32,
    pub height: u32,
    pub rotation: f32,
    pub scale: f32,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            u: 0.0,
            v: 0.0,
            width: 0,
            height: 0,
            rotation: 0.0,
            scale: 1.0,
        }
    }
}

impl Sprite {
    pub fn new() -> Self {
        Self::default()
    }
}
