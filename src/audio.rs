use std::time::Duration;
use rodio::source::{SineWave, Source};

pub struct AudioPlayer {
    _handle: rodio::MixerDeviceSink, 
    player: rodio::Player,
}

impl AudioPlayer {

    pub fn new() -> Self {

        let mut handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");

        handle.log_on_drop(false);
        
        let player = rodio::Player::connect_new(&handle.mixer());
        
        Self {
            _handle: handle, 
            player,
        }

    }

    pub fn beep(&self, freq: f32, ms: u16, vol: f32) {

        let source = SineWave::new(freq)
            .take_duration(Duration::from_millis(ms as u64))
            .amplify(vol);

        self.player.append(source);
        self.player.sleep_until_end();
        
    }

}