use crate::{
    camera::{Camera, CameraConstructor},
    materials::{Diffuse, Light},
    objects::{HittableList, RectPlane, Rectangle, Sphere},
    vector3::Vector,
};

#[derive(Debug)]
pub struct Scene {
    pub objects: HittableList,
    pub camera: Camera,
}

#[derive(Debug)]
pub struct Config {
    pub resolution: (usize, usize),
    pub samples: usize,
    pub scene: Scene,
}

/// Red ball to test with gray floor
pub fn config_test_ball() -> Config {
    Config {
        resolution: (480, 480),
        samples: 400,
        scene: Scene {
            objects: HittableList::new(vec![
                Box::new(Sphere::new(
                    Vector::new(0.0, 1.0, 0.0),
                    1.0,
                    Diffuse {
                        albedo: Vector::new(0.9, 0.1, 0.1),
                    },
                )),
                Box::new(Sphere::new(
                    Vector::new(0.0, -500.0, 0.0),
                    500.0,
                    Diffuse {
                        albedo: Vector::new(0.2, 0.2, 0.2),
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
    }
}

pub fn config_cornell_box() -> Config {
    let green = Vector::new(0.12, 0.45, 0.15);
    let red = Vector::new(0.65, 0.05, 0.05);
    let white = Vector::new(0.73, 0.73, 0.73);
    Config {
        resolution: (480, 480),
        samples: 400,
        scene: Scene {
            objects: HittableList::new(vec![
                Box::new(Rectangle::<Diffuse, { RectPlane::YZ }> {
                    a0: 0.0,
                    a1: 555.0,
                    b0: 0.0,
                    b1: 555.0,
                    k: 555.0,
                    norm: -1.0,
                    material: Diffuse { albedo: green },
                }),
                Box::new(Rectangle::<Diffuse, { RectPlane::YZ }> {
                    a0: 0.0,
                    a1: 555.0,
                    b0: 0.0,
                    b1: 555.0,
                    k: 0.0,
                    norm: 1.0,
                    material: Diffuse { albedo: red },
                }),
                Box::new(Rectangle::<Light, { RectPlane::XZ }> {
                    a0: 213.0,
                    a1: 343.0,
                    b0: 227.0,
                    b1: 332.0,
                    k: 554.9,
                    norm: -1.0,
                    material: Light {
                        emittance: Vector::new(15.0, 15.0, 15.0),
                    },
                }),
                Box::new(Rectangle::<Diffuse, { RectPlane::XZ }> {
                    a0: 0.0,
                    a1: 555.0,
                    b0: 0.0,
                    b1: 555.0,
                    k: 555.0,
                    norm: -1.0,
                    material: Diffuse { albedo: white },
                }),
                Box::new(Rectangle::<Diffuse, { RectPlane::XZ }> {
                    a0: 0.0,
                    a1: 555.0,
                    b0: 0.0,
                    b1: 555.0,
                    k: 0.0,
                    norm: 1.0,
                    material: Diffuse { albedo: white },
                }),
                Box::new(Rectangle::<Diffuse, { RectPlane::XY }> {
                    a0: 0.0,
                    a1: 555.0,
                    b0: 0.0,
                    b1: 555.0,
                    k: 555.0,
                    norm: -1.0,
                    material: Diffuse { albedo: white },
                }),
            ]),
            camera: Camera::new(CameraConstructor {
                look_from: Vector::new(278.0, 278.0, -772.0),
                look_at: Vector::new(278.0, 278.0, 0.0),
                vup: Vector::new(0.0, 1.0, 0.0),
                vfov: 40.0,
                aspect_r: 1.0,
                aperture: 0.0,
                focus_dist: 1.0,
            }),
        },
    }
}
