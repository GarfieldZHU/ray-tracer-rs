use crate::core::{
  color::Color,
  vec3::Vec3,
  ray:: Ray,
};
use crate::geometry::hit::{HitRecord};
use super::{Material, ScatterRecord};

pub struct Lambertian {
  pub albedo: Color,
}

impl Material for Lambertian {
  fn scatter(&self, _r_in: &Ray, record: &HitRecord) -> Option<ScatterRecord> {
    let scattered_direction = record.normal + Vec3::random_unit_vec();
    
    Some(ScatterRecord {
      attenuation: self.albedo,
      scattered: Ray::new(record.point, scattered_direction),
    })
  }
}