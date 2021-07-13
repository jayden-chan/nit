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
use super::{Intersection, Primitive};
use crate::{aabb::Aabb, ray::Ray, Vector};

#[derive(Debug, Copy, Clone)]
pub struct Translate<H: Primitive> {
    pub hittable: H,
    pub offset: Vector,
}

impl<H: Primitive> Primitive for Translate<H> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
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

    fn bounding_box(&self) -> Option<Aabb> {
        self.hittable.bounding_box().map(|mut aabb| {
            aabb.min += self.offset;
            aabb.max += self.offset;
            aabb
        })
    }
}
