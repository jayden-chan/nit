use crate::{
    bvh::Bvh,
    camera::{Camera, CameraConstructor},
    color::ToneMappingOperator,
    materials::{Dielectric, Diffuse, Light},
    objects::{
        Block, Hittable, HittableList, RectPlane, Rectangle, Rotate,
        RotationAxis, Sphere, Translate, Triangle,
    },
    vector3::Vector,
};

#[derive(Debug)]
pub struct Scene {
    pub objects: Box<dyn Hittable>,
    pub camera: Camera,
}

#[derive(Debug)]
pub struct Config {
    pub resolution: (usize, usize),
    pub samples: usize,
    pub tmo: ToneMappingOperator,
    pub scene: Scene,
}

pub fn config_test_ball() -> Config {
    Config {
        resolution: (480, 480),
        samples: 400,
        tmo: ToneMappingOperator::ReinhardJodie,
        scene: Scene {
            objects: Box::new(HittableList::new(vec![
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
            ])),
            camera: Camera::default(1.0),
        },
    }
}

pub fn config_cornell_box_cubes() -> Config {
    let green = Vector::new(0.12, 0.45, 0.15);
    let red = Vector::new(0.65, 0.05, 0.05);
    let white = Vector::new(0.73, 0.73, 0.73);

    let mut objects = cornell_box();
    objects.push(Box::new(Translate {
        offset: Vector::new(130.0, 0.0, 65.0),
        hittable: Rotate::<Block<Diffuse>, { RotationAxis::Y }>::new(
            Block::new(
                Vector::zeros(),
                Vector::new(165.0, 165.0, 165.0),
                Diffuse {
                    albedo: Vector::new(0.73, 0.73, 0.73),
                },
            ),
            -18.0,
        ),
    }));

    objects.push(Box::new(Translate {
        offset: Vector::new(265.0, 0.0, 295.0),
        hittable: Rotate::<Block<Diffuse>, { RotationAxis::Y }>::new(
            Block::new(
                Vector::zeros(),
                Vector::new(165.0, 330.0, 165.0),
                Diffuse {
                    albedo: Vector::new(0.73, 0.73, 0.73),
                },
            ),
            15.0,
        ),
    }));

    Config {
        resolution: (250, 250),
        samples: 400,
        tmo: ToneMappingOperator::ReinhardJodie,
        scene: Scene {
            objects: Box::new(HittableList::new(objects)),
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

pub fn config_glass() -> Config {
    Config {
        resolution: (250, 250),
        samples: 400,
        tmo: ToneMappingOperator::ReinhardJodie,
        scene: Scene {
            objects: Box::new(HittableList::new(vec![
                Box::new(Rectangle::<Diffuse, { RectPlane::XZ }>::new(
                    -50000.0,
                    50000.0,
                    -50000.0,
                    50000.0,
                    0.0,
                    1.0,
                    Diffuse {
                        albedo: Vector::new(0.5, 0.5, 0.5),
                    },
                )),
                Box::new(Sphere::new(
                    Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
                    100.0,
                    Dielectric { ref_idx: 1.52 },
                )),
                Box::new(Rectangle::<Light, { RectPlane::XZ }>::new(
                    213.0,
                    343.0,
                    227.0,
                    332.0,
                    554.9,
                    -1.0,
                    Light {
                        emittance: Vector::new(18.0, 18.0, 18.0),
                    },
                )),
            ])),
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

pub fn config_triangle_test() -> Config {
    Config {
        resolution: (400, 400),
        samples: 1000,
        tmo: ToneMappingOperator::ReinhardJodie,
        scene: Scene {
            objects: Box::new(HittableList::new(vec![
                Box::new(Rectangle::<Diffuse, { RectPlane::XZ }>::new(
                    -50000.0,
                    50000.0,
                    -50000.0,
                    50000.0,
                    0.0,
                    1.0,
                    Diffuse {
                        albedo: Vector::new(0.5, 0.5, 0.5),
                    },
                )),
                Box::new(Triangle::new(
                    Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
                    Vector::new(555.0, 300.0, 555.0),
                    Vector::new(0.0, 300.0, 555.0),
                    -1.0,
                    Diffuse {
                        albedo: Vector::new(1.0, 0.2, 0.2),
                    },
                )),
                Box::new(Triangle::new(
                    Vector::new(555.0, 0.0, 100.0),
                    Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
                    Vector::new(555.0, 300.0, 555.0),
                    1.0,
                    Diffuse {
                        albedo: Vector::new(1.0, 0.2, 0.2),
                    },
                )),
                Box::new(Triangle::new(
                    Vector::new(0.0, 0.0, 100.0),
                    Vector::new(0.0, 300.0, 555.0),
                    Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
                    1.0,
                    Diffuse {
                        albedo: Vector::new(1.0, 0.2, 0.2),
                    },
                )),
                Box::new(Triangle::new(
                    Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
                    Vector::new(555.0, 0.0, 100.0),
                    Vector::new(555.0 / 2.0, 0.0, 555.0 / 2.0),
                    1.0,
                    Diffuse {
                        albedo: Vector::new(1.0, 0.2, 0.2),
                    },
                )),
                Box::new(Triangle::new(
                    Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
                    Vector::new(0.0, 0.0, 100.0),
                    Vector::new(555.0 / 2.0, 0.0, 555.0 / 2.0),
                    -1.0,
                    Diffuse {
                        albedo: Vector::new(1.0, 0.2, 0.2),
                    },
                )),
                Box::new(Sphere::new(
                    Vector::new(555.0 / 2.0, 750.0, 600.0),
                    100.0,
                    Light {
                        emittance: Vector::new(18.0, 18.0, 18.0),
                    },
                )),
            ])),
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

pub fn config_triangle_test_two() -> Config {
    let mut objects = cornell_box();

    objects.append(&mut vec![Box::new(Triangle::new(
        Vector::new(555.0 / 2.0, 100.0, 200.0),
        Vector::new(555.0 - 100.0, 300.0, 400.0),
        Vector::new(100.0, 300.0, 400.0),
        1.0,
        Diffuse {
            albedo: Vector::new(1.0, 0.2, 0.2),
        },
    ))]);

    Config {
        resolution: (300, 300),
        samples: 400,
        tmo: ToneMappingOperator::ReinhardJodie,
        scene: Scene {
            objects: Box::new(HittableList::new(objects)),
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

pub fn config_cornell_box() -> Config {
    let mut objects = cornell_box();

    Config {
        resolution: (250, 250),
        samples: 400,
        tmo: ToneMappingOperator::ReinhardJodie,
        scene: Scene {
            objects: Box::new(HittableList::new(objects)),
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

fn cornell_box() -> Vec<Box<Hittable>> {
    let green = Vector::new(0.12, 0.45, 0.15);
    let red = Vector::new(0.65, 0.05, 0.05);
    let white = Vector::new(0.73, 0.73, 0.73);

    vec![
        Box::new(Rectangle::<Diffuse, { RectPlane::YZ }>::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            -1.0,
            Diffuse { albedo: green },
        )),
        Box::new(Rectangle::<Diffuse, { RectPlane::YZ }>::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            1.0,
            Diffuse { albedo: red },
        )),
        Box::new(Rectangle::<Light, { RectPlane::XZ }>::new(
            213.0,
            343.0,
            227.0,
            332.0,
            554.9,
            -1.0,
            Light {
                emittance: Vector::new(25.2 / 2.0, 18.7 / 2.0, 6.0 / 2.0),
            },
        )),
        Box::new(Rectangle::<Diffuse, { RectPlane::XZ }>::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            -1.0,
            Diffuse { albedo: white },
        )),
        Box::new(Rectangle::<Diffuse, { RectPlane::XZ }>::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            1.0,
            Diffuse { albedo: white },
        )),
        Box::new(Rectangle::<Diffuse, { RectPlane::XY }>::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            -1.0,
            Diffuse { albedo: white },
        )),
    ]
}
