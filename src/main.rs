#[macro_use] extern crate log;
extern crate simplelog;


use macroquad::prelude::*;
use megaui_macroquad::draw_megaui;


mod assets;
mod game_state;
mod ecs;


use game_state::{GameState, MainMenuState, StateEvent};



#[macroquad::main("JRPG")]
async fn main() {
    //setup logging system
    simplelog::CombinedLogger::init(
        vec![
            simplelog::TermLogger::new(
                  simplelog::LevelFilter::Warn, 
                  simplelog::Config::default(), 
                  simplelog::TerminalMode::Mixed
            ).unwrap(),
        ]
    ).unwrap();

    
    let mut game_state: Box<dyn GameState> = Box::new(MainMenuState::new());
    let mut running = true;
    while running {
        clear_background(LIGHTGRAY);
        
        //update game based on current state then handle the state event.
        match game_state.update() {
            StateEvent::Shutdown => running = false,
            StateEvent::ChangeState(state) => game_state = state,
            _ => (),
        }

        draw_megaui();

        next_frame().await;
    }
}
