use crate::{
    materials::{Material, Scatter},
    math::random_on_unit_sphere,
    objects::Hit,
    ray::Ray,
    Vector,
};

#[derive(Debug, Copy, Clone)]
pub struct Diffuse {
    pub albedo: Vector,
}

impl Material for Diffuse {
    fn scatter(&self, _r: Ray, hit: Hit) -> Option<Scatter> {
        let scattered = hit.p + hit.normal + random_on_unit_sphere();

        let specular = Ray {
            origin: hit.p,
            dir: (scattered - hit.p),
        };

        Some(Scatter {
            specular,
            attenuation: self.albedo,
        })
    }
}
