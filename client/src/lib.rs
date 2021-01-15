// use legion::{Resources, Schedule, World};
use legion::Resources;
use tinybit::events::{events, Event, EventModel};
use tinybit::{term_size, Renderer, ScreenSize, StdoutTarget};
use common::{Tx, Rx};

mod keymap;
mod mainmenu;
mod net;
mod scene;
mod signin;

pub use scene::Scene;

// -----------------------------------------------------------------------------
//     - Loggy macro -
// -----------------------------------------------------------------------------
#[macro_export]
macro_rules! loggy {
    ($($arg:tt)*) => { 
        let message = format!("{:?}", $($arg)*);
    }
}

// -----------------------------------------------------------------------------
//     - Game state -
// -----------------------------------------------------------------------------
pub struct GameState {
    net_tx: Tx,
    net_rx: Rx,
}

impl GameState {
    pub fn new(net_tx: Tx, net_rx: Rx) -> Self {
        Self {
            net_tx,
            net_rx,
        }
    }
}

// -----------------------------------------------------------------------------
//     - Transitions -
// -----------------------------------------------------------------------------
pub enum Transition {
    Swap(Box<dyn Scene>),
    Push(Box<dyn Scene>),
    Pop,
}

// -----------------------------------------------------------------------------
//     - Game loop -
// -----------------------------------------------------------------------------
pub fn run() -> Option<()> {
    // Renderer
    let stdout_target = StdoutTarget::new().ok()?;
    let mut renderer = Renderer::new(stdout_target);

    // Screen size
    let (width, height) = term_size().ok()?;
    let mut screen_size = ScreenSize::new(width, height);

    let (client_tx, client_rx) = net::channel();
    let net_tx = net::connect(client_tx);
    let mut gamestate = GameState::new(net_tx, client_rx);

    let mut scene_stack: Vec<Box<dyn Scene>> = vec![Box::new(mainmenu::MainMenu::new(screen_size))];
    for event in events(EventModel::Fps(20)) {
        // If there are no more scenes on the stack
        // consider this quitting
        let current_scene = scene_stack.last_mut()?;

        // Events
        let transition = match event {
            Event::Tick => {
                let t = current_scene.tick();
                current_scene.render(&mut renderer);
                t
            }
            Event::Key(key_ev) => current_scene.input(key_ev),
            Event::Resize(width, height) => {
                screen_size = ScreenSize::new(width, height);
                current_scene.resize(width, height, &mut renderer);
                None
            }
        };

        // Transitions
        if let Some(transition) = transition {
            renderer.clear();
            match transition {
                Transition::Swap(t) => {
                    scene_stack.pop();
                    scene_stack.push(t);
                }
                Transition::Pop => {
                    scene_stack.pop();
                }
                Transition::Push(t) => scene_stack.push(t),
            }
        }
    }

    Some(())
}
