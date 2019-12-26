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
pub struct Bvh {
    children: Option<(Box<Bvh>, Box<Bvh>)>,
    left_leaf: Option<Object>,
    right_leaf: Option<Object>,
    bounding_box: Aabb,
}

// pub struct Bvh<'a> {
//     left: Object<'a>,
//     right: Object<'a>,
//     bounding_box: Aabb,
// }

impl Bvh {
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if self.bounding_box.hit(r, t_min, t_max) {
            let mut curr = self;
            if curr.children.is_some() {
                loop {
                    let (left, right) = curr.children.as_ref().unwrap();
                    let next = match (
                        left.hit(r, t_min, t_max),
                        right.hit(r, t_min, t_max),
                    ) {
                        (None, None) => None,
                        (Some(_), None) => Some(left),
                        (None, Some(_)) => Some(right),
                        (Some(rec_l), Some(rec_r)) => {
                            if rec_l.intersection.t < rec_r.intersection.t {
                                Some(left)
                            } else {
                                Some(right)
                            }
                        }
                    };

                    if next.is_some() {
                        curr = &next.unwrap();
                        if curr.children.is_none() {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            return match (curr.left_leaf.as_ref(), curr.right_leaf.as_ref()) {
                (None, None) => None,
                (Some(leaf), None) | (None, Some(leaf)) => {
                    leaf.primitive.hit(r, t_min, t_max).map(|h| Hit {
                        intersection: h,
                        scattered: leaf.material.scatter(r, h),
                        emitted: leaf.material.emitted(r, h),
                    })
                }
                (Some(left_leaf), Some(right_leaf)) => {
                    match (
                        left_leaf.primitive.hit(r, t_min, t_max),
                        right_leaf.primitive.hit(r, t_min, t_max),
                    ) {
                        (None, None) => None,
                        (Some(h), None) => Some(Hit {
                            intersection: h,
                            scattered: left_leaf.material.scatter(r, h),
                            emitted: left_leaf.material.emitted(r, h),
                        }),
                        (None, Some(h)) => Some(Hit {
                            intersection: h,
                            scattered: right_leaf.material.scatter(r, h),
                            emitted: right_leaf.material.emitted(r, h),
                        }),
                        (Some(h_l), Some(h_r)) => {
                            if h_l.t < h_r.t {
                                Some(Hit {
                                    intersection: h_l,
                                    scattered: left_leaf
                                        .material
                                        .scatter(r, h_l),
                                    emitted: left_leaf.material.emitted(r, h_l),
                                })
                            } else {
                                Some(Hit {
                                    intersection: h_r,
                                    scattered: right_leaf
                                        .material
                                        .scatter(r, h_r),
                                    emitted: right_leaf
                                        .material
                                        .emitted(r, h_r),
                                })
                            }
                        }
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

fn sort_by_axis<const A: SortAxis>(a: &Object, b: &Object) -> Ordering {
    let box_left = a.primitive.bounding_box().unwrap();
    let box_right = b.primitive.bounding_box().unwrap();

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
                let bounding_box = obj.primitive.bounding_box().unwrap();
                return Self {
                    children: None,
                    left_leaf: Some(obj),
                    right_leaf: None,
                    bounding_box,
                };
            }
            2 => {
                let left = objects.remove(0);
                let right = objects.remove(0);
                let box_left = left.primitive.bounding_box().unwrap();
                let box_right = right.primitive.bounding_box().unwrap();

                let bounding_box = Aabb::surrounding_box(box_left, box_right);
                return Self {
                    children: None,
                    left_leaf: Some(left),
                    right_leaf: Some(right),
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
                    children: Some((Box::new(left), Box::new(right))),
                    left_leaf: None,
                    right_leaf: None,
                    bounding_box,
                }
            }
        }
    }
}
