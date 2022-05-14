#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

mod aabb;
mod bvh;
mod camera;
mod color;
mod config;
mod image;
mod materials;
mod math;
mod object;
mod primitives;
mod ray;
mod renderer;
mod stl_loader;
mod vector3;

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
use crate::config::*;
use crate::{image::ImageBuffer, vector3::Vector};

use std::time;

fn main() -> Result<(), String> {
    let start_time = time::Instant::now();
    // let config = config_stl_test();
    let config = config_cornell_box();
    let tmo = config.tmo;

    let mut buffer = ImageBuffer::new(config.resolution);
    renderer::render(&mut buffer, config);
    println!("\nCompleted rendering in {:#?}", start_time.elapsed());
    buffer.to_ppm(String::from("out/image.ppm"), tmo)
}
