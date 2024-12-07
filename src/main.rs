#![no_main]
#![no_std]

use pxl8::{println, Context, Game};

pub(crate) struct Demo;

impl Game for Demo {
    fn init(&mut self, _ctx: &mut Context) {}
    fn update(&mut self, ctx: &mut Context) {
        for event in &ctx.events {
            println!("event: {:?}", event);
        }
    }
    fn frame(&mut self, _ctx: &mut Context) {}
    fn quit(&mut self, _ctx: &mut Context) {}

    fn size(&self) -> (u32, u32) {
        (1280, 720)
    }

    fn title(&self) -> &str {
        "pxl8 demo"
    }
}

pxl8::run!(Demo {});
