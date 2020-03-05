use crate::*;

const TILE_SIZE: i32 = 16;
const SCALE: i32 = 3;

const IDLE_STATE: i32 = 0;
const MOVE_STATE: i32 = 1;
const ATTACK_STATE: i32 = 2;
const WAITING_STATE: i32 = 3;
const MENU_STATE: i32 = 4;

pub struct Game {
    map: Map,
    tiles: Vec<(i32, i32)>,
    state: i32,
}

fn move_heuristic(id: i32) -> i32 {
    if id == 0 {
        return -1; //-1 = wall
    }
    1 //1 is default cost if not defined
}

//Was very iffy about changing this but things still work. Apparently this will allow it to work with non Vec-based slices according to clippy
fn draw_tiles(d: &mut RaylibDrawHandle, tiles: &[(i32, i32)]) {
    //changed from being &Vec<(i32, i32)>
    for tuple in tiles {
        d.draw_rectangle(
            tuple.0 * TILE_SIZE,
            tuple.1 * TILE_SIZE,
            TILE_SIZE,
            TILE_SIZE,
            Color::from((100, 100, 255, 100)),
        );
    }
}

impl State for Game {
    fn enter(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {
        self.tiles = floodfill(&self.map, (10, 10), 11, move_heuristic);
    }

    fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> usize {
        //USER INPUT
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            //Go back to main menu on escape
            return 1;
        }
        let mouse = rl.get_mouse_position();
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {}

        if self.state == IDLE_STATE {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            //if mouse position is on top of a unit
            for unit in &self.map.units {
                if unit.ismoused(mouse, TILE_SIZE as f32, SCALE as f32) {
                    
                }
            }
            //if unit isnt player owned
            //if unit is on a valid attack tile
            //attack it!
        }
    }

        //DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        self.map.draw(&mut d);
        if self.state == MOVE_STATE {
            draw_tiles(&mut d, &self.tiles);
        }
        d.draw_fps(20, 20);

        //Return state change = false
        NO_STATE_CHANGE
    }

    fn leave(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {}
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, thread: &mut RaylibThread) -> Self {
        Game {
            map: Map::new(25, 25, rl, thread),
            tiles: vec![],
            state: IDLE_STATE,
        }
    }
}
