use crate::scene::Scene;

#[derive(Debug)]
pub struct Config {
    pub resolution: (usize, usize),
    pub samples: usize,
    pub scene: Scene,
}
