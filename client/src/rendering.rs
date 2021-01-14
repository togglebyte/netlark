use legion::system;
use legion::systems::Builder;
use legion::world::SubWorld;
use tinybit::widgets::Border;
use tinybit::{Camera, Color, Pixel, Renderer, ScreenPos, StdoutTarget, Viewport, WorldPos};

use crate::player::Cursor;
use crate::stats::StatsViewport;
use crate::tilemap::{Tile, TilemapMeh};
use crate::Rend;

pub struct MainViewport(pub Viewport);

#[derive(Debug, Clone, Copy)]
pub struct Glyph(pub char);

#[system(par_for_each)]
fn world_to_screen(
    #[resource] camera: &Camera,
    world_pos: &WorldPos,
    screen_pos: &mut ScreenPos,
) {
    *screen_pos = camera.to_screen(*world_pos);
}

#[system(for_each)]
fn draw_pixels(
    #[resource] viewport: &mut MainViewport,
    #[resource] cam: &Camera,
    pos: &ScreenPos,
    glyph: &Glyph,
) {
    viewport.0.draw_pixel(Pixel::white(glyph.0, *pos));
}

#[system]
fn draw_tilemap(
    #[resource] tilemap: &mut TilemapMeh,
    #[resource] cam: &Camera,
    #[resource] viewport: &mut MainViewport,
) {
    tilemap.tiles.iter().for_each(|tile| {
        let pixel = Pixel::new(tile.glyph, cam.to_screen(tile.pos), tile.color);
        viewport.0.draw_pixel(pixel);
    });
}

#[system]
fn draw_border(#[resource] viewport: &mut MainViewport) {
    let border = Border::new("╭─╮│╯─╰│".into(), Some(Color::Blue));
    viewport.0.draw_widget(border, ScreenPos::zero());
}

#[system]
fn draw_cursor(
    #[resource] viewport: &mut MainViewport,
    #[resource] cam: &Camera,
    #[resource] cursor: &Cursor,
) {
    if !cursor.visible {
        return;
    }

    let l_pixel = Pixel::new(cursor.left, cam.to_screen(WorldPos::new(cursor.pos.x - 1.0, cursor.pos.y)), None);
    let r_pixel = Pixel::new(cursor.right, cam.to_screen(WorldPos::new(cursor.pos.x + 1.0, cursor.pos.y)), None);
    viewport.0.draw_pixel(l_pixel);
    viewport.0.draw_pixel(r_pixel);
}

#[system]
fn render(
    #[resource] renderer: &mut Rend,
    #[resource] main_viewport: &mut MainViewport,
    #[resource] stats_viewport: &mut StatsViewport,
) {
    renderer.render(&mut main_viewport.0);
    renderer.render(&mut stats_viewport.0);
}

pub fn add_rendering_systems(builder: &mut Builder) {
    builder.add_system(world_to_screen_system())
    .add_system(draw_tilemap_system())
    .add_system(draw_cursor_system())
    .add_system(draw_pixels_system())
    .add_system(draw_border_system())
    // Rendering should be the last system
    .add_system(render_system());
}
