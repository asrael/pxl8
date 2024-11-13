#![no_main]
#![no_std]

use pxl8::{println, Game, Pxl8};

#[derive(Debug)]
struct Editor {
    size: (u32, u32),
    title: &'static str,
}

impl Game for Editor {
    fn init(&mut self, _pxl8: &Pxl8) {
        println!("pxl8 init...");
    }

    fn event(&mut self, _pxl8: &Pxl8) {
        println!("pxl8 event...");
    }

    fn frame(&mut self, _pxl8: &Pxl8) {}

    fn quit(&mut self, _pxl8: &Pxl8) {
        println!("pxl8 quit...");
    }
}

pxl8::run!(Editor {
    size: (1280, 720),
    title: "pxl8 editor",
});
