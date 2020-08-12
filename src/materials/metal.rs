use crate::core::{
  color::Color,
  ray::Ray,
  vec3::Vec3,
};
use crate::geometry::{
  hit::HitRecord,
};
use super::{Material, ScatterRecord};

pub struct Metal {
  pub albedo: Color,
  pub fuzz: f64,
}

impl Metal {
  pub fn new(albedo: Color, f: f64) -> Self {
    Self {
      albedo,
      fuzz:if f < 1.0 { f } else { 1.0 },
    }
  }
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<ScatterRecord> {
    let reflected = Vec3::reflect(r_in.direction.unit(), record.normal);
    let scattered = Ray::new(record.point, reflected + self.fuzz * Vec3::random_in_unit_sphere());
    if Vec3::dot(&scattered.direction, &record.normal) > 0.0 {
      Some(ScatterRecord{
        attenuation: self.albedo,
        scattered,
      })
    } else {
      None
    }
  }
}