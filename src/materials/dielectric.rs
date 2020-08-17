use crate::core::{
  color::Color,
  vec3::Vec3,
  ray:: Ray,
};
use crate::geometry::hit::{HitRecord};
use super::{Material, ScatterRecord};
use crate::utils::utils::{random_double, schlick};

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
  pub ref_idx: f64,
}

impl Dielectric {
  pub fn new(ri: f64) -> Self {
    Self {
      ref_idx: ri,
    }
  }
}

impl Material for Dielectric {
  fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<ScatterRecord> {
    let etai_over_etat: f64 = if record.front_face { 1.0 / self.ref_idx } else { self.ref_idx };
    let unit_direction: Vec3 = r_in.direction.unit();

    let cos_theta: f64 = Vec3::dot(&(-unit_direction), &record.normal).min(1.0);
    let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
    
    let reflect_prob: f64 = schlick(cos_theta, etai_over_etat);
    let random_factor: f64 = random_double();

    if etai_over_etat * sin_theta > 1.0 || random_factor < reflect_prob {
      // Reflection
      let reflected: Vec3 = Vec3::reflect(unit_direction, record.normal);
      Some(ScatterRecord{
        attenuation: Color::new(1.0, 1.0, 1.0),
        scattered: Ray::new(record.point, reflected),
      })
    } else {
      // Refraction
      let refracted: Vec3 = Vec3::refract(unit_direction, record.normal, etai_over_etat);
      Some(ScatterRecord {
        attenuation: Color::new(1.0, 1.0, 1.0),
        scattered: Ray::new(record.point, refracted),
      })
    }

    
  }
}