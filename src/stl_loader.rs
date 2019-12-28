use crate::{
    primitives::{Primitive, Triangle},
    Vector,
};
use std::io;

pub struct StlLoader;

impl StlLoader {
    pub fn parse<R>(source: &mut R) -> Vec<Primitive>
    where
        R: io::Read + io::Seek,
    {
        let stl = stl_io::read_stl(source).unwrap();
        let mut ret: Vec<Primitive> = Vec::new();

        stl.faces.iter().for_each(|f| {
            let v0 = stl.vertices[f.vertices[0]];
            let v0 = Vector::new(v0[0], v0[1], v0[2]);
            let v1 = stl.vertices[f.vertices[1]];
            let v1 = Vector::new(v1[0], v1[1], v1[2]);
            let v2 = stl.vertices[f.vertices[2]];
            let v2 = Vector::new(v2[0], v2[1], v2[2]);

            let normal = -Vector::new(f.normal[0], f.normal[1], f.normal[2]);
            ret.push(Primitive::Triangle(Triangle::with_normal(
                v0, v1, v2, normal,
            )));
        });

        ret
    }
}
