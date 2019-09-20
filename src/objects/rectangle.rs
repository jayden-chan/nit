use super::{Hit, Hittable};
use crate::{materials::Material, ray::Ray, Vector};

pub enum RectPlane {
    XY,
    YZ,
    XZ,
}

#[derive(Debug)]
pub struct Rectangle<M: Material, const P: RectPlane> {
    pub a0: f32,
    pub a1: f32,
    pub b0: f32,
    pub b1: f32,
    pub k: f32,
    pub norm: f32,
    pub material: M,
}

impl<M: Material, const P: RectPlane> Hittable for Rectangle<M, { P }> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let (k_ax, a_ax, b_ax) = match P {
            RectPlane::XY => (2, 0, 1),
            RectPlane::YZ => (0, 1, 2),
            RectPlane::XZ => (1, 0, 2),
        };

        let t = (self.k - r.origin[k_ax]) / r.dir[k_ax];
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin[a_ax] + t * r.dir[a_ax];
        let y = r.origin[b_ax] + t * r.dir[b_ax];

        if x < self.a0 || x > self.a1 || y < self.b0 || y > self.b1 {
            return None;
        }

        let normal = match P {
            RectPlane::XY => Vector::new(0.0, 0.0, 1.0),
            RectPlane::YZ => Vector::new(1.0, 0.0, 0.0),
            RectPlane::XZ => Vector::new(0.0, 1.0, 0.0),
        };

        Some(Hit {
            u: (x - self.a0) / (self.a1 - self.a0),
            v: (y - self.b0) / (self.b1 - self.b0),
            t,
            p: r.point_at_parameter(t),
            normal: normal * self.norm,
            material: &self.material,
        })
    }
}