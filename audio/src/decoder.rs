//! Audio Decoder
//! 
//! Decodes audio streams using Symphonia.

use symphonia::core::formats::FormatReader;
use symphonia::core::codecs::Decoder;
use symphonia::core::audio::SampleBuffer;

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
    
    /// Decode the next packet and return samples as f32 vec
    pub fn decode_next(&mut self) -> symphonia::core::errors::Result<Option<Vec<f32>>> {
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
                Ok(audio_buf) => {
                    // Copy decoded samples into a Vec<f32>
                    let spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;
                    let mut sample_buf = SampleBuffer::<f32>::new(duration, spec);
                    sample_buf.copy_interleaved_ref(audio_buf);
                    return Ok(Some(sample_buf.samples().to_vec()));
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
                Err(err) => return Err(err),
            }
        }
    }
}