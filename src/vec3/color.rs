use std::fmt::Display;

use super::Vec3;

pub type Color = Vec3;

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Vec3(r, g, b) = self;

        // gamma-correct for gamma=2.0
        let r = r.sqrt();
        let g = g.sqrt();
        let b = b.sqrt();

        write!(
            f,
            "{} {} {}",
            (256.0 * r.clamp(0.0, 0.999)) as i32,
            (256.0 * g.clamp(0.0, 0.999)) as i32,
            (256.0 * b.clamp(0.0, 0.999)) as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let c = Color::new(0, 0.5, 1);
        assert_eq!("0 181 255", format!("{}", c));
    }
}
