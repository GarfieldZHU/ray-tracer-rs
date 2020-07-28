use raytracing::misc::samples;

fn main() {
  // Chapter 2: output an basic image
  // samples::output_image();

  // Chapter 4, 5: ray, sphere
  // samples::ray_to_scene(samples::SceneCase::RaySphereScene);

  // Chapter 6: shading
  // samples::ray_to_scene(samples::SceneCase::ShadingWithNormalScene);

  // Chapter 7: hittable objects
  // samples::ray_to_scene(samples::SceneCase::HittableObjectsScene);

  // Chapter 8: antialising
  samples::ray_to_scene_antialiasing()
}