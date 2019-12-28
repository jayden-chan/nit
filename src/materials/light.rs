use crate::{primitives::Intersection, ray::Ray, Vector};

pub fn emitted(emittance: Vector, r: Ray, i: Intersection) -> Vector {
    if i.normal.dot(r.dir) < 0.0 {
        emittance
    } else {
        Vector::zeros()
    }
}
