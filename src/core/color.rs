
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Color {
  r: f64,
  g: f64,
  b: f64,
}

impl Color {
  pub const fn new(r: f64, g: f64, b: f64) -> Self {
    Self { r, g, b }
  }

  pub const fn newi(r: u8, g: u8, b: u8) -> Self {
    Self { r: r as f64, g: g as f64, b: b as f64 }
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



#[cfg(test)]
mod test {
  use super::Color;

  #[test]
  fn test_color() {
    let c = Color::from((255, 0, 120));
    let d = Color::new(255.0, 0.0, 120.0);

    assert_eq!(c, d);
  }
}