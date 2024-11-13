pub enum Event {
    KeyDown(Key),
    KeyUp(Key),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseWheel(MouseScroll),
    WindowResize(Size),
}
