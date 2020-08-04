use specs::{World, WorldExt, Builder, Dispatcher, DispatcherBuilder};

use coffee::graphics::Point;

//pull in systems
mod render_system;
pub use render_system::RenderSystem;

//pull in components
mod position_component;
pub use position_component::PositionComponent;
mod visible_component;
pub use visible_component::VisibleComponent;


pub fn register_components(world: &mut World) {
    world.register::<PositionComponent>();
    world.register::<VisibleComponent>();
}


pub fn build_data_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
    //.with(HelloWorld, "hello_world", &[])
    .build()
}

pub fn build_render_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
    .with(RenderSystem, "RenderSystem", &[])
    .build()
}

pub fn create_test_entities(world: &mut World) {
    //player
    world
    .create_entity()
    .with(PositionComponent { map_pos: Point::new(100.0, 100.0) })
    .with(VisibleComponent { sprite_sheet_name: "test".to_string(), sprite_location: (1,1) })
    .build();

    //another character
    world
    .create_entity()
    .with(PositionComponent { map_pos: Point::new(500.0, 500.0) })
    .with(VisibleComponent { sprite_sheet_name: "test".to_string(), sprite_location: (1,1) })
    .build();
}
