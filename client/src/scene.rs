use tinybit::{Renderer, StdoutTarget};
use tinybit::events::KeyEvent;

use crate::Transition;

pub trait Scene {
    fn tick(&mut self) -> Option<Transition> {
        None
    }

    fn input(&mut self, _: KeyEvent) -> Option<Transition> {
        None
    }

    fn resize(&mut self, _: u16, _: u16, _: &mut Renderer<StdoutTarget>);
    fn render(&mut self, renderer: &mut Renderer<StdoutTarget>);
}
