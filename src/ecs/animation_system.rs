use specs::{WriteExpect, WriteStorage, System};

use super::gfx_components::VisualComponent;
use super::gfx_components::AnimationComponent;

use crate::assets::{AssetDatabase, AssetContainer};

//the animation system advances the sprite selection on spritesheets to make an animation
pub struct AnimationSystem;

impl<'a> System<'a> for AnimationSystem {
    type SystemData = ( WriteExpect<'a, AssetDatabase>,
                        WriteStorage<'a, VisualComponent>,
                        WriteStorage<'a, AnimationComponent>);

    fn run(&mut self, (mut asset_database, mut visual, mut animation): Self::SystemData) {
        use specs::Join;

        for (visual, animation) in (&mut visual, &mut animation).join() {

            //first only advance animation if timer is finished
            if animation.timer > 0 {
                //TODO use an actual delta time for the count down
                animation.timer -= 1;
                continue;

            } else {
                animation.timer = animation.speed;
            }


            //aquire sprite sheet for this work
            match asset_database.get_asset(&visual.sprite_sheet_name) {
                AssetContainer::Spritesheet(atlas) => {

                    let animation_set = atlas.get_animation(&animation.name);

                    //advance animation but make sure it doesn't overflow the vec
                    animation.index += 1;
                    if animation.index >= animation_set.len() {
                        animation.index = 0;
                    }
                    
                    //set the new sprite on the visual component
                    match animation_set.get(animation.index) {
                        Option::Some(location) => visual.sprite_location = *location,
                        Option::None => visual.sprite_location = (1,1),
                    }
                    
                }
                _ => continue
            };

        }
    }
}
