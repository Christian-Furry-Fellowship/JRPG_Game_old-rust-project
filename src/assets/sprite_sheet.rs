use std::collections::HashMap;

use macroquad::texture::{Texture2D, DrawTextureParams, draw_texture_ex};
use macroquad::math::Rect;
use macroquad::prelude::Color;

//size of a single sprite in the Sprite Sheet
struct SpriteSize { pub width: f32, pub height: f32}

//location of a single sprite
pub type SpritePos = ( i32, i32 );

//Sprite locations ordered in a way to create an animation
pub type SpriteAnimation = Vec<SpritePos>;


//An array of sprites packed into a single image, also called an Atlas.
pub struct SpriteSheet {
    pub texture: Texture2D,
    pub rows: i32,
    pub columns: i32,
    sprite_size: SpriteSize,
    animation_sets: HashMap<String, SpriteAnimation>,
}


impl SpriteSheet {

    pub fn new(texture: Texture2D, rows: i32, columns: i32) -> SpriteSheet {
        let sprite_size = SpriteSize {
            width: texture.width()  / columns as f32,
            height: texture.height() / rows as f32,
        };

        SpriteSheet {
            texture,
            rows, columns,
            sprite_size,
            animation_sets: HashMap::new(),
        }
    }


    pub fn draw_sprite(&self, x: f32, y: f32, mut row: i32, mut column: i32,
                       color: Option<Color>) {
        
        //adjust row/column for calculating sprite position in atlas  
        row = row - 1;
        column = column - 1;

        //TODO: Should we make sure requested row/column is within bounds 
        //     or trust program not to make that mistake?

        //define these for brevity
        let sprite_width = self.sprite_size.width as f32;
        let sprite_height = self.sprite_size.height as f32;

        //setup to only draw a single sprite
        let mut params = DrawTextureParams::default();
        params.source = Some(
            Rect{ x: column as f32 * sprite_width, y: row as f32 * sprite_height,
                  w: sprite_width, h: sprite_height}
        );

        //draw portion of texture related to the sprite
        draw_texture_ex(
            self.texture, x, y, 
            color.unwrap_or( Color::new(1.,1.,1.,1.) ), params
        );
    }

    //adds a new sprite sequence using positions that represents an animation
    pub fn add_animation(&mut self, name: String, set: SpriteAnimation) {
        self.animation_sets.insert(name, set);
    }

    //acquires animation pos sequence or empty vector
    pub fn get_animation(&mut self, name: &String) -> SpriteAnimation {
        match self.animation_sets.get_mut(name) {
            Option::Some(sprite_animation) => sprite_animation.to_vec(),
            Option::None => vec![]
        }
    }
}
