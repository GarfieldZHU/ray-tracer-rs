use crate::core::{
  point3::Point3,
  ray::Ray,
  vec3::Vec3,
};


pub struct HitRecord {
  point: Point3,
  normal: Vec3,
  t: f64,
}


trait Hittable {
  fn hit(r: &Ray, t_min: f64, t_max: f64, record: &HitRecord);
}