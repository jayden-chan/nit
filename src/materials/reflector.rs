use crate::{
    materials::{Material, Scatter},
    math::vector_reflect,
    objects::Hit,
    ray::Ray,
    Vector,
};

#[derive(Debug)]
pub struct Reflector {
    pub albedo: Vector,
}

impl Material for Reflector {
    fn scatter(&self, r: Ray, hit: Hit) -> Option<Scatter> {
        let reflected = vector_reflect(r.dir.normalize(), hit.normal);

        let specular = Ray {
            origin: hit.p,
            dir: reflected,
        };

        if Vector::dot(specular.dir, hit.normal) > 0.0 {
            Some(Scatter {
                specular,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
