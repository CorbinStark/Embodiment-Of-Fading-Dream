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
    pub fn new(_rl: &mut RaylibHandle, _thread: &mut RaylibThread) -> Self {
        MapEditor {
            map: Map::create_blank(25, 25),
        }
    }
}
