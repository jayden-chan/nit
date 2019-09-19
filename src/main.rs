// FIXME: REMOVE THIS ONCE STUFF IS BROUGHT OVER FROM OLD TRACER
#![allow(dead_code)]

mod camera;
mod color;
mod config;
mod image;
mod math;
mod objects;
mod onb;
mod ray;
mod scene;
mod vector3;

use crate::{
    camera::Camera, color::ToneMappingOperator, config::Config,
    image::ImageBuffer, math::Onb, scene::Scene, vector3::Vector,
};

fn main() -> Result<(), String> {
    let config = Config {
        resolution: (300, 300),
        samples: 150,
        scene: Scene {
            objects: Vec::new(),
            camera: Camera {
                lower_left_corner: Vector::zeros(),
                horizontal: Vector::zeros(),
                vertical: Vector::zeros(),
                origin: Vector::zeros(),
                lens_radius: 0.0,
                uvw: Onb::default(),
                t0: 0.0,
                t1: 0.0,
            },
        },
    };

    let buffer = ImageBuffer::new(config.resolution);
    buffer.to_ppm(
        String::from("out/image.ppm"),
        ToneMappingOperator::Clamp(255.0),
    )
}
