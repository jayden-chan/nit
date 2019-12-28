use std::fmt::Debug;

use crate::{aabb::Aabb, ray::Ray, Vector};

mod block;
pub use block::*;

mod rectangle;
pub use rectangle::*;

mod sphere;
pub use sphere::*;

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

// pub trait Primitive: Debug + Send + Sync {
//     fn hit(&self, _r: Ray, _t_min: f32, _t_max: f32) -> Option<Intersection>;
//     fn bounding_box(&self) -> Option<Aabb>;
// }

#[derive(Debug, PartialEq, Eq)]
pub enum RotationAxis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub enum Transformation {
    Rotate(f32, RotationAxis),
    Translate(Vector),
}

#[derive(Debug)]
pub enum Primitive {
    Block(Block),
    Rectangle(Rectangle),
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Primitive {
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        match self {
            Primitive::Block(o) => o.hit(r, t_min, t_max),
            Primitive::Rectangle(o) => o.hit(r, t_min, t_max),
            Primitive::Sphere(o) => o.hit(r, t_min, t_max),
            Primitive::Triangle(o) => o.hit(r, t_min, t_max),
        }
    }

    pub fn bounding_box(&self) -> Aabb {
        match self {
            Primitive::Block(o) => o.bounding_box(),
            Primitive::Rectangle(o) => o.bounding_box(),
            Primitive::Sphere(o) => o.bounding_box(),
            Primitive::Triangle(o) => o.bounding_box(),
        }
    }
}
