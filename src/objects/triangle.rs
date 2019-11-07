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
    v0: Vector,
    v1: Vector,
    v2: Vector,
    normal: Vector,
    material: M,
    edge1: Vector,
    edge2: Vector,
}

impl<M: Material> Triangle<M> {
    pub fn new(
        v0: Vector,
        v1: Vector,
        v2: Vector,
        norm: f32,
        material: M,
    ) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        Self {
            v0,
            v1,
            v2,
            normal: edge1.cross(edge2).normalize() * norm,
            material,
            edge1,
            edge2,
        }
    }
}

impl<M: Material> Hittable for Triangle<M> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let h = r.dir.cross(self.edge2);
        let a = self.edge1.dot(h);

        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin - self.v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(self.edge1);
        let v = f * r.dir.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * self.edge2.dot(q);
        if t > EPSILON && t < 1.0 / EPSILON && t > t_min && t < t_max {
            return Some(Hit {
                u,
                v,
                t,
                p: r.origin + r.dir * t,
                normal: self.normal,
                material: &self.material,
            });
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}
