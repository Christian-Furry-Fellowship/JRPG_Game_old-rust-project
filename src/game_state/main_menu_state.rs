use super::game_state::GameState;
use super::state_event::StateEvent;

use macroquad::{
    math::vec2,
    window::{screen_width, screen_height}
};
use megaui_macroquad::{
    draw_window,
    megaui::{hash, Vector2},
    WindowParams,
};

use super::playing_state::PlayingState;

use crate::assets::audio::{AudioClip, ClipCategory, Playlist};
use std::path::{PathBuf, Path};

pub struct MainMenuState {
    event: StateEvent, 
    music_playlist: Playlist,
}

impl MainMenuState {


    pub fn new() -> MainMenuState {
        //TODO building this playlist is a bit rough. Refine it via a loading function for builtin resources.
        let clip1_path: PathBuf = ["builtin", "eclipse.mp3"].iter().collect();
        let clip1 = AudioClip::new( clip1_path, ClipCategory::Music );

        let clip2_path: PathBuf = ["builtin", "in-love.mp3"].iter().collect();
        let clip2 = AudioClip::new(clip2_path, ClipCategory::Music );

        let playlist = Playlist::new(vec![clip1, clip2]);

        MainMenuState {
            event: StateEvent::None,
            music_playlist: playlist,
        }
    }

}

impl GameState for MainMenuState {

    fn ui_logic(&mut self) {
        let main_menu_id = hash!();

        //calculate center position
        let menu_size = vec2(512., 420.);
        let menu_pos = vec2(
            (screen_width()  / 2.) - (menu_size.x / 2.), 
            (screen_height() / 2.) - (menu_size.y / 2.)
        );
        
        draw_window( main_menu_id, menu_pos, menu_size,
            WindowParams {
                label: "Main Menu".to_string(),
                close_button: false,
                titlebar: false,
                movable: false,
            },
            |ui| {
                //keep main menu window centered
                //TODO frankly very confused why this is needed. We provide a new pos each time so it should be needed.
                ui.move_window(main_menu_id, Vector2::new(menu_pos.x, menu_pos.y));
                
                if ui.button(None, "Start New Game") {  
                    let data_path = Path::new("campaigns").join("TestGame");
                    let new_state = PlayingState::new(data_path.to_str().unwrap());
                    self.event = StateEvent::ChangeState(Box::new(new_state));
                }
                ui.button(None, "Load");
                if ui.button(None, "Quit") { self.event = StateEvent::Shutdown; }                
            },
        );

        //keep menu music playing
        self.music_playlist.maintain_looping();
    }


    
    fn state_logic(&mut self) -> StateEvent {
        //Basically hot swaps the current event with a new none event then returns that event. 
        //  Avoids copying the event incase it holds a new state that would be expensive to copy.
        return std::mem::replace(&mut self.event, StateEvent::None);
    }
}
