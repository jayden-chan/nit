#![feature(const_generics)]

// FIXME: REMOVE THIS ONCE STUFF IS BROUGHT OVER FROM OLD TRACER
// #![allow(dead_code)]

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

fn main() -> Result<(), String> {
    let config = config_cornell_box();

    let mut buffer = ImageBuffer::new(config.resolution);

    renderer::render(&mut buffer, config);

    buffer.to_ppm(
        String::from("out/image.ppm"),
        ToneMappingOperator::Clamp(1.0),
    )
}
