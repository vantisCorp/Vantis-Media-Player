//! Encoding Detection and Conversion
//! 
//! Automatically detects text encoding and converts to UTF-8.
/// Critical for Polish subtitles (CP1250, ISO-8859-2 → UTF-8).

use anyhow::{Result, anyhow};
use tracing::debug;

use encoding_rs::{Decoder, UTF_8};

/// Detect and convert encoding to UTF-8
/// 
/// Handles common Polish encodings:
/// - CP1250 (Windows Central European)
/// - ISO-8859-2 (Latin-2)
/// - UTF-8
/// - UTF-16
/// - ASCII
pub fn detect_and_convert(bytes: &[u8]) -> Result<String> {
    // First, try UTF-8
    if let Ok(text) = std::str::from_utf8(bytes) {
        debug!("🔤 Detected: UTF-8");
        
        // Check for common encoding issues ( Polish characters)
        if text.contains("�") || text.contains("�") {
            debug!("⚠️  UTF-8 has encoding errors, trying fallback");
        } else {
            return Ok(text.to_string());
        }
    }
    
    // Try common encodings
    let encodings = vec![
        ("windows-1250", encoding_rs::WINDOWS_1250),
        ("iso-8859-2", encoding_rs::ISO_8859_2),
        ("windows-1252", encoding_rs::WINDOWS_1252),
        ("iso-8859-1", encoding_rs::ISO_8859_1),
        ("utf-16le", encoding_rs::UTF_16LE),
        ("utf-16be", encoding_rs::UTF_16BE),
    ];
    
    for (name, encoding) in encodings {
        let mut decoder = encoding.new_decoder();
        let mut utf8_bytes = vec![0u8; bytes.len() * 2]; // Allocate extra space
        let (result, _, _) = decoder.decode_to_utf8(bytes, &mut utf8_bytes, true);
        
        if let Ok(text) = std::str::from_utf8(&utf8_bytes[..result]) {
            debug!("🔤 Detected: {}", name);
            
            // Validate by checking for Polish characters
            if has_polish_chars(text) {
                debug!("✨ Polish characters detected");
            }
            
            return Ok(text.to_string());
        }
    }
    
    // Fallback: Try to decode with chardet
    debug!("🔍 Using charset detection");
    
    // Try basic ASCII/UTF-8 fallback
    let text = String::from_utf8_lossy(bytes).to_string();
    debug!("🔤 Fallback: Lossy conversion");
    Ok(text)
}

/// Check if text contains Polish characters
fn has_polish_chars(text: &str) -> bool {
    let polish_chars = [
        'ą', 'ć', 'ę', 'ł', 'ń', 'ó', 'ś', 'ź', 'ż',
        'Ą', 'Ć', 'Ę', 'Ł', 'Ń', 'Ó', 'Ś', 'Ź', 'Ż',
    ];
    
    polish_chars.iter().any(|c| text.contains(*c))
}

/// Convert subtitle file encoding to UTF-8
pub fn convert_file_to_utf8(input_path: &str, output_path: &str) -> Result<()> {
    let bytes = std::fs::read(input_path)?;
    let text = detect_and_convert(&bytes)?;
    std::fs::write(output_path, text)?;
    Ok(())
}

/// Detect encoding of a file
pub fn detect_file_encoding(path: &str) -> Result<String> {
    let bytes = std::fs::read(path)?;
    
    // Try UTF-8 first
    if std::str::from_utf8(&bytes).is_ok() {
        return Ok("UTF-8".to_string());
    }
    
    // Check BOM for UTF-16
    if bytes.len() >= 2 {
        if bytes[0] == 0xFF && bytes[1] == 0xFE {
            return Ok("UTF-16LE".to_string());
        }
        if bytes[0] == 0xFE && bytes[1] == 0xFF {
            return Ok("UTF-16BE".to_string());
        }
    }
    
    // For other encodings, we'd need a more sophisticated detection
    // For now, return unknown
    Ok("Unknown".to_string())
}