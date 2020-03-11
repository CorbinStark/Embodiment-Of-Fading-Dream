use crate::*;

const IDLE_STATE: i32 = 0;
const MOVE_STATE: i32 = 1;
const ATTACK_STATE: i32 = 2;
const MENU_STATE: i32 = 3;
const WAITING_STATE: i32 = 4;

pub struct Game {
    map: Map,
    tiles: Vec<(i32, i32)>,
    state: i32,
    nextstate: i32,
    units: Vec<Unit>,
    enemies: Vec<Unit>,
    sprites: Vec<Texture2D>,
    timer: i32,
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
fn draw_tiles(d: &mut RaylibDrawHandle, tiles: &[(i32, i32)], color: Color) {
    //changed from being &Vec<(i32, i32)>
    for tuple in tiles {
        d.draw_rectangle(
            (tuple.0 as f32 * TILE_SIZE as f32 * SCALE) as i32,
            (tuple.1 as f32 * TILE_SIZE as f32 * SCALE) as i32,
            (TILE_SIZE as f32 * SCALE) as i32,
            (TILE_SIZE as f32 * SCALE) as i32,
            color,
        );
    }
}
#[allow(clippy::collapsible_if)]
impl State for Game {
    fn enter(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {
    }

    fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> usize {
        self.timer += 1;
        //USER INPUT
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            //Go back to main menu on escape
            return 1;
        }
        let mouse = rl.get_mouse_position();
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {}

        if self.nextstate != -1 {
            if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
                self.state = self.nextstate;
                self.nextstate = -1;
            }
        }

        if self.state == IDLE_STATE {
            //Select a friendly unit
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                //if mouse position is on top of a unit
                for i in 0..self.units.len() {
                    let unit = &self.units[i];
                    if unit.ismoused(mouse, TILE_SIZE as f32, SCALE as f32) {
                        //  if unit.ismoused(mouse, TILE_SIZE as f32, SCALE as f32) { //Collapsed the statement since it was giving warnings, can undo if neccesary.
                        self.nextstate = MOVE_STATE;
                        self.selected_unit = i;
                        self.tiles.clear();
                        self.tiles = floodfill(
                            &self.map,
                            (
                                unit.x / (TILE_SIZE as f32 * SCALE) as i32,
                                unit.y / (TILE_SIZE as f32 * SCALE) as i32,
                            ),
                            unit.moverange,
                            move_heuristic,
                        );
                    }
                    // }
                }
            }
        }

        if self.state == MOVE_STATE {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                for tuple in &self.tiles.clone() {
                    //if mouse over tile
                    let tile_x = tuple.0 as f32 * TILE_SCALED;
                    let tile_y = tuple.1 as f32 * TILE_SCALED;

                    if mouse.x > tile_x && mouse.y > tile_y && mouse.x < tile_x + TILE_SCALED && mouse.y < tile_y + TILE_SCALED
                    {
                        self.units[self.selected_unit].x = tile_x as i32;
                        self.units[self.selected_unit].y = tile_y as i32;
                        self.nextstate = ATTACK_STATE;
                        self.tiles = floodfill(
                            &self.map,
                            (
                                self.units[self.selected_unit].x
                                    / (TILE_SIZE as f32 * SCALE) as i32,
                                self.units[self.selected_unit].y
                                    / (TILE_SIZE as f32 * SCALE) as i32,
                            ),
                            self.units[self.selected_unit].attackrange,
                            attack_heuristic,
                        );
                    }
                }
            }
        }

        if self.state == ATTACK_STATE {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                //do attack
                let mut selected_enemy: i32 = -1;
                let mut i: i32 = 0;
                for u in &self.enemies {
                    i += 1;
                    if mouse.x > u.x as f32 + self.map.x as f32
                        && mouse.y > u.y as f32 + self.map.y as f32
                        && mouse.x
                            < u.x as f32 + self.map.x as f32 + (TILE_SIZE as f32 * SCALE) as f32
                        && mouse.y
                            < u.y as f32 + self.map.y as f32 + (TILE_SIZE as f32 * SCALE) as f32
                    {
                        selected_enemy = i;
                    }
                }
                if selected_enemy > 0 {
                    let player = &self.units[self.selected_unit];
                    let damage = player.get_damage();
                    self.enemies[selected_enemy as usize].health -= damage;
                }
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F2) {
            return 1;
        }

        if self.state == MENU_STATE {
            //TODO
            //if player chooses atttack action, then self.nextstate = ATTACK_STATE;
            //if player chooses wait action, then self.nextstate = WAITING_STATE;
            //then add some other actions if you wish
        }

        //DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        self.map.draw(&mut d);
        for unit in &mut self.units {
            unit.draw(&mut d, &self.sprites, self.timer);
        }

        if self.state == MOVE_STATE {
            draw_tiles(&mut d, &self.tiles, Color::from((100, 100, 255, 100)));
        }
        if self.state == ATTACK_STATE {
            draw_tiles(&mut d, &self.tiles, Color::from((255, 100, 100, 100)));
        }
        d.draw_fps(20, 20);
        d.draw_text(&"F2: Menu".to_string(),
        220,
        450,
        15,
        Color::WHITE,
        );
        d.draw_text(&"Esc: Exit to Windows".to_string(),
        300,
        450,
        15,
        Color::WHITE,
        );
        


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
            nextstate: -1,
            units: vec![],
            enemies: vec![],
            sprites: vec![
                rl.load_texture(thread, "art/skeleton_v2_1.png").unwrap(),
                rl.load_texture(thread, "art/skeleton_v2_2.png").unwrap(),
                rl.load_texture(thread, "art/skeleton_v2_3.png").unwrap(),
                rl.load_texture(thread, "art/skeleton_v2_4.png").unwrap(),
                rl.load_texture(thread, "art/skeleton2_v2_1.png").unwrap(),
                rl.load_texture(thread, "art/skeleton2_v2_2.png").unwrap(),
                rl.load_texture(thread, "art/skeleton2_v2_3.png").unwrap(),
                rl.load_texture(thread, "art/skeleton2_v2_4.png").unwrap(),
                rl.load_texture(thread, "art/skull_v2_1.png").unwrap(),
                rl.load_texture(thread, "art/skull_v2_2.png").unwrap(),
                rl.load_texture(thread, "art/skull_v2_3.png").unwrap(),
                rl.load_texture(thread, "art/skull_v2_4.png").unwrap(),
                rl.load_texture(thread, "art/vampire_v2_1.png").unwrap(),
                rl.load_texture(thread, "art/vampire_v2_2.png").unwrap(),
                rl.load_texture(thread, "art/vampire_v2_3.png").unwrap(),
                rl.load_texture(thread, "art/vampire_v2_4.png").unwrap(),
            ],
            timer: 0,
            selected_unit: 0,
        }
    }
    pub fn from_unit_population(
        rl: &mut RaylibHandle,
        thread: &mut RaylibThread,
        friendlies: Vec<Unit>,
        enemies: Vec<Unit>,
    ) -> Self {
        Game {
            map: Map::new(25, 25, rl, thread),
            tiles: vec![],
            state: IDLE_STATE,
            nextstate: -1,
            units: friendlies,
            enemies: enemies,
            sprites: vec![
                rl.load_texture(thread, "art/skeleton_v2_1.png").unwrap(),
                rl.load_texture(thread, "art/skeleton_v2_2.png").unwrap(),
                rl.load_texture(thread, "art/skeleton_v2_3.png").unwrap(),
                rl.load_texture(thread, "art/skeleton_v2_4.png").unwrap(),
                rl.load_texture(thread, "art/skeleton2_v2_1.png").unwrap(),
                rl.load_texture(thread, "art/skeleton2_v2_2.png").unwrap(),
                rl.load_texture(thread, "art/skeleton2_v2_3.png").unwrap(),
                rl.load_texture(thread, "art/skeleton2_v2_4.png").unwrap(),
                rl.load_texture(thread, "art/skull_v2_1.png").unwrap(),
                rl.load_texture(thread, "art/skull_v2_2.png").unwrap(),
                rl.load_texture(thread, "art/skull_v2_3.png").unwrap(),
                rl.load_texture(thread, "art/skull_v2_4.png").unwrap(),
                rl.load_texture(thread, "art/vampire_v2_1.png").unwrap(),
                rl.load_texture(thread, "art/vampire_v2_2.png").unwrap(),
                rl.load_texture(thread, "art/vampire_v2_3.png").unwrap(),
                rl.load_texture(thread, "art/vampire_v2_4.png").unwrap(),
            ],
            timer: 0,
            selected_unit: 0,
        }
    }
}
