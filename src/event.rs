use glam::IVec2;
use sdl3_sys::events::{
    SDL_KeyboardEvent, SDL_MouseButtonEvent, SDL_MouseMotionEvent,
    SDL_MouseWheelEvent,
};
use sdl3_sys::scancode::SDL_Scancode;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    MouseDown(MouseButtonEvent),
    MouseUp(MouseButtonEvent),
    MouseMotion(MouseMotionEvent),
    MouseWheel(MouseWheelEvent),
    GamepadDown(GamepadButtonEvent),
    GamepadUp(GamepadButtonEvent),
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
pub struct GamepadButtonEvent {
    pub button: GamepadButton,
    pub timestamp: u64,
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
    pub(crate) fn from_sdl(scancode: SDL_Scancode) -> Self {
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
    pub timestamp: u64,
}

impl KeyEvent {
    pub(crate) fn from_sdl(event: SDL_KeyboardEvent) -> Self {
        let key = Key::from_sdl(event.scancode);

        Self {
            key,
            repeat: event.repeat,
            timestamp: event.timestamp,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Unknown,
}

impl MouseButton {
    pub(crate) fn from_sdl(button: u8) -> Self {
        match button {
            1 => Self::Left,
            2 => Self::Middle,
            3 => Self::Right,

            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MouseButtonEvent {
    pub button: MouseButton,
    pub clicks: u8,
    pub timestamp: u64,
}

impl MouseButtonEvent {
    pub(crate) fn from_sdl(event: SDL_MouseButtonEvent) -> Self {
        let button = MouseButton::from_sdl(event.button);

        Self {
            button,
            clicks: event.clicks,
            timestamp: event.timestamp,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MouseMotionEvent {
    pub motion: IVec2,
    pub position: IVec2,
    pub timestamp: u64,
}

impl MouseMotionEvent {
    pub(crate) fn from_sdl(event: SDL_MouseMotionEvent) -> Self {
        Self {
            motion: IVec2::new(event.xrel as i32, event.yrel as i32),
            position: IVec2::new(event.x as i32, event.y as i32),
            timestamp: event.timestamp,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MouseWheelEvent {
    pub position: IVec2,
    pub scroll: IVec2,
    pub timestamp: u64,
}

impl MouseWheelEvent {
    pub(crate) fn from_sdl(event: SDL_MouseWheelEvent) -> Self {
        Self {
            position: IVec2::new(event.mouse_x as i32, event.mouse_y as i32),
            scroll: IVec2::new(event.x as i32, event.y as i32),
            timestamp: event.timestamp,
        }
    }
}
