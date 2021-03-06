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
use super::{Intersection, RectPlane, Rectangle};
use crate::{aabb::Aabb, ray::Ray, Vector};

#[derive(Debug)]
pub struct Block {
    bbox: Aabb,
    sides: [Rectangle; 6],
}

impl Block {
    pub fn new(p0: Vector, p1: Vector) -> Self {
        let objects: [Rectangle; 6] = [
            Rectangle::new(p0.x, p1.x, p0.y, p1.y, p1.z, 1.0, RectPlane::XY),
            Rectangle::new(p0.x, p1.x, p0.y, p1.y, p0.z, -1.0, RectPlane::XY),
            Rectangle::new(p0.x, p1.x, p0.z, p1.z, p1.y, 1.0, RectPlane::XY),
            Rectangle::new(p0.x, p1.x, p0.z, p1.z, p0.y, -1.0, RectPlane::XY),
            Rectangle::new(p0.y, p1.y, p0.z, p1.z, p1.x, 1.0, RectPlane::XY),
            Rectangle::new(p0.y, p1.y, p0.z, p1.z, p0.x, -1.0, RectPlane::XY),
        ];

        Self {
            bbox: Aabb::new(p0, p1),
            sides: objects,
        }
    }

    pub fn intersect(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for hittable in &self.sides {
            if let Some(hit) = hittable.intersect(r, t_min, closest_so_far) {
                result = Some(hit);
                closest_so_far = hit.t;
            }
        }

        result
    }

    pub fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
