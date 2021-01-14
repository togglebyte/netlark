use std::collections::HashMap;

use legion::{Resources, Schedule, World};
use tinybit::events::{events, EventModel};
use tinybit::{Renderer, StdoutTarget};

mod account;
mod inventory;
mod mainmenu;
mod net;
mod player;
mod state;
mod stats;
mod ui;
mod world;

use mainmenu::MainMenu;
use state::{State, StateStack, Transition};

pub type Rend = Renderer<StdoutTarget>;
pub type NextState = Option<Transition>;

pub struct ClientTx(pub common::Tx);

// -----------------------------------------------------------------------------
//     - Log to a file -
//     Useful because we smush stdout
// -----------------------------------------------------------------------------
#[macro_export]
macro_rules! loggy {
    ($($arg:tt)*) => {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("/tmp/alog.txt")
            .unwrap();

        let data = format!("{:?}", $($arg)*);
        file.write_all(data.as_bytes());
        file.write(&[b'\n']);
    };
}

fn make_resources() -> Resources {
    let mut resources = Resources::default();

    // Renderer
    let stdout_renderer = StdoutTarget::new().expect("Failed to enter raw mode");
    let renderer = Renderer::new(stdout_renderer);
    resources.insert(renderer);
    resources.insert::<Option<common::models::Message>>(None);
    resources.insert::<NextState>(None);

    resources
}

// -----------------------------------------------------------------------------
//     - Schedules -
// -----------------------------------------------------------------------------
struct Schedules {
    schedules: HashMap<State, Schedule>,
}

impl Schedules {
    fn new(resources: &mut Resources) -> Self {
        let mut schedules = HashMap::<State, Schedule>::new();
        schedules.insert(State::MainMenu(MainMenu), MainMenu::schedule(resources));
        Self { schedules }
    }

    fn ensure_exists(&mut self, state: State, world: &mut World, resources: &mut Resources) {
        if !self.schedules.contains_key(&state) {
            let sched = state.schedule(world, resources);
            self.schedules.insert(state, sched);
        }
    }

    fn exec(&mut self, state: State, world: &mut World, resources: &mut Resources) {
        match self.schedules.get_mut(&state) {
            Some(systems) => systems.execute(world, resources),
            None => panic!("System not registered"),
        }
    }
}

fn client_channel() -> (common::Tx, common::Rx) {
    common::channel()
}

// -----------------------------------------------------------------------------
//     - Run -
// -----------------------------------------------------------------------------
pub fn run() {
    let (client_tx, client_rx) = client_channel();

    let server_tx = net::connect(client_tx);

    let mut resources = make_resources();
    resources.insert(server_tx);

    // World setup
    let mut world = World::default();
    let mut schedules = Schedules::new(&mut resources);
    let mut state_stack = StateStack::new();

    // Game loop
    for event in events(EventModel::Fps(20)) {
        resources.insert(event);

        let state = state_stack.top();
        schedules.exec(state, &mut world, &mut resources);

        // Message check here
        if let Ok(msg) = client_rx.try_recv() {
            resources.insert(Some(msg));
        }

        let transition = resources.get::<NextState>().map(|t| *t).flatten();
        let transition = match transition {
            Some(t) => t,
            None => continue,
        };

        match transition {
            Transition::Quit => break,
            Transition::Pop => state_stack.pop(),
            Transition::Swap(new_state) => {
                state_stack.pop();
                state_stack.push(State::from(new_state));
                schedules.ensure_exists(new_state, &mut world, &mut resources);
            }
            Transition::Push(new_state) => {
                state_stack.push(State::from(new_state));
                schedules.ensure_exists(new_state, &mut world, &mut resources);
            }
        }

        resources.insert::<NextState>(None);
        resources.get_mut::<Rend>().map(|mut r| r.clear());
    }
}
