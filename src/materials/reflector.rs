use crate::{
    materials::Scatter, math::vector_reflect, primitives::Intersection,
    ray::Ray, Vector,
};

pub fn scatter(albedo: Vector, r: Ray, i: Intersection) -> Option<Scatter> {
    let reflected = vector_reflect(r.dir.normalize(), i.normal);

    let specular = Ray {
        origin: i.p,
        dir: reflected,
    };

    if specular.dir.dot(i.normal) > 0.0 {
        Some(Scatter {
            specular,
            attenuation: albedo,
        })
    } else {
        None
    }
}
