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
            Box::new(Rectangle::<M, { RectPlane::XY }> {
                a0: p0.x,
                a1: p1.x,
                b0: p0.y,
                b1: p1.y,
                k: p1.z,
                norm: 1.0,
                material,
            }),
            Box::new(Rectangle::<M, { RectPlane::XY }> {
                a0: p0.x,
                a1: p1.x,
                b0: p0.y,
                b1: p1.y,
                k: p0.z,
                norm: -1.0,
                material,
            }),
            Box::new(Rectangle::<M, { RectPlane::XZ }> {
                a0: p0.x,
                a1: p1.x,
                b0: p0.z,
                b1: p1.z,
                k: p1.y,
                norm: 1.0,
                material,
            }),
            Box::new(Rectangle::<M, { RectPlane::XZ }> {
                a0: p0.x,
                a1: p1.x,
                b0: p0.z,
                b1: p1.z,
                k: p0.y,
                norm: -1.0,
                material,
            }),
            Box::new(Rectangle::<M, { RectPlane::YZ }> {
                a0: p0.y,
                a1: p1.y,
                b0: p0.z,
                b1: p1.z,
                k: p1.x,
                norm: 1.0,
                material,
            }),
            Box::new(Rectangle::<M, { RectPlane::YZ }> {
                a0: p0.y,
                a1: p1.y,
                b0: p0.z,
                b1: p1.z,
                k: p0.x,
                norm: -1.0,
                material,
            }),
        ];

        Self {
            p_min: p0,
            p_max: p1,
            sides: HittableList::new(objects),
            material,
        }
    }
}
