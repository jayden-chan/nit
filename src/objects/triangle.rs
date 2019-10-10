//! A triangle primitive implementec with the Möller–Trumbore algorithm

use crate::{
    aabb::Aabb,
    materials::Material,
    objects::{Hit, Hittable},
    ray::Ray,
    Vector,
};

const EPSILON: f32 = 0.0000001;

#[derive(Debug)]
pub struct Triangle<M: Material> {
    pub v0: Vector,
    pub v1: Vector,
    pub v2: Vector,
    pub material: M,
}

impl<M: Material> Hittable for Triangle<M> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;

        let h = r.dir.cross(edge2);
        let a = edge1.dot(h);

        if a > -EPSILON && a < EPSILON {
            // This ray is parallel to this triangle.
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin - self.v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * r.dir.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(q);
        if t > EPSILON && t < 1.0 / EPSILON && t > t_min && t < t_max {
            // ray intersection
            return Some(Hit {
                u,
                v,
                t,
                p: r.origin + r.dir * t,
                normal: -edge1.cross(edge2).normalize(),
                material: &self.material,
            });
        } else {
            // This means that there is a line intersection but not a ray intersection.
            return None;
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}
