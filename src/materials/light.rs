use crate::{materials::Material, objects::Hit, ray::Ray, Vector};

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub emittance: Vector,
}

impl Material for Light {
    fn emitted(&self, r: Ray, hit: Hit) -> Vector {
        if hit.normal.dot(r.dir) < 0.0 {
            self.emittance
        } else {
            Vector::zeros()
        }
    }
}
