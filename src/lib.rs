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
mod sprite;

use core::ffi::c_int;
use core::ptr::{self, NonNull};

use alloc::ffi::CString;
use sdl3_sys::events::{SDL_Event, SDL_EventType};
use sdl3_sys::video::{SDL_CreateWindow, SDL_Window, SDL_WINDOW_RESIZABLE};

pub use alloc::*;
pub use error::{Error, Result};
pub use event::{
    Event, GamepadAxis, GamepadButton, Key, KeyEvent, MouseButton, MouseScroll,
};
pub use gpu::Gpu;
pub use sdl3_sys;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Pxl8<G: Game> {
    game: G,
    window: NonNull<SDL_Window>,

    pub gpu: Gpu,
}

impl<G: Game> Pxl8<G> {
    pub unsafe fn new(game: G) -> Result<Self> {
        let title = CString::new(game.title()).unwrap();
        let (width, height) = game.size();

        let window = SDL_CreateWindow(
            title.as_ptr(),
            width as c_int,
            height as c_int,
            SDL_WINDOW_RESIZABLE as u64,
        );

        let gpu = Gpu::new(window)?;

        if window != ptr::null_mut() {
            let window = NonNull::new_unchecked(window);
            Ok(Pxl8 { game, gpu, window })
        } else {
            Err(Error::from_sdl())
        }
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
