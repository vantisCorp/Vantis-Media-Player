---
sidebar_position: 4
---

# Configuration Guide

Vantis Media Player is highly customizable. This guide covers all available configuration options.

## Configuration Files

Vantis Media Player looks for configuration files in the following locations (in order of priority):

1. `./vantismedia.toml` (Current directory)
2. `~/.vantismedia/config.toml` (User home directory)
3. `/etc/vantismedia/config.toml` (System-wide)

The first file found is used, and other files are ignored.

## Configuration Structure

The configuration file uses TOML format:

```toml
# General settings
[general]
theme = "dark"
language = "en"
volume = 80
auto_save = true

# Player settings
[player]
auto_play = true
hardware_acceleration = true
remember_position = true
loop = false
shuffle = false

# Video settings
[video]
decoder = "auto"
deinterlace = false
contrast = 1.0
brightness = 1.0
saturation = 1.0
hue = 0.0

# Audio settings
[audio]
output_device = "default"
latency = 50
normalization = false
visualizer = "none"

# Subtitle settings
[subtitle]
enabled = true
font = "Roboto"
size = 24
language = "auto"
background = true
encoding = "utf-8"

# Streaming settings
[streaming]
buffer_size = 32768
low_latency = false
adaptive_bitrate = true
max_bitrate = 0  # 0 = unlimited

# Plugin settings
[plugins]
auto_load = true
allowed_extensions = [".wasm", ".so", ".dll", ".dylib"]

# Network settings
[network]
proxy = ""
user_agent = "VantisMedia/1.0"
timeout = 30
max_connections = 4

# Performance settings
[performance]
gpu_acceleration = true
thread_count = 0  # 0 = auto
cache_size = 256  # MB

# Security settings
[security]
allow_remote = false
verify_certificates = true
log_sensitive = false

# Logging settings
[logging]
level = "info"
file = ""
console = true
max_size = 10  # MB
max_files = 5
```

## Configuration Options

### General Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `theme` | string | `"dark"` | Theme: `"light"`, `"dark"`, or `"auto"` |
| `language` | string | `"en"` | Language code (e.g., `"en"`, `"pl"`, `"de"`) |
| `volume` | integer | `80` | Default volume (0-100) |
| `auto_save` | boolean | `true` | Automatically save settings |

### Player Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `auto_play` | boolean | `true` | Automatically play when file is loaded |
| `hardware_acceleration` | boolean | `true` | Use GPU for decoding |
| `remember_position` | boolean | `true` | Remember playback position |
| `loop` | boolean | `false` | Loop playback |
| `shuffle` | boolean | `false` | Shuffle playlist |

### Video Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `decoder` | string | `"auto"` | Video decoder: `"auto"`, `"software"`, `"hardware"` |
| `deinterlace` | boolean | `false` | Deinterlace video |
| `contrast` | float | `1.0` | Contrast (0.0-2.0) |
| `brightness` | float | `1.0` | Brightness (0.0-2.0) |
| `saturation` | float | `1.0` | Saturation (0.0-2.0) |
| `hue` | float | `0.0` | Hue (-180 to 180) |

### Audio Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `output_device` | string | `"default"` | Audio output device name |
| `latency` | integer | `50` | Audio latency in milliseconds |
| `normalization` | boolean | `false` | Normalize audio volume |
| `visualizer` | string | `"none"` | Visualizer: `"none"`, `"bars"`, `"wave"`, `"spectrum"` |

### Subtitle Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable subtitles by default |
| `font` | string | `"Roboto"` | Subtitle font family |
| `size` | integer | `24` | Font size in pixels |
| `language` | string | `"auto"` | Subtitle language (ISO 639-1 code) |
| `background` | boolean | `true` | Show subtitle background |
| `encoding` | string | `"utf-8"` | Subtitle text encoding |

### Streaming Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `buffer_size` | integer | `32768` | Buffer size in bytes |
| `low_latency` | boolean | `false` | Enable low-latency mode |
| `adaptive_bitrate` | boolean | `true` | Enable adaptive bitrate |
| `max_bitrate` | integer | `0` | Maximum bitrate in Mbps (0 = unlimited) |

### Plugin Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `auto_load` | boolean | `true` | Automatically load plugins on startup |
| `allowed_extensions` | array | `[".wasm", ...]` | Allowed plugin file extensions |

### Network Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `proxy` | string | `""` | HTTP proxy URL (empty = no proxy) |
| `user_agent` | string | `"VantisMedia/1.0"` | HTTP user agent string |
| `timeout` | integer | `30` | Network timeout in seconds |
| `max_connections` | integer | `4` | Maximum simultaneous connections |

### Performance Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `gpu_acceleration` | boolean | `true` | Enable GPU acceleration |
| `thread_count` | integer | `0` | Number of threads (0 = auto) |
| `cache_size` | integer | `256` | Cache size in MB |

### Security Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `allow_remote` | boolean | `false` | Allow remote control |
| `verify_certificates` | boolean | `true` | Verify SSL certificates |
| `log_sensitive` | boolean | `false` | Log sensitive information |

### Logging Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `level` | string | `"info"` | Log level: `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"` |
| `file` | string | `""` | Log file path (empty = no file logging) |
| `console` | boolean | `true` | Enable console logging |
| `max_size` | integer | `10` | Maximum log file size in MB |
| `max_files` | integer | `5` | Maximum number of log files to keep |

## Command Line Options

You can override configuration options with command line flags:

```bash
# Override theme
vantismedia --theme light video.mp4

# Set volume
vantismedia --volume 90 video.mp4

# Enable low latency
vantismedia --low-latency stream.m3u8

# Disable hardware acceleration
vantismedia --no-hwaccel video.mp4

# Load specific config file
vantismedia --config /path/to/config.toml video.mp4

# Show available options
vantismedia --help
```

## Environment Variables

Configuration can also be set via environment variables:

```bash
# General settings
export VANTIS_THEME="dark"
export VANTIS_LANGUAGE="en"
export VANTIS_VOLUME="80"

# Player settings
export VANTIS_AUTO_PLAY="true"
export VANTIS_HWACCEL="true"

# Network settings
export VANTIS_PROXY="http://proxy.example.com:8080"
export VANTIS_TIMEOUT="30"

# Logging
export VANTIS_LOG_LEVEL="debug"
export VANTIS_LOG_FILE="/var/log/vantismedia.log"
```

## Example Configurations

### Minimal Configuration

```toml
[general]
theme = "dark"
volume = 80
```

### Streaming Optimization

```toml
[streaming]
low_latency = true
adaptive_bitrate = true
buffer_size = 16384

[player]
hardware_acceleration = true
```

### High Quality Playback

```toml
[video]
decoder = "hardware"
deinterlace = true

[audio]
latency = 25
normalization = true

[performance]
gpu_acceleration = true
cache_size = 512
```

### Developer Configuration

```toml
[logging]
level = "debug"
file = "/tmp/vantismedia.log"
console = true

[plugins]
auto_load = true

[security]
log_sensitive = true
```

## Validation

You can validate your configuration file:

```bash
# Check configuration syntax
vantismedia --validate-config

# Show effective configuration (merged from all sources)
vantismedia --show-config
```

## Next Steps

- [Quick Start Guide](./quickstart) - Get started quickly
- [Core Features](../core-features/) - Learn about player features
- [API Reference](../api/) - Configure programmatically

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)