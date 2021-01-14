use legion::{system, Resources, Schedule};
use tinybit::events::{Event, KeyCode, KeyEvent};
use tinybit::widgets::{Border, Text};
use tinybit::{term_size, ScreenPos, ScreenSize, Viewport};

use crate::account::SignIn;
use crate::state::{State, Transition};
use crate::{NextState, Rend};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct MainMenu;

impl MainMenu {
    pub fn schedule(resources: &mut Resources) -> Schedule {
        resources.insert(MenuSelection {
            max_index: 1,
            selected: 0,
        });

        let mut schedule = Schedule::builder();
        schedule.add_system(select_menu_entry_system());
        schedule.add_system(draw_menu_system());
        schedule.add_system(render_main_menu_system());

        let (width, height) = term_size().expect("Failed to get term size");

        let viewport_size = ScreenSize::new(width - 4, height - 4);
        let viewport = Viewport::new(ScreenPos::new(2, 2), viewport_size);
        resources.insert(MainMenuViewport(viewport));

        schedule.build()
    }
}

// -----------------------------------------------------------------------------
//     - Resources -
// -----------------------------------------------------------------------------

struct MenuSelection {
    max_index: usize,
    selected: usize,
}

impl MenuSelection {
    fn next(&mut self) {
        self.selected = match self.selected {
            n if n < self.max_index => n + 1,
            _ => 0,
        }
    }

    fn prev(&mut self) {
        self.selected = match self.selected {
            n if n == 0 => self.max_index,
            n => n - 1,
        }
    }
}

struct MainMenuViewport(pub Viewport);

// -----------------------------------------------------------------------------
//     - Systems -
// -----------------------------------------------------------------------------
#[system]
fn select_menu_entry(
    #[resource] event: &mut Event,
    #[resource] menu_selection: &mut MenuSelection,
    #[resource] next_state: &mut NextState,
) {
    let key_ev = match event {
        Event::Key(k) => k,
        _ => return,
    };

    match key_ev {
        KeyEvent {
            code: KeyCode::Up, ..
        } => {
            menu_selection.next();
        }
        KeyEvent {
            code: KeyCode::Down,
            ..
        } => {
            menu_selection.prev();
        }
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            let transition = match menu_selection.selected {
                0 => Transition::Push(State::SignIn(SignIn)),
                1 => Transition::Quit,
                _ => unreachable!(),
            };

            *next_state = Some(transition);
            // resources.insert(transition);
        }
        _ => return,
    }
}

#[system]
fn draw_menu(
    #[resource] menu_selection: &mut MenuSelection,
    #[resource] viewport: &mut MainMenuViewport,
) {
    let (start_text, quit_text) = match menu_selection.selected {
        0 => ("> Sign in".to_string(), "  Quit".to_string()),
        1 => ("  Sign in".to_string(), "> Quit".to_string()),
        _ => return,
    };

    viewport.0.draw_widget(
        &Border::new("╔═╗║╝═╚║".to_string(), None, None),
        ScreenPos::zero(),
    );

    viewport.0.draw_widget(
        &Text(start_text, None, None),
        ScreenPos::new(
            viewport.0.size.width / 2 - 7,
            viewport.0.size.height / 2 - 1,
        ),
    );

    viewport.0.draw_widget(
        &Text(quit_text, None, None),
        ScreenPos::new(
            viewport.0.size.width / 2 - 7,
            viewport.0.size.height / 2 + 1,
        ),
    );

    viewport.0.draw_widget(
        &Text("Some title here!".to_string(), None, None),
        ScreenPos::new(2, 2),
    );
}

#[system]
fn render_main_menu(
    #[resource] viewport: &mut MainMenuViewport,
    #[resource] renderer: &mut Rend,
    #[resource] event: &Event,
) {
    if let Event::Resize(width, height) = event {
        renderer.clear();
        viewport.0.resize(*width, *height);
    }
    renderer.render(&mut viewport.0);
}
