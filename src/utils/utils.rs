use crate::core::{
  ray::Ray,
  vec3::Vec3,
  point3::Point3,
  color::Color,
  INFINITY,
};
use crate::geometry::{
  sphere::Sphere,
  hit::{Hittable, HittableList}
};

pub fn ray_color(r: &Ray) -> Color {
  if Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5).is_hitten(r) {
    Color::new(1.0, 0.0, 0.0)
  } else {
    let unit_direction: Vec3 = r.direction.unit();
    let t = (unit_direction.y + 1.0) * 0.5;
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
  }
}

pub fn shading_ray_color(r: &Ray) -> Color {
  let s = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
  let mut t: f64 = s.hit_value(r);
  if t > 0.0 {
    let n: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
    0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
  } else {
    let unit_direction: Vec3 = r.direction.unit();
    t = 0.5 * (unit_direction.y + 1.0); 
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
  }
}

pub fn world_ray_color(r: &Ray) -> Color {
  let mut world = HittableList::new();
  world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
  world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

  if let Some(record) = world.hit(r, 0.0, INFINITY) {
    0.5 * (Color::new(1.0, 1.0, 1.0) + record.normal)
  } else {
    let unit_direction: Vec3 = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0); 
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
  }
  
}

#[cfg(test)]
mod test {
  use super::{ray_color};
  use crate::core::color::Color;
  use crate::core::ray::Ray;
  use crate::core::vec3::Vec3;
  use crate::core::point3::Point3;

  #[test]
  fn test_ray_color() {
    let x = ray_color(&Ray::new(Point3::new(10.0, 10.0, 10.0), Vec3::new(30.0, 40.0, 0.0)));
    assert_eq!(x, Color::new(0.55, 0.73, 1.0));
  }
}