use crate::{
    materials::{Material, Scatter},
    math::vector_reflect,
    primitives::Intersection,
    ray::Ray,
    Vector,
};

#[derive(Debug, Copy, Clone)]
pub struct Reflector {
    pub albedo: Vector,
}

impl Material for Reflector {
    fn scatter(&self, r: Ray, i: Intersection) -> Option<Scatter> {
        let reflected = vector_reflect(r.dir.normalize(), i.normal);

        let specular = Ray {
            origin: i.p,
            dir: reflected,
        };

        if specular.dir.dot(i.normal) > 0.0 {
            Some(Scatter {
                specular,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
