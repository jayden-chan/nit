use crate::math::Onb;
use crate::Vector;

#[derive(Debug)]
pub struct Camera {
    pub lower_left_corner: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub origin: Vector,
    pub lens_radius: f32,
    pub uvw: Onb,
    pub t0: f32,
    pub t1: f32,
}
