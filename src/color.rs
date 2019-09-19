use crate::image::Pixel;

pub enum ToneMappingOperator {
    Clamp(f32),
}

impl ToneMappingOperator {
    pub fn apply(self, buffer: &mut Vec<Vec<Pixel>>) {
        match self {
            ToneMappingOperator::Clamp(max) => clamp(buffer, max),
        }
    }
}

/// Applies the clamp tone-mapping operator on the provided pixel buffer.
/// The pixel values will be clamped between 0 and
fn clamp(buffer: &mut Vec<Vec<Pixel>>, max: f32) {
    buffer.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|pixel| {
            if pixel.r > max {
                pixel.r = max;
            }
            if pixel.g > max {
                pixel.g = max;
            }
            if pixel.g > max {
                pixel.g = max;
            }

            // This shouldn't be possible but... just in case
            if pixel.r < 0.0 {
                pixel.r = 0.0;
            }
            if pixel.g < 0.0 {
                pixel.g = 0.0;
            }
            if pixel.g < 0.0 {
                pixel.g = 0.0;
            }
        });
    });
}
