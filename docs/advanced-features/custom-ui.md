---
sidebar_position: 4
---

# Custom UI

Vantis Media Player provides extensive customization options for creating custom user interfaces and themes.

## UI Customization Overview

Vantis Media Player supports:

- **CSS-based theming** - Custom colors, fonts, and styles
- **Component customization** - Modify or replace UI components
- **Layout changes** - Rearrange UI elements
- **Custom controls** - Add custom buttons and controls
- **Responsive design** - Adapt to different screen sizes
- **Dark/Light modes** - Automatic theme switching

## Themes

### Built-in Themes

```javascript
// Set theme
player.setTheme('dark');
player.setTheme('light');
player.setTheme('auto');  // Follow system preference

// Get current theme
const theme = player.getTheme();
```

### Custom Themes

Create a custom theme:

```css
/* Custom theme - themes/my-theme.css */
:root {
  --primary-color: #25c2a0;
  --secondary-color: #00d4ff;
  --background-color: #1a1a2e;
  --surface-color: #16213e;
  --text-color: #ffffff;
  --text-secondary: #a0a0a0;
  --border-color: #2a2a4a;
  --accent-color: #e94560;
  
  --font-family: 'Roboto', system-ui, sans-serif;
  --font-size: 16px;
  --border-radius: 8px;
}

.player-container {
  background-color: var(--background-color);
  color: var(--text-color);
  font-family: var(--font-family);
}

.controls-bar {
  background-color: var(--surface-color);
  border-top: 1px solid var(--border-color);
}

.button {
  background-color: var(--primary-color);
  color: white;
  border-radius: var(--border-radius);
}

.button:hover {
  background-color: var(--secondary-color);
}

.progress-bar {
  background-color: var(--border-color);
}

.progress-bar-fill {
  background-color: var(--primary-color);
}
```

Load custom theme:

```javascript
// Load CSS theme
player.loadTheme('themes/my-theme.css');

// Or load from URL
player.loadTheme('https://example.com/theme.css');
```

### Theme Variables

Available CSS variables:

| Variable | Description | Default (Dark) |
|----------|-------------|----------------|
| `--primary-color` | Primary action color | `#25c2a0` |
| `--secondary-color` | Secondary action color | `#00d4ff` |
| `--background-color` | Main background | `#1a1a2e` |
| `--surface-color` | Surface background | `#16213e` |
| `--text-color` | Primary text | `#ffffff` |
| `--text-secondary` | Secondary text | `#a0a0a0` |
| `--border-color` | Border color | `#2a2a4a` |
| `--accent-color` | Accent color | `#e94560` |
| `--font-family` | Font family | system-ui |
| `--font-size` | Base font size | 16px |
| `--border-radius` | Border radius | 8px |

## Component Customization

### Control Bar

Customize the control bar:

```javascript
const controls = player.getControls();

// Add custom button
controls.addButton({
    id: 'my-button',
    icon: 'star',
    position: 'right',  // left, center, right
    order: 10,
    onClick: () => {
        console.log('Custom button clicked!');
    }
});

// Remove button
controls.removeButton('my-button');

// Move button
controls.moveButton('my-button', 'center', 5);

// Show/hide button
controls.showButton('my-button', false);
```

### Progress Bar

Customize the progress bar:

```javascript
const progressBar = player.getProgressBar();

// Customize appearance
progressBar.setStyle({
    height: '8px',
    borderRadius: '4px',
    backgroundColor: '#2a2a4a',
    fillBackgroundColor: '#25c2a0',
    thumbColor: '#ffffff',
    thumbSize: '16px'
});

// Add markers
progressBar.addMarker(60, 'Chapter 1');
progressBar.addMarker(120, 'Chapter 2');
progressBar.addMarker(180, 'Chapter 3');

// Add preview thumbnails
progressBar.setThumbnails({
    interval: 10,  // seconds
    urlTemplate: 'thumbnails/{time}.jpg'
});
```

### Volume Control

Customize volume control:

```javascript
const volume = player.getVolumeControl();

// Set volume slider style
volume.setStyle({
    orientation: 'horizontal',  // horizontal, vertical
    showValue: true,
    showIcon: true,
    step: 5,  // percentage steps
    min: 0,
    max: 100
});

// Add mute button
volume.addMuteButton();

// Add volume presets
volume.addPresets([0, 25, 50, 75, 100]);
```

### Fullscreen Toggle

Customize fullscreen behavior:

```javascript
const fullscreen = player.getFullscreenControl();

// Customize button
fullscreen.setStyle({
    showIcon: true,
    position: 'right',
    tooltip: 'Toggle Fullscreen'
});

// Handle fullscreen events
fullscreen.on('enter', () => {
    console.log('Entered fullscreen');
});

fullscreen.on('exit', () => {
    console.log('Exited fullscreen');
});
```

## Custom Controls

### Add Custom Button

```javascript
// Add button with icon
player.addControl({
    type: 'button',
    id: 'screenshot',
    icon: 'camera',
    title: 'Take Screenshot',
    position: 'left',
    onClick: async () => {
        const screenshot = await player.captureScreenshot();
        screenshot.save('screenshot.png');
    }
});

// Add button with custom HTML
player.addControl({
    type: 'custom',
    id: 'clock',
    html: '<span id="clock">00:00:00</span>',
    position: 'right',
    onCreate: (element) => {
        setInterval(() => {
            element.textContent = new Date().toLocaleTimeString();
        }, 1000);
    }
});
```

### Add Custom Slider

```javascript
// Add playback speed slider
player.addControl({
    type: 'slider',
    id: 'speed',
    label: 'Speed',
    min: 0.5,
    max: 2.0,
    step: 0.1,
    value: 1.0,
    position: 'right',
    onChange: (value) => {
        player.setSpeed(value);
    }
});
```

### Add Custom Dropdown

```javascript
// Add quality selector
player.addControl({
    type: 'dropdown',
    id: 'quality',
    label: 'Quality',
    position: 'right',
    options: [
        { value: 'auto', label: 'Auto' },
        { value: '1080p', label: '1080p' },
        { value: '720p', label: '720p' },
        { value: '480p', label: '480p' }
    ],
    onSelect: (value) => {
        player.setQuality(value);
    }
});
```

## Layout Customization

### Control Bar Layout

```javascript
// Define control bar layout
player.setLayout({
    controlBar: {
        left: ['play-pause', 'rewind', 'forward'],
        center: ['progress-bar'],
        right: ['volume', 'time-display', 'fullscreen', 'settings']
    }
});
```

### Custom Layout

```javascript
// Create completely custom layout
player.setLayout({
    topBar: ['logo', 'title', 'menu'],
    videoArea: ['video', 'subtitles'],
    bottomBar: ['controls'],
    overlay: ['chapters', 'thumbnails'],
    sidebar: ['playlist', 'info']
});
```

## Responsive Design

### Breakpoints

Define responsive behavior:

```css
/* Mobile */
@media (max-width: 768px) {
  .controls-bar {
    flex-direction: column;
    padding: 10px;
  }
  
  .button {
    padding: 12px;
  }
  
  .progress-bar {
    width: 100%;
  }
}

/* Tablet */
@media (min-width: 769px) and (max-width: 1024px) {
  .controls-bar {
    padding: 15px;
  }
}

/* Desktop */
@media (min-width: 1025px) {
  .controls-bar {
    padding: 20px;
  }
}
```

### Adaptive UI

```javascript
// Respond to screen size changes
player.on('resize', (size) => {
    if (size.width < 768) {
        player.setLayout('mobile');
    } else if (size.width < 1024) {
        player.setLayout('tablet');
    } else {
        player.setLayout('desktop');
    }
});
```

## Animations

### CSS Animations

```css
/* Fade in animation */
@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(20px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.controls-bar {
    animation: fadeIn 0.3s ease-out;
}

/* Button hover effect */
.button {
    transition: all 0.2s ease;
}

.button:hover {
    transform: scale(1.1);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

/* Progress bar fill animation */
.progress-bar-fill {
    transition: width 0.1s linear;
}
```

### JavaScript Animations

```javascript
// Animate controls visibility
player.animateControls({
    show: {
        duration: 300,
        easing: 'ease-out'
    },
    hide: {
        duration: 200,
        easing: 'ease-in'
    },
    autoHide: {
        enabled: true,
        delay: 3000  // ms
    }
});
```

## Accessibility

### ARIA Attributes

Vantis Media Player automatically adds ARIA attributes for accessibility:

```html
<!-- Example generated HTML -->
<button 
    id="play-button"
    aria-label="Play"
    aria-pressed="false"
    role="button">
    <span class="icon-play"></span>
</button>

<div 
    role="slider"
    aria-label="Volume"
    aria-valuenow="80"
    aria-valuemin="0"
    aria-valuemax="100"
    aria-valuetext="80%">
</div>
```

### Keyboard Navigation

Enable keyboard navigation:

```javascript
player.setKeyboardShortcuts({
    enabled: true,
    shortcuts: {
        'Space': 'toggle-play',
        'ArrowLeft': 'seek-backward',
        'ArrowRight': 'seek-forward',
        'ArrowUp': 'volume-up',
        'ArrowDown': 'volume-down',
        'f': 'fullscreen',
        'm': 'mute'
    }
});
```

### Screen Reader Support

Add screen reader announcements:

```javascript
player.on('play', () => {
    player.announce('Playing');
});

player.on('pause', () => {
    player.announce('Paused');
});

player.on('seek', (position) => {
    const time = formatTime(position);
    player.announce(`Seeked to ${time}`);
});
```

## Custom Components

### Create Custom Component

```javascript
// Define custom component class
class ClockComponent extends vantismedia.Component {
    constructor(options) {
        super(options);
        this.clockElement = null;
    }

    render() {
        return `
            <div class="clock-component">
                <span id="clock">00:00:00</span>
            </div>
        `;
    }

    onMount() {
        this.clockElement = this.element.querySelector('#clock');
        this.interval = setInterval(() => {
            this.clockElement.textContent = new Date().toLocaleTimeString();
        }, 1000);
    }

    onUnmount() {
        clearInterval(this.interval);
    }
}

// Register component
vantismedia.registerComponent('clock', ClockComponent);

// Use component
player.addComponent({
    type: 'clock',
    position: 'top-right'
});
```

### Component Communication

```javascript
// Component A
class ComponentA extends vantismedia.Component {
    onClick() {
        this.emit('custom-event', { data: 'Hello from A' });
    }
}

// Component B
class ComponentB extends vantismedia.Component {
    onMount() {
        this.on('custom-event', (data) => {
            console.log('Received:', data);
        });
    }
}
```

## UI Presets

### Minimal UI

```javascript
player.setUIPreset('minimal');
```

Presets:
- `minimal` - Only essential controls
- `compact` - Compact layout
- `full` - All controls visible
- `cinema` - Cinematic experience
- `default` - Standard layout

## Export/Import UI

### Export UI Configuration

```javascript
// Export current UI configuration
const config = player.exportUIConfig();
const json = JSON.stringify(config, null, 2);
download('ui-config.json', json);
```

### Import UI Configuration

```javascript
// Import UI configuration
const config = await loadConfig('ui-config.json');
player.importUIConfig(config);
```

## Troubleshooting

### Custom Styles Not Applied

```javascript
// Check CSS loading
console.log(player.isThemeLoaded('my-theme.css'));

// Force reload
player.reloadTheme();
```

### Layout Issues

```javascript
// Reset to default layout
player.resetLayout();

// Check layout structure
console.log(player.getLayoutStructure());
```

### Component Not Showing

```javascript
// Check component visibility
const component = player.getComponent('my-component');
console.log(component.isVisible());

// Force render
component.render();
```

## Next Steps

- **[Performance](./performance)** - Performance optimization
- **[Examples](../examples/)** - UI customization examples
- **[API Reference](../api/)** - Complete UI API

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)