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
    pub resolution: (usize, usize),
    pub samples: usize,
    pub tmo: ToneMappingOperator,
    pub scene: Scene,
}
