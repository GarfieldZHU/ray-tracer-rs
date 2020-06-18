use std::ops;

struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec3 {
  pub const fn new (x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z }
  }
}

impl ops::Add<Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, _rhs: Vec3) -> Self::Output {
    Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
  }
}