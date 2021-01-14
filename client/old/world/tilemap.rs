use tinybit::{Color, ScreenSize, WorldPos};

pub type TilemapMeh = Tilemap<ThrowAwayThisProvider>;

pub struct ThrowAwayThisProvider;

pub(super) fn make_rubbish_tilemap() -> TilemapMeh {
    Tilemap::new(ThrowAwayThisProvider)
}

impl MapSource for ThrowAwayThisProvider {
    fn get_region(&self, pos: WorldPos, size: ScreenSize) -> Vec<Tile> {
        let mut tiles = Vec::new();

        let start_x = (pos.x - size.width as f32 / 2.0) as isize;
        let start_y = (pos.y - size.height as f32 / 2.0) as isize;
        let end_x = (pos.x + size.width as f32 / 2.0) as isize;
        let end_y = (pos.y + size.height as f32 / 2.0) as isize;

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                tiles.push(Tile::new(
                    '.',
                    WorldPos::new(x as f32, y as f32),
                    None,
                    None,
                ));
            }
        }

        tiles
    }
}

#[derive(Debug, PartialEq)]
pub struct Tile {
    pub glyph: char,
    pub pos: WorldPos,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl Tile {
    pub fn new(
        glyph: char,
        pos: WorldPos,
        fg_color: Option<Color>,
        bg_color: Option<Color>,
    ) -> Self {
        Self {
            glyph,
            pos,
            fg_color,
            bg_color,
        }
    }
}

pub trait MapSource {
    fn get_region(&self, pos: WorldPos, size: ScreenSize) -> Vec<Tile>;
}

pub struct Tilemap<T> {
    pub tiles: Vec<Tile>,
    data_source: T,
}

impl<T: MapSource> Tilemap<T> {
    pub fn new(data_source: T) -> Self {
        Self {
            tiles: Vec::new(),
            data_source,
        }
    }

    pub fn update(&mut self, pos: WorldPos, size: ScreenSize) {
        self.tiles = self.data_source.get_region(pos, size);
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    // use tinybit::{Pixel, ScreenSize, WorldPos};

    // struct TestProvider;

    // impl MapSource for TestProvider {
    //     fn get_region(&self, pos: WorldPos, size: ScreenSize) -> Vec<Tile> {
    //         let mut tiles = Vec::new();
    //         let start_x = pos.x - size.width as isize / 2;
    //         let start_y = pos.x - size.width as isize / 2;
    //         let end_x = pos.x + size.width as isize / 2;
    //         let end_y = pos.x + size.width as isize / 2;

    //         for x in start_x..=end_x {
    //             for y in start_y..=end_y {
    //                 tiles.push(Tile::new('#', WorldPos::new(x, y), None));
    //             }
    //         }

    //         tiles
    //     }
    // }

    // #[test]
    // fn get_map_data() {
    //     let mut map = Tilemap::new(TestProvider);
    //     let region = map
    //         .data_source
    //         .get_region(WorldPos::zero(), ScreenSize::new(10, 10));

    //     assert_eq!(region[0], Tile::new('#', WorldPos::new(-5, -5), None));
    // }
}
