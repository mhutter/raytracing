use std::io::{Result, Write};

use rand::{distributions::Uniform, Rng};

use super::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range(low: f64, high: f64) -> Self {
        let distr = Uniform::new(low, high);
        let mut rng = rand::thread_rng();
        Self(rng.sample(distr), rng.sample(distr), rng.sample(distr))
    }

    pub fn write(self, out: &mut dyn Write, samples_per_pixel: i32) -> Result<()> {
        let Vec3(r, g, b) = self / (samples_per_pixel as f64);

        // gamma-correct for gamma=2.0
        let r = r.sqrt();
        let g = g.sqrt();
        let b = b.sqrt();

        writeln!(
            out,
            "{} {} {}",
            (256.0 * r.clamp(0.0, 0.999)) as i32,
            (256.0 * g.clamp(0.0, 0.999)) as i32,
            (256.0 * b.clamp(0.0, 0.999)) as i32,
        )
    }
}
