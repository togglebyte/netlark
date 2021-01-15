use tinybit::events::{KeyCode, KeyEvent};
use tinybit::{Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport, Color};
use tinybit::widgets::{TextField, Border, Text};

use crate::{Transition, Scene};

pub struct SignIn {
    username: TextField,
    password: TextField,
    viewport: Viewport,
    border: Border,
    status: Text,
}

impl SignIn {
    pub fn new(size: ScreenSize) -> Self {
        let mut username = TextField::new(None, None);
        username.focus = true;

        let mut password = TextField::new(None, None);
        password.password = true;

        Self {
            username,
            password,
            viewport: Viewport::new(ScreenPos::zero(), size),
            border: Border::new("        ".into(), None, None),
            status: Text::new("Hello, World", None, None),
        }
    }

    fn authenticate(&mut self) {
        let username = &self.username.text;
        let password = &self.password.text;

        // 1. Compose a `Message`.
        // 2. Send the message to the network thread
        // 3. we're done, yay
    }
}

impl Scene for SignIn {
    fn tick(&mut self) -> Option<Transition> {
        None
    }

    fn input(&mut self, ev: KeyEvent) -> Option<Transition> {
        match ev {
            KeyEvent { code: KeyCode::Esc, .. } => return Some(Transition::Pop),
            KeyEvent { code: KeyCode::Tab, .. } if self.username.focus => {
                self.username.focus = false;
                self.password.focus = true;
            }
            KeyEvent { code: KeyCode::Tab, .. } if self.password.focus => {
                self.password.focus = false;
                self.username.focus = true;
            }
            KeyEvent { code: KeyCode::Enter,  .. } => {
                self.status.0 = "Loading ...".into();
            }
            _ => match (self.username.focus, self.password.focus) {
                (true, _) => self.username.event(ev),
                (false, true) => self.password.event(ev),
                _ => { }
            }
        }

        None
    }

    fn resize(&mut self, width: u16, height: u16, renderer: &mut Renderer<StdoutTarget>) {
        self.viewport.resize(width, height);
        renderer.clear();
    }

    fn render(&mut self, renderer: &mut Renderer<StdoutTarget>) {
        let x = self.viewport.size.width / 2 - 7;
        let y = self.viewport.size.height / 2 - 1;
        self.viewport.draw_widget(&self.username, ScreenPos::new(x, y - 2));
        self.viewport.draw_widget(&self.password, ScreenPos::new(x, y));
        self.viewport.draw_widget(&self.border, ScreenPos::zero());
        self.viewport.draw_widget(&self.status, ScreenPos::new(x, y + 2));
        renderer.render(&mut self.viewport);
    }
}
