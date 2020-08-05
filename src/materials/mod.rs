pub mod metal;

pub use crate::core::{
  vec3::Vec3,
  ray::Ray,
  color::Color,
};
pub use crate::geometry::{
  hit::HitRecord,
}

pub trait Material {
  fn scatter(r_in: &Ray, record: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}