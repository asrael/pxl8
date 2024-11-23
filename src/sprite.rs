use core::assert;
use core::u16;

use crate::alloc::Vec;
use crate::println;

pub struct SpriteSheet {
    coordinates: Vec<[u16; 2]>,
    max_width: u16,
    max_height: u16,
    skyline_count: u16,
}

impl SpriteSheet {
    pub fn new(max_width: u16, max_height: u16) -> Self {
        let mut coordinates = Vec::with_capacity(2 * max_width as usize);

        coordinates.push([0, 0]);

        Self {
            coordinates,
            max_width,
            max_height,
            skyline_count: 1,
        }
    }

    pub fn add(&mut self, size: [u16; 2], pos: &mut [u16; 2]) -> bool {
        let [width, height] = size;

        if width % 4 != 0 || height % 4 != 0 {
            return false;
        }

        let mut best = u16::MAX;
        let mut next_best = u16::MAX;
        let mut best_x = u16::MAX;
        let mut best_y = u16::MAX;

        for i in 0..self.coordinates.len() {
            let [x, mut y] = self.coordinates[i];

            if width > self.max_width - x {
                break;
            }

            if y >= best_y {
                continue;
            }

            let x_max = x + width;

            for j in 0..self.skyline_count {
                if x_max <= self.coordinates[j as usize][0] {
                    break;
                }

                if y < self.coordinates[j as usize][1] {
                    y = self.coordinates[j as usize][1];
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

        let removed = next_best - best;
        let new_tl = [best_x, best_y + height];
        let new_br = [best_x + width, self.coordinates[next_best as usize - 1][1]];
        let br_point = if next_best < self.skyline_count {
            new_br[0] < self.coordinates[next_best as usize][0]
        } else {
            new_br[0] < self.max_width
        };
        let inserted = 1 + br_point as u16;

        assert!(self.skyline_count + inserted - removed <= self.max_width);

        if inserted > removed {
            let mut idx = self.skyline_count as usize - 1;
            let mut idx2 = idx + (inserted - removed) as usize;

            while idx >= next_best as usize {
                self.coordinates.swap(idx, idx2);
                idx -= 1;
                idx2 -= 1;
            }
        } else if inserted < removed {
            let mut idx = next_best as usize;
            let mut idx2 = idx - (removed - inserted) as usize;

            while idx < self.skyline_count as usize {
                self.coordinates.swap(idx, idx2);
                idx += 1;
                idx2 += 2;
            }
        }

        self.coordinates[best as usize] = new_tl;

        if br_point {
            self.coordinates[best as usize + 1] = new_br;
        }

        pos[0] = best_x;
        pos[1] = best_y;

        true
    }
}
