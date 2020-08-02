use coffee::graphics::{Point, Rectangle, Image, Sprite};

//size of a single sprite in the Sprite Sheet
struct SpriteSize { pub width: u16, pub height: u16}


//An array of sprites packed into a single image, also called an Atlas.
pub struct SpriteSheet {
    atlas: Image,
    //rows: u16,
    //columns: u16,
    sprite_size: SpriteSize,
}


impl SpriteSheet {

    pub fn new(image: Image, rows: u16, columns: u16) -> SpriteSheet {
        let sprite_size = SpriteSize {
            width: image.width() / columns,
            height: image.height() / rows,
        };

        SpriteSheet {
            atlas: image,
            sprite_size,
        }
    }


    //Extract a specific sprite from the sprite sheet
    // @position: provided position of the sprite on the target screen/frame/etc.
    // @row: Which row of the atlas we are requesting. Note row starts at 1
    // @column: Which column of the atlas we are requesting. Note column starts at 1
    // returns: Image object of full atlas, used in drawing 
    //          Sprite object depicting a single sprite in the atlas

    pub fn get_sprite(&self, position: Point, mut row: u16, mut column: u16)
           -> (Image, Sprite) {

        //adjust row/column for calculating sprite position in atlas  
        row = row - 1;
        column = column - 1;

        //TODO should we make sure requested row/column is within bounds 
        //     or trust program not to make that mistake?

        //define these for brevity
        let sprite_width = self.sprite_size.width;
        let sprite_height = self.sprite_size.height;

        //return full atlas image and requested sprite's location in the atlas.
        (self.atlas.clone(), //note coffee docs says cloning Image is very cheap
         Sprite { 
            source: Rectangle{
                x: column * sprite_width, y: row * sprite_height, 
                width: sprite_width, height: sprite_height,
            },
            position: position, 
            scale: (1.0,1.0) //assume normal scale, lets other code change it as needed 
        })
    }
}
