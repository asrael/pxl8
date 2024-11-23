use core::assert;
use core::u16;

use crate::alloc::Vec;
use crate::println;

pub struct SpriteSheet {
    coordinates: Vec<(u16, u16)>,
    max_width: u16,
    max_height: u16,
    skyline_count: u16,
}

impl SpriteSheet {
    pub fn new(max_width: u16, max_height: u16) -> Self {
        let mut coordinates = Vec::with_capacity(2 * max_width as usize);

        coordinates.push((0, 0));

        Self {
            coordinates,
            max_width,
            max_height,
            skyline_count: 1,
        }
    }

    pub fn add(&mut self, pos: (u16, u16), size: (u16, u16)) -> bool {
        let (width, height) = size;

        if width % 4 != 0 || height % 4 != 0 {
            return false;
        }

        let mut best = u16::MAX;
        let mut next_best = u16::MAX;
        let mut best_x = u16::MAX;
        let mut best_y = u16::MAX;

        for i in 0..self.coordinates.len() {
            let (x, mut y) = self.coordinates[i];

            if width > self.max_width - x {
                break;
            }

            if y >= best_y {
                continue;
            }

            let x_max = x + width;

            for j in (0..self.skyline_count) {
                if x_max <= self.coordinates[j as usize].0 {
                    break;
                }

                if y < self.coordinates[j as usize].1 {
                    y = self.coordinates[j as usize].1;
                }

                if y >= best_y || height > self.max_height {
                    continue;
                }

                best = i as u16;
                next_best = j;
                best_x = x;
                best_y = y;
            }
        }

        if best == u16::MAX {
            return false;
        }

        assert!(best < next_best);
        assert!(next_best > 0);

        true
    }
}
