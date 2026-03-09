//! Audio Decoder
//! 
//! Decodes audio streams using Symphonia.

use symphonia::core::formats::FormatReader;
use symphonia::core::codecs::Decoder;

/// Audio decoder
pub struct AudioDecoder {
    /// Format reader
    format: Box<dyn FormatReader>,
    
    /// Decoder
    decoder: Box<dyn Decoder>,
    
    /// Track ID
    track_id: u32,
}

impl AudioDecoder {
    pub fn new(format: Box<dyn FormatReader>, decoder: Box<dyn Decoder>, track_id: u32) -> Self {
        Self {
            format,
            decoder,
            track_id,
        }
    }
    
    pub fn decode_next(&mut self) -> symphonia::core::errors::Result<Option<symphonia::core::audio::AudioBufferRef>> {
        use symphonia::core::formats::Packet;
        
        loop {
            let packet = match self.format.next_packet() {
                Ok(packet) => packet,
                Err(symphonia::core::errors::Error::ResetRequired) => {
                    // Reset by seeking to the beginning
                    use symphonia::core::formats::SeekTo;
                    let _ = self.format.seek(
                        symphonia::core::formats::SeekMode::Coarse,
                        SeekTo::TimeStamp { ts: 0, track_id: self.track_id },
                    );
                    continue;
                }
                Err(err) => return Err(err),
            };
            
            if packet.track_id() != self.track_id {
                continue;
            }
            
            match self.decoder.decode(&packet) {
                Ok(audio_buf) => return Ok(Some(audio_buf)),
                Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
                Err(err) => return Err(err),
            }
        }
    }
}