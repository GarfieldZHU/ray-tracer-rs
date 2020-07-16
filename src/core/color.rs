use std::ops::{Add, Mul};
use super::vec3::Vec3;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Color {
  r: f64,
  g: f64,
  b: f64,
}

impl Color {
  pub fn new(r: f64, g: f64, b: f64) -> Self {
    Self { r, g, b }
  }

  pub fn newi(r: u8, g: u8, b: u8) -> Self {
    Self { r: (r as f64) / 255.0, g: (g as f64) / 255.0, b: (b as f64) / 255.0}
  }

  pub fn write_color(&self) {
    println!("{0} {1} {2}", (255.999*self.r) as u32, (255.999*self.g) as u32, (255.999*self.b) as u32);
  }
}

impl From<(u8, u8, u8)> for Color {
  fn from((r, g, b): (u8, u8, u8)) -> Color {
    Color::newi(r, g, b)
  }
}

impl From<(f64, f64, f64)> for Color {
  fn from((r, g, b): (f64, f64, f64)) -> Color {
    Color::new(r, g, b)
  }
}

impl Add<Self> for Color {
  type Output = Color;

  fn add(self, _rhs: Self) -> Self::Output {
    Color::new(self.r + _rhs.r, self.g + _rhs.g, self.b + _rhs.b)
  }
}

impl Add<Vec3> for Color {
  type Output = Color;
  fn add(self, _rhs: Vec3) -> Self::Output {
    Color::new(self.r + _rhs.x, self.g + _rhs.y, self.b + _rhs.z)
  }
}

impl Mul<f64> for Color {
  type Output = Color;

  fn mul(self, _rhs: f64) -> Self::Output {
    Color::new(self.r * _rhs, self.g * _rhs, self.b * _rhs)
  }
}

impl Mul<Color> for f64 {
  type Output = Color;

  fn mul(self, _rhs: Color) -> Self::Output {
    _rhs * self
  }
}


#[cfg(test)]
mod test {
  use super::Color;

  #[test]
  fn test_color() {
    let c = Color::from((255, 0, 255));
    let d = Color::new(1.0, 0.0, 1.0);

    assert_eq!(c, d);
  }
}