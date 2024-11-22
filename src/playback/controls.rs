use std::{fs::File, time::Duration};

use rodio::{
    decoder::DecoderError, source::SeekError, Decoder, OutputStream, OutputStreamHandle, Sink,
    Source,
};

#[derive(Default)]
pub struct PlaybackInfo {
    pub total_duration: Duration,
}

pub struct MediaControls {
    sink: Sink,
    #[allow(dead_code)]
    stream: OutputStream,
    #[allow(dead_code)]
    stream_handle: OutputStreamHandle,

    pub playback_info: PlaybackInfo,
}

impl Default for MediaControls {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        MediaControls {
            stream_handle,
            stream,
            sink,

            playback_info: PlaybackInfo::default(),
        }
    }
}

impl MediaControls {
    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn get_pos(&self) -> Duration {
        self.sink.get_pos()
    }

    pub fn append(&mut self, file: File) -> Result<(), DecoderError> {
        Decoder::new(file).map(|source| {
            if let Some(total_duration) = source.total_duration() {
                self.playback_info.total_duration = total_duration;
            }
            self.sink.append(source)
        })
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn seek(&self, position: Duration) -> Result<(), SeekError> {
        self.sink.try_seek(position)
    }
}
