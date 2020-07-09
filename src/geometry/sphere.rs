use crate::core::{
  point3::Point3,
  ray::Ray,
  vec3::Vec3,
};

pub struct Sphere {
  center: Point3,
  radius: f64,
}

impl Sphere {
  /**
   * If the ray hit the area of the sphere.
   * We say the ray cutting the surface of sphere does not hit (able to pass through)
   * */
  fn hit(&self, ray: &Ray) -> bool {
    let oc = ray.origin - self.center;
    let a = ray.direction.length_square();
    let b = 2.0 * Vec3::dot(&oc, &(ray.direction));
    let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
    let discriminant = b * b - 4.0 * a * c;
    (discriminant > 0.0) 
  }
}


#[cfg(test)]
mod tests {
  use super::Ray;
  use super::Vec3;
  use super::Point3;
  use super::Sphere;
  

  #[test]
  fn test_hit() {
    let s = Sphere {center: Point3::new(0.0, 0.0, 0.0), radius: 1.0};
    let r1 = Ray::new(Point3::new(5.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(s.hit(&r1), false);

    // Ray cutting the surface of the sphere, say not hit
    let r2 = Ray::new(Point3::new(1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(s.hit(&r2), false);

    let r3 = Ray::new(Point3::new(0.0, 0.1, -1.0), Vec3::new(0.1, 0.0, 1.0));
    assert_eq!(s.hit(&r3), true);
  }
}