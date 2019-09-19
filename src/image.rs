use std::{error::Error, fs::File, io::prelude::*, path::Path};

use crate::color::ToneMappingOperator;

pub struct Pixel {
    /// Red value
    pub r: f32,
    /// Green value
    pub g: f32,
    /// Blue value
    pub b: f32,
    /// X coordinate
    pub x: usize,
    /// Y coordinate
    pub y: usize,
}

pub struct ImageBuffer {
    buffer: Vec<Vec<Pixel>>,
}

impl ImageBuffer {
    pub fn new(resolution: (usize, usize)) -> Self {
        let (width, height) = resolution;
        let mut buffer = Vec::with_capacity(height);

        for y in 0..height {
            buffer.push(Vec::with_capacity(width));

            for x in 0..width {
                buffer[x].push(Pixel {
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
        self,
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
            .map_err(|why| {
                format!("Couldn't create {}: {}", display, why.description())
            })
            .and_then(|mut file| {
                let img_header = format!("P6\n{} {}\n255\n", width, height);
                let mut img_buffer = Vec::from(img_header.as_bytes());

                self.buffer.iter().for_each(|row| {
                    assert!(row.len() == width);
                    row.iter().for_each(|pixel| {
                        img_buffer.push(pixel.r as u8);
                        img_buffer.push(pixel.g as u8);
                        img_buffer.push(pixel.b as u8);
                    })
                });

                file.write_all(&img_buffer).map_err(|why| {
                    format!(
                        "Couldn't create {}: {}",
                        display,
                        why.description()
                    )
                })
            })
    }
}
