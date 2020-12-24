use super::GameState;

//Dictates changes to overal state
pub enum StateEvent {
    None,
    Shutdown,
    ChangeState(Box<dyn GameState>),
}
