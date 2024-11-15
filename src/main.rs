#![no_main]
#![no_std]

use pxl8::{Event, Game, Pxl8};

struct Editor {
    size: (u32, u32),
    title: &'static str,
}

impl Game for Editor {
    fn init(&self, _pxl8: &Pxl8<Self>) {}
    fn event(&self, _pxl8: &Pxl8<Self>, event: Event) {}
    fn frame(&self, _pxl8: &Pxl8<Self>) {}
    fn quit(&self, _pxl8: &Pxl8<Self>) {}

    fn size(&self) -> (u32, u32) {
        self.size
    }

    fn title(&self) -> &str {
        self.title
    }
}

pxl8::run!(Editor {
    size: (1280, 720),
    title: "pxl8",
});
