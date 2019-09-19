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

use camera::Camera;
use config::Config;
use image::{ImageBuffer, Pixel};
use math::Onb;
use scene::Scene;
use vector3::Vector;

fn main() {
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
}
