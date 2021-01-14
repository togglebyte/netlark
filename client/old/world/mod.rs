use legion::{system, Resources, Schedule, World};

use tinybit::events::{Event, KeyCode, KeyEvent};
use tinybit::widgets::Border;
use tinybit::{term_size, Camera, Color, Pixel, ScreenPos, ScreenSize, Viewport, WorldPos};

use crate::mainmenu::MainMenu;
use crate::player::{add_player_systems, Cursor, Player};
use crate::state::{State, Transition};
use crate::stats::{show_stats_system, Hp, StatsViewport};
use crate::{NextState, Rend};

mod tilemap;

use tilemap::{make_rubbish_tilemap, TilemapMeh};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct GameState;

impl GameState {
    pub fn schedule(world: &mut World, resources: &mut Resources) -> Schedule {
        let mut schedule = Schedule::builder();
        let (width, _) = term_size().expect("No term no play!");

        let stats_viewport = Viewport::new(ScreenPos::zero(), ScreenSize::new(width, 3));

        let gameworld_viewport = Viewport::new(ScreenPos::new(0, 4), ScreenSize::new(80, 20));
        let mut camera = Camera::from_viewport(WorldPos::zero(), &gameworld_viewport);
        camera.set_limit(4, 4, 4, 4);

        let mut rubbish_map = make_rubbish_tilemap();
        rubbish_map.update(WorldPos::zero(), gameworld_viewport.size);

        // Resources
        resources.insert(GameViewport(gameworld_viewport));
        resources.insert(StatsViewport(stats_viewport));
        resources.insert(rubbish_map);
        resources.insert(camera);
        resources.insert(Cursor {
            left: '(',
            right: ')',
            visible: false,
            pos: WorldPos::zero(),
        });

        // Add systems
        add_player_systems(&mut schedule);

        schedule
            .add_system(open_main_menu_system())
            .add_system(world_to_screen_system())
            // Show stats
            .add_system(show_stats_system())
            // ....
            .add_system(draw_tilemap_system())
            .add_system(draw_cursor_system())
            .add_system(draw_pixels_system())
            .add_system(draw_border_system())
            // Rendering should be the last system
            .add_system(render_system());

        // Setup player
        world.push((
            Player(0),
            Glyph('@'),
            WorldPos::zero(),
            Hp(100),
            ScreenPos::zero(),
        ),);

        schedule.build()
    }
}

// -----------------------------------------------------------------------------
//     - Components -
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Glyph(pub char);

// -----------------------------------------------------------------------------
//     - Resources -
// -----------------------------------------------------------------------------

pub struct GameViewport(Viewport);

// -----------------------------------------------------------------------------
//     - Systems -
// -----------------------------------------------------------------------------

#[system]
fn open_main_menu(#[resource] event: &mut Event, #[resource] next_state: &mut NextState) {
    let key_ev = match event {
        Event::Key(k) => k,
        _ => return,
    };

    match key_ev {
        KeyEvent {
            code: KeyCode::Esc, ..
        } => *next_state = Some(Transition::Push(State::MainMenu(MainMenu))),
        _ => return,
    }
}

#[system(par_for_each)]
fn world_to_screen(#[resource] camera: &Camera, world_pos: &WorldPos, screen_pos: &mut ScreenPos) {
    *screen_pos = camera.to_screen(*world_pos);
}

#[system(for_each)]
fn draw_pixels(
    #[resource] viewport: &mut GameViewport,
    pos: &ScreenPos,
    glyph: &Glyph,
) {
    viewport.0.draw_pixel(Pixel::white(glyph.0, *pos));
}

#[system]
fn draw_tilemap(
    #[resource] tilemap: &mut TilemapMeh,
    #[resource] cam: &Camera,
    #[resource] viewport: &mut GameViewport,
) {
    tilemap.tiles.iter().for_each(|tile| {
        let pixel = Pixel::new(tile.glyph, cam.to_screen(tile.pos), tile.fg_color, tile.bg_color);
        viewport.0.draw_pixel(pixel);
    });
}

#[system]
fn draw_border(#[resource] viewport: &mut GameViewport) {
    let border = Border::new("╭─╮│╯─╰│".into(), Some(Color::Blue), None);
    viewport.0.draw_widget(&border, ScreenPos::zero());
}

#[system]
fn draw_cursor(
    #[resource] viewport: &mut GameViewport,
    #[resource] cam: &Camera,
    #[resource] cursor: &Cursor,
) {
    if !cursor.visible {
        return;
    }

    let l_pixel = Pixel::new(
        cursor.left,
        cam.to_screen(WorldPos::new(cursor.pos.x - 1.0, cursor.pos.y)),
        None,
        None,
    );
    let r_pixel = Pixel::new(
        cursor.right,
        cam.to_screen(WorldPos::new(cursor.pos.x + 1.0, cursor.pos.y)),
        None,
        None,
    );
    viewport.0.draw_pixel(l_pixel);
    viewport.0.draw_pixel(r_pixel);
}

#[system]
fn render(
    #[resource] renderer: &mut Rend,
    #[resource] main_viewport: &mut GameViewport,
    #[resource] stats_viewport: &mut StatsViewport,
) {
    renderer.render(&mut main_viewport.0);
    renderer.render(&mut stats_viewport.0);
}
