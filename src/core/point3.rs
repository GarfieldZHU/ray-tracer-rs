use super::vec3::Vec3;

pub type Point3 = Vec3;

#[cfg(test)]
mod test {
  use super::Point3;

  pub fn test_point3_basic() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let q = Point3::new(3.0, 2.0, 1.0);

    assert_eq!(p + q, Point3::new(4.0, 4.0, 4.0));
  }

  pub fn test_point3_dot() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let q = Point3::new(3.0, 2.0, 1.0);

    assert_eq!(Point3::dot(&p, &q), 10.0);
  }
}