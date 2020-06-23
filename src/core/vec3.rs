use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

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

  // Get the squared length from the origin of coordinates
  pub fn length_square(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  // Get the length from the origin of coordinates 
  pub fn length(&self) -> f64 {
    self.length_square().sqrt()
  }

  // Reverse the vector
  pub fn reverse(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
    self.z = -self.z;
  }
  
  pub fn get_reversed(&self) -> Self {
    Self::new(-self.x, -self.y, -self.z )
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

impl Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, _rhs: f64) -> Self::Output {
    Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, _rhs: f64) -> Self::Output {
    self * (1.0 / _rhs)
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

  #[test]
  fn test_length() {
    let vec = Vec3::new(3.0, 4.0, 12.0);
    assert_eq!(vec.length(), 13.0);
  }

  #[test]
  fn test_reverse() {
    let mut vec_to_reverse = Vec3::new(1.0, 0.0, -1.0);
    vec_to_reverse.reverse();
    assert_eq!(vec_to_reverse, Vec3::new(-1.0, 0.0, 1.0));
    let vec_reversed = Vec3::new(2.0, 2.0, 2.0).get_reversed();
    assert_eq!(vec_reversed, Vec3::new(-2.0, -2.0, -2.0));
  }
}