
mod audio_clip;
pub use audio_clip::{AudioClip, ClipCategory};

mod playlist;
pub use playlist::Playlist;



//A bug on windows makes it so cpal and glute and SDL in the same thread causes a crash.
//  The fix is to spawn a temp thread and 
pub fn get_audio_device() -> Option<rodio::Device> {
    std::thread::spawn(|| rodio::default_output_device())
        .join()
        .expect("[Audio] Merging thread while getting default audio device failed.")
}
