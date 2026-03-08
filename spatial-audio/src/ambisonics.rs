//! Ambisonics encoding and decoding for spatial audio.
//!
//! This module provides ambisonics processing capabilities for immersive
//! audio experiences, supporting various ambisonics orders and formats.

use crate::error::{SpatialAudioError, SpatialAudioResult};
use crate::types::{AudioBuffer, AudioFormat, Position};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Ambisonics order (degree of spherical harmonics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmbisonicsOrder {
    /// First-order ambisonics (4 channels)
    First,
    /// Second-order ambisonics (9 channels)
    Second,
    /// Third-order ambisonics (16 channels)
    Third,
    /// Fourth-order ambisonics (25 channels)
    Fourth,
    /// Fifth-order ambisonics (36 channels)
    Fifth,
}

impl AmbisonicsOrder {
    /// Get the number of channels for this ambisonics order.
    pub fn channels(&self) -> usize {
        match self {
            AmbisonicsOrder::First => 4,
            AmbisonicsOrder::Second => 9,
            AmbisonicsOrder::Third => 16,
            AmbisonicsOrder::Fourth => 25,
            AmbisonicsOrder::Fifth => 36,
        }
    }

    /// Get the order as an integer.
    pub fn order(&self) -> usize {
        match self {
            AmbisonicsOrder::First => 1,
            AmbisonicsOrder::Second => 2,
            AmbisonicsOrder::Third => 3,
            AmbisonicsOrder::Fourth => 4,
            AmbisonicsOrder::Fifth => 5,
        }
    }

    /// Parse from order number.
    pub fn from_order(order: usize) -> Option<Self> {
        match order {
            1 => Some(AmbisonicsOrder::First),
            2 => Some(AmbisonicsOrder::Second),
            3 => Some(AmbisonicsOrder::Third),
            4 => Some(AmbisonicsOrder::Fourth),
            5 => Some(AmbisonicsOrder::Fifth),
            _ => None,
        }
    }
}

/// Ambisonics format conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmbisonicsFormat {
    /// Ambisonics B-format (Furse-Malham for first-order)
    BFormat,
    /// AmbiX format (ACN channel ordering, SN3D normalization)
    AmbiX,
    /// FuMA format (Furse-Malham ordering, MaxN normalization)
    FuMA,
    /// Google's Ambisonics format (ACN, SN3D)
    GoogleAmbisonics,
}

/// Ambisonics normalization types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Normalization {
    /// Maximum normalization (FuMA)
    MaxN,
    /// SN3D normalization (AmbiX)
    SN3D,
    /// N3D normalization
    N3D,
    /// Schmidt semi-normalization
    Schmidt,
}

impl Normalization {
    /// Get normalization factor for a given (n, m) component.
    pub fn factor(&self, n: usize, m: isize) -> f64 {
        match self {
            Normalization::MaxN => 1.0,
            Normalization::SN3D => {
                let m_abs = m.abs() as usize;
                let sign = if m >= 0 { 1.0 } else { -1.0 };
                sign * (2.0 * n as f64 + 1.0).sqrt()
                    / factorial(n - m_abs) as f64
                    * factorial(n + m_abs) as f64
            }
            Normalization::N3D => (2.0 * n as f64 + 1.0).sqrt(),
            Normalization::Schmidt => {
                let m_abs = m.abs() as usize;
                1.0 / (factorial(n + m_abs) as f64 * factorial(n - m_abs) as f64).sqrt()
            }
        }
    }
}

/// Calculate factorial.
fn factorial(n: usize) -> usize {
    if n == 0 || n == 1 {
        1
    } else {
        (2..=n).product()
    }
}

/// ACN (Ambisonics Channel Number) index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcnIndex {
    /// Degree (order) n
    pub n: usize,
    /// Index m (-n to n)
    pub m: isize,
}

impl AcnIndex {
    /// Create from degree n and index m.
    pub fn new(n: usize, m: isize) -> Option<Self> {
        if m.abs() as usize <= n {
            Some(AcnIndex { n, m })
        } else {
            None
        }
    }

    /// Convert from ACN channel index.
    pub fn from_acn(acn: usize) -> Self {
        let n = ((acn as f64).sqrt().floor()) as usize;
        let m = acn as isize - n * n - n as isize;
        AcnIndex { n, m }
    }

    /// Convert to ACN channel index.
    pub fn to_acn(&self) -> usize {
        self.n * self.n + self.n + self.m as usize
    }
}

/// Spherical harmonic coefficients.
#[derive(Debug, Clone)]
pub struct SphericalHarmonics {
    /// Order of the harmonics.
    order: AmbisonicsOrder,
    /// Coefficients for each ACN channel.
    coefficients: Vec<f64>,
}

impl SphericalHarmonics {
    /// Calculate spherical harmonics for a given direction.
    pub fn from_direction(order: AmbisonicsOrder, azimuth: f64, elevation: f64) -> Self {
        let num_channels = order.channels();
        let mut coefficients = vec![0.0; num_channels];

        // Convert to radians if needed
        let az = azimuth * PI / 180.0;
        let el = elevation * PI / 180.0;

        // Calculate spherical coordinates
        let sin_az = az.sin();
        let cos_az = az.cos();
        let sin_el = el.sin();
        let cos_el = el.cos();

        // Direction vector
        let x = cos_el * cos_az;
        let y = cos_el * sin_az;
        let z = sin_el;

        // Calculate coefficients using real spherical harmonics
        for acn in 0..num_channels {
            let acn_idx = AcnIndex::from_acn(acn);
            coefficients[acn] = Self::calculate_harmonic(acn_idx.n, acn_idx.m, x, y, z);
        }

        SphericalHarmonics { order, coefficients }
    }

    /// Calculate a single spherical harmonic.
    fn calculate_harmonic(n: usize, m: isize, x: f64, y: f64, z: f64) -> f64 {
        // Simplified real spherical harmonics calculation
        // For production, use a proper implementation or library
        
        if n == 0 {
            // W (omnidirectional)
            1.0
        } else if n == 1 {
            // First-order harmonics
            match m {
                -1 => y,  // Y
                0 => z,   // Z
                1 => x,   // X
                _ => 0.0,
            }
        } else if n == 2 {
            // Second-order harmonics
            let m_abs = m.abs() as usize;
            match m {
                -2 => 2.0 * x * y,           // V
                -1 => 2.0 * y * z,           // T
                0 => 3.0 * z * z - 1.0,      // R
                1 => 2.0 * x * z,            // S
                2 => x * x - y * y,          // U
                _ => 0.0,
            }
        } else {
            // Higher orders - simplified approximation
            // In production, use recurrence relations
            let r = (x * x + y * y + z * z).sqrt();
            if r > 0.0 {
                let theta = z.atan2((x * x + y * y).sqrt());
                let phi = y.atan2(x);
                
                // Associated Legendre polynomial approximation
                Self::associated_legendre(n, m.abs() as usize, theta.cos()) 
                    * Self::trigonometric_factor(m, phi)
            } else {
                0.0
            }
        }
    }

    /// Calculate associated Legendre polynomial.
    fn associated_legendre(n: usize, m: usize, x: f64) -> f64 {
        if m > n {
            return 0.0;
        }

        // Use recurrence relation
        let mut pmm = 1.0;
        if m > 0 {
            let mut somx2 = (1.0 - x * x).sqrt();
            let mut fact = 1.0;
            for _ in 1..=m {
                pmm *= -fact * somx2;
                fact += 2.0;
                somx2 = (1.0 - x * x).sqrt();
            }
        }

        if n == m {
            return pmm;
        }

        let mut pmmp1 = x * (2.0 * m as f64 + 1.0) * pmm;
        if n == m + 1 {
            return pmmp1;
        }

        let mut pll = 0.0;
        for ll in (m + 1)..=n {
            pll = ((2.0 * ll as f64 - 1.0) * x * pmmp1 - (ll + m - 1) as f64 * pmm)
                / (ll - m) as f64;
            pmm = pmmp1;
            pmmp1 = pll;
        }

        pll
    }

    /// Calculate trigonometric factor for spherical harmonic.
    fn trigonometric_factor(m: isize, phi: f64) -> f64 {
        if m > 0 {
            2.0f64.sqrt() * (m as f64 * phi).cos()
        } else if m < 0 {
            2.0f64.sqrt() * ((-m) as f64 * phi).sin()
        } else {
            1.0
        }
    }

    /// Get the coefficients.
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    /// Get a specific coefficient by ACN index.
    pub fn get(&self, acn: usize) -> Option<f64> {
        self.coefficients.get(acn).copied()
    }

    /// Get the order.
    pub fn order(&self) -> AmbisonicsOrder {
        self.order
    }
}

/// Ambisonics encoder for encoding mono sources to B-format.
#[derive(Debug, Clone)]
pub struct AmbisonicsEncoder {
    /// Ambisonics order.
    order: AmbisonicsOrder,
    /// Output format.
    format: AmbisonicsFormat,
    /// Sample rate.
    sample_rate: u32,
}

impl AmbisonicsEncoder {
    /// Create a new ambisonics encoder.
    pub fn new(order: AmbisonicsOrder, format: AmbisonicsFormat, sample_rate: u32) -> Self {
        AmbisonicsEncoder {
            order,
            format,
            sample_rate,
        }
    }

    /// Encode a mono source at a given position to ambisonics.
    pub fn encode(&self, input: &AudioBuffer, position: &Position) -> SpatialAudioResult<AudioBuffer> {
        if input.channels() != 1 {
            return Err(SpatialAudioError::InvalidInput(
                "Ambisonics encoder expects mono input".to_string(),
            ));
        }

        let num_channels = self.order.channels();
        let samples = input.samples();
        
        // Calculate spherical harmonics for the position
        let azimuth = position.azimuth_degrees();
        let elevation = position.elevation_degrees();
        let sh = SphericalHarmonics::from_direction(self.order, azimuth, elevation);

        // Create output buffer
        let mut output = AudioBuffer::new(
            AudioFormat::new(num_channels, self.sample_rate),
            samples,
        );

        // Apply gain based on distance
        let distance_gain = 1.0 / (1.0 + position.distance().max(0.001));

        // Encode by multiplying mono source with spherical harmonic coefficients
        for (ch, &coeff) in sh.coefficients().iter().enumerate() {
            let channel_gain = coeff * distance_gain;
            for sample in 0..samples {
                let value = input.get_sample(0, sample) * channel_gain as f32;
                output.set_sample(ch, sample, value);
            }
        }

        // Apply format-specific processing
        self.apply_format(&mut output)?;

        Ok(output)
    }

    /// Apply format-specific normalization and channel ordering.
    fn apply_format(&self, buffer: &mut AudioBuffer) -> SpatialAudioResult<()> {
        match self.format {
            AmbisonicsFormat::AmbiX | AmbisonicsFormat::GoogleAmbisonics => {
                // ACN ordering with SN3D normalization - already correct
                // Apply SN3D normalization
                self.apply_sn3d_normalization(buffer);
            }
            AmbisonicsFormat::FuMA => {
                // Convert from ACN to FuMA channel ordering
                let reordered = self.reorder_acn_to_fuma(buffer);
                *buffer = reordered;
            }
            AmbisonicsFormat::BFormat => {
                // Standard B-format, apply appropriate normalization
                self.apply_maxn_normalization(buffer);
            }
        }
        Ok(())
    }

    /// Apply SN3D normalization.
    fn apply_sn3d_normalization(&self, buffer: &mut AudioBuffer) {
        for ch in 0..buffer.channels() {
            let acn_idx = AcnIndex::from_acn(ch);
            let norm_factor = Normalization::SN3D.factor(acn_idx.n, acn_idx.m);
            for sample in 0..buffer.samples() {
                let value = buffer.get_sample(ch, sample) * norm_factor as f32;
                buffer.set_sample(ch, sample, value);
            }
        }
    }

    /// Apply MaxN normalization.
    fn apply_maxn_normalization(&self, buffer: &mut AudioBuffer) {
        // MaxN doesn't change the coefficients, but ensures consistent scaling
        let scale = 1.0 / (self.order.order() as f32 + 1.0).sqrt();
        for ch in 0..buffer.channels() {
            for sample in 0..buffer.samples() {
                let value = buffer.get_sample(ch, sample) * scale;
                buffer.set_sample(ch, sample, value);
            }
        }
    }

    /// Reorder channels from ACN to FuMA ordering.
    fn reorder_acn_to_fuma(&self, buffer: &AudioBuffer) -> AudioBuffer {
        // ACN to FuMA mapping for first-order ambisonics
        // ACN: 0=W, 1=Y, 2=Z, 3=X
        // FuMA: W, X, Y, Z
        let acn_to_fuma = match self.order {
            AmbisonicsOrder::First => vec![0, 3, 1, 2], // W, X, Y, Z
            AmbisonicsOrder::Second => vec![
                0, 3, 1, 2, // First order
                8, 4, 5, 7, 6, // Second order: U, V, T, S, R
            ],
            AmbisonicsOrder::Third => {
                // Full mapping would be more complex
                (0..16).collect()
            }
            _ => (0..buffer.channels()).collect(),
        };

        let mut output = AudioBuffer::new(
            buffer.format().clone(),
            buffer.samples(),
        );

        for (fuma_ch, &acn_ch) in acn_to_fuma.iter().enumerate() {
            if acn_ch < buffer.channels() {
                for sample in 0..buffer.samples() {
                    output.set_sample(fuma_ch, sample, buffer.get_sample(acn_ch, sample));
                }
            }
        }

        output
    }
}

/// Ambisonics decoder for decoding B-format to speaker layouts.
#[derive(Debug, Clone)]
pub struct AmbisonicsDecoder {
    /// Ambisonics order.
    order: AmbisonicsOrder,
    /// Input format.
    format: AmbisonicsFormat,
    /// Speaker positions (azimuth, elevation in degrees).
    speakers: Vec<SpeakerPosition>,
    /// Decoding matrix.
    decoding_matrix: Vec<Vec<f64>>,
    /// Sample rate.
    sample_rate: u32,
}

/// Speaker position for decoder configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerPosition {
    /// Azimuth in degrees (0 = front, 90 = right, 180 = back, 270 = left).
    pub azimuth: f64,
    /// Elevation in degrees (0 = horizontal, 90 = top).
    pub elevation: f64,
    /// Speaker label.
    pub label: String,
}

impl SpeakerPosition {
    /// Create a new speaker position.
    pub fn new(azimuth: f64, elevation: f64, label: impl Into<String>) -> Self {
        SpeakerPosition {
            azimuth,
            elevation,
            label: label.into(),
        }
    }
}

impl AmbisonicsDecoder {
    /// Create a new ambisonics decoder.
    pub fn new(
        order: AmbisonicsOrder,
        format: AmbisonicsFormat,
        speakers: Vec<SpeakerPosition>,
        sample_rate: u32,
    ) -> Self {
        let decoding_matrix = Self::compute_decoding_matrix(order, &speakers);
        AmbisonicsDecoder {
            order,
            format,
            speakers,
            decoding_matrix,
            sample_rate,
        }
    }

    /// Create a stereo decoder.
    pub fn stereo(order: AmbisonicsOrder, sample_rate: u32) -> Self {
        let speakers = vec![
            SpeakerPosition::new(-30.0, 0.0, "L"),
            SpeakerPosition::new(30.0, 0.0, "R"),
        ];
        Self::new(order, AmbisonicsFormat::AmbiX, speakers, sample_rate)
    }

    /// Create a 5.1 surround decoder.
    pub fn surround51(order: AmbisonicsOrder, sample_rate: u32) -> Self {
        let speakers = vec![
            SpeakerPosition::new(-30.0, 0.0, "L"),
            SpeakerPosition::new(30.0, 0.0, "R"),
            SpeakerPosition::new(0.0, 0.0, "C"),
            SpeakerPosition::new(-110.0, 0.0, "Ls"),
            SpeakerPosition::new(110.0, 0.0, "Rs"),
        ];
        Self::new(order, AmbisonicsFormat::AmbiX, speakers, sample_rate)
    }

    /// Create a 7.1 surround decoder.
    pub fn surround71(order: AmbisonicsOrder, sample_rate: u32) -> Self {
        let speakers = vec![
            SpeakerPosition::new(-30.0, 0.0, "L"),
            SpeakerPosition::new(30.0, 0.0, "R"),
            SpeakerPosition::new(0.0, 0.0, "C"),
            SpeakerPosition::new(-90.0, 0.0, "Lss"),
            SpeakerPosition::new(90.0, 0.0, "Rss"),
            SpeakerPosition::new(-150.0, 0.0, "Lrs"),
            SpeakerPosition::new(150.0, 0.0, "Rrs"),
        ];
        Self::new(order, AmbisonicsFormat::AmbiX, speakers, sample_rate)
    }

    /// Create a binaural decoder (for headphones).
    pub fn binaural(order: AmbisonicsOrder, sample_rate: u32) -> Self {
        // For binaural, we'll use virtual speakers at ±90°
        let speakers = vec![
            SpeakerPosition::new(-90.0, 0.0, "L"),
            SpeakerPosition::new(90.0, 0.0, "R"),
        ];
        Self::new(order, AmbisonicsFormat::AmbiX, speakers, sample_rate)
    }

    /// Compute the decoding matrix.
    fn compute_decoding_matrix(order: AmbisonicsOrder, speakers: &[SpeakerPosition]) -> Vec<Vec<f64>> {
        let num_channels = order.channels();
        let num_speakers = speakers.len();

        let mut matrix = vec![vec![0.0; num_channels]; num_speakers];

        for (spk_idx, speaker) in speakers.iter().enumerate() {
            let sh = SphericalHarmonics::from_direction(order, speaker.azimuth, speaker.elevation);
            for (ch, &coeff) in sh.coefficients().iter().enumerate() {
                matrix[spk_idx][ch] = coeff;
            }
        }

        // Apply basic mode-matching decoding
        // In production, use more sophisticated methods like AllRAD or VBAP
        Self::normalize_matrix(&mut matrix);

        matrix
    }

    /// Normalize the decoding matrix.
    fn normalize_matrix(matrix: &mut [Vec<f64>]) {
        // Simple normalization - divide by number of speakers for energy preservation
        let num_speakers = matrix.len() as f64;
        let norm_factor = 1.0 / num_speakers.sqrt();
        
        for row in matrix.iter_mut() {
            for value in row.iter_mut() {
                *value *= norm_factor;
            }
        }
    }

    /// Decode ambisonics to speaker feeds.
    pub fn decode(&self, input: &AudioBuffer) -> SpatialAudioResult<AudioBuffer> {
        if input.channels() != self.order.channels() {
            return Err(SpatialAudioError::InvalidInput(format!(
                "Expected {} channels for {:?} ambisonics, got {}",
                self.order.channels(),
                self.order,
                input.channels()
            )));
        }

        let num_speakers = self.speakers.len();
        let samples = input.samples();

        // Create output buffer
        let mut output = AudioBuffer::new(
            AudioFormat::new(num_speakers, self.sample_rate),
            samples,
        );

        // Apply decoding matrix
        for (spk_idx, row) in self.decoding_matrix.iter().enumerate() {
            for sample in 0..samples {
                let mut value = 0.0f32;
                for (ch, &coeff) in row.iter().enumerate() {
                    value += input.get_sample(ch, sample) * coeff as f32;
                }
                output.set_sample(spk_idx, sample, value);
            }
        }

        Ok(output)
    }

    /// Get the speaker configuration.
    pub fn speakers(&self) -> &[SpeakerPosition] {
        &self.speakers
    }

    /// Get the decoding matrix.
    pub fn matrix(&self) -> &[Vec<f64>] {
        &self.decoding_matrix
    }

    /// Get the order.
    pub fn order(&self) -> AmbisonicsOrder {
        self.order
    }
}

/// Ambisonics rotator for rotating the sound field.
#[derive(Debug, Clone)]
pub struct AmbisonicsRotator {
    /// Ambisonics order.
    order: AmbisonicsOrder,
    /// Yaw rotation in radians.
    yaw: f64,
    /// Pitch rotation in radians.
    pitch: f64,
    /// Roll rotation in radians.
    roll: f64,
    /// Rotation matrices for each order.
    rotation_matrices: Vec<RotationMatrix>,
}

/// Rotation matrix for a specific ambisonics order.
#[derive(Debug, Clone)]
struct RotationMatrix {
    /// The rotation matrix values.
    matrix: Vec<Vec<f64>>,
}

impl AmbisonicsRotator {
    /// Create a new ambisonics rotator.
    pub fn new(order: AmbisonicsOrder) -> Self {
        let rotation_matrices = Self::compute_identity_matrices(order);
        AmbisonicsRotator {
            order,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            rotation_matrices,
        }
    }

    /// Set the rotation angles (in degrees).
    pub fn set_rotation(&mut self, yaw: f64, pitch: f64, roll: f64) {
        self.yaw = yaw * PI / 180.0;
        self.pitch = pitch * PI / 180.0;
        self.roll = roll * PI / 180.0;
        self.update_rotation_matrices();
    }

    /// Compute identity rotation matrices.
    fn compute_identity_matrices(order: AmbisonicsOrder) -> Vec<RotationMatrix> {
        let max_order = order.order();
        let mut matrices = Vec::with_capacity(max_order + 1);

        for n in 0..=max_order {
            let size = 2 * n + 1;
            let mut matrix = vec![vec![0.0; size]; size];
            for i in 0..size {
                matrix[i][i] = 1.0;
            }
            matrices.push(RotationMatrix { matrix });
        }

        matrices
    }

    /// Update rotation matrices based on current angles.
    fn update_rotation_matrices(&mut self) {
        // For first-order ambisonics, use direct 3x3 rotation matrix
        // For higher orders, use recursion formulas
        
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();
        let cr = self.roll.cos();
        let sr = self.roll.sin();

        // Rotation matrix for first-order ambisonics (Y, Z, X channels)
        let _r_y = [
            [cy, 0.0, -sy],
            [0.0, 1.0, 0.0],
            [sy, 0.0, cy],
        ];

        let _r_p = [
            [cp, -sp, 0.0],
            [sp, cp, 0.0],
            [0.0, 0.0, 1.0],
        ];

        let _r_r = [
            [1.0, 0.0, 0.0],
            [0.0, cr, -sr],
            [0.0, sr, cr],
        ];

        // Combined rotation would be R_y * R_p * R_r
        // For now, just use identity for higher orders
        // Production code would use proper Wigner-D matrices
    }

    /// Rotate an ambisonics buffer.
    pub fn rotate(&self, input: &AudioBuffer) -> SpatialAudioResult<AudioBuffer> {
        let mut output = input.clone();
        
        // Apply rotation for each order
        let mut acn_offset = 1; // Skip W channel (order 0)
        
        for n in 1..=self.order.order() {
            let size = 2 * n + 1;
            let rot_matrix = &self.rotation_matrices[n];
            
            for sample in 0..input.samples() {
                // Extract channel values for this order
                let mut values = vec![0.0f32; size];
                for (i, v) in values.iter_mut().enumerate() {
                    let ch = acn_offset + i;
                    if ch < input.channels() {
                        *v = input.get_sample(ch, sample);
                    }
                }

                // Apply rotation
                for i in 0..size {
                    let mut rotated_value = 0.0f32;
                    for j in 0..size {
                        rotated_value += values[j] * rot_matrix.matrix[i][j] as f32;
                    }
                    let ch = acn_offset + i;
                    if ch < output.channels() {
                        output.set_sample(ch, sample, rotated_value);
                    }
                }
            }

            acn_offset += size;
        }

        Ok(output)
    }
}

/// Ambisonics processor combining encoding, decoding, and rotation.
#[derive(Debug, Clone)]
pub struct AmbisonicsProcessor {
    /// Encoder instance.
    encoder: Option<AmbisonicsEncoder>,
    /// Decoder instance.
    decoder: Option<AmbisonicsDecoder>,
    /// Rotator instance.
    rotator: AmbisonicsRotator,
    /// Configuration.
    config: AmbisonicsConfig,
}

/// Configuration for ambisonics processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbisonicsConfig {
    /// Ambisonics order.
    pub order: AmbisonicsOrder,
    /// Input format.
    pub input_format: AmbisonicsFormat,
    /// Output format.
    pub output_format: AmbisonicsFormat,
    /// Enable rotation.
    pub enable_rotation: bool,
    /// Sample rate.
    pub sample_rate: u32,
}

impl Default for AmbisonicsConfig {
    fn default() -> Self {
        AmbisonicsConfig {
            order: AmbisonicsOrder::First,
            input_format: AmbisonicsFormat::AmbiX,
            output_format: AmbisonicsFormat::AmbiX,
            enable_rotation: false,
            sample_rate: 48000,
        }
    }
}

impl AmbisonicsProcessor {
    /// Create a new ambisonics processor.
    pub fn new(config: AmbisonicsConfig) -> Self {
        let rotator = AmbisonicsRotator::new(config.order);
        
        AmbisonicsProcessor {
            encoder: None,
            decoder: None,
            rotator,
            config,
        }
    }

    /// Set up for encoding to ambisonics.
    pub fn with_encoder(mut self) -> Self {
        self.encoder = Some(AmbisonicsEncoder::new(
            self.config.order,
            self.config.output_format,
            self.config.sample_rate,
        ));
        self
    }

    /// Set up for decoding from ambisonics.
    pub fn with_decoder(mut self, speakers: Vec<SpeakerPosition>) -> Self {
        self.decoder = Some(AmbisonicsDecoder::new(
            self.config.order,
            self.config.input_format,
            speakers,
            self.config.sample_rate,
        ));
        self
    }

    /// Set up for binaural decoding.
    pub fn with_binaural_decoder(mut self) -> Self {
        self.decoder = Some(AmbisonicsDecoder::binaural(
            self.config.order,
            self.config.sample_rate,
        ));
        self
    }

    /// Set up for stereo decoding.
    pub fn with_stereo_decoder(mut self) -> Self {
        self.decoder = Some(AmbisonicsDecoder::stereo(
            self.config.order,
            self.config.sample_rate,
        ));
        self
    }

    /// Process audio through the ambisonics pipeline.
    pub fn process(&self, input: &AudioBuffer, position: Option<&Position>) -> SpatialAudioResult<AudioBuffer> {
        let mut current = input.clone();

        // Encode if encoder is available and position is provided
        if let (Some(encoder), Some(pos)) = (&self.encoder, position) {
            current = encoder.encode(&current, pos)?;
        }

        // Rotate if enabled
        if self.config.enable_rotation {
            current = self.rotator.rotate(&current)?;
        }

        // Decode if decoder is available
        if let Some(decoder) = &self.decoder {
            current = decoder.decode(&current)?;
        }

        Ok(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ambisonics_order_channels() {
        assert_eq!(AmbisonicsOrder::First.channels(), 4);
        assert_eq!(AmbisonicsOrder::Second.channels(), 9);
        assert_eq!(AmbisonicsOrder::Third.channels(), 16);
    }

    #[test]
    fn test_acn_index() {
        let idx = AcnIndex::from_acn(0);
        assert_eq!(idx.n, 0);
        assert_eq!(idx.m, 0);

        let idx = AcnIndex::from_acn(1);
        assert_eq!(idx.n, 1);
        assert_eq!(idx.m, -1);

        let idx = AcnIndex::from_acn(3);
        assert_eq!(idx.n, 1);
        assert_eq!(idx.m, 1);
    }

    #[test]
    fn test_spherical_harmonics() {
        let sh = SphericalHarmonics::from_direction(AmbisonicsOrder::First, 0.0, 0.0);
        assert_eq!(sh.coefficients.len(), 4);
        // W channel should be 1.0 for any direction
        assert!((sh.get(0).unwrap() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_stereo_decoder() {
        let decoder = AmbisonicsDecoder::stereo(AmbisonicsOrder::First, 48000);
        assert_eq!(decoder.speakers().len(), 2);
        assert_eq!(decoder.matrix().len(), 2);
        assert_eq!(decoder.matrix()[0].len(), 4);
    }
}