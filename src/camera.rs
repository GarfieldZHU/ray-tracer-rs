use crate::core::{
  point3::Point3,
  vec3::Vec3,
  ray::Ray,
  degrees_to_radians,
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

  pub fn new_param(vfov: f64, aspect_ratio: f64) -> Self {
    let theta = degrees_to_radians(vfov);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Point3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let ll = origin - horizontal/2 - vertical/2 - Vec3::new(0.0, 0.0, focal_length);

    Self { origin, ll, horizontal, vertical }
  }

  pub fn new_free_camera(
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    vfov: f64, 
    aspect_ratio: f64,
  ) -> Self {
    let theta = degrees_to_radians(vfov);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (lookfrom - lookat).unit();
    let u = Vec3::cross(&vup, &w).unit();
    let v = Vec3::cross(&w, &u);

    let origin = lookfrom;
    let horizontal = viewport_width * u;
    let vertical = viewport_height * v;
    let ll = origin - horizontal/2 - vertical/2 - w;

    Self { origin, ll, horizontal, vertical }
  }

  pub fn get_ray(self, s: f64, t: f64) -> Ray {
    Ray::new(self.origin, self.ll + s * self.horizontal + t * self.vertical - self.origin)
  }
}