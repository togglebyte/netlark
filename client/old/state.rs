use legion::{Resources, World, Schedule};

use crate::account::SignIn;
use crate::world::GameState;
use crate::mainmenu::MainMenu;
// use crate::inventory::Inventory;

// -----------------------------------------------------------------------------
//     - State -
// -----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum State {
    MainMenu(MainMenu),
    SignIn(SignIn),
    // Inventory(Inventory),
    Game(GameState),
}

impl State {
    pub fn schedule(&self, world: &mut World, resources: &mut Resources) -> Schedule {
        match self {
            Self::MainMenu(_) => MainMenu::schedule(resources),
            Self::SignIn(_) => SignIn::schedule(resources),
            Self::Game(_) => GameState::schedule(world, resources),
        }
    }
}

// -----------------------------------------------------------------------------
//     - Transitions -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum Transition {
    Pop,
    Push(State),
    Swap(State),
    Quit,
}

// -----------------------------------------------------------------------------
//     - State stack -
// -----------------------------------------------------------------------------
pub struct StateStack {
    states: Vec<State>,
}

impl StateStack {
    pub fn new() -> Self {
        let states = vec![
            State::MainMenu(MainMenu),
        ];

        Self { states }
    }

    pub fn top(&self) -> State {
        *self.states.last().expect("This should never be empty")
    }

    pub fn pop(&mut self) {
        let _ = self.states.pop();
    }

    pub fn push(&mut self, state: State) {
        self.states.push(state)
    }
}

