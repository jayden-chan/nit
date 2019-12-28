use crate::{
    aabb::Aabb,
    object::{Hit, Object},
    primitives::{Intersection, Primitive},
    ray::Ray,
};

use rand::prelude::*;
use std::cmp::Ordering;
use std::f32;

#[derive(Debug)]
enum BvhNodeType {
    Internal((Box<Bvh>, Box<Bvh>)),
    SingleLeaf(Object),
    DoubleLeaf((Object, Object)),
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
                BvhNodeType::SingleLeaf(l) => {
                    return l.primitive.hit(r, t_min, t_max).map(|i| Hit {
                        intersection: i,
                        scattered: l.material.scatter(r, i),
                        emitted: l.material.emitted(r, i),
                    });
                }
                BvhNodeType::DoubleLeaf((left, right)) => {
                    return (match (
                        left.primitive.hit(r, t_min, t_max),
                        right.primitive.hit(r, t_min, t_max),
                    ) {
                        (None, None) => None,
                        (Some(i), None) => Some((i, &left.material)),
                        (None, Some(i)) => Some((i, &right.material)),
                        (Some(i_l), Some(i_r)) => {
                            if i_l.t < i_r.t {
                                Some((i_l, &left.material))
                            } else {
                                Some((i_r, &right.material))
                            }
                        }
                    })
                    .map(|(i, m)| Hit {
                        intersection: i,
                        scattered: m.scatter(r, i),
                        emitted: m.emitted(r, i),
                    });
                }
            }
        }

        None
    }
}

#[derive(PartialEq, Eq)]
enum SortAxis {
    X = 0,
    Y = 1,
    Z = 2,
}

fn sort_by_axis<const A: SortAxis>(a: &Object, b: &Object) -> Ordering {
    let box_left = a.primitive.bounding_box();
    let box_right = b.primitive.bounding_box();

    if box_left.min[A as usize] - box_right.min[A as usize] < 0.0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

impl Bvh {
    pub fn construct(mut objects: Vec<Object>) -> Self {
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
            1 => {
                let obj = objects.remove(0);
                let bounding_box = obj.primitive.bounding_box();
                return Self {
                    node_type: BvhNodeType::SingleLeaf(obj),
                    bounding_box,
                };
            }
            2 => {
                let left = objects.remove(0);
                let right = objects.remove(0);
                let box_left = left.primitive.bounding_box();
                let box_right = right.primitive.bounding_box();

                let bounding_box = Aabb::surrounding_box(box_left, box_right);
                return Self {
                    node_type: BvhNodeType::DoubleLeaf((left, right)),
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
