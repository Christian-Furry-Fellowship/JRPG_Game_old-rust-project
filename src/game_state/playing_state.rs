use super::game_state::GameState;

use coffee::{
    graphics::{Gpu, Color, Frame, Window},
    input::KeyboardAndMouse,
    input::keyboard::KeyCode,
    Timer
};

use crate::assets::{AssetDatabase, AssetContainer, load_campaign_data};

use crate::ecs as ecs;

use specs::{World, WorldExt, Dispatcher};


//controller input values used by different ECS Systems
pub struct ControlData {
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
}


pub struct PlayingState {
    world: World, //ECS game world

    //various system managers that run systems added to them in parrellel
    render_dispatcher: Dispatcher<'static, 'static>,
    input_dispatcher: Dispatcher<'static, 'static>,
    data_dispatcher: Dispatcher<'static, 'static>,
}

impl PlayingState {

    pub fn new(gpu: &mut Gpu, game_data_path: &str) -> PlayingState {
        let mut asset_db = AssetDatabase::new();

        load_campaign_data(game_data_path, gpu, &mut asset_db);

        //setup world with all components we need and add in some entities
        let mut world = World::new();
        ecs::register_components(&mut world);
        ecs::create_test_entities(&mut world);

        //insert none ECS data into the world
        world.insert(asset_db); 
        world.insert(ControlData { move_left: false, move_right: false, move_up: false, move_down: false });


        PlayingState {
            world,
            render_dispatcher: ecs::build_render_dispatcher(),
            input_dispatcher: ecs::build_input_handling_dispatcher(),
            data_dispatcher: ecs::build_data_dispatcher(),
        }
    }
 
}

impl GameState for PlayingState {

    fn interact(&mut self, kbm: &mut KeyboardAndMouse, _window: &mut Window) {
        let mut world = & self.world;

        //closure is needed so control_data can go out of scope and be barrowed again when running the system
        {
            let mut control_data = world.write_resource::<ControlData>();

            let kb = kbm.keyboard();
        
            control_data.move_left  = kb.is_key_pressed(KeyCode::A) || kb.is_key_pressed(KeyCode::Left);
            control_data.move_right = kb.is_key_pressed(KeyCode::D) || kb.is_key_pressed(KeyCode::Right);
            control_data.move_up = kb.is_key_pressed(KeyCode::W) || kb.is_key_pressed(KeyCode::Up);
            control_data.move_down = kb.is_key_pressed(KeyCode::S) || kb.is_key_pressed(KeyCode::Down);
        }

        //run all systems related to input handling
        self.input_dispatcher.dispatch(&mut world);
    }


    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);


        let mut world = & self.world;
        self.render_dispatcher.dispatch(&mut world);


        let mut asset_database = world.write_resource::<AssetDatabase>();


        //TODO this isn't good. We should only iterate over assets that need to be drawn
        for (_, asset_container) in asset_database.get_asset_iter_mut() {
        
            match asset_container {

                //TODO we should use a trait or something so we can generically check if asset 
                //  container has an object that is renderable. Then just grab the batch and draw it.
                //  I think Coffee implements a drawable trait so maybe we should just use that.
                AssetContainer::Spritesheet(spritesheet) => {
                    spritesheet.batch.draw( &mut frame.as_target() );
                    spritesheet.batch.clear();
                },

                _ => return
            };

        }
    }
}
