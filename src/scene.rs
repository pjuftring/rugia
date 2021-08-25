use super::visual::Visual;
use crate::visual::{ModelInstance, Spatial};
use std::f32::consts::PI;

pub enum Action {
    // Continue with the same scene
    Continue,
    // Terminate the program
    //Terminate,
    // Switch to a different scene
    // (lazily initialized in order to occupy less resources)
    //Switch(Box<dyn FnOnce() -> Box<dyn Scene>>),
}

pub trait Scene {
    fn run(&mut self, visual: &mut Visual) -> Action;
}

pub struct SceneDummy {
    camera: Spatial,
    time: f32,
    object: ModelInstance,
}

impl SceneDummy {
    pub fn new(visual: &mut Visual) -> SceneDummy {
        println!("Init dummy scene");
        let mut camera = Spatial::new();
        camera.place(0., 0.3, 0.9);
        let model = visual.load_model(
            include_bytes!("scene/penguin/arrays.i16"),
            include_bytes!("scene/penguin/elements.u16"),
            Some(include_bytes!("scene/penguin/penguin.webp")),
        );
        let object = model.new_instance();
        SceneDummy {
            camera,
            time: 0.,
            object,
        }
    }
}

impl Scene for SceneDummy {
    fn run(&mut self, visual: &mut Visual) -> Action {
        self.time += 0.01;
        self.object.with_spatial(|spatial| {
            spatial.place(0., 0., 0.);
            spatial.yaw(self.time as f32);
        });

        visual.clear(0.6, 0.8, 1.0);
        visual.camera(&self.camera, 0.1, 100., PI / 2.);
        Action::Continue
    }
}

pub fn run<T: Scene>(scene: T, mut visual: Visual) {
    let mut scene: Box<dyn Scene> = Box::new(scene);
    loop {
        match scene.run(&mut visual) {
            Action::Continue => {
                visual.swap_and_poll();
            } /*Action::Terminate => {
                  break;
              }
              Action::Switch(next_scene_closure) => {
                  drop(scene);
                  scene = next_scene_closure();
              }*/
        }
        if visual.should_close() {
            break;
        }
    }
}
