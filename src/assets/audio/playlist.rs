use super::audio_clip::AudioClip;
use rodio::Sink;


//intended for looping a music track
pub struct Playlist {
    music_tracks: Vec<AudioClip>,
    audio_queue: Sink,
}

impl Playlist {

    pub fn new(music_tracks: Vec<AudioClip>) -> Playlist {
        let sink = 
            match &super::get_audio_device() {
                Some(device) => Sink::new( device ),
                None => {
                    error!("[Audio/Playlist] No audio device was found.");
                    Sink::new_idle().0
                },
            };
        

        //TODO should pull from some sort of settings structure and get updated whenever that changes
        sink.set_volume(0.2);

        Playlist {
            music_tracks,
            audio_queue: sink,
        }
    }

    //make sure the playlist is still looping
    pub fn maintain_looping(&mut self) {
        //if the current song is finished we take the next song, 
        //  add it to the queue, and place it at the end of the playlist
        if self.audio_queue.empty() {
            let next = self.music_tracks.remove(0);
            self.audio_queue.append(next.make_source());
            self.music_tracks.push(next);
        }
    }

}
