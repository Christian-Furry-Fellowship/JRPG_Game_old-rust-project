

// Possible UI actions that can be triggered 
//FIXME Really stupid but every UI action trigger has to be listed here even though every 
//  state doesn use all of them. This is because the Game trait in the main application 
//  has to define which enum to use for the whole program.
//  Might beable to fix if we implement our own Renderer, not positive.
#[derive(Debug, Clone, Copy)]
pub enum UIAction {
    NewGame,
    LoadGame,
    QuitGame,
}
