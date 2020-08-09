use std::path::{Path, PathBuf};
use std::ffi::OsStr;

use coffee::{
    load::Task,
    graphics::Image,
};
use coffee::graphics::Gpu;

use config::*;

extern crate walkdir;
use walkdir::WalkDir;


use super::{AssetDatabase, AssetContainer, SpriteSheet};


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
            Err(_) => continue, //TODO log error
        };

        //TODO make case insensitive
        match config.get_str("type").unwrap_or("".to_string()).as_str() {
            "sprite sheet" => load_sprite_sheet(&config, &config_path, gpu, asset_db),
            _ => println!("{}", "Type key does not exist or type malformed."),
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
        if is_config_ext( entry.path() ) {
            config_file_paths.push(entry.into_path());
        }
    }
    
    config_file_paths
}


//creates a task for loading a config file and it's resources
fn load_config_task(file_path: &PathBuf) -> Task<Config> {

    //TODO Should return error
    let str_path = file_path.to_str()
                            .expect("Could not convert file path to str")
                            .to_owned(); //needed so closure below can capture

    Task::new(move || {

        let mut config_data = Config::default();
        //TODO Replace unwrap with proper error handling
        config_data.merge(File::with_name(&str_path)).unwrap();

        Ok(config_data)
    })
}


//load sprite sheets
fn load_sprite_sheet(config: &Config, config_path: &PathBuf, 
                        gpu: &mut Gpu, asset_db: &mut AssetDatabase) {

        //pull data we need and validate
        let file = config.get_str("file");
        let rows = config.get_int("rows");
        let columns = config.get_int("columns");
        let animations = config.get_table("animations");

        if file.is_err() || rows.is_err() || columns.is_err() {
            return //config missing required values TODO log exactly what the error was 
        }


        //process the file path and asset name to the right types
        let image_path = config_path.parent()
                              .expect("Parent missing from config path") //TODO handle better
                              .join(file.ok().expect("File value is missing while loading."));

        let asset_name = match image_path.clone().into_os_string().into_string() {
            Ok(name) => name,
            Err(_) => return //name is not UTF-8 compatable TODO log exactly what the error was
        };


        //try to load image
        let image = match Image::load( image_path.clone() ).run(gpu) {
             Ok(image) => image,
             Err(_) => return //load image failed. TODO log exactly what the error was
        };
                        

        //create sprite sheet, add animations, then add the new asset to the database
        let mut spritesheet = SpriteSheet::new( 
            image,
            rows.ok().expect("row convert error") as u16, 
            columns.ok().expect("column convert error") as u16 
        );
        
        if animations.is_ok() {
            for (animation_name, tuple_list) in animations.ok().unwrap().iter() {
                match tuple_list.clone().try_into::< Vec<(u16,u16)> >() {
                    Ok(sprite_pos_array) => 
                        spritesheet.add_animation(animation_name.clone(), sprite_pos_array),
                    Err(_) => {
                        //TODO log error better
                        continue;
                    }
                }
            }
        }

        asset_db.add_asset(asset_name, AssetContainer::Spritesheet(spritesheet));
}

