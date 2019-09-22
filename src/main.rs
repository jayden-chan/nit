#![feature(const_generics)]

mod aabb;
mod bvh;
mod camera;
mod color;
mod config;
mod image;
mod materials;
mod math;
mod objects;
mod ray;
mod renderer;
mod vector3;

use crate::config::*;
use crate::{color::ToneMappingOperator, image::ImageBuffer, vector3::Vector};

use std::time;

fn main() -> Result<(), String> {
    let start_time = time::Instant::now();
    let config = config_cornell_box();

    let mut buffer = ImageBuffer::new(config.resolution);

    renderer::render(&mut buffer, config);

    println!("\nCompleted rendering in {:#?}", start_time.elapsed());

    buffer.to_ppm(
        String::from("out/image.ppm"),
        ToneMappingOperator::Clamp(1.0),
    )
}
