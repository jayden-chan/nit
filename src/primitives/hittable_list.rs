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
use crate::{
    aabb::Aabb,
    primitives::{Intersection, Primitive},
    ray::Ray,
};

#[derive(Debug)]
pub struct HittableList {
    hittables: Vec<Box<dyn Primitive>>,
}

impl Primitive for HittableList {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for hittable in &self.hittables {
            if let Some(hit) = hittable.hit(r, t_min, closest_so_far) {
                result = Some(hit);
                closest_so_far = hit.t;
            }
        }

        result
    }

    fn bounding_box(&self) -> Option<Aabb> {
        if self.hittables.is_empty() {
            return None;
        }

        if let Some(temp_box) = self.hittables[0].bounding_box() {
            let mut ret = temp_box;

            for item in &self.hittables[1..] {
                if let Some(b) = item.bounding_box() {
                    ret = Aabb::surrounding_box(ret, b);
                } else {
                    return None;
                }
            }

            return Some(ret);
        }

        None
    }
}

impl HittableList {
    pub fn new(hittables: Vec<Box<dyn Primitive>>) -> Self {
        Self { hittables }
    }
}
