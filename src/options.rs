use crate::*;

pub struct Options {
    options: Vec<String>,
    current: i16,
    mask: Texture2D,
    bg: Texture2D,
    bg_pos: Vector2,
    timer: f64,
}

impl State for Options {
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
        //Select current options menu
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            //If selected 640x480 resolution
            if self.current == 0 {
                rl.set_window_size(640, 480);
                // return 2;
            }
            //If selected 800x600 resolution
            if self.current == 1 {
                rl.set_window_size(800, 600);
                // return 3;
            }
            //If 960x720 resolution
            if self.current == 2 {
                rl.set_window_size(960, 720);
                // return 4;
            }
            //If selected return to menu
            if self.current == 3 {
                return 1;
                //std::process::exit(0);
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

impl Options {
    pub fn new(rl: &mut RaylibHandle, thread: &mut RaylibThread) -> Self {
        Options {
            mask: rl.load_texture(thread, "art/mask.png").unwrap(),
            bg: rl.load_texture(thread, "art/bg.png").unwrap(),
            options: vec![
                "640x480".to_string(),
                "800x600".to_string(),
                "960x720".to_string(),
                "Return to menu".to_string(),
            ],
            current: 0,
            bg_pos: Vector2 { x: 120.0, y: 0.0 },
            timer: 0.0,
        }
    }
}
