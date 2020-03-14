use rand::*;
use raylib::prelude::*;

//State rs file includes
mod game;
mod main_menu;
mod map;
mod map_editor;
mod options;
mod states;
mod unit;

use game::*;
use main_menu::*;
use map::*;
use map_editor::*;
use options::*;
use states::*;
use std::collections::VecDeque;
use unit::*;

fn main() {
    //Initialize window and global settings
    let (mut rl, mut thread) = raylib::init()
        .size(640, 480)
        .title("Embodiment of Fading Dream")
        .build();
    rl.set_target_fps(60);

    //Hardcode the units for the time being
    let friendlies: Vec<Unit> = vec![
        Unit::new_custom(0, "henry", true, false, 100, 100, 4, 1, 2, 7, 3, 4, 5, 7),
        Unit::new_custom(3, "chad", true, false, 100, 100, 4, 1, 2, 7, 3, 4, 6, 7),
    ];
    let enemies: Vec<Unit> = vec![
        Unit::new_custom(1, "toby", true, false, 100, 100, 4, 1, 2, 7, 3, 4, 3, 2),
        Unit::new_custom(2, "annie", true, false, 100, 100, 4, 1, 2, 7, 3, 4, 6, 2),
    ];

    //Initialize states
    let game = Box::new(Game::from_unit_population(
        &mut rl,
        &mut thread,
        friendlies,
        enemies,
    ));
    let map_editor = Box::new(MapEditor::new(&mut rl, &mut thread));
    let main_menu = Box::new(MainMenu::new(&mut rl, &mut thread));
    let options = Box::new(Options::new(&mut rl, &mut thread));

    //Add states to the state manager
    //State 0 is NO_STATE_CHANGE
    let mut statelist = create_state_group();
    add_state(&mut statelist, &mut thread, &mut rl, main_menu); //state 1
    add_state(&mut statelist, &mut thread, &mut rl, game); //state 2
    add_state(&mut statelist, &mut thread, &mut rl, map_editor); //state 3
    add_state(&mut statelist, &mut thread, &mut rl, options); //state 4
    set_state(&mut statelist, &mut thread, &mut rl, 0); //set state menu

    //Run current state
    while !rl.window_should_close() {
        run_current_state(&mut statelist, &mut thread, &mut rl);
    }
}
