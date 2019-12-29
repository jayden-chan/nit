use crate::math::random_in_unit_disk;
use crate::{ray::Ray, Vector};

use std::f32;

#[derive(Debug, Clone)]
pub struct Camera {
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    origin: Vector,
    lens_radius: f32,
    u: Vector,
    v: Vector,
    w: Vector,
}

pub struct CameraConstructor {
    pub look_from: Vector,
    pub look_at: Vector,
    pub vup: Vector,
    pub vfov: f32,
    pub aspect_r: f32,
    pub aperture: f32,
    pub focus_dist: f32,
}

impl Camera {
    pub fn new(cam: CameraConstructor) -> Self {
        let theta = cam.vfov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = cam.aspect_r * half_height;

        let w = (cam.look_from - cam.look_at).normalize();
        let u = cam.vup.cross(w).normalize();
        let v = w.cross(u);

        Self {
            u,
            v,
            w,
            lower_left_corner: cam.look_from
                - half_width * cam.focus_dist * u
                - half_height * cam.focus_dist * v
                - cam.focus_dist * w,
            horizontal: 2.0 * half_width * cam.focus_dist * u,
            vertical: 2.0 * half_height * cam.focus_dist * v,
            origin: cam.look_from,
            lens_radius: cam.aperture / 2.0,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            dir: self.lower_left_corner
                + self.horizontal * u
                + self.vertical * v
                - self.origin
                - offset,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(CameraConstructor {
            look_at: Vector::zeros(),
            look_from: Vector::new(0.0, 3.0, 7.0),
            vup: Vector::new(0.0, 1.0, 0.0),
            vfov: 45.0,
            aspect_r: (16.0 / 9.0),
            aperture: 0.0,
            focus_dist: 1.0,
        })
    }
}
