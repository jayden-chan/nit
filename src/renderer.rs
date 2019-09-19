use rand::prelude::*;
use rayon::prelude::*;

use crate::{
    config::Config, image::ImageBuffer, ray::Ray, scene::Scene, Vector,
};

pub fn render(image: &mut ImageBuffer, config: Config) {
    let (width, height) = config.resolution;

    image.buffer.iter_mut().for_each(|row| {
        row.par_iter_mut().for_each(|pixel| {
            let mut curr_pixel = Vector::zeros();
            let mut rng = rand::thread_rng();

            for _ in 0..config.samples {
                let u = (pixel.x as f32 + rng.gen::<f32>()) / width as f32;
                let v = (pixel.y as f32 + rng.gen::<f32>()) / height as f32;

                let r = config.scene.camera.get_ray(u, v);

                curr_pixel += trace(r, &config.scene);
            }

            curr_pixel /= config.samples as f32;

            pixel.r = curr_pixel.x;
            pixel.g = curr_pixel.y;
            pixel.b = curr_pixel.z;
        });
    });
}

pub fn trace(r: Ray, scene: &Scene) -> Vector {
    let mut rng = rand::thread_rng();

    loop {
        if rng.gen() {
            break;
        }
    }

    Vector::zeros()
}
