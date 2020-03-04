use crate::*;

const TILE_SIZE: i32 = 16;

pub struct Game {
    map: Map,
    tiles: Vec<(i32, i32)>,
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
        self.tiles = floodfill(&self.map, (8, 15), 4, move_heuristic);
    }

    fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> usize {
        //USER INPUT
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            //Go back to main menu on escape
            return 1;
        }

        //DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        self.map.draw(&mut d);
        draw_tiles(&mut d, &self.tiles);
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
        }
    }
}
