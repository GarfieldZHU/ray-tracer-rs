use raytracing::misc::samples;

fn main() {
  // Chapter 2: output an basic image
  // samples::outputImage();

  // Chapter 4, 5: ray, sphere
  // samples::ray_to_scene(samples::SceneCase::RAY_SPHERE_SCENE);

  // Chapter 6: shading
  samples::ray_to_scene(samples::SceneCase::SHADING_WITH_NORMAL_SCENE);

}