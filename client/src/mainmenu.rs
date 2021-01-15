use tinybit::events::{KeyCode, KeyEvent};
use tinybit::widgets::{Border, Text};
use tinybit::{Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport};

use crate::keymap::Input;
use crate::{Scene, Transition};
use crate::signin::SignIn;

pub struct MainMenu {
    viewport: Viewport,
    border: Border,
    signin: Text,
    quit: Text,
    selected_index: u8,
}

impl MainMenu {
    pub fn new(size: ScreenSize) -> Self {
        Self {
            viewport: Viewport::new(ScreenPos::zero(), size),
            border: Border::new("╔═╗║╝═╚║".to_string(), None, None),
            signin: "> Sign in".to_string().into(),
            quit: "  Quit".to_string().into(),
            selected_index: 0,
        }
    }
}

impl Scene for MainMenu {
    fn tick(&mut self) -> Option<Transition> {
        None
    }

    fn input(&mut self, ev: KeyEvent) -> Option<Transition> {
        let input = Input::from_ev(&ev);
        match input {
            Input::Up => {
                self.selected_index += 1;
                if self.selected_index > 1 {
                    self.selected_index = 0;
                }
            }
            Input::Down => {
                if self.selected_index == 0 {
                    self.selected_index = 1;
                } else {
                    self.selected_index -= 1;
                }
            }
            Input::Select if self.selected_index == 0 => return Some(Transition::Push(Box::new(SignIn::new(self.viewport.size)))),
            Input::Cancel => return Some(Transition::Pop),
            Input::Select if self.selected_index == 1 => return Some(Transition::Pop),
            _ => {}
        }

        match self.selected_index {
            0 => unsafe {
                self.signin.0.as_mut_vec()[0] = b'>';
                self.quit.0.as_mut_vec()[0] = b' ';
            },
            1 => unsafe {
                self.signin.0.as_mut_vec()[0] = b' ';
                self.quit.0.as_mut_vec()[0] = b'>';
            },
            _ => unreachable!(),
        }

        None
    }

    fn resize(&mut self, width: u16, height: u16, renderer: &mut Renderer<StdoutTarget>) {
        self.viewport.resize(width, height);
        renderer.clear();
    }

    fn render(&mut self, renderer: &mut Renderer<StdoutTarget>) {
        self.viewport.draw_widget(&self.border, ScreenPos::zero());

        let size = self.viewport.size;
        let signin_pos = ScreenPos::new(size.width / 2 - 7, size.height / 2 - 1);
        let quit_pos = ScreenPos::new(size.width / 2 - 7, size.height / 2 + 1);

        self.viewport.draw_widget(&self.signin, signin_pos);
        self.viewport.draw_widget(&self.quit, quit_pos);

        renderer.render(&mut self.viewport);
    }
}
