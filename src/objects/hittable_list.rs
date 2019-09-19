use crate::{
    objects::{Hit, Hittable},
    ray::Ray,
};

#[derive(Debug)]
pub struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for hittable in &self.hittables {
            if let Some(hit) = hittable.hit(r, t_min, closest_so_far) {
                result = Some(hit);
                closest_so_far = hit.t;
            }
        }

        result
    }
}

impl HittableList {
    pub fn new(hittables: Vec<Box<dyn Hittable>>) -> Self {
        Self { hittables }
    }
}
