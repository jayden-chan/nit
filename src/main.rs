mod camera;
mod config;
mod math;
mod objects;
mod scene;
mod vector3;

use camera::Camera;
use config::Config;
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
                u: Vector::zeros(),
                v: Vector::zeros(),
                w: Vector::zeros(),
                t0: 0.0,
                t1: 0.0,
            },
        },
    };
}
