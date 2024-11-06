#![no_std]
#![no_main]
#![windows_subsystem = "console"]

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

    fn event(&mut self, ctx: &Context) {
        println!("pxl8 event...");
    }

    fn frame(&mut self, ctx: &Context) {
        println!("pxl8 frame...");
    }

    fn quit(&mut self, ctx: &Context) {
        println!("pxl8 quit...");
    }
}

pxl8::run!(Pxl8 {
    size: (1280, 720),
    title: "pxl8",
});
