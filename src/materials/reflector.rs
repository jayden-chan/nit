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
    materials::Scatter, math::vector_reflect, primitives::Intersection,
    ray::Ray, Vector,
};

pub fn scatter(albedo: Vector, r: Ray, i: Intersection) -> Option<Scatter> {
    let reflected = vector_reflect(r.dir.normalize(), i.normal);

    let specular = Ray {
        origin: i.p,
        dir: reflected,
    };

    if specular.dir.dot(i.normal) > 0.0 {
        Some(Scatter {
            specular,
            attenuation: albedo,
        })
    } else {
        None
    }
}
