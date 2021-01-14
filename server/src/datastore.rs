use tinybit::WorldPos;

use common::models::{GameState, TileData};

pub fn get_game_state(username: &str) -> Option<GameState> {
    let player_pos = get_player(username)?;
    let tilemap = get_tile_data(player_pos);

    let gs = GameState {
        tilemap,
        player_pos,
    };

    Some(gs)
}

fn get_player(username: &str) -> Option<WorldPos> {
    Some(WorldPos::new(10.0, 54.0))
}

fn get_tile_data(player_pos: WorldPos) -> Vec<TileData> {
    let width = 51;
    let height = 40;

    let minx = player_pos.x as isize - width / 2;
    let maxx = player_pos.x as isize + width / 2;
    let miny = player_pos.y as isize - height / 2;
    let maxy = player_pos.y as isize + height / 2;

    let mut tiles = Vec::with_capacity((width * height) as usize);

    for x in minx..maxx {
        for y in miny..maxy {
            let pos = WorldPos::new(x as f32, y as f32);
            let glyph = 'x';
            tiles.push(TileData::new(pos, glyph));
        }
    }

    tiles
}

pub fn put_game_state(username: &str, gamestate: GameState) {}
