#![allow(internal_features)]
#![feature(lang_items)]
#![no_std]

mod alloc;
mod error;
mod event;
mod gpu;
mod macros;
mod result;

use core::ffi::c_int;
use core::ffi::c_void;
use core::panic::PanicInfo;
use core::ptr::{self, NonNull};

use alloc::ffi::CString;
use libc::printf;
use sdl3_sys::events::{SDL_Event, SDL_EventType};
use sdl3_sys::log::SDL_LOG_CATEGORY_ERROR;
use sdl3_sys::video::{SDL_CreateWindow, SDL_Window, SDL_WINDOW_RESIZABLE};

pub(crate) use error::get_sdl_error;

pub type Window = NonNull<SDL_Window>;

pub use sdl3_sys;

pub use alloc::*;
pub use error::Error;
pub use event::{
    Event, GamepadAxis, GamepadButton, Key, KeyEvent, MouseButton, MouseScroll,
};
pub use gpu::Gpu;
pub use result::Result;

#[derive(Debug)]
pub struct Pxl8<G: Game> {
    pub gpu: Gpu,
    game: G,
    window: Window,
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
            Err(get_sdl_error())
        }
    }

    pub fn init(&self) {
        println!("pxl8 init...");

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

            SDL_EventType::MOUSE_BUTTON_DOWN => {
                // event = Some(Event::MouseDown(MouseButton::from_sdl()));
            }

            SDL_EventType::MOUSE_BUTTON_UP => {
                // event = Some(Event::MouseUp(MouseButton::from_sdl()));
            }

            _ => {}
        }

        if let Some(event) = event {
            self.game.event(&self, event);
            println!("pxl8 event... {:?}", event);
        }
    }

    pub fn quit(&self) {
        self.game.quit(&self);
        println!("pxl8 quit...");
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

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
