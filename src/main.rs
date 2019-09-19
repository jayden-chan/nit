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
mod renderer;
mod scene;
mod vector3;

use crate::{
    camera::Camera, color::ToneMappingOperator, config::Config,
    image::ImageBuffer, scene::Scene, vector3::Vector,
};

fn main() -> Result<(), String> {
    let config = Config {
        resolution: (300, 300),
        samples: 1,
        scene: Scene {
            objects: vec![Box::new(objects::Sphere::new(Vector::zeros(), 1.0))],
            camera: Camera::default(16.0 / 9.0),
        },
    };

    let mut buffer = ImageBuffer::new(config.resolution);

    renderer::render(&mut buffer, config);

    buffer.to_ppm(
        String::from("out/image.ppm"),
        ToneMappingOperator::Clamp(255.0),
    )
}
