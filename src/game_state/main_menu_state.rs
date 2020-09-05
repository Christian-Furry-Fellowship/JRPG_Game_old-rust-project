use super::game_state::GameState;

use coffee::{
    graphics::{Frame, Window, Color},
    ui::{button, Button, Column, Row, Element},
    Timer
}; 

use super::UIAction;

use super::playing_state::PlayingState;

use crate::assets::audio::{AudioClip, ClipCategory, Playlist};
use std::path::PathBuf;

pub struct MainMenuState {
    start_button: button::State,
    load_button: button::State,
    quit_button: button::State,

    quit_requested: bool, 
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
            start_button: button::State::new(),
            load_button: button::State::new(),
            quit_button: button::State::new(),

            quit_requested: false,
            music_playlist: playlist,
        }
    }

}

impl GameState for MainMenuState {

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);
    }


    fn react(&mut self, message: UIAction, window: &mut Window) -> Option< Box<dyn GameState> > {
        match message {
            UIAction::NewGame => {
                //TODO temp until we get campaign selection working.
                let path: PathBuf = ["campaigns", "TestGame"].iter().collect();
                return Option::Some(
                    Box::new(PlayingState::new( window.gpu(), path.to_str().unwrap() ))
                )
            },
            UIAction::LoadGame => warn!("Load game triggered"), //TODO Implement game loading
            UIAction::QuitGame => self.quit_requested = true,
        };

        Option::None
    }

    // The layout logic, describing the different components of the user interface
    fn layout(&mut self, window: &Window) -> Element<UIAction> {
        Row::new()
            .push(
                //Adds in some horizontal spacing.
                Column::new().width( (window.width()/8.0) as u32)
            )
            .push(
                Column::new()
                    .width( (window.width()/8.0) as u32)
                    .push(
                        //adds some virtical spacing
                        Row::new().height( (window.width()/8.0) as u32 )
                    )
                    .push( Button::new(&mut self.start_button, "Start New Game")
                            .fill_width()
                            .on_press(UIAction::NewGame),
                    )
                    .push( Button::new(&mut self.load_button, "Load Game")
                            .fill_width()
                            .on_press(UIAction::LoadGame),
                    )
                    .push( Button::new(&mut self.quit_button, "Quit Game")
                            .fill_width()
                            .on_press(UIAction::QuitGame),
                    )
            )
            .into()
    }

    
    fn update(&mut self, _window: &Window) {
        self.music_playlist.maintain_looping();
    }
    

    fn is_finished(&self) -> bool {
        self.quit_requested
    }
}
