use std::fmt::Debug;

pub trait Hittable: Debug + Send + Sync {}
