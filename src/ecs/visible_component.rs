use specs::{Component, VecStorage};

use crate::assets::SpriteSheet;

#[derive(Component)]
#[storage(VecStorage)]
pub struct VisibleComponent {
    pub sprite_sheet_name: String,
    pub sprite_location: (u16, u16), //row, column
}

