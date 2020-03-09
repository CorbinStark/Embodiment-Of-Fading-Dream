use crate::*;

const TILE_SIZE: i32 = 16;
const SCALE: i32 = 3;

const IDLE_STATE: i32 = 0;
const MOVE_STATE: i32 = 1;
const ATTACK_STATE: i32 = 2;
//const WAITING_STATE: i32 = 3;
//const MENU_STATE: i32 = 4;

pub struct Game {
    map: Map,
    tiles: Vec<(i32, i32)>,
    state: i32,
    units: Vec<Unit>,
    sprites: Vec<Texture2D>,
    selected_unit: usize, //index of currently selected unit in map.units
                          //selected_unit: *mut Unit, //mutable pointer to the currently selected unit in the units list
}

fn move_heuristic(id: i32) -> i32 {
    if id == 0 {
        return -1;
    }
    1 //1 is default cost if not defined
}

fn attack_heuristic(id: i32) -> i32 {
    if id == 0 {
        return -1;
    }
    1 //1 is default cost if not defined
}

//Was very iffy about changing this but things still work. Apparently this will allow it to work with non Vec-based slices according to clippy
fn draw_tiles(d: &mut RaylibDrawHandle, tiles: &[(i32, i32)]) {
    //changed from being &Vec<(i32, i32)>
    for tuple in tiles {
        d.draw_rectangle(
            tuple.0 * TILE_SIZE * SCALE,
            tuple.1 * TILE_SIZE * SCALE,
            TILE_SIZE * SCALE,
            TILE_SIZE * SCALE,
            Color::from((100, 100, 255, 100)),
        );
    }
}
#[allow(clippy::collapsible_if)]
impl State for Game {
    fn enter(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {
        self.tiles = floodfill(&self.map, (3, 3), 4, move_heuristic);
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
            //Select a friendly unit
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                //if mouse position is on top of a unit
                for i in 0..self.units.len() {
                    let unit = &self.units[i];
                    if unit.player_owned && unit.ismoused(mouse, TILE_SIZE as f32, SCALE as f32) {
                        //  if unit.ismoused(mouse, TILE_SIZE as f32, SCALE as f32) { //Collapsed the statement since it was giving warnings, can undo if neccesary.
                        self.state = MOVE_STATE;
                        self.selected_unit = i;
                        self.tiles = floodfill(
                            &self.map,
                            (unit.x / TILE_SIZE, unit.y / TILE_SIZE),
                            unit.moverange,
                            move_heuristic,
                        );
                    }
                    // }
                }
            }
        }
        if self.state == MOVE_STATE {
            for tuple in &self.tiles.clone() {
                //if mouse over tile
                if mouse.x > tuple.0 as f32 + self.map.x as f32
                    && mouse.y > tuple.1 as f32 + self.map.y as f32
                    && mouse.x < tuple.0 as f32 + self.map.x as f32 + (TILE_SIZE * SCALE) as f32
                    && mouse.y < tuple.1 as f32 + self.map.y as f32 + (TILE_SIZE * SCALE) as f32
                {
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                        self.units[self.selected_unit].x = tuple.0;
                        self.units[self.selected_unit].y = tuple.1;
                        self.state = ATTACK_STATE;
                        self.tiles = floodfill(
                            &self.map,
                            (
                                self.units[self.selected_unit].x / TILE_SIZE,
                                self.units[self.selected_unit].y / TILE_SIZE,
                            ),
                            self.units[self.selected_unit].attackrange,
                            attack_heuristic,
                        );
                    }
                }
            }
        }
        if self.state == ATTACK_STATE {
            for tuple in &self.tiles {
                if mouse.x > tuple.0 as f32 + self.map.x as f32
                    && mouse.y > tuple.1 as f32 + self.map.y as f32
                    && mouse.x < tuple.0 as f32 + self.map.x as f32 + (TILE_SIZE * SCALE) as f32
                    && mouse.y < tuple.1 as f32 + self.map.y as f32 + (TILE_SIZE * SCALE) as f32
                {
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                        //do attack
                        let mut enemy = self.units[0].clone(); // &mut self.units[0];
                        let range = self.units[self.selected_unit].attackrange;
                        for u in &self.units {
                            if !u.player_owned
                                && (u.x / TILE_SIZE) == tuple.0
                                && (u.y / TILE_SIZE) == tuple.1
                            {
                                enemy = u.clone();
                            }
                        }
                        combat(&mut self.units[self.selected_unit], &mut enemy, range);
                        //self.units[self.selected_unit].attackrange
                    }
                }
            }
        }

        //DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        self.map.draw(&mut d);
        for unit in &self.units {
            unit.draw(&mut d, &self.sprites);
        }
        draw_tiles(&mut d, &self.tiles);
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
            units: vec![],
            sprites: vec![],
            selected_unit: 0,
        }
    }
}
