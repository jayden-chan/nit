use crate::{
    materials::{Material, Scatter},
    primitives::Intersection,
    ray::Ray,
    Vector,
};

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub emittance: Vector,
}

impl Material for Light {
    fn scatter(&self, _r: Ray, _i: Intersection) -> Option<Scatter> {
        None
    }

    fn emitted(&self, r: Ray, i: Intersection) -> Vector {
        if i.normal.dot(r.dir) < 0.0 {
            self.emittance
        } else {
            Vector::zeros()
        }
    }
}
