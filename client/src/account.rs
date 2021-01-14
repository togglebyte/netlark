use common::models::{Auth, Message};
use common::Tx;
use legion::{system, Resources, Schedule};
use tinybit::events::{Event, KeyCode, KeyEvent};
use tinybit::widgets::{Border, Text};
use tinybit::{term_size, Color, ScreenPos, ScreenSize, Viewport};

use crate::state::{State, Transition};
use crate::ui::TextField;
use crate::world::GameState;
use crate::{NextState, Rend};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct SignIn;

pub struct LoadingMessage(bool, Text);
pub struct ErrorMessage(bool, Text);

impl SignIn {
    pub fn schedule(resources: &mut Resources) -> Schedule {
        let (width, height) = term_size().expect("Failed to get term size");

        let viewport_size = ScreenSize::new(width - 4, height - 4);
        let viewport = Viewport::new(ScreenPos::new(2, 2), viewport_size);

        // Resources
        resources.insert(SignInViewport(viewport));

        // Loading message
        let loading_message =
            LoadingMessage(false, Text::new("Loading...".to_string(), None, None));
        resources.insert(loading_message);

        // Error message
        let error_message = ErrorMessage(
            false,
            Text::new("Invalid credentials".to_string(), Some(Color::Red), None),
        );
        resources.insert(error_message);

        // Username
        let mut username = UsernameInput(TextField::new(None));
        username.0.focus = true;
        username.0.max_length = Some(30);
        resources.insert(username);

        // Password
        let mut password = TextField::new(None);
        // password.password = true;
        resources.insert(PasswordInput(password));

        // Systems
        let mut schedule = Schedule::builder();
        schedule.add_system(render_system());
        schedule.add_system(draw_input_fields_system());
        schedule.add_system(read_username_and_password_system());
        schedule.add_system(next_state_system());
        schedule.build()
    }
}

// -----------------------------------------------------------------------------
//     - Resources -
// -----------------------------------------------------------------------------
struct SignInViewport(Viewport);
struct UsernameInput(TextField);
struct PasswordInput(TextField);

// -----------------------------------------------------------------------------
//     - Systems -
// -----------------------------------------------------------------------------
#[system]
fn read_username_and_password(
    #[resource] server_tx: &mut Tx,
    #[resource] event: &mut Event,
    #[resource] loading_message: &mut LoadingMessage,
    #[resource] error_message: &mut ErrorMessage,
    #[resource] username: &mut UsernameInput,
    #[resource] password: &mut PasswordInput,
    #[resource] next_state: &mut NextState,
) {
    let key_ev = match event {
        Event::Key(k) => k,
        _ => return,
    };

    match key_ev {
        KeyEvent {
            code: KeyCode::Esc, ..
        } => {
            *next_state = Some(Transition::Pop);
        }
        KeyEvent {
            code: KeyCode::Tab, ..
        }
        | KeyEvent {
            code: KeyCode::BackTab,
            ..
        } => {
            if username.0.focus {
                username.0.unfocus();
                password.0.focus = true;
            } else if password.0.focus {
                password.0.unfocus();
                username.0.focus = true;
            }
        }
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            // Send username and password
            // Show loading message
            username.0.enabled = false;
            password.0.enabled = false;

            loading_message.0 = true;
            error_message.0 = false;
            let _ = server_tx.send(Message::Auth(Auth::SignIn(
                username.0.text.clone(),
                password.0.text.clone(),
            )));
        }
        _ => {}
    }

    if username.0.focus {
        username.0.event(*event);
    } else if password.0.focus {
        password.0.event(*event);
    }
}

#[system]
fn draw_input_fields(
    #[resource] viewport: &mut SignInViewport,
    #[resource] username: &mut UsernameInput,
    #[resource] password: &mut PasswordInput,
    #[resource] error_message: &mut ErrorMessage,
) {
    viewport.0.draw_widget(
        &Border::new("╔═╗║╝═╚║".to_string(), None, None),
        ScreenPos::zero(),
    );

    let x = viewport.0.size.width / 2 - 7;
    let y = viewport.0.size.height / 2 - 1;
    viewport.0.draw_widget(&username.0, ScreenPos::new(x, y));

    let x = viewport.0.size.width / 2 - 7;
    let y = viewport.0.size.height / 2 + 1;
    viewport.0.draw_widget(&password.0, ScreenPos::new(x, y));

    if error_message.0 {
        viewport
            .0
            .draw_widget(&error_message.1, ScreenPos::new(x, y + 2));
    }
}

#[system]
fn next_state(
    #[resource] loading: &mut LoadingMessage,
    #[resource] error_message: &mut ErrorMessage,
    #[resource] viewport: &mut SignInViewport,
    #[resource] message: &mut Option<Message>,
    #[resource] next_state: &mut NextState,
    #[resource] username: &mut UsernameInput,
    #[resource] password: &mut PasswordInput,
) {
    if loading.0 {
        // Show loading message
        viewport.0.draw_widget(&loading.1, ScreenPos::new(2, 2));
    }

    let gamestate = match message.take() {
        Some(Message::Auth(Auth::Success(gamestate))) => gamestate,
        Some(Message::Auth(Auth::Failed)) => {
            error_message.0 = true;
            return;
        }
        _ => return,
    };

    // Hide loading message
    if loading.0 {
        loading.0 = false;
    }

    // Re-enable the input fields
    username.0.focus = false;
    username.0.enabled = true;
    password.0.enabled = true;
    password.0.focus = true;
    password.0.clear();

    // Clear the message
    message.take();

    *next_state = Some(Transition::Swap(State::Game(GameState)));
}

#[system]
fn render(
    #[resource] viewport: &mut SignInViewport,
    #[resource] renderer: &mut Rend,
    #[resource] event: &Event,
) {
    if let Event::Resize(width, height) = event {
        renderer.clear();
        viewport.0.resize(*width, *height);
    }

    renderer.render(&mut viewport.0);
}
