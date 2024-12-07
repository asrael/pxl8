#![allow(internal_features)]
#![feature(lang_items)]
#![cfg_attr(not(test), no_std)]

mod alloc;
mod anim;
mod audio;
mod error;
mod event;
mod gfx;
mod gpu;
mod macros;
mod sprite;
mod ui;
mod window;

pub use alloc::*;
pub use error::{Error, Result};
pub use event::*;
pub use gfx::Gfx;
pub use gpu::{Gpu, Texture};
pub use sprite::*;
pub use window::Window;

#[derive(Debug)]
pub struct Context {
    pub events: Vec<Event>,
    pub gfx: Gfx,
}

pub trait Game: Sized {
    fn init(&mut self, ctx: &mut Context);
    fn update(&mut self, ctx: &mut Context);
    fn frame(&mut self, ctx: &mut Context);
    fn quit(&mut self, ctx: &mut Context);
    fn size(&self) -> (u32, u32);
    fn title(&self) -> &str;
}

#[doc(hidden)]
pub mod __internal {
    use crate::{
        Context, Event, Game, Gfx, Gpu, KeyEvent, MouseButtonEvent,
        MouseMotionEvent, MouseWheelEvent, Result, Vec, Window,
    };

    use core::mem;
    use sdl3_sys::events::{SDL_Event, SDL_EventType};
    use sdl3_sys::init::SDL_AppResult;

    pub use sdl3_sys;

    #[derive(Debug)]
    pub struct Pxl8<G: Game> {
        ctx: Context,
        game: G,
        window: Window,
    }

    impl<G: Game> Pxl8<G> {
        pub fn new(game: G) -> Result<Self> {
            let events = Vec::with_capacity(8);
            let window = Window::new(game.title(), game.size().0, game.size().1)?;
            let gpu = Gpu::new(&window)?;
            let gfx = Gfx::new(gpu);
            let ctx = Context { events, gfx };

            Ok(Self { ctx, game, window })
        }

        pub fn init(&mut self) -> SDL_AppResult {
            self.game.init(&mut self.ctx);

            SDL_AppResult::CONTINUE
        }

        pub fn frame(&mut self) -> SDL_AppResult {
            self.game.update(&mut self.ctx);
            self.game.frame(&mut self.ctx);

            self.ctx.events.clear();

            SDL_AppResult::CONTINUE
        }

        pub fn event(&mut self, sdl_event: &SDL_Event) -> SDL_AppResult {
            let r#type = unsafe { mem::transmute(sdl_event.r#type) };

            match r#type {
                SDL_EventType::KEY_DOWN => {
                    let kb_event = unsafe { sdl_event.key };

                    self.ctx
                        .events
                        .push(Event::KeyUp(KeyEvent::from_sdl(kb_event)));

                    SDL_AppResult::CONTINUE
                }

                SDL_EventType::KEY_UP => {
                    let kb_event = unsafe { sdl_event.key };

                    self.ctx
                        .events
                        .push(Event::KeyUp(KeyEvent::from_sdl(kb_event)));

                    SDL_AppResult::CONTINUE
                }

                SDL_EventType::MOUSE_BUTTON_DOWN => {
                    let mb_event = unsafe { sdl_event.button };

                    self.ctx.events.push(Event::MouseDown(
                        MouseButtonEvent::from_sdl(mb_event),
                    ));

                    SDL_AppResult::CONTINUE
                }

                SDL_EventType::MOUSE_BUTTON_UP => {
                    let mb_event = unsafe { sdl_event.button };

                    self.ctx
                        .events
                        .push(Event::MouseUp(MouseButtonEvent::from_sdl(mb_event)));

                    SDL_AppResult::CONTINUE
                }

                SDL_EventType::MOUSE_MOTION => {
                    let mouse_motion = unsafe { sdl_event.motion };

                    self.ctx.events.push(Event::MouseMotion(
                        MouseMotionEvent::from_sdl(mouse_motion),
                    ));

                    SDL_AppResult::CONTINUE
                }

                SDL_EventType::MOUSE_WHEEL => {
                    let mw_event = unsafe { sdl_event.wheel };

                    self.ctx.events.push(Event::MouseWheel(
                        MouseWheelEvent::from_sdl(mw_event),
                    ));

                    SDL_AppResult::CONTINUE
                }

                SDL_EventType::QUIT => SDL_AppResult::SUCCESS,
                _ => SDL_AppResult::CONTINUE,
            }
        }

        pub fn quit(&mut self) {
            self.game.quit(&mut self.ctx);
        }
    }
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
