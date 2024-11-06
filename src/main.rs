#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use pxl8::{println, Context};

#[derive(Debug)]
struct Pxl8 {
    size: (u32, u32),
    title: &'static str,
}

impl pxl8::Game for Pxl8 {
    fn init(&mut self, ctx: Context) {
        println!("pxl8 init...");
    }

    fn event(&mut self, ctx: Context) {
        println!("pxl8 frame...");
    }

    fn frame(&mut self, ctx: Context) {
        println!("pxl8 frame...");
    }

    fn quit(&mut self, ctx: Context) {
        println!("pxl8 quit...");
    }

    fn size(&self) -> (u32, u32) {
        self.size
    }

    fn title(&self) -> &str {
        self.title
    }
}

pxl8::load!("./pxl8.json");
