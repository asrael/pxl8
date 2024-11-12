#![no_std]
#![no_main]

use pxl8::{println, Context, Game};

#[derive(Debug)]
struct Pxl8 {
    size: (u32, u32),
    title: &'static str,
}

impl Game for Pxl8 {
    fn init(&mut self, ctx: &Context) {
        println!("pxl8 init...");
    }

    fn event(&mut self, _ctx: &Context) {}
    fn frame(&mut self, _ctx: &Context) {}

    fn quit(&mut self, _ctx: &Context) {
        println!("pxl8 quit...");
    }
}

pxl8::run!(Pxl8 {
    size: (1280, 720),
    title: "pxl8",
});
