macro_rules! or_cont {
    ($e:expr) => {
        match $e {
            Some(val) => val,
            None => continue
        }
    }
}

pub fn run() {
    let mut scenes: Vec<Scene> = Vec::new();

    let target = StdouTarget::new();
    let mut renderer = Renderer::new(target);

    for event in events(EventModel::Fps(20)) {
        let mut current_scene = or_cont!(scenes.last());
        let transition = match event {
            Event::Input(ev) => current_scene.input(ev),
            Event::Tick => {
                let transition = current_scene.tick();
                current_scene.render(&mut renderer); 
                transition
            }
            _ => { None }  // ?
        }

        match current_scene.transition() {
            Some(t) => match t {
                Tranistion::Pop => scenes.pop(),
                Transition::Push(new_scene) => scenes.push(new_scene),
                Transition::Swap(scene) => {
                    scenes.pop();           // get rid of old one
                    scenes.push(new_scene)  // push the new one
                }
            }
            None => {}
        }
    }

}
