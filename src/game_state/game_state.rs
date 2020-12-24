use super::state_event::StateEvent;

//The game state trait lets us handle different stages of the application in a modular
//  way by simply changing to a new state as needed.
pub trait GameState {

    fn update(&mut self) -> StateEvent {
        self.ui_logic();
        return self.state_logic();
    }
    
    //code that handles user interface logic
    fn ui_logic(&mut self);

    //code that just handles manipulating the state
    fn state_logic(&mut self) -> StateEvent;
}
