use coffee::{
    graphics::{Color, Frame, Window, WindowSettings},
    load::{Task},
    //input::keyboard::KeyCode,
    //input::{self, keyboard, Input},
    Game, Timer
};

use specs::{World, WorldExt, Dispatcher};

mod assets;
use assets::{AssetDatabase, AssetContainer, load_campaign_data};

mod ecs;

struct MyGame {
    world: World,
    render_dispatcher: Dispatcher<'static, 'static>,
    data_dispatcher: Dispatcher<'static, 'static>,
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {

        Task::using_gpu(|gpu| {

            let mut asset_db = AssetDatabase::new();

            load_campaign_data("campaigns/TestGame", gpu, &mut asset_db);

            let mut world = World::new();
            ecs::register_components(&mut world);
            ecs::create_test_entities(&mut world);

            //insert data into the world
            world.insert(asset_db); 

            Ok(MyGame { 
                world,
                render_dispatcher: ecs::build_render_dispatcher(),
                data_dispatcher: ecs::build_data_dispatcher(),
            })
         })

    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        let mut world = & self.world;
        self.render_dispatcher.dispatch(&mut world);


        let mut asset_database = world.write_resource::<AssetDatabase>();


        //TODO this isn't good. we should only iterate over assets that need to be drawn
        for (_, asset_container) in asset_database.get_asset_iter_mut() {
        
            match asset_container {

                AssetContainer::Spritesheet(spritesheet) => {
                    spritesheet.batch.draw( &mut frame.as_target() );
                    spritesheet.batch.clear();
                },

                _ => return
            };

        }
    }
}






fn main() {
    MyGame::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
    .expect("An error occured while starting the game");
}


