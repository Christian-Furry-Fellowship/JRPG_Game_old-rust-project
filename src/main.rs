use coffee::{
    graphics::{Color, Image, Frame, Window, WindowSettings},
    load::{Task},
    //input::keyboard::KeyCode,
    //input::{self, keyboard, Input},
    Game, Timer
};

use specs::{World, WorldExt, Dispatcher};

mod assets;
use assets::{AssetDatabase, AssetContainer, SpriteSheet};

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

        Image::load("assets/sara-atlas.png")
         .map(|player_atlas| {
              
              let mut ss = SpriteSheet::new(player_atlas, 5, 5);
              ss.add_animation( "idle".to_string(),
                  vec![(1,1), (2,1), (4,1), (3,1)]
              );
              ss.add_animation( "walk left".to_string(),
                  vec![(1,1), (1,2), (1,3), (1,4), (1,5)]
              );
              ss.add_animation( "walk down".to_string(),
                  vec![(2,1), (2,2), (2,3), (2,4), (2,5)]
              );
              ss.add_animation( "walk up".to_string(),
                  vec![(3,1), (3,2), (3,3), (3,4), (3,5)]
              );              
              ss.add_animation( "walk right".to_string(),
                  vec![(4,1), (4,2), (4,3), (4,4), (4,5)]
              );


              let mut asset_db = AssetDatabase::new();
              asset_db.add_asset(
                   "assets/sara-atlas.png".to_string(),
                   AssetContainer::Spritesheet( ss )
              );

              let mut world = World::new();
              ecs::register_components(&mut world);
              ecs::create_test_entities(&mut world);

              //insert data into the world
              world.insert(asset_db); 

              MyGame { 
                  world,
                  render_dispatcher: ecs::build_render_dispatcher(),
                  data_dispatcher: ecs::build_data_dispatcher(),
              }
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


