mod scene;
mod visual;

fn main() {
    let mut visual = visual::Visual::new();

    let initial_scene = scene::SceneDummy::new(&mut visual);
    scene::run(initial_scene, visual);
}
