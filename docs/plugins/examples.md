---
sidebar_position: 5
---

# Plugin Examples

This section provides complete, working examples of plugins for Vantis Media Player. Each example demonstrates different capabilities and can be used as a starting point for your own plugins.

## JavaScript Plugin Examples

### 1. Simple Logger Plugin

A minimal plugin that logs player events:

```javascript
// logger-plugin.js

export default class LoggerPlugin {
  constructor() {
    this.id = 'logger';
    this.name = 'Logger Plugin';
    this.version = '1.0.0';
    this.player = null;
  }
  
  onLoad(player) {
    this.player = player;
    
    // Log all player events
    const events = [
      'player:ready',
      'player:play',
      'player:pause',
      'player:ended',
      'player:seek',
      'player:error'
    ];
    
    events.forEach(event => {
      player.on(event, (data) => {
        console.log(`[Logger] ${event}:`, data);
      });
    });
    
    console.log('[Logger] Plugin loaded');
  }
  
  onUnload() {
    console.log('[Logger] Plugin unloaded');
    this.player = null;
  }
}
```

**Usage:**

```javascript
import LoggerPlugin from './logger-plugin.js';

const player = new VantisPlayer('#player');
await player.plugins.register(new LoggerPlugin());
```

### 2. Keyboard Shortcuts Plugin

Add keyboard controls to the player:

```javascript
// keyboard-shortcuts-plugin.js

export default class KeyboardShortcutsPlugin {
  constructor(config = {}) {
    this.id = 'keyboard-shortcuts';
    this.name = 'Keyboard Shortcuts';
    this.version = '1.0.0';
    
    this.shortcuts = {
      'Space': 'togglePlay',
      'ArrowRight': 'seekForward',
      'ArrowLeft': 'seekBackward',
      'ArrowUp': 'volumeUp',
      'ArrowDown': 'volumeDown',
      'KeyM': 'toggleMute',
      'KeyF': 'toggleFullscreen',
      'Escape': 'exitFullscreen',
      ...config.customShortcuts
    };
    
    this.seekAmount = config.seekAmount || 10;
    this.volumeStep = config.volumeStep || 0.1;
    this.player = null;
  }
  
  onLoad(player) {
    this.player = player;
    document.addEventListener('keydown', this.handleKeyDown.bind(this));
    console.log('[Keyboard] Plugin loaded');
  }
  
  onUnload() {
    document.removeEventListener('keydown', this.handleKeyDown.bind(this));
    this.player = null;
  }
  
  handleKeyDown(event) {
    const action = this.shortcuts[event.code];
    if (!action) return;
    
    event.preventDefault();
    this.executeAction(action);
  }
  
  executeAction(action) {
    const player = this.player;
    
    switch (action) {
      case 'togglePlay':
        player.getState().playing ? player.pause() : player.play();
        break;
        
      case 'seekForward':
        player.seek(player.getCurrentTime() + this.seekAmount);
        break;
        
      case 'seekBackward':
        player.seek(Math.max(0, player.getCurrentTime() - this.seekAmount));
        break;
        
      case 'volumeUp':
        player.setVolume(Math.min(1, player.getVolume() + this.volumeStep));
        break;
        
      case 'volumeDown':
        player.setVolume(Math.max(0, player.getVolume() - this.volumeStep));
        break;
        
      case 'toggleMute':
        player.setMuted(!player.getMuted());
        break;
        
      case 'toggleFullscreen':
        player.toggleFullscreen();
        break;
        
      case 'exitFullscreen':
        if (document.fullscreenElement) {
          document.exitFullscreen();
        }
        break;
    }
  }
}
```

### 3. Auto-Skip Intro Plugin

Automatically skip intro sequences:

```javascript
// skip-intro-plugin.js

export default class SkipIntroPlugin {
  constructor(config = {}) {
    this.id = 'skip-intro';
    this.name = 'Skip Intro';
    this.version = '1.0.0';
    
    this.intros = config.intros || {};
    this.player = null;
    this.currentIntro = null;
    this.ui = null;
  }
  
  onLoad(player) {
    this.player = player;
    
    player.on('player:load', this.onMediaLoad.bind(this));
    player.on('player:timeupdate', this.checkIntro.bind(this));
    
    this.createSkipButton();
  }
  
  onUnload() {
    this.removeSkipButton();
    this.player = null;
  }
  
  onMediaLoad(source) {
    // Look for intro data
    this.currentIntro = this.intros[source.id];
  }
  
  checkIntro(time) {
    if (!this.currentIntro) return;
    
    const { start, end } = this.currentIntro;
    
    if (time >= start && time < end - 1) {
      this.showSkipButton();
    } else {
      this.hideSkipButton();
    }
  }
  
  createSkipButton() {
    this.skipButton = document.createElement('button');
    this.skipButton.className = 'skip-intro-button';
    this.skipButton.textContent = 'Skip Intro';
    this.skipButton.style.cssText = `
      position: absolute;
      bottom: 80px;
      right: 20px;
      padding: 10px 20px;
      background: rgba(0, 0, 0, 0.7);
      color: white;
      border: 1px solid white;
      border-radius: 4px;
      cursor: pointer;
      display: none;
      font-size: 14px;
    `;
    
    this.skipButton.addEventListener('click', () => {
      this.skipIntro();
    });
    
    const container = this.player.getContainer();
    container.appendChild(this.skipButton);
  }
  
  showSkipButton() {
    this.skipButton.style.display = 'block';
  }
  
  hideSkipButton() {
    this.skipButton.style.display = 'none';
  }
  
  skipIntro() {
    if (this.currentIntro) {
      this.player.seek(this.currentIntro.end);
      this.hideSkipButton();
    }
  }
  
  removeSkipButton() {
    if (this.skipButton && this.skipButton.parentNode) {
      this.skipButton.parentNode.removeChild(this.skipButton);
    }
  }
  
  // API to add intro data
  addIntro(mediaId, start, end) {
    this.intros[mediaId] = { start, end };
  }
}
```

### 4. Analytics Plugin

Track player events for analytics:

```javascript
// analytics-plugin.js

export default class AnalyticsPlugin {
  constructor(config = {}) {
    this.id = 'analytics';
    this.name = 'Analytics Plugin';
    this.version = '1.0.0';
    
    this.endpoint = config.endpoint || '/api/analytics';
    this.batchSize = config.batchSize || 10;
    this.flushInterval = config.flushInterval || 30000;
    
    this.events = [];
    this.sessionId = this.generateSessionId();
    this.player = null;
    this.flushTimer = null;
  }
  
  onLoad(player) {
    this.player = player;
    
    // Track events
    player.on('player:play', () => this.track('play'));
    player.on('player:pause', () => this.track('pause'));
    player.on('player:ended', () => this.track('ended'));
    player.on('player:seek', (time) => this.track('seek', { time }));
    player.on('player:timeupdate', (time) => {
      // Sample every 30 seconds
      if (Math.floor(time) % 30 === 0 && Math.floor(time) !== this.lastSampled) {
        this.lastSampled = Math.floor(time);
        this.track('progress', { time, percent: (time / player.getDuration()) * 100 });
      }
    });
    
    // Start flush timer
    this.flushTimer = setInterval(() => this.flush(), this.flushInterval);
    
    console.log('[Analytics] Plugin loaded');
  }
  
  onUnload() {
    if (this.flushTimer) {
      clearInterval(this.flushTimer);
    }
    this.flush();
    this.player = null;
  }
  
  generateSessionId() {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
      const r = Math.random() * 16 | 0;
      const v = c === 'x' ? r : (r & 0x3 | 0x8);
      return v.toString(16);
    });
  }
  
  track(eventName, data = {}) {
    const event = {
      type: eventName,
      timestamp: Date.now(),
      sessionId: this.sessionId,
      mediaId: this.player.getCurrentMediaId(),
      currentTime: this.player.getCurrentTime(),
      duration: this.player.getDuration(),
      ...data
    };
    
    this.events.push(event);
    
    if (this.events.length >= this.batchSize) {
      this.flush();
    }
  }
  
  async flush() {
    if (this.events.length === 0) return;
    
    const eventsToSend = [...this.events];
    this.events = [];
    
    try {
      await fetch(this.endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ events: eventsToSend })
      });
    } catch (error) {
      console.error('[Analytics] Failed to send events:', error);
      // Re-add events on failure
      this.events = [...eventsToSend, ...this.events];
    }
  }
}
```

## WASM Plugin Examples

### 1. Audio Normalizer (Rust)

Normalize audio levels in real-time:

```rust
// src/lib.rs

use wasm_bindgen::prelude::*;
use vantis_player_wasm::{Plugin, Player, AudioBuffer};

#[wasm_bindgen]
pub struct AudioNormalizer {
    target_level: f64,
    smoothing: f64,
    current_level: f64,
}

#[wasm_bindgen]
impl AudioNormalizer {
    #[wasm_bindgen(constructor)]
    pub fn new(target_level: f64, smoothing: f64) -> Self {
        AudioNormalizer {
            target_level,
            smoothing,
            current_level: 0.0,
        }
    }
    
    pub fn process(&mut self, buffer: &mut AudioBuffer) {
        let channels = buffer.channels();
        
        for ch in 0..channels {
            let data = buffer.get_channel_data_mut(ch);
            
            // Calculate RMS
            let rms = self.calculate_rms(data);
            
            // Smooth level changes
            self.current_level = self.smoothing * rms + (1.0 - self.smoothing) * self.current_level;
            
            // Apply gain
            if self.current_level > 0.0 {
                let gain = self.target_level / self.current_level;
                let clamped_gain = gain.clamp(0.1, 10.0);
                
                for sample in data.iter_mut() {
                    *sample *= clamped_gain as f32;
                }
            }
        }
    }
    
    fn calculate_rms(&self, data: &[f32]) -> f64 {
        let sum: f64 = data.iter().map(|&x| (x * x) as f64).sum();
        (sum / data.len() as f64).sqrt()
    }
    
    pub fn id(&self) -> String { "audio-normalizer".to_string() }
    pub fn name(&self) -> String { "Audio Normalizer".to_string() }
    pub fn version(&self) -> String { "1.0.0".to_string() }
}
```

### 2. Video Filter Plugin (Rust)

Apply real-time video filters:

```rust
// src/lib.rs

use wasm_bindgen::prelude::*;
use vantis_player_wasm::{VideoFrame, VideoFilter};

#[wasm_bindgen]
pub struct ColorGradingFilter {
    contrast: f32,
    brightness: f32,
    saturation: f32,
}

#[wasm_bindgen]
impl ColorGradingFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(contrast: f32, brightness: f32, saturation: f32) -> Self {
        ColorGradingFilter {
            contrast,
            brightness,
            saturation,
        }
    }
    
    pub fn process(&mut self, frame: &mut VideoFrame) {
        let data = frame.data_mut();
        
        for i in (0..data.len()).step_by(4) {
            let r = data[i] as f32;
            let g = data[i + 1] as f32;
            let b = data[i + 2] as f32;
            
            // Apply contrast
            let mut r = (r - 128.0) * self.contrast + 128.0;
            let mut g = (g - 128.0) * self.contrast + 128.0;
            let mut b = (b - 128.0) * self.contrast + 128.0;
            
            // Apply brightness
            r += self.brightness * 255.0;
            g += self.brightness * 255.0;
            b += self.brightness * 255.0;
            
            // Apply saturation
            let gray = 0.299 * r + 0.587 * g + 0.114 * b;
            r = gray + self.saturation * (r - gray);
            g = gray + self.saturation * (g - gray);
            b = gray + self.saturation * (b - gray);
            
            // Clamp values
            data[i] = r.clamp(0.0, 255.0) as u8;
            data[i + 1] = g.clamp(0.0, 255.0) as u8;
            data[i + 2] = b.clamp(0.0, 255.0) as u8;
        }
    }
    
    pub fn set_contrast(&mut self, value: f32) { self.contrast = value; }
    pub fn set_brightness(&mut self, value: f32) { self.brightness = value; }
    pub fn set_saturation(&mut self, value: f32) { self.saturation = value; }
}
```

## Native Plugin Examples

### 1. Hardware Decoder Plugin (C++)

Utilize hardware video decoding:

```cpp
// hardware_decoder.cpp

#include "plugin.h"
#include <string>

#ifdef _WIN32
#include <d3d11.h>
#include <dxgi.h>
#endif

class HardwareDecoder {
private:
    PluginContext* context;
    bool initialized;
    
#ifdef _WIN32
    ID3D11Device* d3d_device;
    ID3D11DeviceContext* d3d_context;
#endif
    
public:
    HardwareDecoder(PluginContext* ctx) 
        : context(ctx), initialized(false) {}
    
    bool initialize() {
#ifdef _WIN32
        HRESULT hr = D3D11CreateDevice(
            nullptr,
            D3D_DRIVER_TYPE_HARDWARE,
            nullptr,
            0,
            nullptr,
            0,
            D3D11_SDK_VERSION,
            &d3d_device,
            nullptr,
            &d3d_context
        );
        
        if (SUCCEEDED(hr)) {
            context->log("Hardware decoder initialized with D3D11");
            initialized = true;
            return true;
        }
#endif
        context->log("Hardware decoder initialization failed");
        return false;
    }
    
    void shutdown() {
#ifdef _WIN32
        if (d3d_context) d3d_context->Release();
        if (d3d_device) d3d_device->Release();
#endif
        initialized = false;
    }
    
    bool decodeFrame(const uint8_t* data, size_t size, VideoFrame* output) {
        if (!initialized) return false;
        
        // Hardware decoding implementation
        // ...
        
        return true;
    }
};

static HardwareDecoder* g_decoder = nullptr;

extern "C" {

EXPORT const PluginInfo* get_plugin_info() {
    static PluginInfo info = {
        "hardware-decoder",
        "Hardware Decoder",
        "1.0.0",
        "Vantis Media",
        "GPU-accelerated video decoding"
    };
    return &info;
}

EXPORT int plugin_init(PluginContext* context) {
    g_decoder = new HardwareDecoder(context);
    return g_decoder->initialize() ? 0 : -1;
}

EXPORT int plugin_shutdown() {
    if (g_decoder) {
        g_decoder->shutdown();
        delete g_decoder;
        g_decoder = nullptr;
    }
    return 0;
}

}
```

### 2. Audio Device Selector (C++)

Enumerate and select audio devices:

```cpp
// audio_device_selector.cpp

#include "plugin.h"
#include <vector>
#include <string>

#ifdef _WIN32
#include <mmdeviceapi.h>
#include <Functiondiscoverykeys_devpkey.h>
#endif

struct AudioDevice {
    std::string id;
    std::string name;
    bool isDefault;
};

class AudioDeviceManager {
private:
    PluginContext* context;
    std::vector<AudioDevice> devices;
    
public:
    AudioDeviceManager(PluginContext* ctx) : context(ctx) {}
    
    std::vector<AudioDevice> enumerateDevices() {
        devices.clear();
        
#ifdef _WIN32
        HRESULT hr;
        IMMDeviceEnumerator* enumerator = nullptr;
        IMMDeviceCollection* collection = nullptr;
        
        hr = CoCreateInstance(
            __uuidof(MMDeviceEnumerator),
            nullptr,
            CLSCTX_ALL,
            __uuidof(IMMDeviceEnumerator),
            (void**)&enumerator
        );
        
        if (SUCCEEDED(hr)) {
            hr = enumerator->EnumAudioEndpoints(
                eRender,
                DEVICE_STATE_ACTIVE,
                &collection
            );
            
            if (SUCCEEDED(hr)) {
                UINT count;
                collection->GetCount(&count);
                
                for (UINT i = 0; i < count; i++) {
                    IMMDevice* device = nullptr;
                    collection->Item(i, &device);
                    
                    if (device) {
                        AudioDevice dev;
                        
                        LPWSTR id;
                        device->GetId(&id);
                        dev.id = wstring_to_string(id);
                        
                        IPropertyStore* props;
                        device->OpenPropertyStore(STGM_READ, &props);
                        
                        PROPVARIANT name;
                        PropVariantInit(&name);
                        props->GetValue(PKEY_Device_FriendlyName, &name);
                        dev.name = wstring_to_string(name.pwszVal);
                        PropVariantClear(&name);
                        
                        props->Release();
                        device->Release();
                        
                        devices.push_back(dev);
                    }
                }
                collection->Release();
            }
            enumerator->Release();
        }
#endif
        
        return devices;
    }
    
    bool selectDevice(const std::string& deviceId) {
        // Implementation to switch audio output
        context->log(("Selected device: " + deviceId).c_str());
        return true;
    }
    
private:
    std::string wstring_to_string(const std::wstring& wstr) {
        return std::string(wstr.begin(), wstr.end());
    }
};

static AudioDeviceManager* g_deviceManager = nullptr;

extern "C" {

EXPORT int plugin_init(PluginContext* context) {
    g_deviceManager = new AudioDeviceManager(context);
    return 0;
}

EXPORT int plugin_shutdown() {
    delete g_deviceManager;
    g_deviceManager = nullptr;
    return 0;
}

EXPORT const char* get_audio_devices() {
    auto devices = g_deviceManager->enumerateDevices();
    // Return JSON string of devices
    return "[]";
}

}
```

## Plugin Installation

### Manual Installation

1. Copy plugin files to the plugins directory
2. Add plugin configuration to player config
3. Restart the player

```bash
# Copy plugin
cp my-plugin.js /path/to/vantis/plugins/

# Or install from npm
npm install @vantis/plugin-my-plugin
```

### Programmatic Installation

```javascript
import MyPlugin from './plugins/my-plugin.js';

const player = new VantisPlayer('#player', {
  plugins: [
    {
      plugin: MyPlugin,
      config: {
        // Plugin-specific configuration
      }
    }
  ]
});
```

## Testing Plugins

```javascript
// test/plugin.test.js

import { describe, it, expect } from 'vitest';
import MyPlugin from '../src/my-plugin.js';

describe('MyPlugin', () => {
  it('should load successfully', async () => {
    const mockPlayer = createMockPlayer();
    const plugin = new MyPlugin();
    
    await plugin.onLoad(mockPlayer);
    
    expect(plugin.id).toBe('my-plugin');
    expect(mockPlayer.plugins.registered).toContain('my-plugin');
  });
  
  it('should handle events', async () => {
    const mockPlayer = createMockPlayer();
    const plugin = new MyPlugin();
    
    await plugin.onLoad(mockPlayer);
    
    mockPlayer.emit('player:play');
    
    expect(plugin.events).toContain('play');
  });
});
```

## Publishing Plugins

### NPM Package

```json
{
  "name": "@vantis/plugin-my-plugin",
  "version": "1.0.0",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "files": ["dist", "README.md"],
  "peerDependencies": {
    "vantis-player": "^3.0.0"
  }
}
```

### Plugin Registry

```bash
# Publish to Vantis Plugin Registry
vantis plugin publish my-plugin
```

For more plugin ideas and community contributions, visit the [Vantis Plugin Gallery](https://plugins.vantis.media).