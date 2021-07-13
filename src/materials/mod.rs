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
use std::fmt::Debug;

use crate::{primitives::Intersection, ray::Ray, Vector};

mod dielectric;
mod diffuse;
mod light;
mod reflector;

#[derive(Debug, Copy, Clone)]
pub struct Scatter {
    pub specular: Ray,
    pub attenuation: Vector,
}

#[derive(Debug)]
pub enum Material {
    Dielectric(f32),
    Diffuse(Vector),
    Light(Vector),
    Reflector(Vector),
}

impl Material {
    pub fn scatter(&self, r: Ray, i: Intersection) -> Option<Scatter> {
        match self {
            Self::Dielectric(ref_idx) => dielectric::scatter(*ref_idx, r, i),
            Self::Diffuse(albedo) => diffuse::scatter(*albedo, r, i),
            Self::Light(_) => None,
            Self::Reflector(albedo) => reflector::scatter(*albedo, r, i),
        }
    }

    pub fn emitted(&self, r: Ray, i: Intersection) -> Vector {
        match self {
            Self::Dielectric(_) => Vector::zeros(),
            Self::Diffuse(_) => Vector::zeros(),
            Self::Light(emittance) => light::emitted(*emittance, r, i),
            Self::Reflector(_) => Vector::zeros(),
        }
    }
}
