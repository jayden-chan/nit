use std::fmt::Debug;

use crate::{materials::Material, ray::Ray, Vector};

mod hittable_list;
pub use hittable_list::*;

mod rectangle;
pub use rectangle::*;

mod sphere;
pub use sphere::*;

/// Describes an interaction between an incoming [`Ray`]
/// and an object.
///
/// [`Ray`]: struct.Ray.html
#[derive(Debug, Copy, Clone)]
pub struct Hit<'a> {
    pub u: f32,
    pub v: f32,
    /// The point t along the ray where the intersection occurs
    pub t: f32,
    /// The intersection point in 3D space
    pub p: Vector,
    /// The intersection surface normal
    pub normal: Vector,
    /// The material that was hit
    pub material: &'a dyn Material,
}

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        None
    }
}
