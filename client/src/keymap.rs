use tinybit::events::{KeyCode, KeyEvent, KeyModifiers};

pub enum Input {
    Left,
    Right,
    Up,
    Down,
    Select,
    Cancel,
    Next,
    Prev,

    Unbound,
}

impl Input {
    pub fn from_ev(ev: &KeyEvent) -> Input {
        match ev {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: km,
            } if km.contains(KeyModifiers::CONTROL) => Input::Cancel,
            KeyEvent {
                code: KeyCode::Esc, ..
            } => Input::Cancel,

            KeyEvent {
                code: KeyCode::Char('h'),
                ..
            } => Input::Left,
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => Input::Left,

            KeyEvent {
                code: KeyCode::Char('j'),
                ..
            } => Input::Down,
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => Input::Down,

            KeyEvent {
                code: KeyCode::Char('k'),
                ..
            } => Input::Up,
            KeyEvent {
                code: KeyCode::Up, ..
            } => Input::Up,

            KeyEvent {
                code: KeyCode::Char('l'),
                ..
            } => Input::Right,
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => Input::Right,

            KeyEvent {
                code: KeyCode::Char('e'),
                ..
            } => Input::Select,
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => Input::Select,

            KeyEvent {
                code: KeyCode::Tab,
                ..
            } => Input::Next,

            KeyEvent {
                code: KeyCode::BackTab,
                ..
            } => Input::Prev,

            _ => Input::Unbound,
        }
    }
}
