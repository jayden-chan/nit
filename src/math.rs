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
use crate::Vector;

use rand::prelude::*;
use std::ops;

// TODO: This function should be replaced with something less hacky
#[allow(clippy::all)] // Pointer casts in here cause warnings
pub fn fast_inv_sqrt(x: f32) -> f32 {
    let i: u32 = unsafe { std::mem::transmute(x) };
    let j = 0x5F3759DF - (i >> 1);
    let y: f32 = unsafe { std::mem::transmute(j) };
    y * (1.5 - 0.5 * x * y * y)
}

pub fn random_in_unit_disk() -> Vector {
    let mut rng = rand::thread_rng();
    let mut p = 2.0 * Vector::new(rng.gen(), rng.gen(), 0.0)
        - Vector::new(1.0, 1.0, 0.0);

    while p.dot(p) >= 1.0 {
        p = 2.0 * Vector::new(rng.gen(), rng.gen(), 0.0)
            - Vector::new(1.0, 1.0, 0.0);
    }

    p
}

pub fn random_in_unit_sphere() -> Vector {
    let mut p = 2.0 * Vector::rand() - Vector::ones();

    while p.dot(p) >= 1.0 {
        p = 2.0 * Vector::rand() - Vector::ones();
    }

    p
}

pub fn random_on_unit_sphere() -> Vector {
    random_in_unit_sphere().normalize()
}

pub fn vector_reflect(v: Vector, n: Vector) -> Vector {
    v - 2.0 * v.dot(n) * n
}

pub fn vector_refract(v: Vector, n: Vector, ni_over_nt: f32) -> Option<Vector> {
    let v = v.normalize();
    let dt = v.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (v - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Onb {
    axis: [Vector; 3],
}

impl Onb {
    pub fn u(&self) -> Vector {
        self.axis[0]
    }

    pub fn v(&self) -> Vector {
        self.axis[1]
    }

    pub fn w(&self) -> Vector {
        self.axis[2]
    }

    pub fn local(&self, a: Vector) -> Vector {
        a.x * self.u() + a.y * self.v() + a.z * self.w()
    }
}

impl Onb {
    pub fn build_from_w(n: Vector) -> Self {
        let mut axis = [Vector::zeros(); 3];
        axis[2] = n.normalize();

        let a = if f32::abs(axis[2].x) > 0.9 {
            Vector::new(0.0, 1.0, 0.0)
        } else {
            Vector::new(1.0, 0.0, 0.0)
        };

        axis[1] = axis[2].cross(a).normalize();
        axis[0] = axis[2].cross(axis[1]);

        Self { axis }
    }
}

impl ops::Index<usize> for Onb {
    type Output = Vector;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.axis[idx]
    }
}
