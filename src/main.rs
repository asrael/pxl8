#![no_main]
#![no_std]

use pxl8::{println, Event, Game, Pxl8};

#[derive(Debug)]
struct Editor {
    size: (u32, u32),
    title: &'static str,
}

impl Game for Editor {
    fn init(&self, _pxl8: &Pxl8<Editor>) {}
    fn event(&self, _pxl8: &Pxl8<Editor>, event: Event) {}
    fn frame(&self, _pxl8: &Pxl8<Editor>) {}
    fn quit(&self, _pxl8: &Pxl8<Editor>) {}
}

pxl8::run!(Editor {
    size: (1280, 720),
    title: "pxl8",
});
