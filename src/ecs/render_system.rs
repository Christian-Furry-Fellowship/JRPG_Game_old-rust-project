use specs::{WriteExpect, ReadStorage, System};

use super::position_component::PositionComponent;
use super::gfx_components::VisualComponent;

use crate::assets::{AssetDatabase, AssetContainer};

//the render system draws stuff onto the next frame before the core application 
//  applies it to the screen
pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = ( WriteExpect<'a, AssetDatabase>,
                        ReadStorage<'a, PositionComponent>,
                        ReadStorage<'a, VisualComponent>);

    fn run(&mut self, (mut asset_database, position, visual): Self::SystemData) {
        use specs::Join;
        //let (batch, position, visible) = data;

        for (position, visual) in (&position, &visual).join() {


            match asset_database.get_asset(&visual.sprite_sheet_name) {
                AssetContainer::Spritesheet(atlas) => {
                    let row    = visual.sprite_location.0;
                    let column = visual.sprite_location.1;
            
                    atlas.add_to_batch(position.map_pos, row, column); 
                }
                _ => continue
            };
        }
    }
}
