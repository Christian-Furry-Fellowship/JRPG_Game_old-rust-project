#[macro_use] extern crate log;
extern crate simplelog;


use coffee::{
    graphics::{Color, Frame, Window, WindowSettings},
    load::{Task},
    input::KeyboardAndMouse,
    input::keyboard::KeyCode,
    Game, Timer
};

use specs::{World, WorldExt, Dispatcher};

mod assets;
use assets::{AssetDatabase, AssetContainer, load_campaign_data};

mod ecs;

mod handle_input;
use handle_input::ControlData;

struct MyGame {
    world: World,
    render_dispatcher: Dispatcher<'static, 'static>,
    input_dispatcher: Dispatcher<'static, 'static>,
    data_dispatcher: Dispatcher<'static, 'static>,
}

impl Game for MyGame {
    type Input = KeyboardAndMouse;
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
            world.insert(ControlData { move_left: false, move_right: false, move_up: false, move_down: false }); 

            Ok(MyGame { 
                world,
                render_dispatcher: ecs::build_render_dispatcher(),
                input_dispatcher: ecs::build_input_handling_dispatcher(),
                data_dispatcher: ecs::build_data_dispatcher(),
            })
         })

    }

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
    //setup logging system
    simplelog::CombinedLogger::init(
        vec![
            simplelog::TermLogger::new(
                  simplelog::LevelFilter::Warn, 
                  simplelog::Config::default(), 
                  simplelog::TerminalMode::Mixed
            ).unwrap(),
        ]
    ).unwrap();

    MyGame::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
    .expect("An error occured while starting the game");
}


