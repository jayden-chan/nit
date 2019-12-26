use crate::{
    aabb::Aabb,
    primatives::{Hit, Primative},
    ray::Ray,
};

use rand::prelude::*;
use std::cmp::Ordering;
use std::f32;

#[derive(Debug)]
pub struct Bvh {
    left: Box<dyn Primative>,
    right: Box<dyn Primative>,
    bounding_box: Aabb,
}

impl Primative for Bvh {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if self.bounding_box.hit(r, t_min, t_max) {
            return match (
                self.left.hit(r, t_min, t_max),
                self.right.hit(r, t_min, t_max),
            ) {
                (None, None) => None,
                (Some(rec), None) | (None, Some(rec)) => Some(rec),
                (Some(rec_l), Some(rec_r)) => {
                    if rec_l.t < rec_r.t {
                        Some(rec_l)
                    } else {
                        Some(rec_r)
                    }
                }
            };
        }

        None
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}

#[derive(PartialEq, Eq)]
enum SortAxis {
    X = 0,
    Y = 1,
    Z = 2,
}

fn sort_by_axis<const A: SortAxis>(
    a: &Box<dyn Primative>,
    b: &Box<dyn Primative>,
) -> Ordering {
    let box_left = a.bounding_box().unwrap();
    let box_right = b.bounding_box().unwrap();

    if box_left.min[A as usize] - box_right.min[A as usize] < 0.0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

impl Bvh {
    pub fn construct(
        objects: &mut Vec<Box<dyn Primative>>,
    ) -> Box<dyn Primative> {
        let axis = (3.0 * random::<f32>()) as u32;

        match axis {
            0 => {
                objects.sort_unstable_by(sort_by_axis::<{ SortAxis::X }>);
            }
            1 => {
                objects.sort_unstable_by(sort_by_axis::<{ SortAxis::Y }>);
            }
            _ => {
                objects.sort_unstable_by(sort_by_axis::<{ SortAxis::Z }>);
            }
        }

        match objects.len() {
            0 => panic!("wrong bvh length"),
            1 => objects.remove(0),
            l => {
                let l_vec = objects;
                let mut r_vec = l_vec.split_off(l / 2);
                let left = Self::construct(l_vec);
                let right = Self::construct(&mut r_vec);

                let box_left = left.bounding_box().unwrap();
                let box_right = right.bounding_box().unwrap();

                let bounding_box = Aabb::surrounding_box(box_left, box_right);
                Box::new(Self {
                    left,
                    right,
                    bounding_box,
                })
            }
        }
    }
}
