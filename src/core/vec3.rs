use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use crate::utils::utils::{random_double, random_double_in_range};
use crate::core::{PI};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec3 {
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

  /**
   * dot product / scalar product for vector
   * Refer to: https://en.wikipedia.org/wiki/Dot_product
   **/
  pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
  }

  /**
   * Cross product for vector
   * Refer to: https://en.wikipedia.org/wiki/Cross_product 
   * */
  pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
      x: u.y * v.z - u.z * v.y,
      y: u.z * v.x - u.x * v.z,
      z: u.x * v.y - u.y * v.x,
    }
  }

  /**
   * Static method to generate a random vector
   */
  pub fn random() -> Self {
    Self {
      x: random_double(), y: random_double(), z: random_double(),
    }
  }

  pub fn random_vec3_in_range(min: f64, max: f64) -> Self {
    Self {
      x: random_double_in_range(min, max),
      y: random_double_in_range(min, max),
      z: random_double_in_range(min, max),
    }
  }

  pub fn random_in_unit_sphere() -> Self {
    let vec: Vec3 = loop {
      let v: Vec3 = Self::random_vec3_in_range(-1.0, 1.0);
      if v.length_square() >= 1.0 { break v; }
    };
    vec
  }

  pub fn random_unit_vec() -> Self {
    let a = random_double_in_range(0.0, 2.0 * PI); 
    let z = random_double_in_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
  }

  pub fn unit(self) -> Vec3 {
    Vec3 {
      x: self.x / self.length(),
      y: self.y / self.length(),
      z: self.z / self.length(),
    }
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

impl Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, _rhs: Vec3) -> Self::Output {
    _rhs * self
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, _rhs: f64) -> Self::Output {
    self * (1.0 / _rhs)
  }
}

impl Div<i32> for Vec3 {
  type Output = Vec3;

  fn div(self, _rhs: i32) -> Self::Output {
    self * (1.0 / _rhs as f64)
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

  #[test]
  fn test_dot_product() {
    let u = Vec3::new(1.0, 2.0, 3.0); 
    let v = Vec3::new(6.0, 3.0, 2.0);
    assert_eq!(Vec3::dot(&u, &v), 18.0);
  }

  #[test]
  fn test_cross_product() {
    let u = Vec3::new(1.0, 2.0, 3.0); 
    let v = Vec3::new(6.0, 3.0, 2.0);
    assert_eq!(Vec3::cross(&u, &v), Vec3::new(-5.0, 16.0, -9.0));
  }

  #[test]
  fn test_random_unit_vec() {
    let v = Vec3::random_unit_vec();
    assert_eq!(v.length(), 1.0);
  }
}