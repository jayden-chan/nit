use crate::{
    materials::{Material, Scatter},
    math::vector_reflect,
    primatives::Hit,
    ray::Ray,
    Vector,
};

#[derive(Debug, Copy, Clone)]
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

        if specular.dir.dot(hit.normal) > 0.0 {
            Some(Scatter {
                specular,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
