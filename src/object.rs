use crate::{
    materials::{Material, Scatter},
    primitives::{Intersection, Primitive, Transformation},
    Vector,
};

#[derive(Debug)]
pub struct Object {
    pub primitive: Primitive,
    pub transformation: Option<Transformation>,
    pub material: Material,
}

/// Describes an interaction between an incoming [`Ray`]
/// and an object.
///
/// [`Ray`]: struct.Ray.html
#[derive(Debug)]
pub struct Hit {
    // The intersection location
    pub intersection: Intersection,
    /// The material that was hit
    pub scattered: Option<Scatter>,
    pub emitted: Vector,
}
