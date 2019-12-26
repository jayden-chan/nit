use super::{HittableList, Intersection, Primitive, RectPlane, Rectangle};
use crate::{aabb::Aabb, ray::Ray, Vector};

#[derive(Debug)]
pub struct Block {
    p_min: Vector,
    p_max: Vector,
    sides: HittableList,
}

impl Primitive for Block {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(self.p_min, self.p_max))
    }
}

impl Block {
    pub fn new(p0: Vector, p1: Vector) -> Self {
        let objects: Vec<Box<dyn Primitive>> = vec![
            Box::new(Rectangle::<{ RectPlane::XY }>::new(
                p0.x, p1.x, p0.y, p1.y, p1.z, 1.0,
            )),
            Box::new(Rectangle::<{ RectPlane::XY }>::new(
                p0.x, p1.x, p0.y, p1.y, p0.z, -1.0,
            )),
            Box::new(Rectangle::<{ RectPlane::XZ }>::new(
                p0.x, p1.x, p0.z, p1.z, p1.y, 1.0,
            )),
            Box::new(Rectangle::<{ RectPlane::XZ }>::new(
                p0.x, p1.x, p0.z, p1.z, p0.y, -1.0,
            )),
            Box::new(Rectangle::<{ RectPlane::YZ }>::new(
                p0.y, p1.y, p0.z, p1.z, p1.x, 1.0,
            )),
            Box::new(Rectangle::<{ RectPlane::YZ }>::new(
                p0.y, p1.y, p0.z, p1.z, p0.x, -1.0,
            )),
        ];

        Self {
            p_min: p0,
            p_max: p1,
            sides: HittableList::new(objects),
        }
    }
}
