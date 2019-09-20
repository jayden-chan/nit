use std::fmt::Debug;

use crate::{objects::Hit, ray::Ray, Vector};

mod diffuse;
pub use diffuse::*;

mod light;
pub use light::*;

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
