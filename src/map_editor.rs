use crate::*;

pub struct MapEditor {
    test: Texture2D,
    camera: Camera,
}

impl State for MapEditor {
    fn enter(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) {
        rl.set_camera_mode(self.camera, CameraMode::CAMERA_FREE);
    }

    fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> StateChange {
        rl.update_camera(&mut self.camera);
        //USER INPUT
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {}

        //DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        {
            let mut d2 = d.begin_mode_3D(self.camera);
            d2.draw_grid(20, 1.0);
        }
        d.draw_fps(20, 20);

        //Return state change = false
        StateChange {
            should_change: false,
            change_to: 0,
        }
    }

    fn leave(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) {}
}

impl MapEditor {
    pub fn new(rl: &mut RaylibHandle, thread: &mut RaylibThread) -> Self {
        MapEditor {
            test: rl.load_texture(thread, "art/test.png").unwrap(),
            camera: Camera::perspective(
                //Position
                Vector3 {
                    x: 10.0,
                    y: 10.0,
                    z: 10.0,
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
                45.0,
            ),
        }
    }
}
