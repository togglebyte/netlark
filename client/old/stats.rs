use legion::system;
use legion::component;
use tinybit::widgets::{Border, Text};
use tinybit::{Color, ScreenPos, Viewport, WorldPos};

use crate::player::Player;

#[derive(Debug)]
pub struct Hp(pub u16);

pub struct StatsViewport(pub Viewport);

#[system(for_each)]
#[filter(component::<Player>())]
pub fn show_stats(
    #[resource] viewport: &mut StatsViewport,
    hp: &Hp,
    pos: &WorldPos,
) {
    let border = Border::new("╭─╮│╯─╰│".into(), Some(Color::Green), None);

    let hp_color = if hp.0 < 20 {
        Some(Color::Red)
    } else if hp.0 < 40 {
        Some(Color::Yellow)
    } else {
        None
    };

    let hp_text = Text::new(
        format!(
            "hp: {}          player: {}|{}\nxp: {}",
            hp.0, pos.x, pos.y, hp.0
        ),
        hp_color,
        None
    );
    viewport.0.draw_widget(&hp_text, ScreenPos::new(1, 1));
    viewport.0.draw_widget(&border, ScreenPos::zero());
}
