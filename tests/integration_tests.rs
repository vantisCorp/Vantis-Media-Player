// Integration tests for Vantis Media Player
//
// These tests verify the integration between different components
// of the media player system.

use vantis_core::{VantisCore, Config};
use vantis_video::VideoEngine;
use vantis_audio::AudioEngine;
use vantis_subtitles::SubtitleEngine;
use vantis_plugins::PluginManager;
use vantis_integrations::IntegrationManager;
use vantis_ai::AIEngine;
use vantis_streaming::StreamingEngine;
use vantis_advanced_audio::AdvancedAudioEngine;
use vantis_advanced_video::AdvancedVideoEngine;
use vantis_advanced_ui::AdvancedUIEngine;

#[tokio::test]
async fn test_core_initialization() {
    let config = Config::default();
    let core = VantisCore::new(config);
    assert!(core.is_ok());
    
    let core = core.unwrap();
    assert_ne!(core.id(), uuid::Uuid::nil());
}

#[tokio::test]
async fn test_video_engine_initialization() {
    let video_engine = VideoEngine::new().await;
    assert!(video_engine.is_ok());
}

#[tokio::test]
async fn test_audio_engine_initialization() {
    let audio_engine = AudioEngine::new();
    assert!(audio_engine.is_ok());
}

#[tokio::test]
async fn test_subtitle_engine_initialization() {
    let subtitle_engine = SubtitleEngine::new();
    assert!(subtitle_engine.is_ok());
}

#[tokio::test]
async fn test_plugin_manager_initialization() {
    let plugin_manager = PluginManager::new();
    assert!(plugin_manager.is_ok());
}

#[tokio::test]
async fn test_integration_manager_initialization() {
    let integration_manager = IntegrationManager::new();
    assert!(integration_manager.is_ok());
}

#[tokio::test]
async fn test_ai_engine_initialization() {
    let ai_engine = AIEngine::new();
    assert!(ai_engine.is_ok());
}

#[tokio::test]
async fn test_streaming_engine_initialization() {
    let streaming_engine = StreamingEngine::new();
    assert!(streaming_engine.is_ok());
}

#[tokio::test]
async fn test_advanced_audio_engine_initialization() {
    let advanced_audio = AdvancedAudioEngine::new();
    assert!(advanced_audio.is_ok());
}

#[tokio::test]
async fn test_advanced_video_engine_initialization() {
    let advanced_video = AdvancedVideoEngine::new();
    assert!(advanced_video.is_ok());
}

#[tokio::test]
async fn test_advanced_ui_engine_initialization() {
    let advanced_ui = AdvancedUIEngine::new();
    assert!(advanced_ui.is_ok());
}

#[tokio::test]
async fn test_full_system_initialization() {
    let config = Config::default();
    let mut core = VantisCore::new(config).unwrap();
    
    // Initialize all engines
    let video_engine = VideoEngine::new().await.unwrap();
    core.set_video_engine(video_engine).unwrap();
    
    let audio_engine = AudioEngine::new().unwrap();
    core.set_audio_engine(audio_engine).unwrap();
    
    let subtitle_engine = SubtitleEngine::new().unwrap();
    core.set_subtitle_engine(subtitle_engine).unwrap();
    
    let plugin_manager = PluginManager::new().unwrap();
    core.set_plugin_manager(plugin_manager).unwrap();
    
    let integration_manager = IntegrationManager::new().unwrap();
    core.set_integration_manager(integration_manager).unwrap();
    
    let ai_engine = AIEngine::new().unwrap();
    core.set_ai_engine(ai_engine).unwrap();
    
    let streaming_engine = StreamingEngine::new().unwrap();
    core.set_streaming_engine(streaming_engine).unwrap();
    
    let advanced_audio = AdvancedAudioEngine::new().unwrap();
    core.set_advanced_audio_engine(advanced_audio).unwrap();
    
    let advanced_video = AdvancedVideoEngine::new().unwrap();
    core.set_advanced_video_engine(advanced_video).unwrap();
    
    let advanced_ui = AdvancedUIEngine::new().unwrap();
    core.set_advanced_ui_engine(advanced_ui).unwrap();
    
    // Verify all engines are set
    assert!(core.video_engine.is_some());
    assert!(core.audio_engine.is_some());
    assert!(core.ui_engine.is_some());
    assert!(core.subtitle_engine.is_some());
    assert!(core.ai_engine.is_some());
    assert!(core.plugin_manager.is_some());
    assert!(core.integration_manager.is_some());
    assert!(core.streaming_engine.is_some());
    assert!(core.advanced_audio_engine.is_some());
    assert!(core.advanced_video_engine.is_some());
    assert!(core.advanced_ui_engine.is_some());
}

#[tokio::test]
async fn test_event_bus_integration() {
    let config = Config::default();
    let core = VantisCore::new(config).unwrap();
    
    let event_bus = core.event_bus();
    let subscription = event_bus.subscribe();
    
    // Publish test event
    event_bus.publish(vantis_core::events::Event::Play {
        file: "test.mp4".to_string(),
    });
    
    // Verify state changes
    let state = core.state();
    let state_read = state.read();
    assert_eq!(state_read.current_media, Some("test.mp4".to_string()));
}

#[tokio::test]
async fn test_playback_control_integration() {
    let config = Config::default();
    let core = VantisCore::new(config).unwrap();
    
    // Test play
    core.play_media("test.mp4").await.unwrap();
    let state = core.state();
    let state_read = state.read();
    assert_eq!(state_read.playback_state, vantis_core::state::PlaybackState::Playing);
    
    // Test pause
    core.pause().await.unwrap();
    let state_read = state.read();
    assert_eq!(state_read.playback_state, vantis_core::state::PlaybackState::Paused);
    
    // Test resume
    core.resume().await.unwrap();
    let state_read = state.read();
    assert_eq!(state_read.playback_state, vantis_core::state::PlaybackState::Playing);
    
    // Test stop
    core.stop().await.unwrap();
    let state_read = state.read();
    assert_eq!(state_read.playback_state, vantis_core::state::PlaybackState::Stopped);
}

#[tokio::test]
async fn test_volume_control_integration() {
    let config = Config::default();
    let core = VantisCore::new(config).unwrap();
    
    // Test volume
    core.set_volume(0.5).await.unwrap();
    let state = core.state();
    let state_read = state.read();
    assert_eq!(state_read.volume, 0.5);
    
    // Test mute
    core.set_mute(true).await.unwrap();
    let state_read = state.read();
    assert!(state_read.muted);
}

#[tokio::test]
async fn test_seek_integration() {
    let config = Config::default();
    let core = VantisCore::new(config).unwrap();
    
    // Test seek
    core.seek(30.0).await.unwrap();
    let state = core.state();
    let state_read = state.read();
    assert_eq!(state_read.position_seconds(), 30.0);
}

#[tokio::test]
async fn test_speed_control_integration() {
    let config = Config::default();
    let core = VantisCore::new(config).unwrap();
    
    // Test speed
    core.set_speed(1.5).await.unwrap();
    let state = core.state();
    let state_read = state.read();
    assert_eq!(state_read.playback_speed, 1.5);
}