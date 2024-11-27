use core::assert;
use core::u16;

use crate::alloc::{vec, Vec};
use crate::math::UVec2;

#[derive(Clone, Debug)]
pub struct SpriteSheet {
    coordinates: Vec<UVec2>,
    max_width: u16,
    max_height: u16,
    skyline_count: u16,
}

impl SpriteSheet {
    pub fn new(max_width: u16, max_height: u16) -> Self {
        Self {
            coordinates: vec![UVec2::zero(); max_width as usize],
            max_width,
            max_height,
            skyline_count: 1,
        }
    }

    pub fn add(&mut self, size: UVec2, pos: &mut UVec2) -> bool {
        let [width, height] = [size.x, size.y];

        if width % 2 != 0
            || height % 2 != 0
            || width > self.max_width
            || height > self.max_height
        {
            return false;
        }

        let mut best = u16::MAX;
        let mut next_best = u16::MAX;
        let mut best_x = u16::MAX;
        let mut best_y = u16::MAX;

        for i in 0..self.coordinates.len() {
            let [x, mut y] = [self.coordinates[i].x, self.coordinates[i].y];

            if width > self.max_width - x {
                break;
            }

            if y >= best_y {
                continue;
            }

            let x_max = x + width;

            for j in 0..self.skyline_count {
                if x_max <= self.coordinates[j as usize].x {
                    break;
                }

                if y < self.coordinates[j as usize].y {
                    y = self.coordinates[j as usize].y;
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
        let new_tl = UVec2::new(best_x, best_y + height);
        let new_br =
            UVec2::new(best_x + width, self.coordinates[next_best as usize - 1].y);
        let br_point = if next_best < self.skyline_count {
            new_br.x < self.coordinates[next_best as usize].x
        } else {
            new_br.x < self.max_width
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

        pos.x = best_x;
        pos.y = best_y;

        true
    }
}
