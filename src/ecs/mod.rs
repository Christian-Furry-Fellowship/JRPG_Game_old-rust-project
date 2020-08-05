use specs::{World, WorldExt, Builder, Dispatcher, DispatcherBuilder};

use coffee::graphics::Point;

//pull in systems
mod render_system;
pub use render_system::RenderSystem;

mod animation_system;
pub use animation_system::AnimationSystem;

//pull in components
mod position_component;
pub use position_component::PositionComponent;
mod gfx_components;
pub use gfx_components::VisualComponent;
pub use gfx_components::AnimationComponent;

pub fn register_components(world: &mut World) {
    world.register::<PositionComponent>();
    world.register::<VisualComponent>();
    world.register::<AnimationComponent>();
}


pub fn build_data_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
    //.with(HelloWorld, "hello_world", &[])
    .build()
}

pub fn build_render_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
    .with(AnimationSystem, "AnimationSystem", &[])
    .with(RenderSystem, "RenderSystem", &[])
    .build()
}

pub fn create_test_entities(world: &mut World) {
    //player
    world
    .create_entity()
    .with(PositionComponent { map_pos: Point::new(100.0, 100.0) })
    .with(VisualComponent { 
               sprite_sheet_name: "assets/sara-atlas.png".to_string(), 
               sprite_location: (1,1) 
    })
    .with(AnimationComponent{
        name: "walk left".to_string(),
        index: 0,
        speed: 5,
        timer: 5,
    })
    .build();

    //another character
    world
    .create_entity()
    .with(PositionComponent { map_pos: Point::new(500.0, 500.0) })
    .with(VisualComponent { 
               sprite_sheet_name: "assets/sara-atlas.png".to_string(), 
               sprite_location: (5,1) 
    })
    .build();
}
