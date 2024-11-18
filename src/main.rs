#![no_main]
#![no_std]

mod editor;
use editor::Editor;

pxl8::run!(Editor {});
