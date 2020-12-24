use super::game_state::GameState;
use super::state_event::StateEvent;

use crate::assets::{AssetDatabase, AssetContainer, load_campaign_data};

use crate::ecs as ecs;

use specs::{World, WorldExt, Dispatcher};

use macroquad::input::{is_key_down, KeyCode};

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

    pub fn new(game_data_path: &str) -> PlayingState {
        let mut asset_db = AssetDatabase::new();

        load_campaign_data(game_data_path, &mut asset_db);
        
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


    fn ui_logic(&mut self) {
        //update all systems related to rendering
        let mut world = & self.world;
        self.render_dispatcher.dispatch(&mut world);

    }

    fn state_logic(&mut self) -> StateEvent {

        let mut world = & self.world;

        //closure is needed so control_data can go out of scope and be barrowed again when running the system
        {
            let mut control_data = world.write_resource::<ControlData>();

            //query microquad for key presses        
            control_data.move_left  = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
            control_data.move_right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
            control_data.move_up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
            control_data.move_down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);
        }

        //run all systems related to input handling
        self.input_dispatcher.dispatch(&mut world);

        

        StateEvent::None
    }

}
