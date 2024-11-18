use pxl8::{Event, Game, Pxl8};

pub(crate) struct Editor;

impl Game for Editor {
    fn init(&self, _pxl8: &Pxl8<Self>) {}
    fn event(&self, _pxl8: &Pxl8<Self>, _event: Event) {}
    fn frame(&self, _pxl8: &Pxl8<Self>) {}
    fn quit(&self, _pxl8: &Pxl8<Self>) {}

    fn size(&self) -> (u32, u32) {
        (1280, 720)
    }

    fn title(&self) -> &str {
        "pxl8"
    }
}
