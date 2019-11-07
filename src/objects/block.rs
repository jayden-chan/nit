use super::{Hit, Hittable, HittableList, RectPlane, Rectangle};
use crate::{aabb::Aabb, materials::Material, ray::Ray, Vector};

#[derive(Debug)]
pub struct Block<M: Material> {
    p_min: Vector,
    p_max: Vector,
    sides: HittableList,
    material: M,
}

impl<M: Material> Hittable for Block<M> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(self.p_min, self.p_max))
    }
}

impl<M> Block<M>
where
    M: 'static + Material + Copy,
{
    pub fn new(p0: Vector, p1: Vector, material: M) -> Self {
        let objects: Vec<Box<dyn Hittable>> = vec![
            Box::new(Rectangle::<M, { RectPlane::XY }>::new(
                p0.x, p1.x, p0.y, p1.y, p1.z, 1.0, material,
            )),
            Box::new(Rectangle::<M, { RectPlane::XY }>::new(
                p0.x, p1.x, p0.y, p1.y, p0.z, -1.0, material,
            )),
            Box::new(Rectangle::<M, { RectPlane::XZ }>::new(
                p0.x, p1.x, p0.z, p1.z, p1.y, 1.0, material,
            )),
            Box::new(Rectangle::<M, { RectPlane::XZ }>::new(
                p0.x, p1.x, p0.z, p1.z, p0.y, -1.0, material,
            )),
            Box::new(Rectangle::<M, { RectPlane::YZ }>::new(
                p0.y, p1.y, p0.z, p1.z, p1.x, 1.0, material,
            )),
            Box::new(Rectangle::<M, { RectPlane::YZ }>::new(
                p0.y, p1.y, p0.z, p1.z, p0.x, -1.0, material,
            )),
        ];

        Self {
            p_min: p0,
            p_max: p1,
            sides: HittableList::new(objects),
            material,
        }
    }
}
