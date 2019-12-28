use crate::{
    materials::Scatter, math::random_on_unit_sphere, primitives::Intersection,
    ray::Ray, Vector,
};

pub fn scatter(albedo: Vector, _r: Ray, i: Intersection) -> Option<Scatter> {
    let scattered = i.p + i.normal + random_on_unit_sphere();

    let specular = Ray {
        origin: i.p,
        dir: (scattered - i.p),
    };

    Some(Scatter {
        specular,
        attenuation: albedo,
    })
}
