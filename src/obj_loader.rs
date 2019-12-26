/**
 * Adapted from obj_parser.rs by Aleksey Kladov
 * @matklad on GitHub
 * https://github.com/matklad/rustraytracer/blob/946e5c1f7d7dfadf5209c6f505453835bcd12f50/libs/geom/src/shape/mesh/obj_parser.rs
 */
use crate::{
    materials::Material,
    objects::{Hittable, HittableList, Triangle},
    Vector,
};

use std::{error::Error, fmt, io, num};

#[derive(Debug)]
pub struct ObjLoadError;

impl fmt::Display for ObjLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl Error for ObjLoadError {
    fn description(&self) -> &str {
        "Failed to load Obj file"
    }
}

pub struct ObjLoader {
    points: Vec<Vector>,
    normals: Vec<Vector>,
    faces: Vec<Box<dyn Hittable>>,
}

impl ObjLoader {
    pub fn new() -> Self {
        ObjLoader {
            points: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn parse<M: 'static + Material>(
        &mut self,
        source: &mut dyn io::Read,
        material: M,
    ) -> Result<Box<dyn Hittable>, Box<dyn Error>> {
        let mut s = String::new();
        source.read_to_string(&mut s)?;
        for line in s.lines() {
            if line.starts_with("v ") {
                self.parse_vertex(line)?;
            } else if line.starts_with("vn ") {
                self.parse_normal(line)?;
            } else if line.starts_with("f ") {
                self.parse_face(line, material.clone())?;
            }
        }

        Ok(Box::new(HittableList::new(self.faces)))
    }

    fn parse_vertex(&mut self, s: &str) -> Result<(), Box<dyn Error>> {
        let coords = ObjLoader::parse_coordinates(s)?;
        self.points.push(Vector::new(coords.0, coords.1, coords.2));
        Ok(())
    }

    fn parse_normal(&mut self, s: &str) -> Result<(), Box<dyn Error>> {
        let coords = ObjLoader::parse_coordinates(s)?;
        self.normals
            .push(Vector::new(coords.0, coords.1, coords.2).normalize());
        Ok(())
    }

    fn parse_face_simple<M: 'static + Material>(
        &mut self,
        s: &str,
        material: M,
    ) -> Result<(), Box<dyn Error>> {
        let inds = s
            .split_whitespace()
            .skip(1)
            .map(read_index)
            .collect::<Result<Vec<usize>, num::ParseIntError>>()?;

        if inds.len() != 3 {
            return Err(Box::new(ObjLoadError));
        }
        let (a, b, c) = (
            self.points[inds[0]],
            self.points[inds[1]],
            self.points[inds[2]],
        );

        if are_valid_points(a, b, c) {
            let f = Triangle::new(a, b, c, 1.0, material);
            self.faces.push(Box::new(f));
        }
        Ok(())
    }

    fn parse_face_normals<M: 'static + Material>(
        &mut self,
        s: &str,
        material: M,
    ) -> Result<(), Box<dyn Error>> {
        fn read_group(
            s: &str,
        ) -> Result<(usize, usize, usize), Box<dyn Error>> {
            let inds = s
                .split('/')
                .map(|s| read_index(s))
                .collect::<Result<Vec<_>, _>>()?;
            if inds.len() != 3 {
                return Err(Box::new(ObjLoadError));
            }
            Ok((inds[0], inds[1], inds[2]))
        }

        let verts = s
            .split_whitespace()
            .skip(1)
            .map(read_group)
            .collect::<Result<Vec<_>, _>>()?;

        if verts.len() != 3 {
            return Err(Box::new(ObjLoadError));
        }

        let (a, b, c) = (
            self.points[verts[0].0],
            self.points[verts[1].0],
            self.points[verts[2].0],
        );

        if are_valid_points(a, b, c) {
            let f = Triangle::with_normals(
                self.points[verts[0].0],
                self.points[verts[1].0],
                self.points[verts[2].0],
                1.0,
                [
                    self.normals[verts[0].2],
                    self.normals[verts[1].2],
                    self.normals[verts[2].2],
                ],
                material,
            );

            self.faces.push(Box::new(f));
        }
        Ok(())
    }

    fn parse_face<M: 'static + Material>(
        &mut self,
        s: &str,
        material: M,
    ) -> Result<(), Box<dyn Error>> {
        if s.contains('/') {
            self.parse_face_normals(s, material)
        } else {
            self.parse_face_simple(s, material)
        }
    }

    fn parse_coordinates(s: &str) -> Result<(f32, f32, f32), Box<dyn Error>> {
        let coords = s
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<f32>())
            .collect::<Result<Vec<_>, _>>()?;
        if coords.len() != 3 {
            return Err(Box::new(ObjLoadError));
        }

        Ok((coords[0], coords[1], coords[2]))
    }
}

fn read_index(s: &str) -> Result<usize, num::ParseIntError> {
    s.parse::<usize>().map(|i| i - 1)
}

fn are_valid_points(a: Vector, b: Vector, c: Vector) -> bool {
    let ab = b - a;
    let ac = c - a;
    let n = ab.cross(ac);

    n.length() != 0.0
}
