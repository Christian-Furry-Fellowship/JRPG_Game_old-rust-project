use specs::{Component, VecStorage};


#[derive(Component)]
#[storage(VecStorage)]
pub struct PositionComponent {
    pub map_pos: (f32, f32),
}
