use coffee::{
    graphics::{Frame, Window},
    ui::{Element, Row},
    input::KeyboardAndMouse,
    Timer
};

use super::UIAction;


//The game state trait lets us handle different stages of the application in a modular
//  way by simply changing to a new state as needed.
pub trait GameState {
    //These 3 functions handle general input, output, and updating data
    fn interact(&mut self, _kbm: &mut KeyboardAndMouse, _window: &mut Window) {}
    fn   update(&mut self) {}
    fn     draw(&mut self, _frame: &mut Frame, _timer: &Timer) {}

    fn react(&mut self, _message: UIAction, _window: &mut Window) 
        -> Option< Box<dyn GameState> > { Option::None }
    fn layout(&mut self, _window: &Window) -> Element<UIAction> { Row::new().into() }

    fn is_finished(&self) -> bool { false }
}
