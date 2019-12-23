use crate::{
    materials::{Material, Scatter},
    math::random_on_unit_sphere,
    objects::Hit,
    ray::Ray,
    Vector,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Diffuse {
    pub albedo: Vector,
}

#[typetag::serde]
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
