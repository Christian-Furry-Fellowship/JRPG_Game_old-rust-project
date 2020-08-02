use coffee::graphics::{Color, Image, Frame, Window, WindowSettings, Sprite, Rectangle, Point};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, Input};

mod assets;
use assets::SpriteSheet;

struct MyGame {
    player: SpriteSheet
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        Image::load("assets/sara-atlas.png")
            .map(|image| MyGame { player: SpriteSheet::new(image, 5, 5) })

    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);


        // Draw your game here. Check out the `graphics` module!
        let position = Point::new(100.0, 100.0);
        let (image, sprite) = self.player.get_sprite(position, 5, 1);
        image.draw(sprite, &mut frame.as_target());
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


