use std::fmt::Debug;

use crate::{aabb::Aabb, materials::Material, ray::Ray, Vector};

mod block;
pub use block::*;

mod hittable_list;
pub use hittable_list::*;

mod rectangle;
pub use rectangle::*;

mod rotate;
pub use rotate::*;

mod sphere;
pub use sphere::*;

mod translate;
pub use translate::*;

mod triangle;
pub use triangle::*;

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

pub trait Primative: Debug + Send + Sync {
    fn hit(&self, _r: Ray, _t_min: f32, _t_max: f32) -> Option<Hit>;
    fn bounding_box(&self) -> Option<Aabb>;
}
