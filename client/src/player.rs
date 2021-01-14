use legion::systems::Builder;
use legion::system;
use tinybit::{Camera, WorldPos};
use tinybit::events::{Event, KeyCode, KeyEvent};

// -----------------------------------------------------------------------------
//     - Resources -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Cursor {
    pub left: char,
    pub right: char,
    pub pos: WorldPos,
    pub visible: bool,
}

// -----------------------------------------------------------------------------
//     - Components -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Player(pub u8);

// -----------------------------------------------------------------------------
//     - Systems -
// -----------------------------------------------------------------------------

#[system(for_each)]
fn move_player(
    #[resource] event: &mut Event,
    #[resource] cursor: &Cursor,
    _: &Player,
    pos: &mut WorldPos,
) {
    let key_ev = match event {
        Event::Key(k) => k,
        _ => return,
    };

    if cursor.visible {
        return;
    }

    match key_ev {
        KeyEvent {code: KeyCode::Left, .. } => {
            pos.x -= 1.0;
        }
        KeyEvent {code: KeyCode::Right, .. } => {
            pos.x += 1.0;
        }
        KeyEvent {code: KeyCode::Up, .. } => {
            pos.y -= 1.0;
        }
        KeyEvent {code: KeyCode::Down, .. } => {
            pos.y += 1.0;
        }
        _ => return,
    }

    // Send player position to the server
    // let _ = tx.send(Message::PlayerPos(*pos));
}

#[system(for_each)]
fn track_player(#[resource] camera: &mut Camera, _: &Player, pos: &mut WorldPos) {
    camera.track(*pos);
}

#[system(for_each)]
fn move_cursor(
    #[resource] cursor: &mut Cursor,
    #[resource] event: &mut Event,
    player_pos: &WorldPos,
    _: &Player,
) {
    let key_ev = match event {
        Event::Key(k) => k,
        _ => return,
    };

    if !cursor.visible {
        if let  KeyEvent {code: KeyCode::Char('s'), .. } = key_ev {
            cursor.pos = *player_pos;
            cursor.visible  = true;
        }
        return;
    }

    match key_ev {
        KeyEvent {code: KeyCode::Left, .. } => {
            cursor.pos.x -= 1.0;
        }
        KeyEvent {code: KeyCode::Right, .. } => {
            cursor.pos.x += 1.0;
        }
        KeyEvent {code: KeyCode::Up, .. } => {
            cursor.pos.y -= 1.0;
        }
        KeyEvent {code: KeyCode::Down, .. } => {
            cursor.pos.y += 1.0;
        }
        KeyEvent {code: KeyCode::Char('a'), .. } => {
            unimplemented!("attack");
        }
        _ => return,
    }
}

pub fn add_player_systems(builder: &mut Builder) {
    builder
        .add_system(move_player_system())
        .add_system(track_player_system())
        .add_system(move_cursor_system());
}
