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
    materials::{Material, Scatter},
    primitives::{Intersection, Primitive, Transformation},
    Vector,
};

#[derive(Debug)]
pub struct Object {
    pub primitive: Primitive,
    pub transformation: Option<Transformation>,
    pub material: Material,
}

/// Describes an interaction between an incoming [`Ray`]
/// and an object.
///
/// [`Ray`]: struct.Ray.html
#[derive(Debug)]
pub struct Hit {
    // The intersection location
    pub intersection: Intersection,
    /// The material that was hit
    pub scattered: Option<Scatter>,
    pub emitted: Vector,
}
