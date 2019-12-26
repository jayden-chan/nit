mod configs;
pub use configs::*;

use crate::{camera::Camera, color::ToneMappingOperator, primatives::Hittable};

#[derive(Debug)]
pub struct Scene {
    pub objects: Box<dyn Hittable>,
    pub camera: Camera,
}

#[derive(Debug)]
pub struct Config {
    pub resolution: (usize, usize),
    pub samples: usize,
    pub tmo: ToneMappingOperator,
    pub scene: Scene,
}
