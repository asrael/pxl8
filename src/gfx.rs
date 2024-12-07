use crate::{Error, Gpu, Result, Texture};

use glam::Mat3A;

#[derive(Clone, Copy, Debug, Default)]
struct State {
    color: Color,
    texture: Option<Texture>,
    transform: Mat3A,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Color(u32);

impl Color {
    pub const RED: Self = Self(0xFF0000FF);
    pub const GREEN: Self = Self(0x00FF00FF);
    pub const BLUE: Self = Self(0x0000FFFF);
    pub const WHITE: Self = Self(0xFFFFFFFF);
}

#[derive(Debug)]
pub struct Gfx {
    gpu: Gpu,
    state: usize,
    states: [Option<State>; Self::MAX_STATES],
}

impl Gfx {
    pub const MAX_STATES: usize = 8;

    pub fn new(gpu: Gpu) -> Self {
        let mut states = [None; Self::MAX_STATES];

        states[0] = Some(State::default());

        Self {
            gpu,
            state: 0,
            states,
        }
    }

    pub fn push(&mut self) -> Result<&'_ mut Self> {
        if self.state + 1 >= Self::MAX_STATES {
            return Err(Error::new("Exceeded maximum number of graphics states."));
        }

        self.states[self.state + 1] = self.states[self.state];
        self.state += 1;

        Ok(self)
    }

    pub fn pop(&mut self) -> Result<()> {
        if self.state == 0 {
            return Err(Error::new("Cannot pop the root state."));
        }

        self.states[self.state] = None;
        self.state -= 1;

        Ok(())
    }
}
