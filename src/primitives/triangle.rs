//! A triangle primitive implemented with the Möller–Trumbore algorithm

use crate::{aabb::Aabb, primitives::Intersection, ray::Ray, Vector};

const EPSILON: f32 = 0.0000001;

#[derive(Debug)]
pub struct Triangle {
    v0: Vector,
    v1: Vector,
    v2: Vector,
    normal: Vector,
    edge1: Vector,
    edge2: Vector,
    bbox: Aabb,
}

impl Triangle {
    pub fn new(v0: Vector, v1: Vector, v2: Vector, normal: Vector) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        let min_x = f32::min(f32::min(v0.x, v1.x), f32::min(v1.x, v2.x));
        let min_y = f32::min(f32::min(v0.y, v1.y), f32::min(v1.y, v2.y));
        let min_z = f32::min(f32::min(v0.z, v1.z), f32::min(v1.z, v2.z));
        let max_x = f32::max(f32::max(v0.x, v1.x), f32::max(v1.x, v2.x));
        let max_y = f32::max(f32::max(v0.y, v1.y), f32::max(v1.y, v2.y));
        let max_z = f32::max(f32::max(v0.z, v1.z), f32::max(v1.z, v2.z));

        let bbox = Aabb::new(
            Vector::new(min_x, min_y, min_z),
            Vector::new(max_x, max_y, max_z),
        );

        Self {
            v0,
            v1,
            v2,
            normal,
            edge1,
            edge2,
            bbox,
        }
    }

    pub fn intersect(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection> {
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
            Some(Intersection {
                u,
                v,
                t,
                p: r.origin + r.dir * t,
                normal: self.normal,
            })
        } else {
            None
        }
    }

    pub fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
