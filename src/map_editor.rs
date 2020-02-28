use crate::*;

const PLACE: u8 = 0;
const TILES: u8 = 1;
const UNITS: u8 = 2;
const TILE_SIZE: f32 = 16.0;

pub struct MapEditor {
    map: Map,
    state: u8,
    selected_tile: i32,
}

impl State for MapEditor {
    fn enter(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {}

    fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> usize {
        //USER INPUT
        let mouse = rl.get_mouse_position();
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {}
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse = rl.get_mouse_position();
            //if mouse position is on top of a unit
                //if unit isnt player owned
                    //if unit is on a valid attack tile
                        //attack it!
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F1) {
            if self.state == TILES {
                self.state = PLACE;
            } else {
                self.state = TILES;
            }
        }

        let mut clicked_tileset:  bool = false;
        let mut hovering_tileset: bool = false;
        let clicked: bool = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);

        if self.state == TILES {
            if mouse.x > 0.0 && mouse.y > 0.0 && mouse.x < self.map.tiles.width as f32 && mouse.y < self.map.tiles.height as f32 {
                hovering_tileset = true;
            }
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) && hovering_tileset {
                //if mouse over tilesheet image
                clicked_tileset = true;
            }
        }

        //DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        self.map.draw(&mut d);
        d.draw_fps(20, 20);
        if self.state == TILES {
            d.draw_texture(&self.map.tiles, 0, 0, Color::WHITE);
            let tile_x = (mouse.x / TILE_SIZE) as i32;
            let tile_y = (mouse.y / TILE_SIZE) as i32;
            if hovering_tileset {
                d.draw_rectangle(tile_x * TILE_SIZE as i32, tile_y * TILE_SIZE as i32, TILE_SIZE as i32, TILE_SIZE as i32, Color::GREEN);
            }
            if clicked_tileset {
                self.selected_tile = tile_x + tile_y * 10;
            }
            let src_x: f32 = (self.selected_tile % 10) as f32 * TILE_SIZE;
            let src_y: f32 = (self.selected_tile / 10) as f32 * TILE_SIZE;
            d.draw_rectangle(src_x as i32, src_y as i32, TILE_SIZE as i32, TILE_SIZE as i32, Color::RED);
        } else if self.state == PLACE {
            let tile_x = (mouse.x / TILE_SIZE) as i32;
            let tile_y = (mouse.y / TILE_SIZE) as i32;
            d.draw_rectangle(tile_x * TILE_SIZE as i32, tile_y * TILE_SIZE as i32, TILE_SIZE as i32, TILE_SIZE as i32, Color::GREEN);
            if clicked {
                //place tile
                self.map.grid[tile_x as usize][tile_y as usize] = self.selected_tile;
            }
        }

        //Return state change = false
        NO_STATE_CHANGE
    }
    
    fn leave(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {}
}

impl MapEditor {
    pub fn new(rl: &mut RaylibHandle, thread: &mut RaylibThread) -> Self {
        MapEditor {
            map: Map::new(25, 25, rl, thread),
            state: PLACE,
            selected_tile: 0,
        }
    }
}