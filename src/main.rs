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
  // samples::ray_to_scene_advance(samples::AdvanceSceneCase::AntialiasingScene);

  // Chapter 9: material: diffuse -> gamma correction
  // samples::ray_to_scene_advance(samples::AdvanceSceneCase::MaterialScene);
  // samples::ray_to_scene_advance(samples::AdvanceSceneCase::MetalScene)
  
  // Chapter 10: Dielectrics: refraction
  // samples::ray_to_scene_advance(samples::AdvanceSceneCase::RefractionScene)

  // Chapter 11: Move camera
  samples::ray_to_scene_camera()
}