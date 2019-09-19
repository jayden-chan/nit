use crate::{camera::Camera, objects::HittableList};

#[derive(Debug)]
pub struct Scene {
    pub objects: HittableList,
    pub camera: Camera,
}
