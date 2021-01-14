use tinybit::{Renderer, StdoutTarget, Viewport, ScreenSize, ScreenPos};
use tinybit::widgets::Border;
use tinybit::events::{KeyCode, KeyEvent};

use crate::{Transition, Scene};

pub struct MainMenu {
    viewport: Viewport
}

impl MainMenu {
    pub fn new(size: ScreenSize) -> Self {
        Self {
            viewport: Viewport::new(ScreenPos::zero(), size)
        }
    }
}

impl Scene for MainMenu {
    fn tick(&mut self) -> Option<Transition> {
        None
    }

    fn input(&mut self, input: KeyEvent) -> Option<Transition> {
        match input {
            KeyEvent { code: KeyCode::Esc, .. } => Some(Transition::Pop),
            _ => None
        }
    }

    fn resize(&mut self, width: u16, height: u16, renderer: &mut Renderer<StdoutTarget>) {
        self.viewport.resize(width, height);
        renderer.clear();
    }

    fn render(&mut self, renderer: &mut Renderer<StdoutTarget>) {
        self.viewport.draw_widget(
            &Border::new("╔═╗║╝═╚║".to_string(), None, None),
            ScreenPos::zero(),
        );

        renderer.render(&mut self.viewport);
    }
}
