struct Game {
    tilemap: TileMap, 
    input_map: InputMap,
}

impl Scene for Game {
    fn tick(&mut self) {
        // Draw player position etc.
    }

    fn input(&mut self, input: InputEvent) {
        match self.input_map.map(input) {
            Input::Activate => {
                // Get the object under the cursor
                // if no cursor present: under player
                let cursor_pos = tilemap.get_cursor();
                let tile = tilemap.get_tile(cursor_pos);
                if let Some(menu) = get_context_menu(tile) {
                }
            }
            Input::Move(direction) => {
                if self.cursor {
                    self.move_cursor(direction);
                } else {
                    self.move_player(direction);
                }
            }
            _ => {
            }
        }
        // Move player unit and send coords to server
    }

    fn render(&mut self, renderer: Renderer) {
        // draw all units on screen 
    }

    fn transition(&mut self) -> Option<impl Scene> {
        None
    }
}
