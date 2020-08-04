use specs::{Component, VecStorage};

use coffee::graphics::Point;


#[derive(Component)]
#[storage(VecStorage)]
pub struct PositionComponent {
    pub map_pos: Point
}
