---
sidebar_position: 3
---

# Custom Controls Example

This example demonstrates how to create custom controls for Vantis Media Player, giving you full control over the player's appearance and behavior.

## Basic Custom Controls

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Custom Controls</title>
  <style>
    * {
      box-sizing: border-box;
    }
    
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      margin: 0;
      padding: 20px;
      background: #1a1a2e;
      color: white;
    }
    
    .player-container {
      max-width: 900px;
      margin: 0 auto;
    }
    
    #player {
      width: 100%;
      aspect-ratio: 16 / 9;
      background: #000;
      border-radius: 8px;
      overflow: hidden;
      position: relative;
    }
    
    /* Custom controls overlay */
    .controls-overlay {
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
      background: linear-gradient(transparent, rgba(0, 0, 0, 0.9));
      padding: 20px;
      opacity: 0;
      transition: opacity 0.3s;
    }
    
    #player:hover .controls-overlay,
    #player.paused .controls-overlay {
      opacity: 1;
    }
    
    /* Progress bar */
    .progress-container {
      margin-bottom: 15px;
    }
    
    .progress-bar {
      height: 6px;
      background: rgba(255, 255, 255, 0.2);
      border-radius: 3px;
      cursor: pointer;
      position: relative;
    }
    
    .progress-bar .buffer-progress {
      position: absolute;
      top: 0;
      left: 0;
      height: 100%;
      background: rgba(99, 102, 241, 0.3);
      border-radius: 3px;
    }
    
    .progress-bar .play-progress {
      position: absolute;
      top: 0;
      left: 0;
      height: 100%;
      background: #6366f1;
      border-radius: 3px;
      transition: width 0.1s;
    }
    
    .progress-bar .progress-thumb {
      position: absolute;
      top: 50%;
      left: 0;
      transform: translate(-50%, -50%);
      width: 14px;
      height: 14px;
      background: white;
      border-radius: 50%;
      opacity: 0;
      transition: opacity 0.2s;
    }
    
    .progress-bar:hover .progress-thumb {
      opacity: 1;
    }
    
    /* Time display */
    .time-display {
      font-size: 13px;
      color: rgba(255, 255, 255, 0.8);
      margin-bottom: 12px;
    }
    
    /* Control buttons */
    .controls-row {
      display: flex;
      align-items: center;
      gap: 12px;
    }
    
    .control-btn {
      background: none;
      border: none;
      color: white;
      cursor: pointer;
      padding: 8px;
      border-radius: 4px;
      transition: background 0.2s;
      display: flex;
      align-items: center;
      justify-content: center;
    }
    
    .control-btn:hover {
      background: rgba(255, 255, 255, 0.1);
    }
    
    .control-btn svg {
      width: 24px;
      height: 24px;
      fill: currentColor;
    }
    
    .control-btn.primary {
      background: #6366f1;
    }
    
    .control-btn.primary:hover {
      background: #4f46e5;
    }
    
    /* Volume control */
    .volume-control {
      display: flex;
      align-items: center;
      gap: 8px;
    }
    
    .volume-slider {
      width: 80px;
      height: 4px;
      background: rgba(255, 255, 255, 0.2);
      border-radius: 2px;
      cursor: pointer;
      position: relative;
    }
    
    .volume-slider .volume-fill {
      position: absolute;
      top: 0;
      left: 0;
      height: 100%;
      background: white;
      border-radius: 2px;
    }
    
    /* Menu controls */
    .menu-btn {
      position: relative;
    }
    
    .menu-dropdown {
      position: absolute;
      bottom: 100%;
      right: 0;
      background: rgba(30, 30, 46, 0.95);
      border-radius: 8px;
      padding: 8px 0;
      min-width: 200px;
      margin-bottom: 8px;
      display: none;
      z-index: 100;
    }
    
    .menu-dropdown.show {
      display: block;
    }
    
    .menu-item {
      padding: 10px 16px;
      cursor: pointer;
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    
    .menu-item:hover {
      background: rgba(99, 102, 241, 0.2);
    }
    
    .menu-item.active {
      color: #6366f1;
    }
    
    /* Loading spinner */
    .loading-spinner {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      display: none;
    }
    
    .loading-spinner.show {
      display: block;
    }
    
    .spinner {
      width: 50px;
      height: 50px;
      border: 4px solid rgba(255, 255, 255, 0.2);
      border-top-color: #6366f1;
      border-radius: 50%;
      animation: spin 1s linear infinite;
    }
    
    @keyframes spin {
      to { transform: rotate(360deg); }
    }
    
    /* Big play button */
    .big-play-btn {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 80px;
      height: 80px;
      background: rgba(99, 102, 241, 0.8);
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer;
      transition: transform 0.2s, background 0.2s;
    }
    
    .big-play-btn:hover {
      transform: translate(-50%, -50%) scale(1.1);
      background: rgba(99, 102, 241, 1);
    }
    
    .big-play-btn svg {
      width: 40px;
      height: 40px;
      fill: white;
      margin-left: 5px;
    }
  </style>
</head>
<body>
  <div class="player-container">
    <div id="player">
      <!-- Loading spinner -->
      <div class="loading-spinner" id="loading-spinner">
        <div class="spinner"></div>
      </div>
      
      <!-- Big play button -->
      <div class="big-play-btn" id="big-play-btn">
        <svg viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
      </div>
      
      <!-- Custom controls -->
      <div class="controls-overlay">
        <!-- Progress bar -->
        <div class="progress-container">
          <div class="progress-bar" id="progress-bar">
            <div class="buffer-progress" id="buffer-progress"></div>
            <div class="play-progress" id="play-progress"></div>
            <div class="progress-thumb" id="progress-thumb"></div>
          </div>
        </div>
        
        <!-- Time display -->
        <div class="time-display">
          <span id="current-time">00:00</span> / <span id="duration">00:00</span>
        </div>
        
        <!-- Control buttons -->
        <div class="controls-row">
          <!-- Play/Pause -->
          <button class="control-btn primary" id="play-btn">
            <svg viewBox="0 0 24 24" id="play-icon"><path d="M8 5v14l11-7z"/></svg>
            <svg viewBox="0 0 24 24" id="pause-icon" style="display:none"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>
          </button>
          
          <!-- Skip backward -->
          <button class="control-btn" id="skip-back-btn">
            <svg viewBox="0 0 24 24"><path d="M11 18V6l-8.5 6 8.5 6zm.5-6l8.5 6V6l-8.5 6z"/></svg>
          </button>
          
          <!-- Skip forward -->
          <button class="control-btn" id="skip-forward-btn">
            <svg viewBox="0 0 24 24"><path d="M4 18l8.5-6L4 6v12zm9-12v12l8.5-6L13 6z"/></svg>
          </button>
          
          <!-- Volume -->
          <div class="volume-control">
            <button class="control-btn" id="mute-btn">
              <svg viewBox="0 0 24 24" id="volume-icon"><path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z"/></svg>
              <svg viewBox="0 0 24 24" id="mute-icon" style="display:none"><path d="M16.5 12c0-1.77-1.02-3.29-2.5-4.03v2.21l2.45 2.45c.03-.2.05-.41.05-.63zm2.5 0c0 .94-.2 1.82-.54 2.64l1.51 1.51C20.63 14.91 21 13.5 21 12c0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zM4.27 3L3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06c1.38-.31 2.63-.95 3.69-1.81L19.73 21 21 19.73l-9-9L4.27 3zM12 4L9.91 6.09 12 8.18V4z"/></svg>
            </button>
            <div class="volume-slider" id="volume-slider">
              <div class="volume-fill" id="volume-fill"></div>
            </div>
          </div>
          
          <!-- Time -->
          <div style="flex:1"></div>
          
          <!-- Playback rate -->
          <button class="control-btn menu-btn" id="rate-btn">1x</button>
          
          <!-- Quality -->
          <button class="control-btn menu-btn" id="quality-btn">
            <svg viewBox="0 0 24 24" style="width:20px;height:20px"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/></svg>
          </button>
          
          <!-- Subtitles -->
          <button class="control-btn menu-btn" id="subtitle-btn">
            <svg viewBox="0 0 24 24" style="width:20px;height:20px"><path d="M19 4H5c-1.11 0-2 .9-2 2v12c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V6c0-1.1-.89-2-2-2zm-4 8c0 .55-.45 1-1 1h-2v2h3v2h-5v-6h4c.55 0 1 .45 1 1zM9 12h2v2H9v-2zm0-2h2v-2H9v2z"/></svg>
          </button>
          
          <!-- Fullscreen -->
          <button class="control-btn" id="fullscreen-btn">
            <svg viewBox="0 0 24 24" id="fullscreen-icon"><path d="M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z"/></svg>
            <svg viewBox="0 0 24 24" id="exit-fullscreen-icon" style="display:none"><path d="M5 16h3v3h2v-5H5v2zm3-8H5v2h5V5H8v3zm6 11h2v-3h3v-2h-5v5zm2-11V5h-2v5h5V8h-3z"/></svg>
          </button>
        </div>
        
        <!-- Rate menu -->
        <div class="menu-dropdown" id="rate-menu">
          <div class="menu-item" data-rate="0.25">0.25x</div>
          <div class="menu-item" data-rate="0.5">0.5x</div>
          <div class="menu-item" data-rate="0.75">0.75x</div>
          <div class="menu-item active" data-rate="1">1x</div>
          <div class="menu-item" data-rate="1.25">1.25x</div>
          <div class="menu-item" data-rate="1.5">1.5x</div>
          <div class="menu-item" data-rate="2">2x</div>
        </div>
        
        <!-- Quality menu -->
        <div class="menu-dropdown" id="quality-menu"></div>
        
        <!-- Subtitle menu -->
        <div class="menu-dropdown" id="subtitle-menu"></div>
      </div>
    </div>
  </div>

  <script src="https://cdn.vantis.media/player/latest/vantis.min.js"></script>
  <script>
    // Initialize player with disabled default controls
    const player = new VantisPlayer('#player', {
      controls: false,
      autoplay: false,
      muted: false,
      volume: 0.8
    });
    
    // DOM elements
    const progressBar = document.getElementById('progress-bar');
    const playProgress = document.getElementById('play-progress');
    const bufferProgress = document.getElementById('buffer-progress');
    const progressThumb = document.getElementById('progress-thumb');
    const currentTimeEl = document.getElementById('current-time');
    const durationEl = document.getElementById('duration');
    const playBtn = document.getElementById('play-btn');
    const pauseIcon = document.getElementById('pause-icon');
    const playIcon = document.getElementById('play-icon');
    const muteBtn = document.getElementById('mute-btn');
    const volumeIcon = document.getElementById('volume-icon');
    const muteIcon = document.getElementById('mute-icon');
    const volumeSlider = document.getElementById('volume-slider');
    const volumeFill = document.getElementById('volume-fill');
    const fullscreenBtn = document.getElementById('fullscreen-btn');
    const fullscreenIcon = document.getElementById('fullscreen-icon');
    const exitFullscreenIcon = document.getElementById('exit-fullscreen-icon');
    const bigPlayBtn = document.getElementById('big-play-btn');
    const loadingSpinner = document.getElementById('loading-spinner');
    const playerContainer = document.getElementById('player');
    
    // Format time
    function formatTime(seconds) {
      const mins = Math.floor(seconds / 60);
      const secs = Math.floor(seconds % 60);
      return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
    }
    
    // Update progress
    function updateProgress() {
      const current = player.getCurrentTime();
      const duration = player.getDuration();
      const buffered = player.getBuffered();
      
      const playPercent = (current / duration) * 100;
      const bufferPercent = (buffered / duration) * 100;
      
      playProgress.style.width = `${playPercent}%`;
      bufferProgress.style.width = `${bufferPercent}%`;
      progressThumb.style.left = `${playPercent}%`;
      
      currentTimeEl.textContent = formatTime(current);
      durationEl.textContent = formatTime(duration);
    }
    
    // Seek
    progressBar.addEventListener('click', (e) => {
      const rect = progressBar.getBoundingClientRect();
      const percent = (e.clientX - rect.left) / rect.width;
      const duration = player.getDuration();
      player.seek(duration * percent);
    });
    
    // Play/Pause
    function togglePlay() {
      const state = player.getState();
      if (state.playing) {
        player.pause();
      } else {
        player.play();
      }
    }
    
    playBtn.addEventListener('click', togglePlay);
    bigPlayBtn.addEventListener('click', togglePlay);
    
    // Update play/pause button
    function updatePlayButton() {
      const state = player.getState();
      playerContainer.classList.toggle('paused', !state.playing);
      playIcon.style.display = state.playing ? 'none' : 'block';
      pauseIcon.style.display = state.playing ? 'block' : 'none';
      bigPlayBtn.style.display = state.playing ? 'none' : 'flex';
    }
    
    // Volume control
    function updateVolumeUI() {
      const volume = player.getVolume();
      const muted = player.getMuted();
      
      volumeFill.style.width = `${volume * 100}%`;
      volumeIcon.style.display = muted || volume === 0 ? 'none' : 'block';
      muteIcon.style.display = muted || volume === 0 ? 'block' : 'none';
    }
    
    volumeSlider.addEventListener('click', (e) => {
      const rect = volumeSlider.getBoundingClientRect();
      const volume = (e.clientX - rect.left) / rect.width;
      player.setVolume(Math.max(0, Math.min(1, volume)));
    });
    
    muteBtn.addEventListener('click', () => {
      player.setMuted(!player.getMuted());
    });
    
    // Fullscreen
    fullscreenBtn.addEventListener('click', () => {
      if (document.fullscreenElement) {
        document.exitFullscreen();
      } else {
        playerContainer.requestFullscreen();
      }
    });
    
    // Handle fullscreen change
    document.addEventListener('fullscreenchange', () => {
      const isFullscreen = !!document.fullscreenElement;
      fullscreenIcon.style.display = isFullscreen ? 'none' : 'block';
      exitFullscreenIcon.style.display = isFullscreen ? 'block' : 'none';
    });
    
    // Skip buttons
    document.getElementById('skip-back-btn').addEventListener('click', () => {
      player.seek(Math.max(0, player.getCurrentTime() - 10));
    });
    
    document.getElementById('skip-forward-btn').addEventListener('click', () => {
      player.seek(Math.min(player.getDuration(), player.getCurrentTime() + 10));
    });
    
    // Playback rate menu
    const rateBtn = document.getElementById('rate-btn');
    const rateMenu = document.getElementById('rate-menu');
    
    rateBtn.addEventListener('click', () => {
      rateMenu.classList.toggle('show');
    });
    
    rateMenu.querySelectorAll('.menu-item').forEach(item => {
      item.addEventListener('click', () => {
        const rate = parseFloat(item.dataset.rate);
        player.setPlaybackRate(rate);
        rateBtn.textContent = `${rate}x`;
        rateMenu.querySelectorAll('.menu-item').forEach(i => i.classList.remove('active'));
        item.classList.add('active');
        rateMenu.classList.remove('show');
      });
    });
    
    // Close menus on outside click
    document.addEventListener('click', (e) => {
      if (!e.target.closest('.menu-btn')) {
        document.querySelectorAll('.menu-dropdown').forEach(m => m.classList.remove('show'));
      }
    });
    
    // Player events
    player.on('player:ready', async () => {
      await player.load({
        type: 'video',
        src: 'https://test-videos.co.uk/vids/bigbuckbunny/mp4/h264/360/Big_Buck_Bunny_360_10s_1MB.mp4'
      });
    });
    
    player.on('player:play', () => {
      updatePlayButton();
      loadingSpinner.classList.remove('show');
    });
    
    player.on('player:pause', () => {
      updatePlayButton();
    });
    
    player.on('player:timeupdate', updateProgress);
    
    player.on('player:buffer', () => {
      loadingSpinner.classList.add('show');
    });
    
    player.on('player:ready', () => {
      loadingSpinner.classList.remove('show');
    });
    
    player.on('player:load', () => {
      loadingSpinner.classList.add('show');
    });
  </script>
</body>
</html>
```

## Custom Component Architecture

```javascript
// CustomControls.js

export class CustomControls {
  constructor(player, options = {}) {
    this.player = player;
    this.options = {
      container: null,
      showOnHover: true,
      hideDelay: 3000,
      ...options
    };
    
    this.elements = {};
    this.hideTimer = null;
    this.init();
  }
  
  init() {
    this.createControls();
    this.bindEvents();
    this.setupPlayerEvents();
  }
  
  createControls() {
    // Create control container
    this.elements.container = document.createElement('div');
    this.elements.container.className = 'custom-controls';
    
    // Create progress bar
    this.elements.progress = this.createProgressBar();
    this.elements.container.appendChild(this.elements.progress);
    
    // Create button row
    this.elements.buttons = this.createButtonRow();
    this.elements.container.appendChild(this.elements.buttons);
    
    // Add to DOM
    const target = this.options.container || this.player.getContainer();
    target.appendChild(this.elements.container);
  }
  
  createProgressBar() {
    const container = document.createElement('div');
    container.className = 'progress-container';
    
    const bar = document.createElement('div');
    bar.className = 'progress-bar';
    
    const playProgress = document.createElement('div');
    playProgress.className = 'play-progress';
    
    const bufferProgress = document.createElement('div');
    bufferProgress.className = 'buffer-progress';
    
    const thumb = document.createElement('div');
    thumb.className = 'progress-thumb';
    
    bar.appendChild(bufferProgress);
    bar.appendChild(playProgress);
    bar.appendChild(thumb);
    container.appendChild(bar);
    
    return container;
  }
  
  createButtonRow() {
    const row = document.createElement('div');
    row.className = 'button-row';
    
    // Add buttons
    row.appendChild(this.createButton('play', this.handlePlay.bind(this)));
    row.appendChild(this.createButton('skip-back', this.handleSkipBack.bind(this)));
    row.appendChild(this.createButton('skip-forward', this.handleSkipForward.bind(this)));
    row.appendChild(this.createVolumeControl());
    
    // Spacer
    const spacer = document.createElement('div');
    spacer.style.flex = '1';
    row.appendChild(spacer);
    
    row.appendChild(this.createButton('fullscreen', this.handleFullscreen.bind(this)));
    
    return row;
  }
  
  createButton(type, handler) {
    const button = document.createElement('button');
    button.className = `control-btn ${type}`;
    button.innerHTML = this.getButtonIcon(type);
    button.addEventListener('click', handler);
    return button;
  }
  
  getButtonIcon(type) {
    const icons = {
      play: '<svg viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>',
      pause: '<svg viewBox="0 0 24 24"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>',
      'skip-back': '<svg viewBox="0 0 24 24"><path d="M11 18V6l-8.5 6 8.5 6zm.5-6l8.5 6V6l-8.5 6z"/></svg>',
      'skip-forward': '<svg viewBox="0 0 24 24"><path d="M4 18l8.5-6L4 6v12zm9-12v12l8.5-6L13 6z"/></svg>',
      fullscreen: '<svg viewBox="0 0 24 24"><path d="M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z"/></svg>'
    };
    return icons[type] || '';
  }
  
  createVolumeControl() {
    const container = document.createElement('div');
    container.className = 'volume-control';
    
    const muteBtn = document.createElement('button');
    muteBtn.className = 'control-btn mute';
    muteBtn.innerHTML = this.getButtonIcon('mute');
    muteBtn.addEventListener('click', this.handleMute.bind(this));
    
    const slider = document.createElement('div');
    slider.className = 'volume-slider';
    
    const fill = document.createElement('div');
    fill.className = 'volume-fill';
    
    slider.appendChild(fill);
    container.appendChild(muteBtn);
    container.appendChild(slider);
    
    return container;
  }
  
  bindEvents() {
    // Progress bar click
    this.elements.progress.querySelector('.progress-bar').addEventListener('click', (e) => {
      const rect = e.currentTarget.getBoundingClientRect();
      const percent = (e.clientX - rect.left) / rect.width;
      this.player.seek(this.player.getDuration() * percent);
    });
    
    // Volume slider
    this.elements.volume.querySelector('.volume-slider').addEventListener('click', (e) => {
      const rect = e.currentTarget.getBoundingClientRect();
      const volume = (e.clientX - rect.left) / rect.width;
      this.player.setVolume(Math.max(0, Math.min(1, volume)));
    });
  }
  
  setupPlayerEvents() {
    this.player.on('player:play', () => this.updatePlayButton(true));
    this.player.on('player:pause', () => this.updatePlayButton(false));
    this.player.on('player:timeupdate', () => this.updateProgress());
    this.player.on('player:volumechange', () => this.updateVolume());
  }
  
  updatePlayButton(playing) {
    const btn = this.elements.buttons.querySelector('.play');
    btn.innerHTML = playing ? this.getButtonIcon('pause') : this.getButtonIcon('play');
  }
  
  updateProgress() {
    const current = this.player.getCurrentTime();
    const duration = this.player.getDuration();
    const buffered = this.player.getBuffered();
    
    const playPercent = (current / duration) * 100;
    const bufferPercent = (buffered / duration) * 100;
    
    this.elements.progress.querySelector('.play-progress').style.width = `${playPercent}%`;
    this.elements.progress.querySelector('.buffer-progress').style.width = `${bufferPercent}%`;
  }
  
  updateVolume() {
    const volume = this.player.getVolume();
    this.elements.volume.querySelector('.volume-fill').style.width = `${volume * 100}%`;
  }
  
  handlePlay() {
    const state = this.player.getState();
    state.playing ? this.player.pause() : this.player.play();
  }
  
  handleSkipBack() {
    this.player.seek(Math.max(0, this.player.getCurrentTime() - 10));
  }
  
  handleSkipForward() {
    this.player.seek(this.player.getCurrentTime() + 10);
  }
  
  handleMute() {
    this.player.setMuted(!this.player.getMuted());
  }
  
  handleFullscreen() {
    if (document.fullscreenElement) {
      document.exitFullscreen();
    } else {
      this.player.getContainer().requestFullscreen();
    }
  }
  
  show() {
    this.elements.container.classList.add('visible');
    this.resetHideTimer();
  }
  
  hide() {
    this.elements.container.classList.remove('visible');
  }
  
  resetHideTimer() {
    clearTimeout(this.hideTimer);
    if (this.options.showOnHover) {
      this.hideTimer = setTimeout(() => this.hide(), this.options.hideDelay);
    }
  }
  
  destroy() {
    this.elements.container.remove();
    clearTimeout(this.hideTimer);
  }
}

// Usage
const player = new VantisPlayer('#player', { controls: false });
const customControls = new CustomControls(player);
```

## Next Steps

- [Plugin Development](./plugin-development) - Create custom player plugins
- [Integration Examples](./integration) - Integrate with frameworks
- [API Reference](../api/overview) - Explore the full API