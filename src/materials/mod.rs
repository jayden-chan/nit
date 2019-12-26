use std::fmt::Debug;

use crate::{primitives::Intersection, ray::Ray, Vector};

mod dielectric;
pub use dielectric::*;

mod diffuse;
pub use diffuse::*;

mod light;
pub use light::*;

mod reflector;
pub use reflector::*;

#[derive(Debug, Copy, Clone)]
pub struct Scatter {
    pub specular: Ray,
    pub attenuation: Vector,
}

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, _r: Ray, _i: Intersection) -> Option<Scatter>;

    fn emitted(&self, _r: Ray, _i: Intersection) -> Vector {
        Vector::zeros()
    }
}
