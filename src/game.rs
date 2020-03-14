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
    selected_unit: usize,      //index of currently selected unit in map.units
    prev_position: (i32, i32), //selected_unit: *mut Unit, //mutable pointer to the currently selected unit in the units list
}

fn move_heuristic(id: i32) -> i32 {
    //return 1 for ground tiles
    if id >= 21 && id <= 24 {
        return 1;
    }
    if id >= 31 && id <= 34 {
        return 1;
    }
    //return -1 for wall tiles (quite a few of them lol)
    if id >= 0 && id <= 5 {
        return -1;
    }
    if id >= 10 && id <= 15 {
        return -1;
    }
    if id >= 20 && id <= 25 {
        return -1;
    }
    if id >= 30 && id <= 35 {
        return -1;
    }
    if id >= 40 && id <= 45 {
        return -1;
    }
    if id >= 50 && id <= 55 {
        return -1;
    }
    if id >= 60 && id <= 65 {
        return -1;
    }
    //return -1 for misc items like chests
    if id == 90 {
        return -1;
    }

    //return 1 if anything else
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
            (tuple.0 as f32 * TILE_SIZE as f32 * SCALE) as i32 + 2,
            (tuple.1 as f32 * TILE_SIZE as f32 * SCALE) as i32 + 2,
            (TILE_SIZE as f32 * SCALE) as i32 - 2,
            (TILE_SIZE as f32 * SCALE) as i32 - 2,
            color,
        );
    }
}

#[allow(clippy::explicit_counter_loop)]
impl State for Game {
    fn enter(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {
        self.map.load().expect("load failed");
    }
    #[allow(clippy::cognitive_complexity)]
    fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> usize {
        self.timer += 1;
        //USER INPUT
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            //Go back to main menu on escape
            return 1;
        }
        let mouse = rl.get_mouse_position();
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {}

        if self.nextstate != -1 && rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            self.state = self.nextstate;
            self.nextstate = -1;
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
        if self.state == WAITING_STATE {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
                self.nextstate = MENU_STATE;
                self.state = self.nextstate;
            }
            self.nextstate = IDLE_STATE;
        }

        if self.state == MOVE_STATE && rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            for tuple in &self.tiles.clone() {
                //if mouse over tile
                let tile_x = tuple.0 as f32 * TILE_SCALED;
                let tile_y = tuple.1 as f32 * TILE_SCALED;

                if mouse.x > tile_x
                    && mouse.y > tile_y
                    && mouse.x < tile_x + TILE_SCALED
                    && mouse.y < tile_y + TILE_SCALED
                {
                    self.prev_position.0 = self.units[self.selected_unit].x;
                    self.prev_position.1 = self.units[self.selected_unit].y;
                    self.units[self.selected_unit].x = tile_x as i32;
                    self.units[self.selected_unit].y = tile_y as i32;
                    self.nextstate = MENU_STATE;
                    self.tiles = floodfill(
                        &self.map,
                        (
                            self.units[self.selected_unit].x / (TILE_SIZE as f32 * SCALE) as i32,
                            self.units[self.selected_unit].y / (TILE_SIZE as f32 * SCALE) as i32,
                        ),
                        self.units[self.selected_unit].attackrange,
                        attack_heuristic,
                    );
                }
            }
        }

        if self.state == ATTACK_STATE {
            //back to previous state
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
                self.nextstate = MENU_STATE;
                self.state = self.nextstate;
            }
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                //do attack
                let mut selected_enemy: i32 = -1;
                let mut i: i32 = 0;
                for u in &self.enemies {
                    //for u in &self.enemies {
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
                if selected_enemy > -1 {
                    let player = &self.units[self.selected_unit];
                    let damage = player.get_damage();
                    self.enemies[selected_enemy as usize - 1].health -= damage;
                }
                self.nextstate = IDLE_STATE;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F2) {
            return 1;
        }
        let mut attack_moused = false;
        let mut wait_moused = false;
        if self.state == MENU_STATE {
            //if player chooses attack action, then self.nextstate = ATTACK_STATE;
            //if player chooses wait action, then self.nextstate = WAITING_STATE;

            //go back to previous state
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
                self.nextstate = IDLE_STATE;
                self.units[self.selected_unit].x = self.prev_position.0;
                self.units[self.selected_unit].y = self.prev_position.1;
                self.state = self.nextstate;
            }
            let attack_button = Rectangle::new(
                (self.units[self.selected_unit].x + 65) as f32,
                (self.units[self.selected_unit].y - 20) as f32,
                75.0,
                20.0,
            );
            let wait_button = Rectangle::new(
                (self.units[self.selected_unit].x + 65) as f32,
                (self.units[self.selected_unit].y + 10) as f32,
                75.0,
                20.0,
            );

            if mouse.x > attack_button.x
                && mouse.y > attack_button.y
                && mouse.x < attack_button.x + attack_button.width
                && mouse.y < attack_button.y + attack_button.height
            {
                attack_moused = true;
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    self.nextstate = ATTACK_STATE;
                    self.state = self.nextstate;
                }
            } else {
                attack_moused = false;
            }
            if mouse.x > wait_button.x
                && mouse.y > wait_button.y
                && mouse.x < wait_button.x + wait_button.width
                && mouse.y < wait_button.y + wait_button.height
            {
                wait_moused = true;
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    self.nextstate = WAITING_STATE;
                    self.state = self.nextstate;
                }
            } else {
                wait_moused = false;
            }
        }

        //DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        self.map.draw(&mut d);
        for unit in &mut self.units {
            unit.draw(&mut d, &self.sprites, self.timer);
            d.draw_rectangle(
                unit.x,
                unit.y - 15,
                TILE_SCALED as i32 - 5,
                15,
                Color::BLACK,
            );
            d.draw_rectangle(
                unit.x + 2,
                unit.y - 13,
                (unit.health / unit.maxhealth) * (TILE_SCALED as i32 - 8),
                12,
                Color::GREEN,
            );
        }
        for unit in &mut self.enemies {
            unit.draw(&mut d, &self.sprites, self.timer);
            d.draw_rectangle(
                unit.x,
                unit.y - 15,
                TILE_SCALED as i32 - 5,
                15,
                Color::BLACK,
            );
            d.draw_rectangle(
                unit.x + 2,
                unit.y - 13,
                (unit.health / unit.maxhealth) * (TILE_SCALED as i32 - 8),
                12,
                Color::RED,
            );
        }

        if self.state == MOVE_STATE {
            draw_tiles(&mut d, &self.tiles, Color::from((100, 100, 255, 100)));
        }
        if self.state == ATTACK_STATE {
            draw_tiles(&mut d, &self.tiles, Color::from((255, 100, 100, 100)));
        }
        d.draw_fps(20, 20);
        d.draw_text(&"F2: Menu".to_string(), 220, 450, 15, Color::WHITE);
        d.draw_text(
            &"Esc: Exit to Windows".to_string(),
            300,
            450,
            15,
            Color::WHITE,
        );
        if self.state == MENU_STATE {
            d.draw_rectangle(
                self.units[self.selected_unit].x + 50,
                self.units[self.selected_unit].y - 40,
                100,
                160,
                Color::WHITE,
            );
            d.draw_rectangle_lines(
                self.units[self.selected_unit].x + 60,
                self.units[self.selected_unit].y - 30,
                80,
                140,
                Color::BLACK,
            );
            if attack_moused {
                d.draw_text(
                    &"Attack".to_string(),
                    self.units[self.selected_unit].x + 65,
                    self.units[self.selected_unit].y - 20,
                    20,
                    Color::RED,
                );
            } else {
                d.draw_text(
                    &"Attack".to_string(),
                    self.units[self.selected_unit].x + 65,
                    self.units[self.selected_unit].y - 20,
                    20,
                    Color::BLACK,
                );
            }
            if wait_moused {
                d.draw_text(
                    &"Wait".to_string(),
                    self.units[self.selected_unit].x + 65,
                    self.units[self.selected_unit].y + 10,
                    20,
                    Color::RED,
                );
            } else {
                d.draw_text(
                    &"Wait".to_string(),
                    self.units[self.selected_unit].x + 65,
                    self.units[self.selected_unit].y + 10,
                    20,
                    Color::BLACK,
                );
            }
        }
        //Return state change = false
        NO_STATE_CHANGE
    }

    fn leave(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {}
}

impl Game {
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
            enemies,
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
            prev_position: (0, 0),
        }
    }
}
