use std::collections::HashMap;

use coffee::graphics::{Point, Rectangle, Image, Batch, Sprite};

//size of a single sprite in the Sprite Sheet
struct SpriteSize { pub width: u16, pub height: u16}

//location of a single sprite
pub type SpritePos = ( u16, u16 );

//Sprite locations ordered in a way to create an animation
pub type SpriteAnimation = Vec<SpritePos>;


//An array of sprites packed into a single image, also called an Atlas.
pub struct SpriteSheet {
    //atlas: Image,
    pub batch: Batch,
    pub rows: u16,
    pub columns: u16,
    sprite_size: SpriteSize,
    animation_sets: HashMap<String, SpriteAnimation>,
}


impl SpriteSheet {

    pub fn new(image: Image, rows: u16, columns: u16) -> SpriteSheet {
        let sprite_size = SpriteSize {
            width: image.width() / columns,
            height: image.height() / rows,
        };

        SpriteSheet {
            //atlas: image.clone(),
            batch: Batch::new(image),
            rows,
            columns,
            sprite_size,
            animation_sets: HashMap::new(),
        }
    }


    //Extract a specific sprite from the sprite sheet
    // @position: provided position of the sprite on the target screen/frame/etc.
    // @row: Which row of the atlas we are requesting. Note row starts at 1
    // @column: Which column of the atlas we are requesting. Note column starts at 1
    // returns: Image object of full atlas, used in drawing 
    //          Sprite object depicting a single sprite in the atlas

    pub fn get_sprite(&self, position: Point, mut row: u16, mut column: u16)
           -> Sprite {

        //adjust row/column for calculating sprite position in atlas  
        row = row - 1;
        column = column - 1;

        //TODO should we make sure requested row/column is within bounds 
        //     or trust program not to make that mistake?

        //define these for brevity
        let sprite_width = self.sprite_size.width;
        let sprite_height = self.sprite_size.height;

        //return full atlas image and requested sprite's location in the atlas.
        Sprite { 
            source: Rectangle{
                x: column * sprite_width, y: row * sprite_height, 
                width: sprite_width, height: sprite_height,
            },
            position: position, 
            scale: (1.0,1.0) //assume normal scale, lets other code change it as needed 
        }
    }

    //add a sprite quad to the batch for later drawing
    pub fn add_to_batch(&mut self, position: Point, row: u16, column: u16) {
        self.batch.add( self.get_sprite( position, row, column ) );
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
