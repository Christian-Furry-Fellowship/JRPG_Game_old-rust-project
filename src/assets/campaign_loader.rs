use std::path::{Path, PathBuf};
use std::ffi::OsStr;

use tokio::runtime::Runtime;

use macroquad::texture::load_texture;

//For config file loading and parsing 
use config::*;

//To locate all config files
extern crate walkdir;
use walkdir::WalkDir;


use super::{AssetDatabase, AssetContainer, SpriteSheet};
//use super::audio::{ClipCategory, AudioClip};


//loads the metadata for each campaign so we can display the options
//pub fn load_all_campaign_metadata(asset_db: &mut AssetDatabase) {//-> Config {
//}


//loads all data for a given campaign
pub fn load_campaign_data(path: &str, asset_db: &mut AssetDatabase) {
    
    //lets us run async processes
    let mut rt = Runtime::new().unwrap();

    //load all config files under the campain folder
    let mut campaign_config_paths = find_config_files(path);
    
    //for each config file, load it then load the associated asset
    while let Some(config_path) = campaign_config_paths.pop() {
        
        let config =
        match load_config(&config_path) {
            Ok(config) => config,
            Err(e) => {
                warn!("[Asset Loading] Could not load config file. Following error returned: {}", e);
                continue //skip this config file. TODO never gets to the error message at end of loop
            },
        };

        //TODO make type case insensitive
        let asset_was_loaded = match config.get_str("type").unwrap_or("".to_string()).as_str() {
            "sprite sheet" => rt.block_on(load_sprite_sheet(&config, &config_path, asset_db)),
            //"audio clip" => load_audio_clip(&config, &config_path, asset_db),
            _ => {
                warn!("[Asset Loading] 'Type' key does not exist or value is not supported. Config File Path: {}",
                       config_path.to_str().unwrap());
                false
            },
        };

        //do some extra logging to help bring errors to people's attention.
        if asset_was_loaded { 
            info!("[Asset Loading] Loaded asset relating to config file {}", 
                  config_path.to_str().unwrap());
        } else {
            error!("[Asset Loading] Failed to load asset relating to config file {}. {}", 
                   config_path.to_str().unwrap(),
                   "Please review previous warnings."
            );
        }
    }
}


//make sure file is one of the right formats for configuration files
fn is_config_ext(file_path: &Path) -> bool {
   match file_path.extension().and_then(OsStr::to_str) {
       Some("yml") => true,
       _ => false
   }
}


//locates all config files under a given path recursivly
fn find_config_files(path: &str) -> Vec<PathBuf> {
    let mut config_file_paths = vec![];

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if is_config_ext( entry.path() ) 
        && entry.path().file_stem().unwrap_or(OsStr::new("")) != "campaign" {

            config_file_paths.push(entry.into_path());

        }
    }
    
    config_file_paths
}



//creates a task for loading a config file and it's resources
fn load_config(path: &PathBuf) -> Result<Config, String> {
    
    //coerce into string value or return error
    let str_path = match path.to_str() {
        Some(string) => string,

        //Will be logged in the function that runs this function.
        None => return Err(format!("Could not convert path to str. Path: {:?}", path)),
    };

    //create the config struct and load in the given file either retuning populated
    //   config file or a relevant error
    let mut config_data = Config::default();
    match config_data.merge(File::with_name(&str_path)) {
        Ok(_) => Ok(config_data),

        //Coerce err to an error type we can return. 
        //Will be logged in the function that runs this function.
        Err(err) => Err( err.to_string() ),
    }

}


//load sprite sheets
async fn load_sprite_sheet(config: &Config, config_path: &PathBuf, 
                       asset_db: &mut AssetDatabase) -> bool {

    //pull data we need and validate
    let file = config.get_str("file");
    let id = config.get_str("asset id");
    let rows = config.get_int("rows");
    let columns = config.get_int("columns");
    let animations = config.get_table("animations");

    if file.is_err() || id.is_err() || rows.is_err() || columns.is_err() {
        let err_msg_head = format!("{} {} {}. {}",
                               "[Asset Loading]",
                               "Could not find required config value for sprite sheet type in config file",
                               config_path.to_str().unwrap_or("<error could not convert config path to str>"),
                               "Error follows: ");

        if let Err(err) = file { warn!("{} {}", err_msg_head, err); }
        if let Err(err) = id { warn!("{} {}", err_msg_head, err); }
        if let Err(err) = rows { warn!("{} {}", err_msg_head, err); }
        if let Err(err) = columns { warn!("{} {}", err_msg_head, err); }

        return false //config missing required values
    }

    //if the id already exists (i.e. equals anything other then DoesNotExist), error out
    let asset_id = id.ok().expect("[spritesheet] ID failed to unwrap inexplicatly");
    if let AssetContainer::DoesNotExist = asset_db.get_asset(&asset_id) {
       //We are good, continue running the function normally
    } else {
        warn!("[Asset Loading] The ID {} is already used. Caused by loading {:?}", 
              asset_id, config_path
        );
        return false;
    }

    //process the file path and asset name to the right types

    // assume image path is given as relative to config path hence taking the parent as a starting point. 
    let image_path = match config_path.parent() {
           

        Some(dir_path) => {
            //TODO if the path string uses a delimiter (i.e. / or \) not supported by the 
            //      current OS finding the file will fail
            let relative_path = dir_path.join(
                file.ok().expect("File value is missing while loading.")
            );
            
            match relative_path.to_str() {
                Some(name) => name.to_string(),
                None => {
                    warn!("[Asset Loading] Could not convert Path into str.");
                    return false //name is not UTF-8 compatable so abort
                }
            }
        },

        //getting parent from path failed somehow. Shouldn't ever happen naturally.
        None => {
            warn!("{} {}", 
                  "[Asset Loading] Parent missing from config path when processing",
                  config_path.to_str().unwrap_or("<error could not convert config path to str>"),
            ); 
            return false;
        },
    };


    //try to load image
    let texture = load_texture(image_path.as_str()).await;

    /* Would like to do error handling on the above but don't think it is avalible.
    match  {
         Ok(image) => image,
         Err(err) => {
             warn!("[Asset Loading] Could not load Image at {} related to config file {}. Following error returned: {}", 
                   image_path.clone().to_str().unwrap_or("<error could not convert image path to str>"),
                   config_path.to_str().unwrap_or("<error could not convert config path to str>"),
                   err,
             );
             return false //load image failed.
         }
    };*/
                        

    //create sprite sheet, add animations, then add the new asset to the database
    let mut spritesheet = SpriteSheet::new( 
        texture,
        rows.ok().expect("row convert error") as i32, 
        columns.ok().expect("column convert error") as i32, 
    );
        
    if animations.is_ok() {
        for (animation_name, tuple_list) in animations.ok().unwrap().iter() {
            match tuple_list.clone().try_into::< Vec<(i32,i32)> >() {
                Ok(sprite_pos_array) => 
                    //TODO might want to do additional checking of data. 
                    //    No error is thrown for having an extra value regardless if it is an int or not.
                    //    Error branch will happen if a string is in 1st or 2nd location or if a tuple is 
                    //      replaced by something else.
                    spritesheet.add_animation(animation_name.clone(), sprite_pos_array),

                Err(err) => {
                    warn!("[Asset Loading] Animation {} does not follow form {} in config file {}. Following error returned: {}", 
                          animation_name,
                          "[ [row_1, col_1], ..., [row_n, col_n] ]",
                          config_path.to_str().unwrap_or("<error could not convert config path to str>"),
                          err,
                    );
                    continue;
                }
            }
        }
    }

    asset_db.add_asset(asset_id, AssetContainer::Spritesheet(spritesheet));
    return true;
}


/*//load sound clips
//TODO, maybe should make this return a task also?
fn load_audio_clip(config: &Config, config_path: &PathBuf, asset_db: &mut AssetDatabase) -> bool {

    //pull data we need and validate
    let file = config.get_str("file");
    let category = config.get_str("category");

    if file.is_err() || category.is_err() {
        let err_msg_head = format!("{} {} {}. {}",
                               "[Asset Loading]",
                               "Could not find required config value for audio clip type in config file",
                               config_path.to_str().unwrap_or("<error could not convert config path to str>"),
                               "Error follows: ");

        if let Err(err) = file { warn!("{} {}", err_msg_head, err); }
        if let Err(err) = category { warn!("{} {}", err_msg_head, err); }

        return false //config missing required values
    }

    //TODO make case insensitive
    let clip_category = match category.unwrap().as_str() {
         "voice"   => ClipCategory::Voice,
         "music"   => ClipCategory::Music,
         "effects" => ClipCategory::Effects,
         failed_category => {
             warn!("[Asset Loading] Provided audio category '{}' is not a valid option. Related to config file {}.",
                   failed_category,
                   config_path.to_str().unwrap_or("<error could not convert config path to str>"),
             );
             return false;
         }
    };

    // assume image path is given as relative to config path hence taking the parent as a starting point. 
    let audio_path = match config_path.parent() {
           
        Some(dir_path) => dir_path.join(file.ok().expect("File value is missing while loading.")),

        //getting parent from path failed somehow. Shouldn't ever happen naturally.
        None => {
            warn!("{} {}", 
                  "[Asset Loading] Parent missing from config path when processing",
                  config_path.to_str().unwrap_or("<error could not convert config path to str>"),
            ); 
            return false;
        },
    };


    let asset_name = match audio_path.clone().into_os_string().into_string() {
        Ok(name) => name,
        Err(err) => {
            warn!("[Asset Loading] {}", 
                  err.into_string().unwrap_or("<Could not convert OsString err into string>".to_string()));
            return false; //name is not UTF-8 compatable so abort
        }
    }; 

    let audio_clip = AudioClip::new(audio_path, clip_category);

    asset_db.add_asset(asset_name, AssetContainer::AudioClip(audio_clip));
    return true;
}*/

