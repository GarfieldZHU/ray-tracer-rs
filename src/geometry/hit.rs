use std::fmt::{Debug, Formatter};

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

pub struct HittableList {
  objects: Vec<Box<dyn Hittable>>,
}

impl Debug for HittableList {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      f.write_fmt(format_args!(
          "HittableList: {{ objects: {}}}",
          self.objects.len()
      ))
  }
}