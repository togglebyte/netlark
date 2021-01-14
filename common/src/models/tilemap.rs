use serde::{Deserialize, Serialize};
use tinybit::WorldPos;

#[derive(Debug, Serialize, Deserialize)]
pub struct TileData {
    pub coords: WorldPos,
    pub glyph: char,
}

impl TileData {
    pub fn new(coords: WorldPos, glyph: char) -> Self {
        Self {
            coords,
            glyph
        }
    }
}
