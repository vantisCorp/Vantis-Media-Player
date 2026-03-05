---
sidebar_position: 4
---

# Native Plugins

Native plugins allow you to extend Vantis Media Player with system-level capabilities, hardware access, and high-performance processing using native languages like C++, Rust, or Swift. This guide covers native plugin development for desktop and mobile platforms.

## Why Native Plugins?

Native plugins provide:

- **Direct hardware access**: GPU, audio devices, system APIs
- **Maximum performance**: Zero-copy, native speed
- **System integration**: Native UI, file system access
- **Legacy support**: Integrate existing native libraries
- **Platform-specific features**: Platform-specific APIs

## Architecture

```
┌─────────────────────────────────────┐
│  Vantis Media Player (JS/WASM)     │
└──────────┬──────────────────────────┘
           │ FFI Bridge
┌──────────▼──────────────────────────┐
│  Native Plugin Manager              │
├─────────────────────────────────────┤
│  Plugin Loader                      │
│  IPC Communication                  │
│  Memory Management                  │
└──────────┬──────────────────────────┘
           │
    ┌──────┴──────┬──────────┐
    ▼             ▼          ▼
┌───────┐    ┌────────┐  ┌──────┐
│ macOS │    │ Windows│  │ Linux│
└───────┘    └────────┘  └──────┘
```

## Native Plugin Interface

### Plugin Manifest

```json
{
  "name": "my-native-plugin",
  "version": "1.0.0",
  "type": "native",
  "platform": ["windows", "macos", "linux"],
  "architecture": ["x64", "arm64"],
  "entry": "libmyplugin.so",
  "api_version": "2.0",
  "capabilities": [
    "audio_processing",
    "video_decoding",
    "hardware_access"
  ]
}
```

### C++ Plugin Header

```cpp
#ifndef MY_PLUGIN_H
#define MY_PLUGIN_H

#include <stdint.h>
#include <stdbool.h>

#ifdef _WIN32
    #define EXPORT __declspec(dllexport)
#else
    #define EXPORT __attribute__((visibility("default")))
#endif

#ifdef __cplusplus
extern "C" {
#endif

// Plugin info
typedef struct {
    const char* id;
    const char* name;
    const char* version;
    const char* author;
    const char* description;
} PluginInfo;

// Context passed to plugin
typedef struct {
    void* player_context;
    void (*log)(const char* message);
    void (*emit_event)(const char* event, const char* data);
    void* (*get_config)(const char* key);
} PluginContext;

// Audio buffer
typedef struct {
    float* data;
    int channels;
    int samples;
    int sample_rate;
} AudioBuffer;

// Video frame
typedef struct {
    uint8_t* data;
    int width;
    int height;
    int format;  // 0=RGBA, 1=RGB, 2=YUV420
    int stride;
} VideoFrame;

// Plugin lifecycle functions
EXPORT const PluginInfo* get_plugin_info();
EXPORT int plugin_init(PluginContext* context);
EXPORT int plugin_shutdown();
EXPORT int plugin_configure(const char* config_json);

// Processing functions
EXPORT int process_audio(AudioBuffer* buffer);
EXPORT int process_video(VideoFrame* frame);

#ifdef __cplusplus
}
#endif

#endif // MY_PLUGIN_H
```

## Windows Native Plugins

### Project Structure

```
my-native-plugin/
├── CMakeLists.txt
├── include/
│   └── my_plugin.h
├── src/
│   ├── plugin.cpp
│   ├── audio_processor.cpp
│   └── video_decoder.cpp
└── manifest.json
```

### CMakeLists.txt (Windows)

```cmake
cmake_minimum_required(VERSION 3.15)
project(MyNativePlugin)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Windows-specific settings
if(WIN32)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /O2 /MT")
    set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} /DEF:plugin.def")
endif()

# Source files
set(SOURCES
    src/plugin.cpp
    src/audio_processor.cpp
    src/video_decoder.cpp
)

# Create shared library
add_library(${PROJECT_NAME} SHARED ${SOURCES})

# Include directories
target_include_directories(${PROJECT_NAME} PRIVATE
    ${CMAKE_SOURCE_DIR}/include
)

# Link libraries
if(WIN32)
    target_link_libraries(${PROJECT_NAME}
        d3d11
        dxgi
        mfplat
        mfreadwrite
    )
endif()

# Output directory
set_target_properties(${PROJECT_NAME} PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

# Copy manifest
configure_file(
    ${CMAKE_SOURCE_DIR}/manifest.json
    ${CMAKE_BINARY_DIR}/bin/manifest.json
    COPYONLY
)
```

### Audio Processing Plugin (Windows)

```cpp
#include "my_plugin.h"
#include <windows.h>
#include <mmdeviceapi.h>
#include <audioclient.h>
#include <cmath>

class AudioEnhancer {
private:
    PluginContext* context;
    float bass_gain;
    float treble_gain;
    
public:
    AudioEnhancer(PluginContext* ctx) : context(ctx), bass_gain(1.0f), treble_gain(1.0f) {}
    
    void setBassGain(float gain) {
        bass_gain = gain;
        context->log("Bass gain set to " + std::to_string(gain));
    }
    
    void setTrebleGain(float gain) {
        treble_gain = gain;
        context->log("Treble gain set to " + std::to_string(gain));
    }
    
    void process(AudioBuffer* buffer) {
        // Apply bass boost to low frequencies
        for (int i = 0; i < buffer->samples; i++) {
            for (int ch = 0; ch < buffer->channels; ch++) {
                int idx = i * buffer->channels + ch;
                float sample = buffer->data[idx];
                
                // Simple bass boost
                if (ch == 0) { // Process left channel
                    buffer->data[idx] *= bass_gain;
                } else {
                    buffer->data[idx] *= treble_gain;
                }
            }
        }
    }
};

// Global plugin instance
static AudioEnhancer* g_enhancer = nullptr;

extern "C" {

EXPORT const PluginInfo* get_plugin_info() {
    static PluginInfo info = {
        "audio-enhancer-native",
        "Audio Enhancer Native",
        "1.0.0",
        "Vantis Media",
        "Native audio enhancement plugin with bass and treble control"
    };
    return &info;
}

EXPORT int plugin_init(PluginContext* context) {
    context->log("Audio Enhancer plugin initializing...");
    
    g_enhancer = new AudioEnhancer(context);
    
    context->emit_event("plugin:loaded", "{&quot;plugin&quot;:&quot;audio-enhancer-native&quot;}");
    
    return 0; // Success
}

EXPORT int plugin_shutdown() {
    if (g_enhancer) {
        delete g_enhancer;
        g_enhancer = nullptr;
    }
    return 0;
}

EXPORT int plugin_configure(const char* config_json) {
    // Parse JSON configuration
    // For simplicity, use hardcoded values
    if (g_enhancer) {
        g_enhancer->setBassGain(1.5f);
        g_enhancer->setTrebleGain(1.2f);
    }
    return 0;
}

EXPORT int process_audio(AudioBuffer* buffer) {
    if (g_enhancer && buffer) {
        g_enhancer->process(buffer);
    }
    return 0;
}

} // extern "C"
```

### Building for Windows

```bash
# Configure
cmake -B build -G "Visual Studio 17 2022" -A x64

# Build
cmake --build build --config Release

# Output: build/bin/AudioEnhancer.dll
```

## macOS Native Plugins

### Project Structure

```
my-native-plugin/
├── CMakeLists.txt
├── include/
│   └── my_plugin.h
├── src/
│   └── plugin.mm
└── manifest.json
```

### CMakeLists.txt (macOS)

```cmake
cmake_minimum_required(VERSION 3.15)
project(MyNativePlugin)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# macOS-specific settings
if(APPLE)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3 -fPIC")
    set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -undefined dynamic_lookup")
endif()

# Source files
set(SOURCES
    src/plugin.mm
)

# Create shared library
add_library(${PROJECT_NAME} SHARED ${SOURCES})

# Include directories
target_include_directories(${PROJECT_NAME} PRIVATE
    ${CMAKE_SOURCE_DIR}/include
)

# Link frameworks
if(APPLE)
    target_link_libraries(${PROJECT_NAME}
        "-framework CoreAudio"
        "-framework AudioToolbox"
        "-framework AVFoundation"
        "-framework CoreVideo"
    )
endif()

# Output
set_target_properties(${PROJECT_NAME} PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)
```

### Audio Processing Plugin (macOS)

```objectivec
#include "my_plugin.h"
#include <CoreAudio/CoreAudio.h>
#include <AudioToolbox/AudioToolbox.h>
#include <AVFoundation/AVFoundation.h>
#include <Accelerate/Accelerate.h>

@interface AudioProcessor : NSObject {
    PluginContext* context;
    Float32 bassGain;
    Float32 trebleGain;
    vDSP_DFT_Setup dftSetup;
}

- (instancetype)initWithContext:(PluginContext*)ctx;
- (void)setBassGain:(Float32)gain;
- (void)setTrebleGain:(Float32)gain;
- (void)processBuffer:(AudioBuffer*)buffer;

@end

@implementation AudioProcessor

- (instancetype)initWithContext:(PluginContext*)ctx {
    self = [super init];
    if (self) {
        context = ctx;
        bassGain = 1.0f;
        trebleGain = 1.0f;
        
        // Initialize FFT setup
        dftSetup = vDSP_DFT_zrop_CreateSetupD(NULL, 1024);
    }
    return self;
}

- (void)dealloc {
    if (dftSetup) {
        vDSP_DFT_DestroySetup(dftSetup);
    }
}

- (void)setBassGain:(Float32)gain {
    bassGain = gain;
    context->log([NSString stringWithFormat:@"Bass gain: %.2f", gain].UTF8String);
}

- (void)setTrebleGain:(Float32)gain {
    trebleGain = gain;
    context->log([NSString stringWithFormat:@"Treble gain: %.2f", gain].UTF8String);
}

- (void)processBuffer:(AudioBuffer*)buffer {
    Float32* data = (Float32*)buffer->data;
    int samples = buffer->samples;
    
    // Apply frequency-domain processing
    for (int ch = 0; ch < buffer->channels; ch++) {
        Float32* channelData = data + ch * samples;
        
        // Create complex buffers for FFT
        DSPSplitComplex splitComplex;
        splitComplex.realp = (Float32*)malloc(samples * sizeof(Float32));
        splitComplex.imagp = (Float32*)malloc(samples * sizeof(Float32));
        
        // Copy input
        memcpy(splitComplex.realp, channelData, samples * sizeof(Float32));
        memset(splitComplex.imagp, 0, samples * sizeof(Float32));
        
        // Perform FFT
        vDSP_DFT_Execute(dftSetup,
                        splitComplex.realp, splitComplex.imagp,
                        splitComplex.realp, splitComplex.imagp);
        
        // Apply frequency response
        for (int i = 0; i < samples / 2; i++) {
            float freq = i * buffer->sample_rate / (float)samples;
            
            // Bass boost (20-250 Hz)
            if (freq >= 20 && freq <= 250) {
                splitComplex.realp[i] *= bassGain;
                splitComplex.imagp[i] *= bassGain;
            }
            
            // Treble boost (2-20 kHz)
            if (freq >= 2000 && freq <= 20000) {
                splitComplex.realp[i] *= trebleGain;
                splitComplex.imagp[i] *= trebleGain;
            }
        }
        
        // Inverse FFT
        vDSP_DFT_Execute(dftSetup,
                        splitComplex.realp, splitComplex.imagp,
                        splitComplex.realp, splitComplex.imagp);
        
        // Copy output
        for (int i = 0; i < samples; i++) {
            channelData[i] = splitComplex.realp[i];
        }
        
        free(splitComplex.realp);
        free(splitComplex.imagp);
    }
}

@end

// Global instance
static AudioProcessor* g_processor = nil;

extern "C" {

EXPORT const PluginInfo* get_plugin_info() {
    static PluginInfo info = {
        "audio-enhancer-macos",
        "Audio Enhancer macOS",
        "1.0.0",
        "Vantis Media",
        "Native audio enhancement for macOS with FFT processing"
    };
    return &info;
}

EXPORT int plugin_init(PluginContext* context) {
    context->log("macOS Audio Enhancer initializing...");
    
    g_processor = [[AudioProcessor alloc] initWithContext:context];
    
    context->emit_event("plugin:loaded", "{&quot;plugin&quot;:&quot;audio-enhancer-macos&quot;}");
    
    return 0;
}

EXPORT int plugin_shutdown() {
    if (g_processor) {
        g_processor = nil;
    }
    return 0;
}

EXPORT int process_audio(AudioBuffer* buffer) {
    if (g_processor && buffer) {
        [g_processor processBuffer:buffer];
    }
    return 0;
}

}
```

### Building for macOS

```bash
# Configure
cmake -B build -DCMAKE_BUILD_TYPE=Release

# Build
cmake --build build

# Output: build/bin/libMyNativePlugin.dylib
```

## Linux Native Plugins

### Project Structure

```
my-native-plugin/
├── CMakeLists.txt
├── include/
│   └── my_plugin.h
├── src/
│   └── plugin.cpp
└── manifest.json
```

### CMakeLists.txt (Linux)

```cmake
cmake_minimum_required(VERSION 3.15)
project(MyNativePlugin)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Linux-specific settings
if(UNIX AND NOT APPLE)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3 -fPIC -fvisibility=hidden")
    set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -Wl,--as-needed")
endif()

# Source files
set(SOURCES
    src/plugin.cpp
)

# Create shared library
add_library(${PROJECT_NAME} SHARED ${SOURCES})

# Include directories
target_include_directories(${PROJECT_NAME} PRIVATE
    ${CMAKE_SOURCE_DIR}/include
)

# Link libraries
if(UNIX AND NOT APPLE)
    target_link_libraries(${PROJECT_NAME}
        pthread
        dl
        asound
        pulse
    )
endif()

# Output
set_target_properties(${PROJECT_NAME} PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

# Set version
set_target_properties(${PROJECT_NAME} PROPERTIES
    VERSION ${PROJECT_VERSION}
    SOVERSION 1
)
```

### Audio Processing Plugin (Linux)

```cpp
#include "my_plugin.h"
#include <alsa/asoundlib.h>
#include <pulse/simple.h>
#include <pulse/error.h>
#include <cmath>

class ALSAEnhancer {
private:
    PluginContext* context;
    snd_pcm_t* pcm_handle;
    float bass_gain;
    float treble_gain;
    
public:
    ALSAEnhancer(PluginContext* ctx) : context(ctx), pcm_handle(nullptr), bass_gain(1.0f), treble_gain(1.0f) {
        // Initialize ALSA
        initializeALSA();
    }
    
    ~ALSAEnhancer() {
        if (pcm_handle) {
            snd_pcm_close(pcm_handle);
        }
    }
    
    void initializeALSA() {
        int err;
        snd_pcm_hw_params_t* hw_params;
        
        // Open PCM device
        err = snd_pcm_open(&pcm_handle, "default", SND_PCM_STREAM_PLAYBACK, 0);
        if (err < 0) {
            context->log(std::string("Cannot open audio device: ") + snd_strerror(err));
            return;
        }
        
        // Allocate hardware parameters
        snd_pcm_hw_params_alloca(&hw_params);
        
        // Initialize hardware parameters
        snd_pcm_hw_params_any(pcm_handle, hw_params);
        snd_pcm_hw_params_set_access(pcm_handle, hw_params, SND_PCM_ACCESS_RW_INTERLEAVED);
        snd_pcm_hw_params_set_format(pcm_handle, hw_params, SND_PCM_FORMAT_FLOAT_LE);
        snd_pcm_hw_params_set_channels(pcm_handle, hw_params, 2);
        
        unsigned int rate = 48000;
        snd_pcm_hw_params_set_rate_near(pcm_handle, hw_params, &rate, 0);
        
        // Set parameters
        err = snd_pcm_hw_params(pcm_handle, hw_params);
        if (err < 0) {
            context->log(std::string("Cannot set parameters: ") + snd_strerror(err));
        }
    }
    
    void setBassGain(float gain) {
        bass_gain = gain;
        context->log("Bass gain set to " + std::to_string(gain));
    }
    
    void setTrebleGain(float gain) {
        treble_gain = gain;
        context->log("Treble gain set to " + std::to_string(gain));
    }
    
    void process(AudioBuffer* buffer) {
        // Apply enhancement
        for (int i = 0; i < buffer->samples; i++) {
            for (int ch = 0; ch < buffer->channels; ch++) {
                int idx = i * buffer->channels + ch;
                float sample = buffer->data[idx];
                
                // Simple bass/treble boost
                if (ch == 0) {
                    buffer->data[idx] *= bass_gain;
                } else {
                    buffer->data[idx] *= treble_gain;
                }
            }
        }
    }
};

// Global instance
static ALSAEnhancer* g_enhancer = nullptr;

extern "C" {

EXPORT const PluginInfo* get_plugin_info() {
    static PluginInfo info = {
        "audio-enhancer-linux",
        "Audio Enhancer Linux",
        "1.0.0",
        "Vantis Media",
        "Native audio enhancement for Linux with ALSA integration"
    };
    return &info;
}

EXPORT int plugin_init(PluginContext* context) {
    context->log("Linux Audio Enhancer initializing...");
    
    g_enhancer = new ALSAEnhancer(context);
    
    context->emit_event("plugin:loaded", "{&quot;plugin&quot;:&quot;audio-enhancer-linux&quot;}");
    
    return 0;
}

EXPORT int plugin_shutdown() {
    if (g_enhancer) {
        delete g_enhancer;
        g_enhancer = nullptr;
    }
    return 0;
}

EXPORT int plugin_configure(const char* config_json) {
    if (g_enhancer) {
        g_enhancer->setBassGain(1.5f);
        g_enhancer->setTrebleGain(1.2f);
    }
    return 0;
}

EXPORT int process_audio(AudioBuffer* buffer) {
    if (g_enhancer && buffer) {
        g_enhancer->process(buffer);
    }
    return 0;
}

}
```

### Building for Linux

```bash
# Configure
cmake -B build -DCMAKE_BUILD_TYPE=Release

# Build
cmake --build build

# Output: build/bin/libMyNativePlugin.so
```

## Loading Native Plugins

### JavaScript Integration

```javascript
// Load native plugin
async function loadNativePlugin(pluginPath) {
  const player = VantisPlayer.getInstance();
  
  try {
    // Load plugin
    const plugin = await player.plugins.loadNative(pluginPath);
    
    // Configure plugin
    await plugin.configure({
      bassGain: 1.5,
      trebleGain: 1.2
    });
    
    console.log('Native plugin loaded:', plugin.info);
    
  } catch (error) {
    console.error('Failed to load native plugin:', error);
  }
}

// Use plugin
loadNativePlugin('plugins/audio-enhancer');
```

## Best Practices

1. **Memory safety**: Use smart pointers, avoid leaks
2. **Thread safety**: Use mutexes for shared resources
3. **Error handling**: Check all return codes
4. **Logging**: Use context->log for debugging
5. **Performance**: Minimize allocations in hot paths
6. **Compatibility**: Support multiple platforms

## Security Considerations

1. **Sandboxing**: Run plugins in separate process
2. **Validation**: Validate all inputs
3. **Resource limits**: Enforce memory/CPU limits
4. **Code signing**: Sign plugins for verification
5. **Permissions**: Request explicit permissions

## Debugging

### Windows

```bash
# Attach debugger
# Set breakpoint in Visual Studio
# Run player with plugin
```

### macOS

```bash
# Use lldb
lldb ./VantisPlayer
(lldb) break set --name plugin_init
(lldb) run
```

### Linux

```bash
# Use gdb
gdb ./VantisPlayer
(gdb) break plugin_init
(gdb) run
```

## Distribution

```bash
# Create platform-specific packages

# Windows
# package installer with DLLs and dependencies

# macOS
# create .app bundle with framework

# Linux
# create .deb/.rpm packages with shared libraries
```