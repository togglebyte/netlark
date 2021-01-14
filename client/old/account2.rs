pub trait Scene {
    fn tick(&mut self) -> Option<Transition> {
        None
    }
    fn input(&mut self, input: InputEvent) -> Option<Transition> {
        None
    }
    fn render(&mut self, renderer: Renderer);
}

pub struct Account {
    username: TextField,
    password: TextField,
    viewport: Viewport,
}

impl Account {
    pub fn new() -> Self {
        let username = TextField::new();
        let password = TextField::new();
        let viewport = Viewport::new();

        Self {
            username,
            password,
            vewport,
        }
    }
}

impl Scene for Accounts {
    fn input(&mut self, input: InputEvent) {
        match input {
            Esc => transition_scene(),
            Char(c) => {
                if self.username.focus {
                    self.username.input(c);
                } else if self.password.focus {
                    self.password.input(c);
                }
            }
            _ => {}
        }

        self.viewport
            .draw_widget(&self.username, ScreenPos::new(x, y));
        self.viewport
            .draw_widget(&self.password, ScreenPos::new(x, y));
    }

    fn tick(&mut self) {
        let resp = pull_response();
        if resp.signed_in_yay() {
            self.transition = Some(Transition::Pop);
        }
    }

    fn render(&mut self, renderer: &mut Renderer) {
        renderer.render(&self.viewport);
    }
}
