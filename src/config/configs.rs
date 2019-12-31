use crate::{
    bvh::Bvh,
    camera::{Camera, CameraConstructor},
    color::ToneMappingOperator,
    config::{Config, Scene},
    materials::Material,
    object::Object,
    primitives::{Block, Primitive, RectPlane, Rectangle, Sphere},
    stl_loader::StlLoader,
    vector3::Vector,
};

#[allow(dead_code)]
const R_240: (usize, usize) = (240, 135);
#[allow(dead_code)]
const R_480: (usize, usize) = (480, 270);
#[allow(dead_code)]
const R_1600: (usize, usize) = (1600, 900);
#[allow(dead_code)]
const R_1920: (usize, usize) = (1920, 1080);

use std::{fs, io};

#[allow(dead_code)]
pub fn config_stl_test() -> Config {
    let mut file = fs::File::open("test/squirtle_starter_1gen_flowalistik.STL")
        .map(io::BufReader::new)
        .unwrap();

    let mut objects: Vec<Object> = StlLoader::parse(&mut file)
        .into_iter()
        .map(|t| Object {
            primitive: t,
            transformation: None,
            material: Material::Diffuse(Vector::new(0.9, 0.1, 0.1)),
        })
        .collect();

    objects.push(Object {
        primitive: Primitive::Rectangle(Rectangle::new(
            -100_000.0,
            100_000.0,
            -100_000.0,
            100_000.0,
            -1.0,
            1.0,
            RectPlane::XY,
        )),
        transformation: None,
        material: Material::Diffuse(Vector::new(0.73, 0.73, 0.73)),
    });

    objects.push(Object {
        primitive: Primitive::Block(Block::new(
            Vector::new(90.0, -90.0, 0.0),
            Vector::new(110.0, -110.0, 20.0),
        )),
        transformation: None,
        material: Material::Dielectric(1.52),
    });

    objects.push(Object {
        primitive: Primitive::Sphere(Sphere::new(
            Vector::new(20.0, 0.0, 120.0),
            15.0,
        )),
        transformation: None,
        material: Material::Light(Vector::new(15.0, 14.0, 12.0)),
    });

    Config {
        resolution: R_240,
        samples: 100,
        tmo: ToneMappingOperator::ReinhardJodie,
        scene: Scene {
            objects: Bvh::new(objects),
            camera: Camera::new(CameraConstructor {
                look_at: Vector::new(-90.0, 10.0, 30.0),
                look_from: Vector::new(120.0, -60.0, 20.0),
                vup: Vector::new(0.0, 0.0, 1.0),
                vfov: 32.0,
                aspect_r: 16.0 / 9.0,
                aperture: 0.0,
                focus_dist: 1.0,
            }),
        },
    }
}

// #[allow(dead_code)]
// pub fn config_test_ball() -> Config {
//     Config {
//         resolution: (480, 480),
//         samples: 400,
//         tmo: ToneMappingOperator::ReinhardJodie,
//         scene: Scene {
//             objects: Box::new(HittableList::new(vec![
//                 Box::new(Sphere::new(
//                     Vector::new(0.0, 1.0, 0.0),
//                     1.0,
//                     Diffuse {
//                         albedo: Vector::new(0.9, 0.1, 0.1),
//                     },
//                 )),
//                 Box::new(Sphere::new(
//                     Vector::new(0.0, -500.0, 0.0),
//                     500.0,
//                     Diffuse {
//                         albedo: Vector::new(0.2, 0.2, 0.2),
//                     },
//                 )),
//                 Box::new(Sphere::new(
//                     Vector::new(10.0, 14.0, 7.0),
//                     4.0,
//                     Light {
//                         emittance: Vector::new(15.0, 15.0, 15.0),
//                     },
//                 )),
//             ])),
//             camera: Camera::default(1.0),
//         },
//     }
// }

// #[allow(dead_code)]
// pub fn config_cornell_box_cubes() -> Config {
//     let size = 1000.0;
//     let mut objects = cornell_box(size);

//     objects.push(Box::new(Translate {
//         offset: Vector::new(130.0, 0.0, 65.0),
//         hittable: Rotate::<Block<Diffuse>, { RotationAxis::Y }>::new(
//             Block::new(
//                 Vector::zeros(),
//                 Vector::new(165.0, 165.0, 165.0),
//                 Diffuse {
//                     albedo: Vector::new(0.73, 0.73, 0.73),
//                 },
//             ),
//             -18.0,
//         ),
//     }));

//     objects.push(Box::new(Translate {
//         offset: Vector::new(265.0, 0.0, 295.0),
//         hittable: Rotate::<Block<Diffuse>, { RotationAxis::Y }>::new(
//             Block::new(
//                 Vector::zeros(),
//                 Vector::new(165.0, 330.0, 165.0),
//                 Diffuse {
//                     albedo: Vector::new(0.73, 0.73, 0.73),
//                 },
//             ),
//             15.0,
//         ),
//     }));

//     Config {
//         resolution: (250, 250),
//         samples: 400,
//         tmo: ToneMappingOperator::ReinhardJodie,
//         scene: Scene {
//             objects: Box::new(HittableList::new(objects)),
//             camera: Camera::new(CameraConstructor {
//                 look_from: Vector::new(size / 2.0, size / 2.0, -1100.0),
//                 look_at: Vector::new(size / 2.0, size / 2.0, 0.0),
//                 vup: Vector::new(0.0, 1.0, 0.0),
//                 vfov: 40.0,
//                 aspect_r: 1.0,
//                 aperture: 0.0,
//                 focus_dist: 1.0,
//             }),
//         },
//     }
// }

// #[allow(dead_code)]
// pub fn config_glass() -> Config {
//     Config {
//         resolution: (250, 250),
//         samples: 400,
//         tmo: ToneMappingOperator::ReinhardJodie,
//         scene: Scene {
//             objects: Box::new(HittableList::new(vec![
//                 Box::new(Rectangle::<Diffuse, { RectPlane::XZ }>::new(
//                     -50000.0,
//                     50000.0,
//                     -50000.0,
//                     50000.0,
//                     0.0,
//                     1.0,
//                     Diffuse {
//                         albedo: Vector::new(0.5, 0.5, 0.5),
//                     },
//                 )),
//                 Box::new(Sphere::new(
//                     Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
//                     100.0,
//                     Dielectric { ref_idx: 1.52 },
//                 )),
//                 Box::new(Rectangle::<Light, { RectPlane::XZ }>::new(
//                     213.0,
//                     343.0,
//                     227.0,
//                     332.0,
//                     554.9,
//                     -1.0,
//                     Light {
//                         emittance: Vector::new(18.0, 18.0, 18.0),
//                     },
//                 )),
//             ])),
//             camera: Camera::new(CameraConstructor {
//                 look_from: Vector::new(278.0, 278.0, -772.0),
//                 look_at: Vector::new(278.0, 278.0, 0.0),
//                 vup: Vector::new(0.0, 1.0, 0.0),
//                 vfov: 40.0,
//                 aspect_r: 1.0,
//                 aperture: 0.0,
//                 focus_dist: 1.0,
//             }),
//         },
//     }
// }

// #[allow(dead_code)]
// pub fn config_triangle_test() -> Config {
//     Config {
//         resolution: (400, 400),
//         samples: 1000,
//         tmo: ToneMappingOperator::ReinhardJodie,
//         scene: Scene {
//             objects: Box::new(HittableList::new(vec![
//                 Box::new(Rectangle::<Diffuse, { RectPlane::XZ }>::new(
//                     -50000.0,
//                     50000.0,
//                     -50000.0,
//                     50000.0,
//                     0.0,
//                     1.0,
//                     Diffuse {
//                         albedo: Vector::new(0.5, 0.5, 0.5),
//                     },
//                 )),
//                 Box::new(Triangle::new(
//                     Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
//                     Vector::new(555.0, 300.0, 555.0),
//                     Vector::new(0.0, 300.0, 555.0),
//                     -1.0,
//                     Diffuse {
//                         albedo: Vector::new(1.0, 0.2, 0.2),
//                     },
//                 )),
//                 Box::new(Triangle::new(
//                     Vector::new(555.0, 0.0, 100.0),
//                     Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
//                     Vector::new(555.0, 300.0, 555.0),
//                     1.0,
//                     Diffuse {
//                         albedo: Vector::new(1.0, 0.2, 0.2),
//                     },
//                 )),
//                 Box::new(Triangle::new(
//                     Vector::new(0.0, 0.0, 100.0),
//                     Vector::new(0.0, 300.0, 555.0),
//                     Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
//                     1.0,
//                     Diffuse {
//                         albedo: Vector::new(1.0, 0.2, 0.2),
//                     },
//                 )),
//                 Box::new(Triangle::new(
//                     Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
//                     Vector::new(555.0, 0.0, 100.0),
//                     Vector::new(555.0 / 2.0, 0.0, 555.0 / 2.0),
//                     1.0,
//                     Diffuse {
//                         albedo: Vector::new(1.0, 0.2, 0.2),
//                     },
//                 )),
//                 Box::new(Triangle::new(
//                     Vector::new(555.0 / 2.0, 100.0, 555.0 / 2.0),
//                     Vector::new(0.0, 0.0, 100.0),
//                     Vector::new(555.0 / 2.0, 0.0, 555.0 / 2.0),
//                     -1.0,
//                     Diffuse {
//                         albedo: Vector::new(1.0, 0.2, 0.2),
//                     },
//                 )),
//                 Box::new(Sphere::new(
//                     Vector::new(555.0 / 2.0, 750.0, 600.0),
//                     100.0,
//                     Light {
//                         emittance: Vector::new(18.0, 18.0, 18.0),
//                     },
//                 )),
//             ])),
//             camera: Camera::new(CameraConstructor {
//                 look_from: Vector::new(278.0, 278.0, -772.0),
//                 look_at: Vector::new(278.0, 278.0, 0.0),
//                 vup: Vector::new(0.0, 1.0, 0.0),
//                 vfov: 40.0,
//                 aspect_r: 1.0,
//                 aperture: 0.0,
//                 focus_dist: 1.0,
//             }),
//         },
//     }
// }

// #[allow(dead_code)]
// pub fn config_triangle_test_two() -> Config {
//     let mut objects = cornell_box(555.0);

//     objects.append(&mut vec![Box::new(Triangle::new(
//         Vector::new(555.0 / 2.0, 100.0, 200.0),
//         Vector::new(555.0 - 100.0, 300.0, 400.0),
//         Vector::new(100.0, 300.0, 400.0),
//         1.0,
//         Diffuse {
//             albedo: Vector::new(1.0, 0.2, 0.2),
//         },
//     ))]);

//     Config {
//         resolution: (300, 300),
//         samples: 400,
//         tmo: ToneMappingOperator::ReinhardJodie,
//         scene: Scene {
//             objects: Box::new(HittableList::new(objects)),
//             camera: Camera::new(CameraConstructor {
//                 look_from: Vector::new(278.0, 278.0, -772.0),
//                 look_at: Vector::new(278.0, 278.0, 0.0),
//                 vup: Vector::new(0.0, 1.0, 0.0),
//                 vfov: 40.0,
//                 aspect_r: 1.0,
//                 aperture: 0.0,
//                 focus_dist: 1.0,
//             }),
//         },
//     }
// }

// #[allow(dead_code)]
// pub fn config_cornell_box() -> Config {
//     let objects = cornell_box(555.0);

//     Config {
//         resolution: (250, 250),
//         samples: 400,
//         tmo: ToneMappingOperator::ReinhardJodie,
//         scene: Scene {
//             objects: Box::new(HittableList::new(objects)),
//             camera: Camera::new(CameraConstructor {
//                 look_from: Vector::new(278.0, 278.0, -772.0),
//                 look_at: Vector::new(278.0, 278.0, 0.0),
//                 vup: Vector::new(0.0, 1.0, 0.0),
//                 vfov: 40.0,
//                 aspect_r: 1.0,
//                 aperture: 0.0,
//                 focus_dist: 1.0,
//             }),
//         },
//     }
// }

// #[allow(dead_code)]
// fn cornell_box(size: f32) -> Vec<Box<dyn Primitive>> {
//     let green = Vector::new(0.12, 0.45, 0.15);
//     let red = Vector::new(0.65, 0.05, 0.05);
//     let white = Vector::new(0.73, 0.73, 0.73);

//     vec![
//         // Green wall (left)
//         Box::new(Rectangle::<Diffuse, { RectPlane::YZ }>::new(
//             0.0,
//             size,
//             0.0,
//             size,
//             size,
//             -1.0,
//             Diffuse { albedo: green },
//         )),
//         // Red wall (right)
//         Box::new(Rectangle::<Diffuse, { RectPlane::YZ }>::new(
//             0.0,
//             size,
//             0.0,
//             size,
//             0.0,
//             1.0,
//             Diffuse { albedo: red },
//         )),
//         // Light
//         Box::new(Rectangle::<Light, { RectPlane::XZ }>::new(
//             // Magic numbers Pepega Clap
//             size / 2.60563380281690140845,
//             size / 1.61807580174927113702,
//             size / 2.44493392070484581497,
//             size / 1.67168674698795180722,
//             size - 0.01,
//             -1.0,
//             Light {
//                 emittance: Vector::new(25.2 / 2.0, 18.7 / 2.0, 6.0 / 2.0),
//             },
//         )),
//         // Ceiling
//         Box::new(Rectangle::<Diffuse, { RectPlane::XZ }>::new(
//             0.0,
//             size,
//             0.0,
//             size,
//             size,
//             -1.0,
//             Diffuse { albedo: white },
//         )),
//         // Floor
//         Box::new(Rectangle::<Diffuse, { RectPlane::XZ }>::new(
//             0.0,
//             size,
//             0.0,
//             size,
//             0.0,
//             1.0,
//             Diffuse { albedo: white },
//         )),
//         // Back wall
//         Box::new(Rectangle::<Diffuse, { RectPlane::XY }>::new(
//             0.0,
//             size,
//             0.0,
//             size,
//             size,
//             -1.0,
//             Diffuse { albedo: white },
//         )),
//     ]
// }
