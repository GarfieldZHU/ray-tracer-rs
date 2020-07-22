use crate::core::{
  point3::Point3,
  vec3::Vec3,
  ray::Ray,
};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
  origin: Point3,
  ll: Point3, 
  horizontal: Vec3,
  vertical: Vec3,
}

impl Camera {
  pub fn new() -> Self {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Point3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let ll = origin - horizontal/2 - vertical/2 - Vec3::new(0.0, 0.0, focal_length);

    Self { origin, ll, horizontal, vertical }
  }

  pub fn get_ray(self, u: f64, v: f64) -> Ray {
    Ray::new(self.origin, self.ll + u * self.horizontal + v * self.vertical - self.origin)
  }
}