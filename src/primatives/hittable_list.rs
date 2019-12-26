use crate::{
    aabb::Aabb,
    primatives::{Hit, Primative},
    ray::Ray,
};

#[derive(Debug)]
pub struct HittableList {
    hittables: Vec<Box<dyn Primative>>,
}

impl Primative for HittableList {
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

    fn bounding_box(&self) -> Option<Aabb> {
        if self.hittables.is_empty() {
            return None;
        }

        if let Some(temp_box) = self.hittables[0].bounding_box() {
            let mut ret = temp_box;

            for item in &self.hittables[1..] {
                if let Some(b) = item.bounding_box() {
                    ret = Aabb::surrounding_box(ret, b);
                } else {
                    return None;
                }
            }

            return Some(ret);
        }

        None
    }
}

impl HittableList {
    pub fn new(hittables: Vec<Box<dyn Primative>>) -> Self {
        Self { hittables }
    }
}
