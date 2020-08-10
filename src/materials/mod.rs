pub mod metal;

pub use crate::core::{
  vec3::Vec3,
  ray::Ray,
  color::Color,
};
pub use crate::geometry::{
  hit::HitRecord,
};

pub trait Material {
  fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<ScatterRecord>;
}

pub struct ScatterRecord {
  pub attenuation: Color,
  pub scattered: Ray,
}

pub struct DefaultMaterial { }
impl DefaultMaterial {
  pub fn new () -> Self {
    Self { }
  }
}
impl Material for DefaultMaterial {
  fn scatter(&self, _r_in: &Ray, _record: &HitRecord) -> Option<ScatterRecord> {
    None
  }
}