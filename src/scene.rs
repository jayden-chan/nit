use crate::{camera::Camera, objects::Hittable};

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub camera: Camera,
}
