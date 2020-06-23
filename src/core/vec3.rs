use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Default, Debug, PartialEq)]
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

impl Add for Vec3 {
  type Output = Vec3;

  fn add(self, _rhs: Self) -> Self::Output {
    Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
  }
}

impl Sub for Vec3 {
  type Output = Vec3;
  
  fn sub(self, _rhs: Self) -> Self::Output {
    Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, _rhs: Self) {
    self.x += _rhs.x;
    self.y += _rhs.y;
    self.z += _rhs.z;
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, _rhs: Self) {
    self.x -= _rhs.x;
    self.y -= _rhs.y;
    self.z -= _rhs.z;
  }
}


#[cfg(test)]
mod test {
  use super::Vec3;

  #[test]
  fn test_add() {
    let vec_added = Vec3::new(1.2, 3.4, 5.0) + Vec3::new(-0.2, 0.6, 2.0);
    assert_eq!(vec_added, Vec3::new(1.0, 4.0, 7.0));
  }

  #[test]
  fn test_sub() {
    let vec_subed = Vec3::new(1.2, 3.4, 5.0) - Vec3::new(-0.2, 0.6, 2.0);
    assert_eq!(vec_subed, Vec3::new(1.4, 2.8, 3.0));
  }

  #[test]
  fn test_add_assigned() {
    let mut vec = Vec3::new(1.0, 2.0, 3.0);
    let vec_to_add = Vec3::new(3.0, 2.0, 1.0);
    vec += vec_to_add;
    assert_eq!(vec, Vec3::new(4.0, 4.0, 4.0));
  }

  #[test]
  fn test_sub_assigned() {
    let mut vec = Vec3::new(1.0, 2.0, 3.0);
    let vec_to_sub = Vec3::new(3.0, 2.0, 1.0);
    vec -= vec_to_sub;
    assert_eq!(vec, Vec3::new(-2.0, 0.0, 2.0));
  }
}