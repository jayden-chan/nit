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
mod configs;
pub use configs::*;

use crate::{bvh::Bvh, camera::Camera, color::ToneMappingOperator};

#[derive(Debug)]
pub struct Scene {
    pub objects: Bvh,
    pub camera: Camera,
}

#[derive(Debug)]
pub struct Config {
    pub resolution: (u16, u16),
    pub samples: usize,
    pub tmo: ToneMappingOperator,
    pub scene: Scene,
}
