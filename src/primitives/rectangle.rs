/**
 * Copyright © 2019 Jayden Chan. All rights reserved.
 *
 * Nit is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3
 * as published by the Free Software Foundation.
 *
 * Nit is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Nit. If not, see <https://www.gnu.org/licenses/>.
 */
use super::Intersection;
use crate::{aabb::Aabb, ray::Ray, Vector};

#[derive(PartialEq, Eq)]
pub enum RectPlane {
    XY,
    YZ,
    XZ,
}

#[derive(Debug)]
pub struct Rectangle {
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    norm: Vector,
    plane: (usize, usize, usize),
    bbox: Aabb,
}

impl Rectangle {
    pub fn new(
        a0: f32,
        a1: f32,
        b0: f32,
        b1: f32,
        k: f32,
        norm: f32,
        P: RectPlane,
    ) -> Self {
        Rectangle {
            a0,
            a1,
            b0,
            b1,
            k,
            norm: match P {
                RectPlane::XY => Vector::new(0.0, 0.0, 1.0) * norm,
                RectPlane::YZ => Vector::new(1.0, 0.0, 0.0) * norm,
                RectPlane::XZ => Vector::new(0.0, 1.0, 0.0) * norm,
            },
            plane: match P {
                RectPlane::XY => (2, 0, 1),
                RectPlane::YZ => (0, 1, 2),
                RectPlane::XZ => (1, 0, 2),
            },
            bbox: match P {
                RectPlane::XY => Aabb::new(
                    Vector::new(a0, b0, k - 0.0001),
                    Vector::new(a1, b1, k + 0.0001),
                ),
                RectPlane::YZ => Aabb::new(
                    Vector::new(k - 0.0001, a0, b0),
                    Vector::new(k + 0.0001, a1, b1),
                ),
                RectPlane::XZ => Aabb::new(
                    Vector::new(a0, k - 0.0001, b0),
                    Vector::new(a1, k + 0.0001, b1),
                ),
            },
        }
    }

    pub fn intersect(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection> {
        let (k_ax, a_ax, b_ax) = self.plane;

        let t = (self.k - r.origin[k_ax]) / r.dir[k_ax];
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin[a_ax] + t * r.dir[a_ax];
        let y = r.origin[b_ax] + t * r.dir[b_ax];

        if x < self.a0 || x > self.a1 || y < self.b0 || y > self.b1 {
            return None;
        }

        Some(Intersection {
            u: (x - self.a0) / (self.a1 - self.a0),
            v: (y - self.b0) / (self.b1 - self.b0),
            t,
            p: r.point_at_parameter(t),
            normal: self.norm,
        })
    }

    pub fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
