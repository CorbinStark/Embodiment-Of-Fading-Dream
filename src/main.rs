#![allow(unused_variables)]

use raylib::prelude::*;

mod game;
mod main_menu;
mod map_editor;
mod states;
use game::*;
use main_menu::*;
use map_editor::*;
use states::*;

fn main() {
    //Initialize window and global settings
    let (mut rl, mut thread) = raylib::init()
        .size(640, 480)
        .title("Embodiment of Fading Dream")
        .build();
    rl.set_target_fps(60);

    //Initialize states
    let game = Box::new(Game::new(&mut rl, &mut thread));
    let map_editor = Box::new(MapEditor::new(&mut rl, &mut thread));
    let main_menu = Box::new(MainMenu::new(&mut rl, &mut thread));

    //Add states to the state manager
    //State 0 is NO_STATE_CHANGE
    let mut statelist = create_state_group();
    add_state(&mut statelist, &mut thread, &mut rl, main_menu); //state 1
    add_state(&mut statelist, &mut thread, &mut rl, game); //state 2
    add_state(&mut statelist, &mut thread, &mut rl, map_editor); //state 3
    set_state(&mut statelist, &mut thread, &mut rl, 0); //set state menu

    //Run current state
    while !rl.window_should_close() {
        run_current_state(&mut statelist, &mut thread, &mut rl);
    }
}
