pub trait MemoryInterface {
    fn read_u8(&self, addr: u32) -> u8;
    fn read_u16(&self, addr: u32) -> u16;
    fn read_u32(&self, addr: u32) -> u32;
    fn write_u8(&mut self, addr: u32, value: u8);
    fn write_u16(&mut self, addr: u32, value: u16);
    fn write_u32(&mut self, addr: u32, value: u32);
}

pub trait Cpu: MemoryInterface {
    fn res(&mut self);
    fn exe(&mut self) -> u8;
    fn irq(&mut self, level: u8) -> bool;
}

pub struct M68k;
