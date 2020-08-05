use specs::{WriteExpect, ReadStorage, System};

use super::position_component::PositionComponent;
use super::gfx_components::VisibleComponent;

use crate::assets::{AssetDatabase, AssetContainer};

//the render system draws stuff onto the next frame before the core application 
//  applies it to the screen
pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = ( WriteExpect<'a, AssetDatabase>,
                        ReadStorage<'a, PositionComponent>,
                        ReadStorage<'a, VisibleComponent>);

    fn run(&mut self, (mut asset_database, position, visible): Self::SystemData) {
        use specs::Join;
        //let (batch, position, visible) = data;

        for (position, visible) in (&position, &visible).join() {


            match asset_database.get_asset(&visible.sprite_sheet_name) {
                AssetContainer::Spritesheet(atlas) => {
                    let row    = visible.sprite_location.0;
                    let column = visible.sprite_location.1;
            
                    atlas.add_to_batch(position.map_pos, row, column); 
                }
                _ => continue
            };
        }
    }
}
