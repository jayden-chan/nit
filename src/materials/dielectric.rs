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
    materials::Scatter,
    math::{vector_reflect, vector_refract},
    primitives::Intersection,
    ray::Ray,
    Vector,
};

use rand::prelude::*;
use std::f32;

/// The Dielectric material type. This material partially
/// reflects and refracts rays that interact with it.
#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    pub ref_idx: f32,
}

/// Provides an approximation for the contribution of the Fresnel factor
/// during reflection calculations
///
/// [More info](https://en.wikipedia.org/wiki/Schlick%27s_approximation)
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}

pub fn scatter(ref_idx: f32, r_in: Ray, i: Intersection) -> Option<Scatter> {
    let reflected = vector_reflect(r_in.dir, i.normal);

    let (outward_normal, ni_over_nt, cosine) = if r_in.dir.dot(i.normal) > 0.0 {
        (
            -i.normal,
            ref_idx,
            ref_idx * r_in.dir.dot(i.normal) * r_in.dir.inv_mag(),
        )
    } else {
        (
            i.normal,
            1.0 / ref_idx,
            -r_in.dir.dot(i.normal) * r_in.dir.inv_mag(),
        )
    };

    let refracted = vector_refract(r_in.dir, outward_normal, ni_over_nt);

    let reflect_probability = if refracted.is_some() {
        schlick(cosine, ref_idx)
    } else {
        1.0
    };

    if random::<f32>() >= reflect_probability {
        Some(Scatter {
            specular: Ray {
                origin: i.p,
                dir: refracted.unwrap(),
            },
            attenuation: Vector::ones(),
        })
    } else {
        Some(Scatter {
            specular: Ray {
                origin: i.p,
                dir: reflected,
            },
            attenuation: Vector::ones(),
        })
    }
}
