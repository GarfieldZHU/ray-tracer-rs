use crate::core::{
  point3::Point3,
  ray::Ray,
  vec3::Vec3,
};

use super::hit::{HitRecord, Hittable};

pub struct Sphere {
  center: Point3,
  radius: f64,
}

impl Sphere {
  pub const fn new(center: Point3, radius: f64) -> Self {
    Sphere { center, radius }
  }

  /**
   * If the ray hit the area of the sphere.
   * We say the ray cutting the surface of sphere does not hit (able to pass through)
   * */
  pub fn is_hitten(&self, ray: &Ray) -> bool {
    let oc: Vec3 = ray.origin - self.center;
    let a: f64 = ray.direction.length_square();
    let b: f64 = 2.0 * Vec3::dot(&oc, &(ray.direction));
    let c: f64 = Vec3::dot(&oc, &oc) - self.radius * self.radius;
    let discriminant: f64 = b * b - 4.0 * a * c;
    (discriminant > 0.0) 
  }

  /**
   * Get the hit point value as the solution of ray hit the sphere
   * Return -1 if no hit point
   * */
  pub fn hit_value(&self, ray: &Ray) -> f64 {
    let oc: Vec3 = ray.origin - self.center;
    let a: f64 = ray.direction.length_square();
    let half_b: f64 = Vec3::dot(&oc, &(ray.direction));
    let c: f64 = Vec3::dot(&oc, &oc) - self.radius * self.radius;
    let discriminant: f64 = half_b * half_b - a * c;
    if discriminant < 0.0 {
      -1.0
    } else {
      (-half_b - discriminant.sqrt() ) / a
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
    let oc: Vec3 = ray.origin - self.center;
    let a: f64 = ray.direction.length_square();
    let half_b: f64 = Vec3::dot(&oc, &(ray.direction));
    let c: f64 = Vec3::dot(&oc, &oc) - self.radius * self.radius;
    let discriminant: f64 = half_b * half_b - a * c;

    if discriminant > 0.0 {
      let root: f64 = discriminant.sqrt();
      let solution_1: f64 = (-half_b - root) / a;
      if solution_1 < t_max && solution_1 > t_min {
        record.t = solution_1;
        record.point = ray.at(record.t);
        record.normal = (record.point - self.center) / self.radius;
        return true;
      }
      let solution_2: f64 = (-half_b + root) / a;
      if solution_2 < t_max && solution_2 > t_min {
        record.t = solution_2;
        record.point = ray.at(record.t);
        record.normal = (record.point - self.center) / self.radius;
        return true;
      }
    } 
    false
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
    assert_eq!(s.is_hitten(&r1), false);

    // Ray cutting the surface of the sphere, say not hit
    let r2 = Ray::new(Point3::new(1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(s.is_hitten(&r2), false);

    let r3 = Ray::new(Point3::new(0.0, 0.1, -1.0), Vec3::new(0.1, 0.0, 1.0));
    assert_eq!(s.is_hitten(&r3), true);
  }
}