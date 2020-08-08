use crate::core::{
  color::Color,
  vec3::Vec3,
};

use super::Material;

pub struct Lambertian {
  pub albedo: Color;
}

impl Material for Lambertian {
  fn scatter(self, r_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &Ray) -> bool { -> bool {
    let scattered_direction = record.normal + Vec3::random_unit_vec();
    let scattered = Ray::new(record.point, scatter_direction);
    *attenuation = self.albedo;
    true
  }
}