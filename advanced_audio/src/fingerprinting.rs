//! Audio Fingerprinting Engine
//! 
//! Provides audio fingerprinting for music recognition and content identification
//! using Chromaprint and custom FFT-based algorithms.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, error, debug, warn};
use rustfft::{FftPlanner, num_complex::Complex};
use chromaprint::{Context, Fingerprinter};

use crate::FingerprintAlgorithm;

/// Audio fingerprinter
pub struct AudioFingerprinter {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Chromaprint fingerprinter
    chromaprint_fingerprinter: Arc<RwLock<Option<Context>>>,
    
    /// Fingerprint database
    database: Arc<RwLock<FingerprintDatabase>>,
    
    /// Current algorithm
    algorithm: Arc<RwLock<FingerprintAlgorithm>>,
    
    /// Confidence threshold
    confidence_threshold: Arc<RwLock<f32>>,
}

/// Fingerprint database
#[derive(Debug, Clone)]
pub struct FingerprintDatabase {
    /// Fingerprint entries
    pub entries: Vec<FingerprintEntry>,
    
    /// Index for fast lookup
    pub index: HashMap<u32, Vec<usize>>,
}

/// Fingerprint entry
#[derive(Debug, Clone)]
pub struct FingerprintEntry {
    /// Unique ID
    pub id: String,
    
    /// Track title
    pub title: String,
    
    /// Artist
    pub artist: String,
    
    /// Album
    pub album: String,
    
    /// Duration in seconds
    pub duration: u32,
    
    /// Fingerprint data
    pub fingerprint: Vec<u32>,
    
    /// Fingerprint algorithm used
    pub algorithm: FingerprintAlgorithm,
}

/// Recognition result
#[derive(Debug, Clone)]
pub struct RecognitionResult {
    /// Matched track
    pub track: FingerprintEntry,
    
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    
    /// Match offset in seconds
    pub offset: f32,
    
    /// Match duration in seconds
    pub duration: f32,
}

/// Fingerprint analysis
#[derive(Debug, Clone)]
pub struct FingerprintAnalysis {
    /// Fingerprint data
    pub fingerprint: Vec<u32>,
    
    /// Duration in seconds
    pub duration: f32,
    
    /// Sample rate
    pub sample_rate: u32,
    
    /// Number of channels
    pub channels: u32,
    
    /// Algorithm used
    pub algorithm: FingerprintAlgorithm,
}

impl AudioFingerprinter {
    /// Create a new audio fingerprinter
    pub fn new() -> Result<Self> {
        info!("Initializing audio fingerprinter");
        
        let chromaprint_fingerprinter = Context::new()
            .map_err(|e| anyhow::anyhow!("Failed to create Chromaprint context: {}", e))?;
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            chromaprint_fingerprinter: Arc::new(RwLock::new(Some(chromaprint_fingerprinter))),
            database: Arc::new(RwLock::new(FingerprintDatabase {
                entries: Vec::new(),
                index: HashMap::new(),
            })),
            algorithm: Arc::new(RwLock::new(FingerprintAlgorithm::Chromaprint)),
            confidence_threshold: Arc::new(RwLock::new(0.8)),
        })
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                *self.is_initialized.read().await
            })
        })
    }
    
    /// Set fingerprint algorithm
    pub async fn set_algorithm(&self, algorithm: FingerprintAlgorithm) -> Result<()> {
        *self.algorithm.write().await = algorithm;
        info!("Fingerprint algorithm set to {:?}", algorithm);
        Ok(())
    }
    
    /// Set confidence threshold
    pub async fn set_confidence_threshold(&self, threshold: f32) -> Result<()> {
        *self.confidence_threshold.write().await = threshold.clamp(0.0, 1.0);
        info!("Confidence threshold set to {}", threshold);
        Ok(())
    }
    
    /// Generate fingerprint from audio samples
    pub async fn generate_fingerprint(&self, samples: &[f32], sample_rate: u32) -> Result<FingerprintAnalysis> {
        let algorithm = *self.algorithm.read().await;
        
        match algorithm {
            FingerprintAlgorithm::Chromaprint => {
                self.generate_chromaprint_fingerprint(samples, sample_rate).await
            }
            FingerprintAlgorithm::CustomFFT => {
                self.generate_custom_fft_fingerprint(samples, sample_rate).await
            }
            FingerprintAlgorithm::DeepLearning => {
                self.generate_deep_learning_fingerprint(samples, sample_rate).await
            }
        }
    }
    
    /// Generate Chromaprint fingerprint
    async fn generate_chromaprint_fingerprint(&self, samples: &[f32], sample_rate: u32) -> Result<FingerprintAnalysis> {
        debug!("Generating Chromaprint fingerprint");
        
        let mut fingerprinter = self.chromaprint_fingerprinter.write().await;
        let fp = fingerprinter.as_mut().context("Chromaprint not initialized")?;
        
        // Start fingerprinting
        fp.start(sample_rate, 1)?;
        
        // Process samples in chunks
        let chunk_size = 4096;
        for chunk in samples.chunks(chunk_size) {
            // Convert f32 to i16
            let i16_samples: Vec<i16> = chunk.iter().map(|&s| (s * 32767.0) as i16).collect();
            fp.process(&i16_samples)?;
        }
        
        // Finish and get fingerprint
        fp.finish()?;
        let fingerprint = fp.fingerprint();
        
        Ok(FingerprintAnalysis {
            fingerprint,
            duration: samples.len() as f32 / sample_rate as f32,
            sample_rate,
            channels: 1,
            algorithm: FingerprintAlgorithm::Chromaprint,
        })
    }
    
    /// Generate custom FFT-based fingerprint
    async fn generate_custom_fft_fingerprint(&self, samples: &[f32], sample_rate: u32) -> Result<FingerprintAnalysis> {
        debug!("Generating custom FFT fingerprint");
        
        // Compute FFT
        let fft_size = 2048;
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);
        
        let mut spectrum = vec![Complex::new(0.0, 0.0); fft_size];
        for (i, &sample) in samples.iter().take(fft_size).enumerate() {
            spectrum[i] = Complex::new(sample as f64, 0.0);
        }
        
        fft.process(&mut spectrum);
        
        // Extract features from spectrum
        let mut fingerprint = Vec::new();
        for (i, complex) in spectrum.iter().take(fft_size / 2).enumerate() {
            let magnitude = complex.norm() as f32;
            let phase = complex.arg() as f32;
            
            // Quantize magnitude and phase
            let mag_quantized = (magnitude * 1000.0) as u32;
            let phase_quantized = ((phase + std::f32::consts::PI) / (2.0 * std::f32::consts::PI) * 255.0) as u32;
            
            fingerprint.push(mag_quantized);
            fingerprint.push(phase_quantized);
        }
        
        Ok(FingerprintAnalysis {
            fingerprint,
            duration: samples.len() as f32 / sample_rate as f32,
            sample_rate,
            channels: 1,
            algorithm: FingerprintAlgorithm::CustomFFT,
        })
    }
    
    /// Generate deep learning-based fingerprint
    async fn generate_deep_learning_fingerprint(&self, samples: &[f32], sample_rate: u32) -> Result<FingerprintAnalysis> {
        debug!("Generating deep learning fingerprint");
        
        // For now, use FFT-based approach as placeholder
        // In a real implementation, this would use a neural network
        self.generate_custom_fft_fingerprint(samples, sample_rate).await
    }
    
    /// Recognize audio from fingerprint
    pub async fn recognize(&self, analysis: &FingerprintAnalysis) -> Result<Option<RecognitionResult>> {
        let database = self.database.read().await;
        let confidence_threshold = *self.confidence_threshold.read().await;
        
        if database.entries.is_empty() {
            return Ok(None);
        }
        
        let mut best_match: Option<(usize, f32)> = None;
        
        // Compare with all entries in database
        for (entry_idx, entry) in database.entries.iter().enumerate() {
            if entry.algorithm != analysis.algorithm {
                continue;
            }
            
            let similarity = self.calculate_similarity(&analysis.fingerprint, &entry.fingerprint)?;
            
            if similarity > confidence_threshold {
                match best_match {
                    None => best_match = Some((entry_idx, similarity)),
                    Some((_, best_similarity)) => {
                        if similarity > best_similarity {
                            best_match = Some((entry_idx, similarity));
                        }
                    }
                }
            }
        }
        
        match best_match {
            Some((entry_idx, confidence)) => {
                let track = database.entries[entry_idx].clone();
                Ok(Some(RecognitionResult {
                    track,
                    confidence,
                    offset: 0.0,
                    duration: analysis.duration,
                }))
            }
            None => Ok(None),
        }
    }
    
    /// Calculate similarity between two fingerprints
    fn calculate_similarity(&self, fp1: &[u32], fp2: &[u32]) -> Result<f32> {
        let min_len = fp1.len().min(fp2.len);
        if min_len == 0 {
            return Ok(0.0);
        }
        
        let mut matches = 0;
        for i in 0..min_len {
            if fp1[i] == fp2[i] {
                matches += 1;
            }
        }
        
        Ok(matches as f32 / min_len as f32)
    }
    
    /// Add fingerprint to database
    pub async fn add_to_database(&self, entry: FingerprintEntry) -> Result<()> {
        let mut database = self.database.write().await;
        
        let entry_idx = database.entries.len();
        
        // Index fingerprint for fast lookup
        for (i, &hash) in entry.fingerprint.iter().enumerate() {
            database.index.entry(hash).or_insert_with(Vec::new).push(entry_idx);
        }
        
        database.entries.push(entry);
        
        info!("Added fingerprint to database (total: {})", database.entries.len());
        Ok(())
    }
    
    /// Remove fingerprint from database
    pub async fn remove_from_database(&self, id: &str) -> Result<bool> {
        let mut database = self.database.write().await;
        
        let entry_idx = database.entries.iter().position(|e| e.id == id);
        
        match entry_idx {
            Some(idx) => {
                // Remove from index
                for hash in database.entries[idx].fingerprint.iter() {
                    if let Some(entries) = database.index.get_mut(hash) {
                        entries.retain(|&i| i != idx);
                    }
                }
                
                // Remove entry
                database.entries.remove(idx);
                
                info!("Removed fingerprint from database (total: {})", database.entries.len());
                Ok(true)
            }
            None => Ok(false),
        }
    }
    
    /// Search database by metadata
    pub async fn search_database(&self, query: &str) -> Result<Vec<FingerprintEntry>> {
        let database = self.database.read().await;
        
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        for entry in &database.entries {
            if entry.title.to_lowercase().contains(&query_lower)
                || entry.artist.to_lowercase().contains(&query_lower)
                || entry.album.to_lowercase().contains(&query_lower)
            {
                results.push(entry.clone());
            }
        }
        
        Ok(results)
    }
    
    /// Get database statistics
    pub async fn get_database_stats(&self) -> DatabaseStats {
        let database = self.database.read().await;
        
        DatabaseStats {
            total_entries: database.entries.len(),
            total_fingerprints: database.entries.iter().map(|e| e.fingerprint.len()).sum(),
            index_size: database.index.len(),
            algorithms: {
                let mut algos = HashMap::new();
                for entry in &database.entries {
                    *algos.entry(entry.algorithm).or_insert(0) += 1;
                }
                algos
            },
        }
    }
    
    /// Export database
    pub async fn export_database(&self) -> Result<String> {
        let database = self.database.read().await;
        serde_json::to_string_pretty(&*database)
            .context("Failed to serialize database")
    }
    
    /// Import database
    pub async fn import_database(&self, data: &str) -> Result<()> {
        let imported: FingerprintDatabase = serde_json::from_str(data)
            .context("Failed to deserialize database")?;
        
        let mut database = self.database.write().await;
        database.entries.extend(imported.entries);
        
        // Rebuild index
        database.index.clear();
        for (entry_idx, entry) in database.entries.iter().enumerate() {
            for hash in entry.fingerprint.iter() {
                database.index.entry(*hash).or_insert_with(Vec::new).push(entry_idx);
            }
        }
        
        info!("Imported database (total: {} entries)", database.entries.len());
        Ok(())
    }
}

/// Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub total_entries: usize,
    pub total_fingerprints: usize,
    pub index_size: usize,
    pub algorithms: HashMap<FingerprintAlgorithm, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_fingerprinter_creation() {
        let fingerprinter = AudioFingerprinter::new().unwrap();
        assert!(fingerprinter.is_initialized());
    }
    
    #[tokio::test]
    async fn test_chromaprint_fingerprint() {
        let fingerprinter = AudioFingerprinter::new().unwrap();
        
        // Generate test audio
        let samples: Vec<f32> = (0..48000).map(|i| (i as f32 / 48000.0 * 2.0 * std::f32::consts::PI * 440.0).sin() as f32).collect();
        
        let analysis = fingerprinter.generate_fingerprint(&samples, 48000).await.unwrap();
        assert!(!analysis.fingerprint.is_empty());
        assert_eq!(analysis.algorithm, FingerprintAlgorithm::Chromaprint);
    }
    
    #[tokio::test]
    async fn test_custom_fft_fingerprint() {
        let fingerprinter = AudioFingerprinter::new().unwrap();
        fingerprinter.set_algorithm(FingerprintAlgorithm::CustomFFT).await.unwrap();
        
        let samples: Vec<f32> = (0..48000).map(|i| (i as f32 / 48000.0 * 2.0 * std::f32::consts::PI * 440.0).sin() as f32).collect();
        
        let analysis = fingerprinter.generate_fingerprint(&samples, 48000).await.unwrap();
        assert!(!analysis.fingerprint.is_empty());
        assert_eq!(analysis.algorithm, FingerprintAlgorithm::CustomFFT);
    }
    
    #[tokio::test]
    async fn test_database_operations() {
        let fingerprinter = AudioFingerprinter::new().unwrap();
        
        let entry = FingerprintEntry {
            id: "test1".to_string(),
            title: "Test Song".to_string(),
            artist: "Test Artist".to_string(),
            album: "Test Album".to_string(),
            duration: 180,
            fingerprint: vec![1, 2, 3, 4, 5],
            algorithm: FingerprintAlgorithm::Chromaprint,
        };
        
        fingerprinter.add_to_database(entry).await.unwrap();
        
        let stats = fingerprinter.get_database_stats().await;
        assert_eq!(stats.total_entries, 1);
    }
    
    #[tokio::test]
    async fn test_recognition() {
        let fingerprinter = AudioFingerprinter::new().unwrap();
        
        let entry = FingerprintEntry {
            id: "test1".to_string(),
            title: "Test Song".to_string(),
            artist: "Test Artist".to_string(),
            album: "Test Album".to_string(),
            duration: 180,
            fingerprint: vec![1, 2, 3, 4, 5],
            algorithm: FingerprintAlgorithm::CustomFFT,
        };
        
        fingerprinter.add_to_database(entry).await.unwrap();
        
        let analysis = FingerprintAnalysis {
            fingerprint: vec![1, 2, 3, 4, 5],
            duration: 10.0,
            sample_rate: 48000,
            channels: 1,
            algorithm: FingerprintAlgorithm::CustomFFT,
        };
        
        let result = fingerprinter.recognize(&analysis).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().track.title, "Test Song");
    }
}