use tinybit::events::{Event, KeyCode, KeyEvent};
use tinybit::widgets::Widget;
use tinybit::{Color, Pixel, ScreenPos, ScreenSize};

pub struct TextField {
    pub text: String,
    pub password: bool,
    pub focus: bool,
    pub submit: bool,
    pub enabled: bool,
    pub max_length: Option<usize>,
    color: Option<Color>,
    cursor: usize,
}

impl TextField {
    pub fn new(color: Option<Color>) -> Self {
        Self {
            text: String::new(),
            password: false,
            focus: false,
            submit: false,
            enabled: true,
            max_length: None,
            color,
            cursor: 0,
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
    }

    pub fn unfocus(&mut self) {
        self.focus = false;
        self.cursor = self.text.chars().count();
    }

    pub fn event(&mut self, event: Event) {
        if !self.focus || !self.enabled {
            return;
        }

        let key_code = match event {
            Event::Key(KeyEvent { code: k, .. }) => k,
            _ => return,
        };

        match key_code {
            KeyCode::Left if self.cursor > 0 => {
                self.cursor -= 1;
            }
            KeyCode::Right if self.cursor < self.text.len() => {
                self.cursor += 1;
            }
            KeyCode::Backspace if self.cursor > 0 => {
                self.cursor -= 1;
                self.text.remove(self.cursor);
            }
            KeyCode::Delete if self.text.len() > 0 => {
                self.text.remove(self.cursor);
                if self.cursor > self.text.len() {
                    self.cursor = self.text.len();
                }
            }
            KeyCode::Enter => {
                self.submit = true;
            }
            KeyCode::Char(c) => {
                match self.max_length {
                    Some(max_len) if max_len <= self.text.chars().count() => return,
                    _ => {}
                }

                self.text.insert(self.cursor, c);
                self.cursor += 1;
            }
            _ => {}
        }
    }
}

impl Widget for TextField {
    fn pixels(&self, _size: ScreenSize) -> Vec<Pixel> {
        let mut pixels = self
            .text
            .chars()
            .enumerate()
            .map(|(x, c)| if self.password { (x, '*') } else { (x, c) })
            .map(|(x, c)| Pixel::new(c, ScreenPos::new(x as u16, 0), self.color, None))
            .collect::<Vec<Pixel>>();

        if !self.focus || !self.enabled {
            return pixels;
        }

        // Get char under cursor
        let c = match self.password {
            true => self
                .text
                .chars()
                .nth(self.cursor)
                .map(|_| '*')
                .unwrap_or(' '),
            false => self.text.chars().nth(self.cursor).unwrap_or(' '),
        };

        // Draw cursor
        pixels.push(Pixel::new(
            c,
            ScreenPos::new(self.cursor as u16, 0),
            Some(Color::Black),
            Some(self.color.unwrap_or(Color::White)),
        ));

        pixels
    }
}
