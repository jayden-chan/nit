mod configs;
pub use configs::*;

use crate::{camera::Camera, color::ToneMappingOperator, primatives::Primative};

#[derive(Debug)]
pub struct Scene {
    pub objects: Box<dyn Primative>,
    pub camera: Camera,
}

#[derive(Debug)]
pub struct Config {
    pub resolution: (usize, usize),
    pub samples: usize,
    pub tmo: ToneMappingOperator,
    pub scene: Scene,
}
