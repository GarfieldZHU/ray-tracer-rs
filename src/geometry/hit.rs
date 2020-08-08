use std::fmt::{Debug, Formatter};

use crate::core::{
  point3::Point3,
  ray::Ray,
  vec3::Vec3,
};
use crate::materials::{Material};

// #[derive(Clone, Copy)]
pub struct HitRecord<'a> {
  pub point: Point3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
  pub material: &'a dyn Material,
}

impl<'m> HitRecord<'m> {
  pub fn new(point: Point3, normal: Vec3, t: f64, material: &'m dyn Material) -> Self {
    HitRecord {
      point,
      normal,
      t,
      front_face: true,
      material,
    }
  }

  pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
    self.front_face = Vec3::dot(&r.direction, &outward_normal) < 0.0;
    self.normal = if self.front_face { outward_normal } else { outward_normal.get_reversed() };
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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

impl HittableList {
  pub fn new() -> Self {
    HittableList {
      objects: Vec::new(),
    }
  }

  pub fn add_boxed(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object);
  }

  pub fn add<H: Hittable + 'static>(&mut self, object: H) {
    let object: Box<dyn Hittable> = Box::new(object);
    self.add_boxed(object);
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    self.objects.iter()
      .filter_map(|object| object.hit(ray, t_min, t_max))
      .min_by(|rl, rr| rl.t.partial_cmp(&rr.t).unwrap())
  }
}