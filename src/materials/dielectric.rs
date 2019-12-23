use crate::{
    materials::{Material, Scatter},
    math::{vector_reflect, vector_refract},
    objects::Hit,
    ray::Ray,
    Vector,
};

use rand::prelude::*;

use serde::{Deserialize, Serialize};
use std::f32;

/// The Dielectric material type. This material partially
/// reflects and refracts rays that interact with it.
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
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

#[typetag::serde]
impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, hit_record: Hit) -> Option<Scatter> {
        let reflected = vector_reflect(r_in.dir, hit_record.normal);

        let (outward_normal, ni_over_nt, cosine) =
            if r_in.dir.dot(hit_record.normal) > 0.0 {
                (
                    -hit_record.normal,
                    self.ref_idx,
                    self.ref_idx
                        * r_in.dir.dot(hit_record.normal)
                        * r_in.dir.inv_mag(),
                )
            } else {
                (
                    hit_record.normal,
                    1.0 / self.ref_idx,
                    -r_in.dir.dot(hit_record.normal) * r_in.dir.inv_mag(),
                )
            };

        let refracted = vector_refract(r_in.dir, outward_normal, ni_over_nt);

        let reflect_probability = if refracted.is_some() {
            schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        if random::<f32>() >= reflect_probability {
            Some(Scatter {
                specular: Ray {
                    origin: hit_record.p,
                    dir: refracted.unwrap(),
                },
                attenuation: Vector::ones(),
            })
        } else {
            Some(Scatter {
                specular: Ray {
                    origin: hit_record.p,
                    dir: reflected,
                },
                attenuation: Vector::ones(),
            })
        }
    }
}
