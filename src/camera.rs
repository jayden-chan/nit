use crate::Vector;

#[derive(Debug)]
pub struct Camera {
    pub lower_left_corner: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub origin: Vector,
    pub lens_radius: f32,
    pub u: Vector,
    pub v: Vector,
    pub w: Vector,
    pub t0: f32,
    pub t1: f32,
}
