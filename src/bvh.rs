use crate::{
    aabb::Aabb,
    object::{Hit, Object},
    ray::Ray,
};

use rand::prelude::*;
use std::cmp::Ordering;
use std::f32;

#[derive(Debug)]
enum BvhNodeType {
    Internal((Box<Bvh>, Box<Bvh>)),
    Leaf(Object),
}

#[derive(Debug)]
pub struct Bvh {
    node_type: BvhNodeType,
    bounding_box: Aabb,
}

impl Bvh {
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if self.bounding_box.hit(r, t_min, t_max) {
            match &self.node_type {
                BvhNodeType::Internal((left, right)) => {
                    match (
                        left.hit(r, t_min, t_max),
                        right.hit(r, t_min, t_max),
                    ) {
                        (None, None) => return None,
                        (Some(h), None) | (None, Some(h)) => return Some(h),
                        (Some(h_l), Some(h_r)) => {
                            if h_l.intersection.t < h_r.intersection.t {
                                return Some(h_l);
                            } else {
                                return Some(h_r);
                            }
                        }
                    }
                }
                BvhNodeType::Leaf(l) => {
                    return l.primitive.hit(r, t_min, t_max).map(|i| Hit {
                        intersection: i,
                        scattered: l.material.scatter(r, i),
                        emitted: l.material.emitted(r, i),
                    });
                }
            }
        }

        None
    }
}

impl Bvh {
    pub fn construct(mut objects: Vec<Object>) -> Self {
        let idx = (3.0 * random::<f32>()) as u32;
        let len = objects.len();

        objects.partition_at_index_by(len / 2, |a: &Object, b: &Object| {
            let box_left = a.primitive.bounding_box();
            let box_right = b.primitive.bounding_box();

            if box_left.min[idx as usize] - box_right.min[idx as usize] < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        match len {
            0 => panic!("wrong bvh length"),
            1 => {
                let obj = objects.remove(0);
                let bounding_box = obj.primitive.bounding_box();
                return Self {
                    node_type: BvhNodeType::Leaf(obj),
                    bounding_box,
                };
            }
            l => {
                let mut l_vec = objects;
                let r_vec = l_vec.split_off(l / 2);
                let left = Self::construct(l_vec);
                let right = Self::construct(r_vec);

                let bounding_box = Aabb::surrounding_box(
                    left.bounding_box,
                    right.bounding_box,
                );

                Self {
                    node_type: BvhNodeType::Internal((
                        Box::new(left),
                        Box::new(right),
                    )),
                    bounding_box,
                }
            }
        }
    }
}
