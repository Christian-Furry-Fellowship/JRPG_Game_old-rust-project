use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use rodio::Decoder; //Source

//used to determin which volume control in the settings should be applied to this clip
pub enum ClipCategory {
    Voice,
    Music,
    Effects,
}


//Holds info for playing a single audio file.
pub struct AudioClip {
    path: PathBuf,
    clip_category: ClipCategory, //Used in volume control
}

impl AudioClip {

    pub fn new(path: PathBuf, clip_category: ClipCategory) -> AudioClip {
        AudioClip {
            path,
            clip_category
        }
    }

    //create a source 
    pub fn make_source(&self) -> rodio::Decoder<BufReader<File>> {
        let file = File::open(self.path.clone()).unwrap();
        Decoder::new(BufReader::new(file)).unwrap()
    }

    //immediatly play the audio clip. No control over this for volume or anything else
    /*pub fn play_once(&self) {
        let device = super::get_audio_device().unwrap();

        rodio::play_raw(&device, self.make_source().convert_samples());
    }*/

}
