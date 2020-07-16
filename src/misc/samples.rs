use crate::core::{
  vec3::Vec3,
  point3::Point3,
  ray::Ray,
};
use crate::utils::utils;

const IMAGE_WIDTH: u32 =256;
const IMAGE_HEIGHT: u32 =256;

pub enum SceneCase {
  RaySphereScene,
  ShadingWithNormalScene,
  HittableObjectsScene,
}

pub fn output_image() -> () {
  // PPM meta
  println!("P3\n{0} {1}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
  for j in (0..IMAGE_HEIGHT).rev() {
    for i in 0..IMAGE_WIDTH {
      let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
      let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
      let b = 0.25;

      let ir = (255.999 * r) as u32;
      let ig = (255.999 * g) as u32;
      let ib = (255.999 * b) as u32;
      print!("{0} {1} {2} \n", ir, ig, ib);
    }
  }
}


pub fn ray_to_scene(scene: SceneCase) {
  let aspect_ratio = 16.0 / 9.0;
  let image_width = 384;
  let image_height = (image_width as f64 / aspect_ratio) as u32;

  println!("P3\n{0} {1}\n255\n", image_width, image_height);

  let viewport_height = 2.0;
  let viewport_width = aspect_ratio * viewport_height;
  let focal_length = 1.0;

  let origin = Point3::new(0.0, 0.0, 0.0);
  let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
  let vertical = Vec3::new(0.0, viewport_height, 0.0);
  let lower_left_corner = origin - horizontal / 2 - vertical / 2 - Vec3::new(0.0, 0.0, focal_length);
  eprintln!("Lower left corner coordinate: {0}", lower_left_corner);

  for j in (0..image_height).rev() {
    eprintln!("\rScanlines remaining: {0} ", j);

    for i in 0..image_width {
      let u = i as f64 / ((image_width-1) as f64);
      let v = j as f64 / ((image_height-1) as f64);
      let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
      let pixel_color = match scene {
        SceneCase::RaySphereScene => utils::ray_color(&r),
        SceneCase::ShadingWithNormalScene => utils::shading_ray_color(&r),
        SceneCase::HittableObjectsScene => utils::world_ray_color(&r),
      };
      
      pixel_color.write_color();
    }
  }

  eprintln!("\nDone.\n");
}