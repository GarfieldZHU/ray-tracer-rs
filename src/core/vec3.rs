use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Default, Debug)]
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

impl Add<Vec3> for &Vec3 {
  type Output = Vec3;

  fn add(self, _rhs: Vec3) -> Self::Output {
    Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
  }
}

impl Sub<Vec3> for &Vec3 {
  type Output = Vec3;
  
  fn sub(self, _rhs: Vec3) -> Self::Output {
    Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
  }
}

impl AddAssign<Vec3> for Vec3 {
  fn add_assign(&mut self, _rhs: Self) {
    self.x += _rhs.x;
    self.y += _rhs.y;
    self.z += _rhs.z;
  }
}

impl SubAssign<Vec3> for Vec3 {
  fn sub_assign(&mut self, _rhs: Self) {
    self.x -= _rhs.x;
    self.y -= _rhs.y;
    self.z -= _rhs.z;
  }
}
