const IMAGE_WIDTH: u32 =256;
const IMAGE_HEIGHT: u32 =256;

pub fn outputImage() -> () {
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


pub fn ray_to_scene() {
  
}