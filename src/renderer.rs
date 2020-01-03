use rand::prelude::*;
use rayon::prelude::*;

use crate::{
    config::{Config, Scene},
    image::ImageBuffer,
    ray::Ray,
    Vector,
};

use std::f32;
use std::io::stdout;
use std::io::Write;

const T_MIN: f32 = 0.0005;
const MAX_RECURSIVE_DEPTH: usize = 50;

pub fn render(image: &mut ImageBuffer, config: Config) {
    let (width, height) = config.resolution;
    let mut done_rows = 0;

    image.buffer.iter_mut().for_each(|row| {
        progress_bar(done_rows, height);

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

        done_rows += 1;
    });

    // Make sure the progress bar reaches 100% :)
    progress_bar(height, height);
}

pub fn trace(r: Ray, scene: &Scene) -> Vector {
    let mut curr_ray = r;
    let mut curr_att = Vector::ones();
    let mut total_emitted = Vector::zeros();

    for _ in 0..MAX_RECURSIVE_DEPTH {
        let hit_result = scene.objects.hit(curr_ray, T_MIN, f32::MAX);

        match hit_result {
            None => return Vector::zeros(),
            Some(hit) => match hit.scattered {
                None => return curr_att * (total_emitted + hit.emitted),
                Some(scatter) => {
                    curr_ray = scatter.specular;
                    curr_att *= scatter.attenuation;
                    total_emitted += hit.emitted;
                }
            },
        }
    }

    // Loop broke - max recursive depth exceeded
    Vector::zeros()
}

/// Renders a progress bar on the command line
fn progress_bar(curr: u16, total: u16) {
    let done_chars = (curr as f32 / total as f32) * 80_f32;
    let blank_chars = 80 - done_chars as usize;

    print!(
        "    {:10} [{}{}] {:.2}%\r",
        "Rendering",
        "=".repeat(done_chars as usize),
        " ".repeat(blank_chars),
        (curr as f32 / total as f32) * 100.0
    );

    stdout().flush().unwrap();
}
