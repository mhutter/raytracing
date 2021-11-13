use std::io::{Result, Write};

use super::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write(self, out: &mut dyn Write, samples_per_pixel: i32) -> Result<()> {
        let Vec3(r, g, b) = self / (samples_per_pixel as f64);
        writeln!(
            out,
            "{} {} {}",
            (256.0 * r.clamp(0.0, 0.999)) as i32,
            (256.0 * g.clamp(0.0, 0.999)) as i32,
            (256.0 * b.clamp(0.0, 0.999)) as i32,
        )
    }
}
