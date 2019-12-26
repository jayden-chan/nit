use std::fmt::Debug;

use crate::{primatives::Hit, ray::Ray, Vector};

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
    fn scatter(&self, _r: Ray, _hit: Hit) -> Option<Scatter>;

    fn emitted(&self, _r: Ray, _hit: Hit) -> Vector {
        Vector::zeros()
    }
}
