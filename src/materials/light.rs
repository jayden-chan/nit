use crate::{
    materials::{Material, Scatter},
    objects::Hit,
    ray::Ray,
    Vector,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Light {
    pub emittance: Vector,
}

#[typetag::serde]
impl Material for Light {
    fn scatter(&self, _r: Ray, _hit: Hit) -> Option<Scatter> {
        None
    }

    fn emitted(&self, r: Ray, hit: Hit) -> Vector {
        if hit.normal.dot(r.dir) < 0.0 {
            self.emittance
        } else {
            Vector::zeros()
        }
    }
}
