use std::path::{Path, PathBuf};
use std::ffi::OsStr;

//For image loading and hooking into the Task system
use coffee::{
    load::Task,
    graphics::Image,
};
use coffee::graphics::Gpu;

//For config file loading and parsing 
use config::*;

//To locate all config files
extern crate walkdir;
use walkdir::WalkDir;


use super::{AssetDatabase, AssetContainer, SpriteSheet};
use super::audio::{ClipCategory, AudioClip};


//loads the metadata for each campaign so we can display the options
//pub fn load_all_campaign_metadata(asset_db: &mut AssetDatabase) {//-> Config {
//}


//loads all data for a given campaign
pub fn load_campaign_data(path: &str, gpu: &mut Gpu, asset_db: &mut AssetDatabase) {
    
    //load all config files under the campain folder
    let mut campaign_config_paths = find_config_files(path);
    
    //for each config file, load it then load the associated asset
    while let Some(config_path) = campaign_config_paths.pop() {
        
        let config =
        match load_config_task(&config_path).run(gpu) {
            Ok(config) => config,
            Err(e) => {
                warn!("[Asset Loading] Could not load config file. Following error returned: {}", e);
                continue //skip this config file. TODO never gets to the error message at end of loop
            },
        };

        //TODO make type case insensitive
        let asset_was_loaded = match config.get_str("type").unwrap_or("".to_string()).as_str() {
            "sprite sheet" => load_sprite_sheet(&config, &config_path, gpu, asset_db),
            "audio clip" => load_audio_clip(&config, &config_path, asset_db),
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




//utility function to create a coffee error since it's a bit of a pain.
fn make_coffee_err_from_str(msg: &str) -> coffee::Error {
    coffee::Error::IO(
        std::io::Error::new( std::io::ErrorKind::Other, msg )
    )
}

//creates a task for loading a config file and it's resources
fn load_config_task(file_path: &PathBuf) -> Task<Config> {
    //needed so closure below can capture
    let path = file_path.clone();
                     

    Task::new(move || {
        //coerce into string value or return error
        let str_path = match path.to_str() {
            Some(string) => string,

            //Will be logged in the function that runs the task.
            None => return Err(
                make_coffee_err_from_str("Config path cannot be converted to string.")
            ),
        };

        //create the config struct and load in the given file either retuning populated
        //   config file or a relevant error
        let mut config_data = Config::default();
        match config_data.merge(File::with_name(&str_path)) {
            Ok(_) => Ok(config_data),

            //Coerce err to an error type we can return. 
            //Will be logged in the function that runs the task.
            Err(err) => Err( make_coffee_err_from_str( err.to_string().as_str() ) ),
        }
    })
}


//load sprite sheets
//TODO, maybe should make this return a task also?
fn load_sprite_sheet(config: &Config, config_path: &PathBuf, 
                        gpu: &mut Gpu, asset_db: &mut AssetDatabase) -> bool {

    //pull data we need and validate
    let file = config.get_str("file");
    let rows = config.get_int("rows");
    let columns = config.get_int("columns");
    let animations = config.get_table("animations");

    if file.is_err() || rows.is_err() || columns.is_err() {
        let err_msg_head = format!("{} {} {}. {}",
                               "[Asset Loading]",
                               "Could not find required config value for sprite sheet type in config file",
                               config_path.to_str().unwrap_or("<error could not convert config path to str>"),
                               "Error follows: ");

        if let Err(err) = file { warn!("{} {}", err_msg_head, err); }
        if let Err(err) = rows { warn!("{} {}", err_msg_head, err); }
        if let Err(err) = columns { warn!("{} {}", err_msg_head, err); }

        return false //config missing required values
    }


    //process the file path and asset name to the right types

    // assume image path is given as relative to config path hence taking the parent as a starting point. 
    let image_path = match config_path.parent() {
           
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


    let asset_name = match image_path.clone().into_os_string().into_string() {
        Ok(name) => name,
        Err(err) => {
            warn!("[Asset Loading] {}", 
                  err.into_string().unwrap_or("<Could not convert OsString err into string>".to_string()));
            return false //name is not UTF-8 compatable so abort
        }
    };


    //try to load image
    let image = match Image::load( image_path.clone() ).run(gpu) {
         Ok(image) => image,
         Err(err) => {
             warn!("[Asset Loading] Could not load Image at {} related to config file {}. Following error returned: {}", 
                   image_path.clone().to_str().unwrap_or("<error could not convert image path to str>"),
                   config_path.to_str().unwrap_or("<error could not convert config path to str>"),
                   err,
             );
             return false //load image failed.
         }
    };
                        

    //create sprite sheet, add animations, then add the new asset to the database
    let mut spritesheet = SpriteSheet::new( 
        image,
        rows.ok().expect("row convert error") as u16, 
        columns.ok().expect("column convert error") as u16, 
    );
        
    if animations.is_ok() {
        for (animation_name, tuple_list) in animations.ok().unwrap().iter() {
            match tuple_list.clone().try_into::< Vec<(u16,u16)> >() {
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

    asset_db.add_asset(asset_name, AssetContainer::Spritesheet(spritesheet));
    return true;
}


//load sound clips
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
}

