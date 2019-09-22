use super::{Hit, Hittable};
use crate::{aabb::Aabb, ray::Ray, Vector};

#[derive(Debug, Copy, Clone)]
pub struct Translate<H: Hittable> {
    pub hittable: H,
    pub offset: Vector,
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let moved_r = Ray {
            origin: r.origin - self.offset,
            dir: r.dir,
        };

        self.hittable
            .hit(moved_r, t_min, t_max)
            .map(|mut hit_record| {
                hit_record.p += self.offset;
                hit_record
            })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        self.hittable.bounding_box(t0, t1).map(|mut aabb| {
            aabb.min += self.offset;
            aabb.max += self.offset;
            aabb
        })
    }
}
