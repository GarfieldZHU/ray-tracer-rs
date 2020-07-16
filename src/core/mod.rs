pub mod color;
pub mod image;
pub mod point3;
pub mod ray;
pub mod vec3;

pub use std::f64::{INFINITY, consts::PI};

// --- Utilities ---
pub fn degrees_to_radians(degrees: f64) -> f64 {
  degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 {
  radians * 180.0 / PI
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_degrees_to_radians() {
    assert_eq!(degrees_to_radians(45.0), 0.25 * PI);
    assert_eq!(degrees_to_radians(360.0), 2.0 * PI);
  }

  #[test]
  fn test_radians_to_degrees() {
    assert_eq!(radians_to_degrees(4.0 * PI), 720.0);
    assert_eq!(radians_to_degrees(0.5 * PI), 90.0);
  }
}