use crate::*;

pub struct MapEditor {
    map: Map,
}

impl State for MapEditor {
    fn enter(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {
    }

    fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> usize {
        //USER INPUT
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {}
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse = rl.get_mouse_position();
            //if mouse position is on top of a unit
                //if unit isnt player owned
                    //if unit is on a valid attack tile
                        //attack it!
        }

        //DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        self.map.draw(&mut d);
        d.draw_fps(20, 20);

        //Return state change = false
        NO_STATE_CHANGE
    }

    fn leave(&mut self, _rl: &mut RaylibHandle, _thread: &mut RaylibThread) {}
}

impl MapEditor {
    pub fn new(rl: &mut RaylibHandle, thread: &mut RaylibThread) -> Self {
        MapEditor {
            map: Map::new(25, 25, rl, thread),
        }
    }
}
