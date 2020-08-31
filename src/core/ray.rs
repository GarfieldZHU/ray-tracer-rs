use super::vec3::Vec3;
use super::point3::Point3;

#[derive(Debug)]
pub struct Ray {
  pub origin: Point3,
  pub direction: Vec3, 
  pub time: f64,
}

impl Ray {
  pub const fn new(origin: Point3, direction: Vec3) -> Self {
    Self {
      origin,
      direction,
      time: 0.0,
    }
  }

  pub const fn new_with_time(origin: Point3, direction: Vec3, time: f64) -> Self {
    Self {
      origin,
      direction,
      time,
    }
  }

  pub fn at(&self, t: f64) -> Point3 {
    return self.origin + self.direction * t;
  }
}


#[cfg(test)]
mod test {
  use super::Ray;
  use super::Vec3;
  use super::Point3;

  #[test]
  fn test_ray() {
    let r = Ray::new(Point3::new(1.0, 1.0, 1.0), Vec3::new(3.0, 4.0, 0.0));
    assert_eq!(r.origin, Point3::new(1.0, 1.0, 1.0));
  }

  #[test]
  fn test_at() {
    let r = Ray::new(Point3::new(3.0, 4.0, 5.0), Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(r.at(2.0), Point3::new(5.0, 6.0, 7.0));
  }
}