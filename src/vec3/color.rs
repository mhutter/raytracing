use std::fmt::{Display, Formatter, Result};

use super::Vec3;

pub type Color = Vec3;

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let r = (255.999 * self.0) as i64;
        let g = (255.999 * self.1) as i64;
        let b = (255.999 * self.2) as i64;

        write!(f, "{} {} {}", r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!("0 0 0", format!("{}", Color::new(0, 0, 0)));
        assert_eq!("25 127 230", format!("{}", Color::new(0.1, 0.5, 0.9)));
        assert_eq!("255 255 255", format!("{}", Color::new(1, 1, 1)));
    }
}
