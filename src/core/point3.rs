use super::vec3::Vec3;

pub type Point3 = Vec3;

impl std::fmt::Display for Point3 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
  }
}

#[cfg(test)]
mod test {
  use super::Point3;
  use super::Vec3;

  #[test]
  fn test_point3_basic() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let q = Point3::new(3.0, 2.0, 1.0);

    assert_eq!(p + q, Point3::new(4.0, 4.0, 4.0));
  }

  #[test]
  fn test_point3_dot() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let q = Point3::new(3.0, 2.0, 1.0);

    assert_eq!(Point3::dot(&p, &q), 10.0);
  }

  #[test]
  fn test_add_vector() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let r = Vec3::new(2.0, 2.0, 2.0);

    assert_eq!(p + r, Point3::new(3.0, 4.0, 5.0));
  }
}