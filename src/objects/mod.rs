use std::fmt::Debug;

use crate::{ray::Ray, Vector};

mod sphere;
pub use sphere::*;

/// Describes an interaction between an incoming [`Ray`]
/// and an object.
///
/// [`Ray`]: struct.Ray.html
#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub u: f32,
    pub v: f32,
    pub t: f32,
    pub p: Vector,
    pub normal: Vector,
}

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        None
    }
}
