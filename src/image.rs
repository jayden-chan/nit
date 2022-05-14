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
use std::{error::Error, fs::File, io::prelude::*, path::Path};

use crate::{color::ToneMappingOperator, Vector};

pub struct Pixel {
    /// Red value
    pub r: f32,
    /// Green value
    pub g: f32,
    /// Blue value
    pub b: f32,
    /// X coordinate
    pub x: u16,
    /// Y coordinate
    pub y: u16,
}

impl Pixel {
    const LUMINANCE_TRIPLE: Vector = Vector::new(0.2126, 0.7152, 0.0722);

    pub fn luminance(&self) -> f32 {
        Vector::new(self.r, self.b, self.g).dot(Self::LUMINANCE_TRIPLE)
    }
}

pub struct ImageBuffer {
    pub buffer: Vec<Vec<Pixel>>,
}

impl ImageBuffer {
    pub fn new(resolution: (u16, u16)) -> Self {
        let (width, height) = resolution;
        let mut buffer = Vec::with_capacity(height as usize);

        for y in 0..height {
            buffer.push(Vec::with_capacity(width as usize));

            for x in 0..width {
                buffer[y as usize].push(Pixel {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    x,
                    y: height - 1 - y,
                });
            }
        }

        Self { buffer }
    }
}

impl ImageBuffer {
    pub fn to_ppm(
        mut self,
        out_path: String,
        tmo: ToneMappingOperator,
    ) -> Result<(), String> {
        let path = Path::new(&out_path);
        let display = path.display();
        let height = self.buffer.len();

        if height == 0 {
            return Err(String::from("Height must be greater than 0"));
        }

        let width = self.buffer[0].len();

        // Apply tone mapping operator
        tmo.apply(&mut self.buffer);

        File::create(&path)
            .map_err(|why| format!("Couldn't create {}: {}", display, why))
            .and_then(|mut file| {
                let img_header = format!("P6\n{} {}\n255\n", width, height);
                let mut img_buffer = Vec::from(img_header.as_bytes());

                self.buffer.iter().for_each(|row| {
                    assert!(row.len() == width);
                    row.iter().for_each(|pixel| {
                        img_buffer.push((pixel.r.sqrt() * 255.0) as u8);
                        img_buffer.push((pixel.g.sqrt() * 255.0) as u8);
                        img_buffer.push((pixel.b.sqrt() * 255.0) as u8);
                    })
                });

                file.write_all(&img_buffer).map_err(|why| {
                    format!("Couldn't create {}: {}", display, why)
                })
            })
    }
}
