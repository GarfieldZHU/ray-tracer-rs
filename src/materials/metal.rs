use crate::core::{
  color::Color,
  ray::Ray,
  vec3::Vec3,
};
use crate::geometry::{
  hit::HitRecord,
};
use super::Material;

pub struct Metal {
  pub albedo: Color,
}

impl Material for Metal {
  fn scatter(self, r_in: &Ray, record: &HitRecord, attenuation: &mut Color, _scattered: &Ray) -> bool {
    let reflected = Vec3::reflect(r_in.direction.unit(), record.normal);
    let scattered = Ray::new(record.point, reflected);
    *attenuation = self.albedo;
    Vec3::dot(&scattered.direction, &record.normal) > 0.0
  }
}