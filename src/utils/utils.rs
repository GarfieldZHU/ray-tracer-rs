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
use crate::materials::{
  DefaultMaterial,
  lambertian::Lambertian,
  metal::Metal,
  dielectric::Dielectric,
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
  if Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, DefaultMaterial::new()).is_hitten(r) {
    Color::new(1.0, 0.0, 0.0)
  } else {
    let unit_direction: Vec3 = r.direction.unit();
    let t = (unit_direction.y + 1.0) * 0.5;
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
  }
}

pub fn shading_ray_color(r: &Ray) -> Color {
  let s = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, DefaultMaterial::new());
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

pub fn world_ray_color(r: &Ray, world: &HittableList) -> Color {
  if let Some(record) = world.hit(r, 0.0, INFINITY) {
    0.5 * (Color::new(1.0, 1.0, 1.0) + record.normal)
  } else {
    let unit_direction: Vec3 = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0); 
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
  }
  
}

pub fn material_ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {  
  if depth <= 0 {
    return Color::new(0.0, 0.0, 0.0);
  }

  if let Some(record) = world.hit(r, 0.001, INFINITY) {
    let target: Point3 = record.point + record.normal + Vec3::random_unit_vec();
    0.5 * material_ray_color(&Ray::new(record.point, target - record.point), world, depth-1)
  } else {
    let unit_direction: Vec3 = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0); 
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
  }
}

pub fn metal_ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {  
  if depth <= 0 {
    return Color::new(0.0, 0.0, 0.0);
  }

  if let Some(record) = world.hit(r, 0.001, INFINITY) {
    if let Some(scattered_record) = record.material.scatter(r, &record) {
      return &scattered_record.attenuation * &metal_ray_color(&scattered_record.scattered, world, depth-1)
    }

    Color::new(0.0, 0.0, 0.0)
  } else {
    let unit_direction: Vec3 = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0); 
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
  }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
  if x < min {
    min
  } else if x > max {
    max
  } else {
    x
  }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
  let r = (1.0 - ref_idx) / (1.0 + ref_idx);
  let r0 = r * r;
  r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

/**
 * Generate a random scene to be rendered
 * */
pub fn random_scene() -> HittableList {
  let mut world = HittableList::new();
  // Add ground
  let material_ground = Lambertian { albedo: Color::new(0.5, 0.5, 0.5) };
  world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));
  
  // Randomly add some spheres
  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = random_double();
      // Vertical position is fixed (they are all on the ground)
      let center = Point3::new(
        a as f64 + 0.9 * random_double(), 
        0.2, 
        b as f64 + 0.9 * random_double()
      );

      if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          // Use diffuse
          let albedo = Color::random() * Color::random();
          let sphere_material = Lambertian { albedo };
          world.add(Sphere::new(center, 0.2, sphere_material))
        } else if choose_mat < 0.95 {
          // metal 
          let albedo = Color::random_in_range(0.5, 1.0);
          let fuzz = random_double_in_range(0.0, 0.5);
          let sphere_material = Metal::new(albedo, fuzz);
          world.add(Sphere::new(center, 0.2, sphere_material))
        } else {
          // glass
          let sphere_material = Dielectric::new(1.5);
          world.add(Sphere::new(center, 0.2, sphere_material))
        }
      }
    }
  }

  // Add three fixed objects
  let material1 = Dielectric::new(1.5);
  world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));
  
  let material2 = Lambertian { albedo: Color::new(0.4, 0.2, 0.1) };
  world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));
  
  let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
  world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

  world
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
    clamp,
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
    for _i in 1..100 {
      assert!(random_double_in_range(min, max) >= min);
      assert!(random_double_in_range(min, max) < max);
    }

    assert_eq!(random_double_in_range(max, min), 0.0);
  }

  #[test]
  fn test_clamp() {
    assert_eq!(clamp(5.0, 3.0, 10.0), 5.0);
    assert_eq!(clamp(5.0, 7.0, 10.0), 7.0);
    assert_eq!(clamp(5.0, -1.0, 3.5), 3.5);
  }
}