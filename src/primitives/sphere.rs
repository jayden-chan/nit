//! A simple Sphere object

use crate::{aabb::Aabb, primitives::Intersection, ray::Ray, Vector};

use std::f32::consts::PI;

#[derive(Debug)]
pub struct Sphere {
    center: Vector,
    radius: f32,
    bbox: Aabb,
}

impl Sphere {
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let oc = r.origin - self.center;

        let a = r.dir.dot(r.dir);
        let b = oc.dot(r.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut q_eq = (-b - discriminant.sqrt()) / a;

            // If the minus variant is out of range try the plus one
            if q_eq >= t_max || q_eq <= t_min {
                q_eq = (-b + discriminant.sqrt()) / a;
            }

            if q_eq < t_max && q_eq > t_min {
                let point_at_parameter = r.point_at_parameter(q_eq);
                let (u, v) =
                    sphere_uv((point_at_parameter - self.center) / self.radius);
                return Some(Intersection {
                    u,
                    v,
                    t: q_eq,
                    p: point_at_parameter,
                    normal: (point_at_parameter - self.center) / self.radius,
                });
            }
        }

        None
    }

    pub fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

impl Sphere {
    pub fn new(center: Vector, radius: f32) -> Self {
        Self {
            center,
            radius,
            bbox: Aabb::new(
                center - Vector::new(radius, radius, radius),
                center + Vector::new(radius, radius, radius),
            ),
        }
    }
}

/// Computes the u and v values for a sphere
fn sphere_uv(p: Vector) -> (f32, f32) {
    let phi = f32::atan2(p.z, p.x);
    let theta = f32::asin(p.y);
    ((1.0 - (phi + PI) / (2.0 * PI)), ((theta + PI / 2.0) / PI))
}
