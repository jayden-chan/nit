use std::fmt::Debug;

use crate::{primitives::Intersection, ray::Ray, Vector};

mod dielectric;
mod diffuse;
mod light;
mod reflector;

#[derive(Debug, Copy, Clone)]
pub struct Scatter {
    pub specular: Ray,
    pub attenuation: Vector,
}

#[derive(Debug)]
pub enum Material {
    Dielectric(f32),
    Diffuse(Vector),
    Light(Vector),
    Reflector(Vector),
}

impl Material {
    pub fn scatter(&self, r: Ray, i: Intersection) -> Option<Scatter> {
        match self {
            Self::Dielectric(ref_idx) => dielectric::scatter(*ref_idx, r, i),
            Self::Diffuse(albedo) => diffuse::scatter(*albedo, r, i),
            Self::Light(_) => None,
            Self::Reflector(albedo) => reflector::scatter(*albedo, r, i),
        }
    }

    pub fn emitted(&self, r: Ray, i: Intersection) -> Vector {
        match self {
            Self::Dielectric(_) => Vector::zeros(),
            Self::Diffuse(_) => Vector::zeros(),
            Self::Light(emittance) => light::emitted(*emittance, r, i),
            Self::Reflector(_) => Vector::zeros(),
        }
    }
}
