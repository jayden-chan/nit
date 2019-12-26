use std::fmt::Debug;

use crate::{aabb::Aabb, ray::Ray, Vector};

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

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    pub u: f32,
    pub v: f32,
    /// The point t along the ray where the intersection occurs
    pub t: f32,
    /// The intersection point in 3D space
    pub p: Vector,
    /// The intersection surface normal
    pub normal: Vector,
}

pub trait Primitive: Debug + Send + Sync {
    fn hit(&self, _r: Ray, _t_min: f32, _t_max: f32) -> Option<Intersection>;
    fn bounding_box(&self) -> Option<Aabb>;
}
