use coffee::{
    graphics::{Color, Image, Frame, Window, WindowSettings},
    load::{Task},
    //input::keyboard::KeyCode,
    //input::{self, keyboard, Input},
    Game, Timer
};

use specs::{World, WorldExt, Dispatcher};

mod assets;
use assets::{AssetDatabase, SpriteSheet};

mod ecs;

struct MyGame {
    world: World,
    render_dispatcher: Dispatcher<'static, 'static>, 
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {

        Image::load("assets/sara-atlas.png")
         .map(|player_atlas| {
              
              let sprite_sheet = SpriteSheet::new(player_atlas, 5, 5);

              let mut world = World::new();
              ecs::register_components(&mut world);
              ecs::create_test_entities(&mut world);

              //insert data into the world
              world.insert(AssetDatabase{sprite_sheet}); 

              MyGame { 
                  world,
                  render_dispatcher: ecs::build_render_dispatcher()
              }
         })

        //insert data into the world
        //let bg_img = Image::from_image(_window.gpu(),
        //                   DynamicImage::new_rgba8(_window.width(), _window.height()))
        //             .expect("Error creating batch image.");
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        let mut world = & self.world;
        
        self.render_dispatcher.dispatch(&mut world);

        let mut asset_database = world.write_resource::<AssetDatabase>();        
        asset_database.sprite_sheet.batch.draw( &mut frame.as_target() );
        asset_database.sprite_sheet.batch.clear();
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


