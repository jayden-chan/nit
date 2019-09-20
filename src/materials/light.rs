use crate::{materials::Material, objects::Hit, ray::Ray, Vector};

#[derive(Debug)]
pub struct Light {
    pub emittance: Vector,
}

impl Material for Light {
    fn emitted(&self, r: Ray, hit: Hit) -> Vector {
        if Vector::dot(hit.normal, r.dir) < 0.0 {
            self.emittance
        } else {
            Vector::zeros()
        }
    }
}
