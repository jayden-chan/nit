use crate::{materials::Material, Vector};

#[derive(Debug)]
pub struct Light {
    pub emittance: Vector,
}

impl Material for Light {
    fn emitted(&self) -> Vector {
        self.emittance
    }
}
