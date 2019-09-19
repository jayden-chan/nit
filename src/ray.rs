use crate::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn point_at_parameter(self, t: f32) -> Vector {
        self.origin + self.dir * t
    }
}
