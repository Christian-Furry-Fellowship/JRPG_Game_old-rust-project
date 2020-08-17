#[macro_use] extern crate log;
extern crate simplelog;

mod assets;
mod game_state;
mod ecs;


use coffee::{
    graphics::{Frame, Window, WindowSettings},
    load::{Task},
    input::KeyboardAndMouse,
    ui::{UserInterface, Renderer, Element},
    Game, Timer
};

use game_state::{UIAction, GameState, MainMenuState};


struct Application {
    current_game_state: Box<dyn GameState>,
}

impl Game for Application {
    type Input = KeyboardAndMouse;
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<Application> {
        Task::succeed(|| {
            Application { 
                current_game_state: Box::new( MainMenuState::new() ),
            }
         })
    }

    //handles general input
    fn interact(&mut self, kbm: &mut KeyboardAndMouse, window: &mut Window) {
        self.current_game_state.interact(kbm, window);
    }

    //draws non ui elements to the screen
    fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
        self.current_game_state.draw(frame, timer);
    }

    //check if we should shutdown the program
    fn is_finished(&self) -> bool {
        self.current_game_state.is_finished()
    }
}


impl UserInterface for Application {
    type Message = UIAction;
    type Renderer = Renderer; // We use the built-in Renderer

    // The update logic, called when a UI message is produced
    fn react(&mut self, message: UIAction, window: &mut Window) {
        //change our state if the current one requests it 
        match self.current_game_state.react(message, window) {
            Option::None => return,
            Option::Some(new_state) => self.current_game_state = new_state,
        }
    }

    // gets the UI layout and draws it to the screen ontop of whatever else has been drawn
    fn layout(&mut self, window: &Window) -> Element<UIAction> {
        self.current_game_state.layout(window)
    }

}



fn main() {
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

    <Application as UserInterface>::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: true,
    })
    .expect("An error occured while starting the game");
}


