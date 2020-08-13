use specs::{Component, VecStorage};

use crate::assets::{SpritePos};

//For entities that should have some visual representation on the screen
#[derive(Component)]
#[storage(VecStorage)]
pub struct VisualComponent {
    pub sprite_sheet_name: String,
    pub sprite_location: SpritePos,
}


//For entities that are animated, should be paired with a visual component
#[derive(Component)]
#[storage(VecStorage)]
pub struct AnimationComponent {
    pub name: String,
    pub index: usize,
    pub speed: u16,
    pub timer: u16,
}

impl AnimationComponent {
  pub fn new(speed: u16) -> AnimationComponent {
      AnimationComponent{
          name: "idle".to_string(),
          index: 0,
          speed,
          timer: speed,
      }
  }

  pub fn set(&mut self, new_name: &str) {
      if self.name != new_name {
          self.name = new_name.to_string();
          self.index = 0;
          self.timer = self.speed;
      }
  }
}
