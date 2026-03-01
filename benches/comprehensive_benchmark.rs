// Comprehensive benchmark suite for Vantis Media Player
//
// This benchmark suite measures performance across all major components
// of the media player system.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use vantis_core::buffer::BufferPool;
use vantis_core::event::EventBus;
use vantis_subtitles::SubtitleParser;
use std::time::Duration;

/// Benchmark zero-copy buffer operations
fn bench_buffer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_operations");
    
    for size in [1024, 4096, 16384, 65536].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let pool = BufferPool::new(size);
            
            b.iter(|| {
                let buffer = pool.allocate(size / 2);
                black_box(buffer);
            });
        });
    }
    
    group.finish();
}

/// Benchmark event bus throughput
fn bench_event_bus(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_bus");
    
    for event_count in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(event_count), event_count, |b, &count| {
            let bus = EventBus::new();
            let mut receivers = Vec::new();
            
            // Create multiple subscribers
            for _ in 0..10 {
                receivers.push(bus.subscribe());
            }
            
            b.iter(|| {
                for i in 0..count {
                    bus.publish(vantis_core::event::Event::VolumeChanged(i as f64 / 100.0));
                }
            });
        });
    }
    
    group.finish();
}

/// Benchmark subtitle parsing
fn bench_subtitle_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("subtitle_parsing");
    
    let parser = SubtitleParser::new();
    
    // Benchmark different formats
    let formats = vec![
        ("SRT", "tests/test_data/subtitles.srt"),
        ("ASS", "tests/test_data/subtitles.ass"),
        ("VTT", "tests/test_data/subtitles.vtt"),
    ];
    
    for (name, file) in formats {
        if std::path::Path::new(file).exists() {
            group.bench_function(name, |b| {
                b.iter(|| {
                    let result = parser.parse_file(file, vantis_video::VideoFormat::SRT);
                    black_box(result);
                });
            });
        }
    }
    
    group.finish();
}

/// Benchmark encoding detection
fn bench_encoding_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding_detection");
    
    let parser = SubtitleParser::new();
    
    let files = vec![
        ("UTF-8", "tests/test_data/subtitles_utf8.srt"),
        ("CP1250", "tests/test_data/subtitles_cp1250.srt"),
        ("ISO-8859-2", "tests/test_data/subtitles_iso8859.srt"),
    ];
    
    for (name, file) in files {
        if std::path::Path::new(file).exists() {
            group.bench_function(name, |b| {
                b.iter(|| {
                    let result = parser.detect_encoding(file);
                    black_box(result);
                });
            });
        }
    }
    
    group.finish();
}

/// Benchmark video frame processing
fn bench_video_frame_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("video_frame_processing");
    
    for resolution in [(1920, 1080), (3840, 2160)].iter() {
        let (width, height) = *resolution;
        let pixel_count = width * height;
        
        group.throughput(Throughput::Pixels(pixel_count as u64));
        
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}x{}", width, height)),
            resolution,
            |b, &(width, height)| {
                let mut frame = create_test_frame(width, height);
                
                b.iter(|| {
                    // Simulate frame processing
                    process_frame(&mut frame);
                    black_box(&frame);
                });
            }
        );
    }
    
    group.finish();
}

/// Benchmark audio processing
fn bench_audio_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("audio_processing");
    
    for sample_count in [1024, 4096, 16384].iter() {
        group.throughput(Throughput::Elements(*sample_count as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(sample_count), sample_count, |b, &count| {
            let mut samples = vec![0.0f32; count];
            
            b.iter(|| {
                // Simulate audio processing
                apply_gain(&mut samples, 1.5);
                black_box(&samples);
            });
        });
    }
    
    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");
    
    group.bench_function("vec_allocation", |b| {
        b.iter(|| {
            let vec: Vec<u8> = vec![0; 1024 * 1024];
            black_box(vec);
        });
    });
    
    group.bench_function("vec_with_capacity", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1024 * 1024);
            vec.resize(1024 * 1024, 0);
            black_box(vec);
        });
    });
    
    group.finish();
}

/// Benchmark concurrent operations
fn bench_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    
    for thread_count in [1, 2, 4, 8].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(thread_count), thread_count, |b, &count| {
            b.iter(|| {
                let handles: Vec<_> = (0..count)
                    .map(|_| {
                        std::thread::spawn(|| {
                            let mut sum = 0;
                            for i in 0..1000000 {
                                sum += i;
                            }
                            sum
                        })
                    })
                    .collect();
                
                for handle in handles {
                    black_box(handle.join().unwrap());
                }
            });
        });
    }
    
    group.finish();
}

/// Benchmark configuration parsing
fn bench_configuration_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("configuration_parsing");
    
    let config_content = r#"
[audio]
exclusive_mode = true
sample_rate = 48000
channels = 2

[video]
hardware_acceleration = true
ai_upscaling = true

[subtitles]
default_language = "pl"
auto_download = true
"#;
    
    group.bench_function("parse_config", |b| {
        b.iter(|| {
            let result: Result<toml::Value, _> = config_content.parse();
            black_box(result);
        });
    });
    
    group.finish();
}

/// Benchmark JSON serialization
fn bench_json_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_serialization");
    
    let data = create_test_data();
    
    group.bench_function("serialize", |b| {
        b.iter(|| {
            let json = serde_json::to_string(&data);
            black_box(json);
        });
    });
    
    group.bench_function("deserialize", |b| {
        let json = serde_json::to_string(&data).unwrap();
        b.iter(|| {
            let result: serde_json::Value = serde_json::from_str(&json);
            black_box(result);
        });
    });
    
    group.finish();
}

/// Benchmark TOML serialization
fn bench_toml_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("toml_serialization");
    
    let data = create_test_data();
    
    group.bench_function("serialize", |b| {
        b.iter(|| {
            let toml = toml::to_string(&data);
            black_box(toml);
        });
    });
    
    group.bench_function("deserialize", |b| {
        let toml = toml::to_string(&data).unwrap();
        b.iter(|| {
            let result: toml::Value = toml::from_str(&toml);
            black_box(result);
        });
    });
    
    group.finish();
}

/// Benchmark string operations
fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");
    
    let test_string = "This is a test string for benchmarking string operations";
    
    group.bench_function("to_uppercase", |b| {
        b.iter(|| {
            let result = test_string.to_uppercase();
            black_box(result);
        });
    });
    
    group.bench_function("to_lowercase", |b| {
        b.iter(|| {
            let result = test_string.to_lowercase();
            black_box(result);
        });
    });
    
    group.bench_function("contains", |b| {
        b.iter(|| {
            let result = test_string.contains("test");
            black_box(result);
        });
    });
    
    group.finish();
}

/// Benchmark hash operations
fn bench_hash_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_operations");
    
    let data = vec![0u8; 1024];
    
    group.bench_function("sha256", |b| {
        use sha2::{Sha256, Digest};
        b.iter(|| {
            let mut hasher = Sha256::new();
            hasher.update(&data);
            let result = hasher.finalize();
            black_box(result);
        });
    });
    
    group.bench_function("md5", |b| {
        use md5::{Md5, Digest};
        b.iter(|| {
            let mut hasher = Md5::new();
            hasher.update(&data);
            let result = hasher.finalize();
            black_box(result);
        });
    });
    
    group.finish();
}

// Helper functions

fn create_test_frame(width: u32, height: u32) -> Vec<u8> {
    vec![0u8; (width * height * 4) as usize]
}

fn process_frame(frame: &mut [u8]) {
    // Simple frame processing
    for pixel in frame.chunks_mut(4) {
        pixel[0] = pixel[0].saturating_add(10);
        pixel[1] = pixel[1].saturating_add(10);
        pixel[2] = pixel[2].saturating_add(10);
    }
}

fn apply_gain(samples: &mut [f32], gain: f32) {
    for sample in samples.iter_mut() {
        *sample = (*sample * gain).clamp(-1.0, 1.0);
    }
}

fn create_test_data() -> serde_json::Value {
    serde_json::json!({
        "title": "Test Video",
        "duration": 7200.0,
        "width": 1920,
        "height": 1080,
        "audio": {
            "channels": 2,
            "sample_rate": 48000
        },
        "subtitles": [
            {
                "language": "pl",
                "format": "SRT"
            },
            {
                "language": "en",
                "format": "SRT"
            }
        ]
    })
}

criterion_group!(
    benches,
    bench_buffer_operations,
    bench_event_bus,
    bench_subtitle_parsing,
    bench_encoding_detection,
    bench_video_frame_processing,
    bench_audio_processing,
    bench_memory_allocation,
    bench_concurrent_operations,
    bench_configuration_parsing,
    bench_json_serialization,
    bench_toml_serialization,
    bench_string_operations,
    bench_hash_operations
);

criterion_main!(benches);