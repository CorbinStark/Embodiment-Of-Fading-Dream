use crate::*;

pub struct MainMenu {
    options: Vec<String>,
    current: i16,
    mask: Texture2D,
    bg: Texture2D,
    bg_pos: Vector2,
    timer: f64,
}

impl State for MainMenu {
    fn enter(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {
        self.current = -1;
    }

    fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> usize {
        //USER INPUT
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            self.current += 1;
            if self.current > self.options.len() as i16 - 1 {
                self.current = 0;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            if self.current <= 0 {
                self.current = self.options.len() as i16;
            }
            self.current -= 1;
        }
        //Select current menu option
        if rl.is_key_released(KeyboardKey::KEY_ENTER) {
            //If selected play game
            if self.current == 0 {
                return 2;
            }
            //If selected map editor
            if self.current == 1 {
                return 3;
            }
            //If selected options
            if self.current == 2 {
                return 4;
            }
            //If selected quit
            if self.current == 3 {
                std::process::exit(0);
            }
        }

        self.timer += 0.01;
        self.bg_pos.x += (f64::sin(self.timer) as f32) * 0.25;
        self.bg_pos.y += f64::cos(self.timer) as f32;

        //DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(
            &mut self.bg,
            self.bg_pos.x as i32,
            self.bg_pos.y as i32,
            Color::WHITE,
        );
        d.draw_texture(&mut self.mask, 0, 0, Color::WHITE);
        d.draw_text("Use arrow keys to change", 450, 425, 14, Color::WHITE);
        d.draw_text("selection, enter to select", 450, 445, 14, Color::WHITE);
        d.draw_text("Embodiment of Fading", 50, 15, 55, Color::WHITE);
        d.draw_text("Dream", 50, 75, 55, Color::WHITE);

        for i in 0..self.options.len() {
            if i as i16 == self.current {
                d.draw_text(
                    &self.options[i][..],
                    55,
                    (i as i32 * 70) + 220,
                    34,
                    Color::RED,
                );
            } else {
                d.draw_text(
                    &self.options[i][..],
                    55,
                    (i as i32 * 70) + 220,
                    34,
                    Color::WHITE,
                );
            }
        }

        //Return state change = false
        NO_STATE_CHANGE
    }

    fn leave(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {}
}

impl MainMenu {
    pub fn new(rl: &mut RaylibHandle, thread: &mut RaylibThread) -> Self {
        MainMenu {
            mask: rl.load_texture(thread, "art/mask.png").unwrap(),
            bg: rl.load_texture(thread, "art/bg.png").unwrap(),
            options: vec![
                "Start Game".to_string(),
                "Map Editor".to_string(),
                "Options".to_string(),
                "Quit Game".to_string(),
            ],
            current: 0,
            bg_pos: Vector2 { x: 120.0, y: 0.0 },
            timer: 0.0,
        }
    }
}
