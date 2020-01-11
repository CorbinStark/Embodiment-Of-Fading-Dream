use crate::*;

pub struct Game {
    block: Model,
    camera: Camera,
    cube_pos: Vector3,
    cube_screen_pos: Vector2,
}

impl State for Game {
    fn enter(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) {
        rl.set_camera_mode(self.camera, CameraMode::CAMERA_ORBITAL);
        let materials = self.block.materials_mut();
        let material = &mut materials[0];
        let mut maps = material.maps_mut();

        let texture = unsafe {
            let mut t = rl.load_texture(&thread, "art/test.png").unwrap();
            t.gen_texture_mipmaps();
            t.set_texture_filter(TextureFilterMode::FILTER_POINT);
            t.unwrap()
        };

        maps[MaterialMapType::MAP_ALBEDO as usize].texture = texture;
    }

    fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> StateChange {
        rl.update_camera(&mut self.camera);
        //USER INPUT
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            //Go back to main menu on escape
            return StateChange {
                should_change: true,
                change_to: 0,
            };
        }

        //DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        self.cube_screen_pos = d.get_world_to_screen(
            Vector3 {
                x: self.cube_pos.x,
                y: self.cube_pos.y + 2.5,
                z: self.cube_pos.z,
            },
            self.camera,
        );
        {
            let mut d2 = d.begin_mode_3D(self.camera);
            d2.draw_model_ex(
                &mut self.block,
                self.cube_pos,
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                -90.0,
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                Color::WHITE,
            );
            d2.draw_cube_wires(self.cube_pos, 2.2, 2.2, 2.2, Color::MAROON);
            d2.draw_grid(20, 1.0);
        }
        d.draw_text(
            "Enemy: 100/100",
            (self.cube_screen_pos.x - (measure_text("Enemy: 100/100", 20) / 2) as f32) as i32,
            self.cube_screen_pos.y as i32,
            20,
            Color::BLACK,
        );
        //d.draw_texture(&mut self.test, 50, 50, Color::WHITE);
        //d.draw_text("This is the game state!", 120, 420, 40, Color::BLACK);
        d.draw_fps(20, 20);

        //Return state change = false
        StateChange {
            should_change: false,
            change_to: 0,
        }
    }

    fn leave(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) {}
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, thread: &mut RaylibThread) -> Self {
        Game {
            block: rl.load_model(thread, "models/cube.obj").unwrap(),
            camera: Camera::perspective(
                //Position
                Vector3 {
                    x: 100.0,
                    y: 100.0,
                    z: 100.0,
                },
                //Target
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                //Up
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                5.0,
            ),
            cube_pos: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            cube_screen_pos: Vector2 { x: 0.0, y: 0.0 },
        }
    }
}
