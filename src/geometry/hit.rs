use crate::core::{
  point3::Point3,
  ray::Ray,
  vec3::Vec3,
};


pub struct HitRecord {
  pub point: Point3,
  pub normal: Vec3,
  pub t: f64,
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}