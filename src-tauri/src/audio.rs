use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Option<Sink>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| AppError::Rodio(e.to_string()))?;

        Ok(Self {
            _stream: stream,
            stream_handle,
            sink: None,
        })
    }

    pub fn play_adhan(&mut self, path: &str) -> Result<()> {
        // Stop any currently playing audio
        self.stop();

        // Create new sink
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| AppError::Rodio(e.to_string()))?;

        // Resolve path - try both absolute and relative to resources
        let audio_path = if Path::new(path).exists() {
            PathBuf::from(path)
        } else {
            // Try relative to app directory
            PathBuf::from(format!("../{}", path))
        };

        println!("Playing audio from: {:?}", audio_path);

        let file = File::open(&audio_path)
            .map_err(|e| AppError::Custom(format!("Failed to open audio file '{}': {}", path, e)))?;
        
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| AppError::Rodio(format!("Failed to decode audio: {}", e)))?;
        
        sink.append(source);
        sink.play();

        self.sink = Some(sink);

        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        if let Some(ref sink) = self.sink {
            sink.set_volume(volume.clamp(0.0, 1.0));
        }
    }

    pub fn is_playing(&self) -> bool {
        self.sink.as_ref().map(|s| !s.empty()).unwrap_or(false)
    }

    pub fn pause(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.pause();
        }
    }

    pub fn resume(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.play();
        }
    }
}
