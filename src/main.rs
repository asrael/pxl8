#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use pxl8::Context;

#[derive(Debug)]
struct Pxl8;

impl pxl8::Game for Pxl8 {
    fn init(&mut self, ctx: Context) {
        pxl8::println!("pxl8 init...");
    }

    fn event(&mut self, ctx: Context) {
        pxl8::println!("pxl8 frame...");
    }

    fn frame(&mut self, ctx: Context) {
        pxl8::println!("pxl8 frame...");
    }

    fn quit(&mut self, ctx: Context) {
        pxl8::println!("pxl8 quit...");
    }
}

pxl8::load!(Pxl8 {});
