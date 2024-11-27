#![allow(internal_features)]
#![feature(lang_items)]
#![cfg_attr(not(test), no_std)]

mod alloc;
mod anim;
mod audio;
mod error;
mod event;
mod gpu;
mod macros;
pub mod math;
mod sprite;
pub mod ui;
mod window;

use sdl3_sys::events::{SDL_Event, SDL_EventType};

pub use alloc::*;
pub use error::{Error, Result};
pub use event::*;
pub use gpu::{Gpu, Texture};
pub use sdl3_sys;
pub use sprite::*;
pub use window::Window;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Pxl8<G: Game> {
    game: G,
    window: Window,

    pub gpu: Gpu,
}

impl<G: Game> Pxl8<G> {
    pub fn new(game: G) -> Result<Self> {
        let (width, height) = game.size();
        let window = Window::new(game.title(), width, height)?;
        let gpu = Gpu::new(&window)?;

        Ok(Self { game, gpu, window })
    }

    pub fn init(&self) {
        self.game.init(&self);
    }

    pub fn frame(&self) {
        self.game.frame(&self);
    }

    pub fn event(&self, sdl_event_type: SDL_EventType, sdl_event: SDL_Event) {
        let mut event = None;

        match sdl_event_type {
            SDL_EventType::KEY_DOWN => unsafe {
                event = Some(Event::KeyDown(KeyEvent {
                    key: Key::from_scancode(sdl_event.key.scancode),
                    repeat: sdl_event.key.repeat,
                }));
            },

            SDL_EventType::KEY_UP => unsafe {
                event = Some(Event::KeyUp(KeyEvent {
                    key: Key::from_scancode(sdl_event.key.scancode),
                    repeat: false,
                }));
            },

            SDL_EventType::MOUSE_BUTTON_DOWN => {}
            SDL_EventType::MOUSE_BUTTON_UP => {}

            _ => {}
        }

        if let Some(event) = event {
            self.game.event(&self, event);
        }
    }

    pub fn quit(&self) {
        self.game.quit(&self);
    }
}

pub trait Game: Sized {
    fn init(&self, pxl8: &Pxl8<Self>);
    fn event(&self, pxl8: &Pxl8<Self>, event: Event);
    fn frame(&self, pxl8: &Pxl8<Self>);
    fn quit(&self, pxl8: &Pxl8<Self>);
    fn size(&self) -> (u32, u32);
    fn title(&self) -> &str;
}

#[cfg(not(test))]
mod lang_items {
    use core::panic::PanicInfo;

    #[lang = "eh_personality"]
    extern "C" fn eh_personality() {}

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}
