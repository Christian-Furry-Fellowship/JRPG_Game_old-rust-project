use std::collections::HashMap;
use std::collections::hash_map::IterMut;

use config::Config;

use super::SpriteSheet;

pub enum AssetContainer {
    DoesNotExist, //does not exist
    Spritesheet(SpriteSheet),
    ConfigStore(Config),
}


pub struct AssetDatabase {
    does_not_exist: AssetContainer,
    assets: HashMap<String, AssetContainer>,
}


impl AssetDatabase {

    pub fn new() -> AssetDatabase {
        AssetDatabase {
            does_not_exist: AssetContainer::DoesNotExist,
            assets: HashMap::new(),
        }
    }

    pub fn get_asset(&mut self, asset_name: &String) -> &mut AssetContainer {
        match self.assets.get_mut(asset_name) {
            Option::Some(asset_container) => asset_container,
            Option::None => &mut self.does_not_exist
        }
    }

    pub fn add_asset(&mut self, name: String, data: AssetContainer) -> Option<AssetContainer> {
        self.assets.insert(name, data)
    }

    pub fn get_asset_iter_mut(&mut self) -> IterMut<String, AssetContainer> {
        self.assets.iter_mut()
    }
}
