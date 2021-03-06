/**
 * Copyright © 2019 Jayden Chan. All rights reserved.
 *
 * Nit is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3
 * as published by the Free Software Foundation.
 *
 * Nit is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Nit. If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{image::Pixel, Vector};

#[derive(Debug, Copy, Clone)]
pub enum ToneMappingOperator {
    Clamp(f32),
    ReinhardJodie,
}

impl ToneMappingOperator {
    pub fn apply(self, buffer: &mut Vec<Vec<Pixel>>) {
        match self {
            ToneMappingOperator::Clamp(max) => clamp(buffer, max),
            ToneMappingOperator::ReinhardJodie => reinhard_jodie(buffer),
        }
    }
}

/// Applies the clamp tone-mapping operator on the provided pixel buffer.
/// The pixel values will be clamped between 0 and max
fn clamp(buffer: &mut Vec<Vec<Pixel>>, max: f32) {
    buffer.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|pixel| {
            if pixel.r > max {
                pixel.r = max;
            }
            if pixel.g > max {
                pixel.g = max;
            }
            if pixel.b > max {
                pixel.b = max;
            }

            // This shouldn't be possible but... just in case
            if pixel.r < 0.0 {
                pixel.r = 0.0;
            }
            if pixel.g < 0.0 {
                pixel.g = 0.0;
            }
            if pixel.b < 0.0 {
                pixel.b = 0.0;
            }
        });
    });
}

fn reinhard_jodie(buffer: &mut Vec<Vec<Pixel>>) {
    buffer.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|pixel| {
            let v = Vector::new(pixel.r, pixel.g, pixel.b);
            let l = pixel.luminance();

            let tv = v / (v + 1.0);

            let p_new = lerp(v / (1.0 + l), tv, tv);

            pixel.r = p_new.x;
            pixel.g = p_new.y;
            pixel.b = p_new.z;
        });
    });
}

fn lerp(a: Vector, b: Vector, t: Vector) -> Vector {
    a + t * (b - a)
}
