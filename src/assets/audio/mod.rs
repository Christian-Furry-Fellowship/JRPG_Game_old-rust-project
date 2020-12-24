
mod audio_clip;
pub use audio_clip::{AudioClip, ClipCategory};

mod playlist;
pub use playlist::Playlist;



//A bug on windows makes it so cpal and glute/SDL in the same thread causes a crash.
//  The fix is to spawn a temp thread and creat or do your action based on that

pub fn create_sink() -> rodio::Sink {
    std::thread::spawn(|| {
        match &rodio::default_output_device() {
            Some(device) => rodio::Sink::new( device ),
            None => {
                error!("[Audio/Playlist] No audio device was found.");
                rodio::Sink::new_idle().0
            },
        }
    })
        .join()
        .expect("[Audio] Merging thread while getting default audio device failed.")
}
