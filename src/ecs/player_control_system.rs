use specs::{ReadExpect, ReadStorage, WriteStorage, System};

use super::position_component::PositionComponent;
use super::control_components::PlayerControlComponent;
use super::gfx_components::AnimationComponent;

use crate::game_state::playing_state::ControlData;

//the animation system advances the sprite selection on spritesheets to make an animation
pub struct PlayerControlSystem;

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = ( ReadExpect<'a, ControlData>,
                        WriteStorage<'a, PositionComponent>,
                        ReadStorage<'a, PlayerControlComponent>,
                        WriteStorage<'a, AnimationComponent>);

    fn run(&mut self, (control_data, mut position, control, mut animation): Self::SystemData) {
        use specs::Join;

        //there should only be one really, but maybe multiple will be useful at some point
        for (position, control, animation) in (&mut position, &control, &mut animation).join() {

            let mut displacement = (0.0 as f32, 0.0 as f32);
            if control_data.move_left  { displacement.0 -= 1.0; }
            if control_data.move_right { displacement.0 += 1.0; }
            if control_data.move_up    { displacement.1 -= 1.0; }
            if control_data.move_down  { displacement.1 += 1.0; }


            //set animation
                 if displacement.0 ==  1.0  { animation.set("walk right"); }
            else if displacement.0 == -1.0  { animation.set("walk left"); }
            else if displacement.1 ==  1.0  { animation.set("walk down"); }
            else if displacement.1 == -1.0  { animation.set("walk up"); }
            else { animation.set("idle") }


            //if we move at an angle then speed would be evenly split between the two directions
            if displacement.0.abs() == 1.0 && displacement.1.abs() == 1.0 {
                displacement.0 *= control.speed / 2.0;
                displacement.1 *= control.speed / 2.0;

            //otherwise whichever direction has a non-zero value will get the full speed
            } else {
                displacement.0 *= control.speed;
                displacement.1 *= control.speed;
            }

            
            //update position
            position.map_pos.0 += displacement.0;
            position.map_pos.1 += displacement.1;

        }
    }
}
