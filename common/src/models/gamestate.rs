use serde::{Deserialize, Serialize};
use tinybit::WorldPos;

use super::TileData;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub tilemap: Vec<TileData>,
    pub player_pos: WorldPos,
}
