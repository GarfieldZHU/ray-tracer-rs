use rand::prelude::*;
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


pub fn random_double() -> f64 {
  let mut rng = rand::thread_rng();
  let y: f64 = rng.gen();
  y
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
  if min > max {
    return 0.0
  }
  min + (max - min) * random_double()
}

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
  use crate::core::color::Color;
  use crate::core::ray::Ray;
  use crate::core::vec3::Vec3;
  use crate::core::point3::Point3;
  use super::{
    ray_color,
    random_double,
    random_double_in_range,
  };


  #[test]
  fn test_ray_color() {
    let x = ray_color(&Ray::new(Point3::new(10.0, 10.0, 10.0), Vec3::new(30.0, 40.0, 0.0)));
    assert_eq!(x, Color::new(0.55, 0.73, 1.0));
  }

  #[test]
  fn test_ramdom_double() {
    let a: f64 = random_double();
    assert!(a >= 0.0);
    assert!(a < 1.0);
  }

  #[test]
  fn test_ramdom_double_in_range() {
    let min = 5.125;
    let max = 6.25;
    for i in 1..100 {
      assert!(random_double_in_range(min, max) >= min);
      assert!(random_double_in_range(min, max) < max);
    }

    assert_eq!(random_double_in_range(max, min), 0.0);
  }
}