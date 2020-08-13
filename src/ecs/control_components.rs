use specs::{Component, VecStorage};

//For entities that should have some visual representation on the screen
#[derive(Component)]
#[storage(VecStorage)]
pub struct PlayerControlComponent {
    pub speed: f32,
}
