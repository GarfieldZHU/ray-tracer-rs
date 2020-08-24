use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use std::time;
use crate::core::{
  vec3::Vec3,
  point3::Point3,
  ray::Ray,
  color::Color,
  PI,
};
use crate::geometry::{
  sphere::Sphere,
  hit::{HittableList}
};
use crate::materials::{
  DefaultMaterial,
  lambertian::Lambertian,
  metal::Metal,
  dielectric::Dielectric,
};
use crate::utils::utils;
use crate::camera::Camera;

const IMAGE_WIDTH: u32 =256;
const IMAGE_HEIGHT: u32 =256;

pub enum SceneCase {
  RaySphereScene,
  ShadingWithNormalScene,
  HittableObjectsScene,
}

#[derive(std::cmp::PartialEq)]
pub enum AdvanceSceneCase {
  AntialiasingScene,
  MaterialScene,
  MetalScene,
  RefractionScene,
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

      let mut world = HittableList::new();
      world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, DefaultMaterial::new()));
      world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, DefaultMaterial::new())); 
      let pixel_color = match scene {
        SceneCase::RaySphereScene => utils::ray_color(&r),
        SceneCase::ShadingWithNormalScene => utils::shading_ray_color(&r),
        SceneCase::HittableObjectsScene => utils::world_ray_color(&r, &world),
      };
      
      pixel_color.write_color();
    }
  }

  eprintln!("\nDone.\n");
}

pub fn ray_to_scene_advance(scene: AdvanceSceneCase) {
  let aspect_ratio = 16.0 / 9.0;
  let image_width = 384;
  let image_height = (image_width as f64 / aspect_ratio) as u32;
  let samples_per_pixel = 100;
  let camera = Camera::new();

  println!("P3\n{0} {1}\n255\n", image_width, image_height);

  let mut world = HittableList::new();
  world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, DefaultMaterial::new()));
  world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, DefaultMaterial::new()));


  for j in (0..image_height).rev() {
    eprintln!("\rScanlines remaining: {0} ", j);

    for i in 0..image_width {
      let mut pixel_color = Color::new(0.0, 0.0, 0.0);
      for _s in 0..samples_per_pixel {
        let u = (i as f64 + utils::random_double()) / (image_width - 1) as f64;
        let v = (j as f64 + utils::random_double()) / (image_height - 1) as f64;
        let r: Ray = camera.get_ray(u, v);
        let depth: u32 = 50;

        let mut world = HittableList::new();
        if scene == AdvanceSceneCase::MetalScene {
          let material_ground = Lambertian { albedo: Color::new(0.8, 0.8, 0.0) };
          let material_center = Lambertian { albedo: Color::new(0.7, 0.3, 0.3) };
          let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
          let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

          world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
          world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center));
          world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
          world.add(Sphere::new(Point3::new( 1.0, 0.0, -1.0), 0.5, material_right));
        } else if scene == AdvanceSceneCase::RefractionScene  {
          let material_ground = Lambertian { albedo: Color::new(0.8, 0.8, 0.0) };
          let material_center = Lambertian { albedo: Color::new(0.1, 0.2, 0.5) };
          let material_left = Dielectric::new(1.5);
          let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

          world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
          world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center));
          world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
          world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, material_left));
          world.add(Sphere::new(Point3::new( 1.0, 0.0, -1.0), 0.5, material_right));
        } else {
          world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, DefaultMaterial::new()));
          world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, DefaultMaterial::new()));        
        }

        pixel_color += match scene {
          AdvanceSceneCase::AntialiasingScene => utils::world_ray_color(&r, &world),
          AdvanceSceneCase::MaterialScene => utils::material_ray_color(&r, &world, depth),
          AdvanceSceneCase::MetalScene => utils::metal_ray_color(&r, &world, depth),
          AdvanceSceneCase::RefractionScene => utils::metal_ray_color(&r, &world, depth),
        };
      }
      pixel_color.write_color_gamma_corrected(samples_per_pixel);
    }
  }
  
}

pub fn ray_to_scene_camera() {
  let aspect_ratio = 16.0 / 9.0;
  let image_width = 384;
  let image_height = (image_width as f64 / aspect_ratio) as u32;
  let samples_per_pixel = 100;

  let radius = (PI / 4.0).cos();
  let mut world = HittableList::new();

  // Simple scene
  // let material_left  = Lambertian { albedo: Color::new(0.0,0.0,1.0) };
  // let material_right = Lambertian { albedo: Color::new(1.0,0.0,0.0) };
  // world.add(Sphere::new(Point3::new(-radius, 0.0, -1.0), radius, material_left));
  // world.add(Sphere::new(Point3::new(radius, 0.0, -1.0), radius, material_right));
  // let camera = Camera::new_param(90.0, aspect_ratio);

  // Free camera scene
  let material_ground = Lambertian { albedo: Color::new(0.8, 0.8, 0.0) };
  let material_center = Lambertian { albedo: Color::new(0.7, 0.3, 0.3) };
  let material_left = Dielectric::new(1.5);
  let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

  world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
  world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center));
  world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
  world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, material_left));
  world.add(Sphere::new(Point3::new( 1.0, 0.0, -1.0), 0.5, material_right));
  
  // let lookfrom = Point3::new(-2.0, 2.0, 1.0);  // Camera poition for zoom cases
  let lookfrom = Point3::new(3.0, 3.0, 2.0);  // Camera poition for defocus cases
  let lookat = Point3::new(0.0, 0.0, -1.0);
  
  let camera = Camera::new_with_lens(
    lookfrom, 
    lookat, 
    Vec3::new(0.0, 1.0, 0.0), 
    // 90.0,  // Wide angle lens
    20.0,  // Long focal lens
    aspect_ratio,
    2.0,
    (lookfrom-lookat).length(),
  );

  println!("P3\n{0} {1}\n255\n", image_width, image_height);

  for j in (0..image_height).rev() {
    eprintln!("\rScanlines remaining: {0} ", j);

    for i in 0..image_width {
      let mut pixel_color = Color::new(0.0, 0.0, 0.0);
      for _s in 0..samples_per_pixel {
        let u = (i as f64 + utils::random_double()) / (image_width - 1) as f64;
        let v = (j as f64 + utils::random_double()) / (image_height - 1) as f64;
        // let r: Ray = camera.get_ray(u, v);  // No defocus blur 
        let r: Ray = camera.get_ray_for_lens(u, v);  // With defocus blur
        let depth: u32 = 50;
        
        pixel_color += utils::metal_ray_color(&r, &world, depth);
      }
      pixel_color.write_color_gamma_corrected(samples_per_pixel);
    }
  }
}


// Final scene
pub fn final_scene() {
  const N_THREAD: usize = 20;  // Concurrent thread amount
  let aspect_ratio = 3.0 / 2.0;
  let image_width = 1200;
  let image_height = (image_width as f64 / aspect_ratio) as u32;
  let samples_per_pixel = 500;
  let max_depth = 50;

  let world = Arc::new(utils::random_scene());

  let lookfrom = Point3::new(5.0, 2.0, 1.0);  // Camera poition for defocus cases
  let lookat = Point3::new(-2.0, 0.0, 0.0);
  
  let camera = Camera::new_with_lens(
    lookfrom, 
    lookat, 
    Vec3::new(0.0, 1.0, 0.0), 
    // 90.0,  // Wide angle lens
    20.0,  // Long focal lens
    aspect_ratio,
    0.1,
    10.0,
  );

  let timer = time::SystemTime::now();
  println!("P3\n{0} {1}\n255\n", image_width, image_height);

  // Add multiple thread for sampling
  let (tx, rx) = mpsc::channel();
  let mut children = vec![];
  let thread_height = image_height / (N_THREAD as u32);

  for t in 0..N_THREAD {
    let thread_tx = tx.clone();
    let thread_world = world.clone();
    // Each thread do partial sampling value
    children.push(thread::spawn(move || {
      let mut colors = vec![vec![]];

      for j in (0..thread_height).rev() {
        let now = time::SystemTime::now();
        let thread_j = j + (t as u32) * thread_height;
        eprintln!("\rScanning line: {0} ", thread_j);
        let mut row_colors = vec![];
    
        for i in 0..image_width {
          let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    
          for _s in 0..samples_per_pixel  {
            let u = (i as f64 + utils::random_double()) / (image_width - 1) as f64;
            let v = (thread_j as f64 + utils::random_double()) / (image_height - 1) as f64;
            // let r: Ray = camera.get_ray(u, v);  // No defocus blur 
            let r: Ray = camera.get_ray_for_lens(u, v);  // With defocus blur
            pixel_color += utils::metal_ray_color(&r, &thread_world, max_depth);
          }
          row_colors.push(pixel_color);
        }
        colors.push(row_colors);
        match now.elapsed() {
          Ok(elapsed) => {
            eprintln!("\rScan line {0} cost: {1} s.", 
              thread_j,  
              elapsed.as_secs(),
            );
          }
          Err(e) => {
            eprintln!("Error: {:?}", e);
          }
        }
      }
      thread_tx.send((t, colors)).unwrap();
    }))
  }

  // Join point, waiting for all calculation results before continue.
  for child in children {
    child.join().expect("failed...");
  }

  eprintln!("\r--- Rendering time: {0} s --- ", timer.elapsed().unwrap().as_secs());
 
  // Output the image in order of PPM formt.
  let mut res: Vec<Vec<Vec<Color>>> = vec![vec![vec![]]; N_THREAD];
  for (_idx, (id, colors)) in rx.iter().enumerate().take(N_THREAD) {
    res[id] = colors;
  }

  for i in (0..N_THREAD).rev() {
    for rows in res[i].clone() {
      for pixel in rows {
        pixel.write_color_gamma_corrected(samples_per_pixel);
      }
    }
  }

  eprintln!("\r--- Overall time cost: {0} s --- ", timer.elapsed().unwrap().as_secs());
}
