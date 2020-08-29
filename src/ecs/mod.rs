use specs::{World, WorldExt, Builder, Dispatcher, DispatcherBuilder};

use coffee::graphics::Point;
use std::path::PathBuf;

//pull in systems
mod render_system;
pub use render_system::RenderSystem;

mod animation_system;
pub use animation_system::AnimationSystem;

mod player_control_system;
pub use player_control_system::PlayerControlSystem;

//pull in components
mod position_component;
pub use position_component::PositionComponent;

mod gfx_components;
pub use gfx_components::{VisualComponent, AnimationComponent};

mod control_components;
pub use control_components::PlayerControlComponent;


pub fn register_components(world: &mut World) {
    world.register::<PositionComponent>();
    world.register::<VisualComponent>();
    world.register::<AnimationComponent>();
    world.register::<PlayerControlComponent>();
}


pub fn build_data_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
    //.with(HelloWorld, "hello_world", &[])
    .build()
}

pub fn build_input_handling_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
    .with(PlayerControlSystem, "PlayerControlSystem", &[])
    .build()
}

pub fn build_render_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
    .with(AnimationSystem, "AnimationSystem", &[])
    .with(RenderSystem, "RenderSystem", &[])
    .build()
}

pub fn create_test_entities(world: &mut World) {
    //TODO temp until we get better entity spawning functionality.
    let sprite_sheet_name = PathBuf::from("campaigns/TestGame/sprite_sheets/sara-atlas.png").to_str().unwrap().to_string();

    //player
    world
    .create_entity()
    .with(PlayerControlComponent { speed: 5.0 })
    .with(PositionComponent { map_pos: Point::new(100.0, 100.0) })
    .with(VisualComponent { 
               sprite_sheet_name: sprite_sheet_name.clone(), 
               sprite_location: (1,1) 
    })
    .with(AnimationComponent::new(5))
    .build();

    //another character
    world
    .create_entity()
    .with(PositionComponent { map_pos: Point::new(500.0, 500.0) })
    .with(VisualComponent { 
               sprite_sheet_name: sprite_sheet_name.clone(), 
               sprite_location: (5,1) 
    })
    .build();
}
