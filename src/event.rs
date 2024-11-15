use sdl3_sys::scancode::SDL_Scancode;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseWheel(MouseScroll),
    GamepadDown(GamepadButton),
    GamepadUp(GamepadButton),
}

#[derive(Debug, Clone, Copy)]
pub enum GamepadAxis {
    LeftX(i16),
    LeftY(i16),
    RightX(i16),
    RightY(i16),
    LeftTrigger(i16),
    RightTrigger(i16),
}

#[derive(Debug, Clone, Copy)]
pub enum GamepadButton {
    North,
    South,
    East,
    West,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

#[derive(Debug, Clone, Copy)]
pub enum Key {
    W,
    A,
    S,
    D,
    Escape,
    Unknown,
}

impl Key {
    pub fn from_scancode(scancode: SDL_Scancode) -> Self {
        match scancode {
            SDL_Scancode::W => Self::W,
            SDL_Scancode::A => Self::A,
            SDL_Scancode::S => Self::S,
            SDL_Scancode::D => Self::D,
            SDL_Scancode::ESCAPE => Self::Escape,

            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub key: Key,
    pub repeat: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct MouseScroll {
    pub dx: i32,
    pub dy: i32,
}
