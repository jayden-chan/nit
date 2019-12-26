use crate::{
    materials::{Material, Scatter},
    math::random_on_unit_sphere,
    primitives::Intersection,
    ray::Ray,
    Vector,
};

#[derive(Debug, Copy, Clone)]
pub struct Diffuse {
    pub albedo: Vector,
}

impl Material for Diffuse {
    fn scatter(&self, _r: Ray, i: Intersection) -> Option<Scatter> {
        let scattered = i.p + i.normal + random_on_unit_sphere();

        let specular = Ray {
            origin: i.p,
            dir: (scattered - i.p),
        };

        Some(Scatter {
            specular,
            attenuation: self.albedo,
        })
    }
}
