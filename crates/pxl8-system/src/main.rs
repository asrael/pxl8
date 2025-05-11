#![no_main]
#![no_std]

use pxl8_system as pxl8;

#[derive(Clone, Copy, Debug)]
struct State {
    x: i32,
}

#[pxl8::main]
fn main() {
    pxl8::init(State { x: 0 });
}
