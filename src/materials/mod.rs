use std::fmt::Debug;

use crate::{objects::Hit, ray::Ray, Vector};

mod dielectric;
pub use dielectric::*;

mod diffuse;
pub use diffuse::*;

mod light;
pub use light::*;

mod reflector;
pub use reflector::*;

pub struct Scatter {
    pub specular: Ray,
    pub attenuation: Vector,
}

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, r: Ray, hit: Hit) -> Option<Scatter> {
        None
    }

    fn emitted(&self) -> Vector {
        Vector::zeros()
    }
}
