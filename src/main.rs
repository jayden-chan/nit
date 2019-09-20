// FIXME: REMOVE THIS ONCE STUFF IS BROUGHT OVER FROM OLD TRACER
#![allow(dead_code)]

mod camera;
mod color;
mod config;
mod image;
mod materials;
mod math;
mod objects;
mod ray;
mod renderer;
mod scene;
mod vector3;

use crate::{
    camera::Camera,
    color::ToneMappingOperator,
    config::Config,
    image::ImageBuffer,
    materials::{Dielectric, Diffuse, Light, Reflector},
    objects::HittableList,
    objects::Sphere,
    scene::Scene,
    vector3::Vector,
};

fn main() -> Result<(), String> {
    let config = Config {
        resolution: (480, 480),
        samples: 400,
        scene: Scene {
            objects: HittableList::new(vec![
                Box::new(Sphere::new(
                    Vector::new(0.0, 1.0, 0.0),
                    1.0,
                    Dielectric { ref_idx: 1.52 },
                )),
                Box::new(Sphere::new(
                    Vector::new(0.0, -500.0, 0.0),
                    500.0,
                    Diffuse {
                        albedo: Vector::new(0.3, 0.3, 0.3),
                    },
                )),
                Box::new(Sphere::new(
                    Vector::new(10.0, 14.0, 7.0),
                    4.0,
                    Light {
                        emittance: Vector::new(15.0, 15.0, 15.0),
                    },
                )),
            ]),
            camera: Camera::default(1.0),
        },
    };

    let mut buffer = ImageBuffer::new(config.resolution);

    renderer::render(&mut buffer, config);

    buffer.to_ppm(
        String::from("out/image.ppm"),
        ToneMappingOperator::Clamp(1.0),
    )
}
