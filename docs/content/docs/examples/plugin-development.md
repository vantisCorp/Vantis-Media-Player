---
sidebar_position: 4
---

# Plugin Development Example

This example walks through creating a complete, production-ready plugin for Vantis Media Player. We'll build a "Video Markers" plugin that allows users to bookmark specific timestamps in a video.

## Plugin Overview

The Video Markers plugin will:
- Add markers to specific timestamps
- Display markers on the progress bar
- Allow jumping to markers
- Persist markers in local storage
- Export/import markers

## Complete Plugin Implementation

```javascript
// VideoMarkersPlugin.js

/**
 * Video Markers Plugin for Vantis Media Player
 * 
 * Features:
 * - Add/remove markers at specific timestamps
 * - Visual markers on progress bar
 * - Jump to markers
 * - Local storage persistence
 * - Export/import functionality
 */

export default class VideoMarkersPlugin {
  constructor(config = {}) {
    // Plugin metadata
    this.id = 'video-markers';
    this.name = 'Video Markers';
    this.version = '1.0.0';
    this.author = 'Vantis Media';
    this.description = 'Add and manage video bookmarks';
    
    // Configuration
    this.config = {
      storage: config.storage !== false,
      storageKey: config.storageKey || 'vantis-markers',
      maxMarkers: config.maxMarkers || 100,
      markerColor: config.markerColor || '#ff4444',
      onMarkerClick: config.onMarkerClick,
      ...config
    };
    
    // Internal state
    this.player = null;
    this.markers = [];
    this.currentMediaId = null;
    this.ui = {
      container: null,
      markersOverlay: null,
      panel: null,
      addButton: null
    };
  }
  
  // ============================================
  // Lifecycle Methods
  // ============================================
  
  async onLoad(player) {
    this.player = player;
    
    // Load saved markers
    if (this.config.storage) {
      this.loadFromStorage();
    }
    
    // Create UI
    this.createUI();
    
    // Bind events
    this.bindEvents();
    
    // Register with player
    player.emit('plugin:loaded', {
      id: this.id,
      name: this.name
    });
    
    console.log('[VideoMarkers] Plugin loaded');
  }
  
  onUnload() {
    // Save markers before unloading
    if (this.config.storage) {
      this.saveToStorage();
    }
    
    // Clean up UI
    this.destroyUI();
    
    // Clean up events
    this.unbindEvents();
    
    this.player = null;
    console.log('[VideoMarkers] Plugin unloaded');
  }
  
  // ============================================
  // UI Creation
  // ============================================
  
  createUI() {
    const container = this.player.getContainer();
    
    // Create markers overlay on progress bar
    this.ui.markersOverlay = document.createElement('div');
    this.ui.markersOverlay.className = 'vantis-markers-overlay';
    this.ui.markersOverlay.style.cssText = `
      position: absolute;
      bottom: 45px;
      left: 0;
      right: 0;
      height: 6px;
      pointer-events: none;
    `;
    
    // Create control bar button
    this.ui.addButton = document.createElement('button');
    this.ui.addButton.className = 'vantis-control-btn marker-btn';
    this.ui.addButton.innerHTML = `
      <svg viewBox="0 0 24 24" width="24" height="24">
        <path fill="currentColor" d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-13h2v6h-2zm0 8h2v2h-2z"/>
      </svg>
    `;
    this.ui.addButton.title = 'Add Marker';
    this.ui.addButton.style.cssText = `
      background: none;
      border: none;
      color: white;
      cursor: pointer;
      padding: 8px;
      border-radius: 4px;
      transition: background 0.2s;
    `;
    
    // Create markers panel
    this.ui.panel = document.createElement('div');
    this.ui.panel.className = 'vantis-markers-panel';
    this.ui.panel.style.cssText = `
      position: absolute;
      bottom: 60px;
      right: 10px;
      width: 280px;
      max-height: 300px;
      background: rgba(0, 0, 0, 0.9);
      border-radius: 8px;
      padding: 15px;
      display: none;
      overflow-y: auto;
      z-index: 100;
    `;
    
    this.ui.panel.innerHTML = `
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
        <h3 style="margin: 0; font-size: 14px; color: #fff;">Markers</h3>
        <div>
          <button class="export-btn" title="Export" style="background: none; border: none; color: #888; cursor: pointer; padding: 4px;">
            <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/></svg>
          </button>
          <button class="import-btn" title="Import" style="background: none; border: none; color: #888; cursor: pointer; padding: 4px;">
            <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M9 16h6v-6h4l-7-7-7 7h4zm-4 2h14v2H5z"/></svg>
          </button>
          <button class="clear-btn" title="Clear All" style="background: none; border: none; color: #888; cursor: pointer; padding: 4px;">
            <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
          </button>
        </div>
      </div>
      <div class="markers-list"></div>
      <div class="no-markers" style="text-align: center; color: #888; padding: 20px;">
        No markers yet. Click the marker button or press 'M' to add one.
      </div>
    `;
    
    // Add to container
    container.appendChild(this.ui.markersOverlay);
    container.appendChild(this.ui.addButton);
    container.appendChild(this.ui.panel);
    
    // Add styles
    this.injectStyles();
  }
  
  injectStyles() {
    if (document.getElementById('vantis-markers-styles')) return;
    
    const styles = document.createElement('style');
    styles.id = 'vantis-markers-styles';
    styles.textContent = `
      .vantis-control-btn:hover {
        background: rgba(255, 255, 255, 0.1);
      }
      
      .vantis-marker {
        position: absolute;
        width: 8px;
        height: 8px;
        background: ${this.config.markerColor};
        border-radius: 50%;
        transform: translateX(-50%);
        cursor: pointer;
        pointer-events: auto;
        transition: transform 0.2s;
      }
      
      .vantis-marker:hover {
        transform: translateX(-50%) scale(1.5);
      }
      
      .vantis-marker-item {
        display: flex;
        align-items: center;
        padding: 8px;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 4px;
        margin-bottom: 8px;
        cursor: pointer;
        transition: background 0.2s;
      }
      
      .vantis-marker-item:hover {
        background: rgba(255, 255, 255, 0.2);
      }
      
      .vantis-marker-item .time {
        color: #6366f1;
        font-weight: bold;
        margin-right: 10px;
        min-width: 50px;
      }
      
      .vantis-marker-item .title {
        flex: 1;
        color: #fff;
        font-size: 13px;
      }
      
      .vantis-marker-item .delete-btn {
        background: none;
        border: none;
        color: #888;
        cursor: pointer;
        padding: 4px;
        opacity: 0;
        transition: opacity 0.2s;
      }
      
      .vantis-marker-item:hover .delete-btn {
        opacity: 1;
      }
      
      .vantis-marker-item .delete-btn:hover {
        color: #ff4444;
      }
    `;
    
    document.head.appendChild(styles);
  }
  
  destroyUI() {
    if (this.ui.markersOverlay) this.ui.markersOverlay.remove();
    if (this.ui.addButton) this.ui.addButton.remove();
    if (this.ui.panel) this.ui.panel.remove();
  }
  
  // ============================================
  // Event Binding
  // ============================================
  
  bindEvents() {
    // Click on add button
    this.ui.addButton.addEventListener('click', () => this.togglePanel());
    
    // Panel button events
    this.ui.panel.querySelector('.export-btn').addEventListener('click', () => this.exportMarkers());
    this.ui.panel.querySelector('.import-btn').addEventListener('click', () => this.importMarkers());
    this.ui.panel.querySelector('.clear-btn').addEventListener('click', () => this.clearMarkers());
    
    // Keyboard shortcut
    this.keyHandler = (e) => {
      if (e.key === 'm' || e.key === 'M') {
        if (!e.ctrlKey && !e.metaKey && !e.altKey) {
          e.preventDefault();
          this.addMarkerAtCurrentTime();
        }
      }
    };
    document.addEventListener('keydown', this.keyHandler);
    
    // Media change
    this.mediaChangeHandler = (source) => {
      this.currentMediaId = source?.id;
      this.loadMarkersForMedia();
    };
    this.player.on('player:load', this.mediaChangeHandler);
  }
  
  unbindEvents() {
    document.removeEventListener('keydown', this.keyHandler);
    this.player.off('player:load', this.mediaChangeHandler);
  }
  
  // ============================================
  // Marker Management
  // ============================================
  
  addMarker(time, title = '') {
    if (this.markers.length >= this.config.maxMarkers) {
      console.warn('[VideoMarkers] Maximum markers reached');
      return null;
    }
    
    const duration = this.player.getDuration();
    if (time < 0 || time > duration) {
      console.warn('[VideoMarkers] Invalid marker time');
      return null;
    }
    
    const marker = {
      id: this.generateId(),
      time,
      title: title || `Marker at ${this.formatTime(time)}`,
      createdAt: Date.now()
    };
    
    this.markers.push(marker);
    this.markers.sort((a, b) => a.time - b.time);
    
    this.renderMarkers();
    this.saveToStorage();
    
    this.player.emit('marker:added', marker);
    
    return marker;
  }
  
  removeMarker(markerId) {
    const index = this.markers.findIndex(m => m.id === markerId);
    if (index === -1) return false;
    
    const marker = this.markers[index];
    this.markers.splice(index, 1);
    
    this.renderMarkers();
    this.saveToStorage();
    
    this.player.emit('marker:removed', marker);
    
    return true;
  }
  
  addMarkerAtCurrentTime() {
    const time = this.player.getCurrentTime();
    const marker = this.addMarker(time);
    
    if (marker) {
      // Show notification
      this.showNotification(`Marker added at ${this.formatTime(time)}`);
    }
    
    return marker;
  }
  
  jumpToMarker(markerId) {
    const marker = this.markers.find(m => m.id === markerId);
    if (marker) {
      this.player.seek(marker.time);
      this.player.emit('marker:jumped', marker);
    }
  }
  
  clearMarkers() {
    this.markers = [];
    this.renderMarkers();
    this.saveToStorage();
    this.player.emit('marker:cleared');
  }
  
  getMarkers() {
    return [...this.markers];
  }
  
  // ============================================
  // Rendering
  // ============================================
  
  renderMarkers() {
    const duration = this.player.getDuration();
    
    // Render overlay markers
    this.ui.markersOverlay.innerHTML = '';
    
    this.markers.forEach(marker => {
      const percent = (marker.time / duration) * 100;
      
      const markerEl = document.createElement('div');
      markerEl.className = 'vantis-marker';
      markerEl.style.left = `${percent}%`;
      markerEl.title = marker.title;
      markerEl.addEventListener('click', (e) => {
        e.stopPropagation();
        this.jumpToMarker(marker.id);
      });
      
      this.ui.markersOverlay.appendChild(markerEl);
    });
    
    // Render panel list
    const list = this.ui.panel.querySelector('.markers-list');
    const noMarkers = this.ui.panel.querySelector('.no-markers');
    
    if (this.markers.length === 0) {
      list.innerHTML = '';
      noMarkers.style.display = 'block';
    } else {
      noMarkers.style.display = 'none';
      
      list.innerHTML = this.markers.map(marker => `
        <div class="vantis-marker-item" data-id="${marker.id}">
          <span class="time">${this.formatTime(marker.time)}</span>
          <span class="title">${marker.title}</span>
          <button class="delete-btn" data-id="${marker.id}">
            <svg viewBox="0 0 24 24" width="16" height="16">
              <path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
          </button>
        </div>
      `).join('');
      
      // Bind click events
      list.querySelectorAll('.vantis-marker-item').forEach(item => {
        item.addEventListener('click', (e) => {
          if (!e.target.closest('.delete-btn')) {
            this.jumpToMarker(item.dataset.id);
          }
        });
      });
      
      list.querySelectorAll('.delete-btn').forEach(btn => {
        btn.addEventListener('click', () => {
          this.removeMarker(btn.dataset.id);
        });
      });
    }
  }
  
  // ============================================
  // Panel Control
  // ============================================
  
  togglePanel() {
    const isVisible = this.ui.panel.style.display === 'block';
    this.ui.panel.style.display = isVisible ? 'none' : 'block';
  }
  
  // ============================================
  // Storage
  // ============================================
  
  saveToStorage() {
    if (!this.config.storage) return;
    
    const data = {
      version: this.version,
      mediaMarkers: {}
    };
    
    // Load existing data
    const existing = localStorage.getItem(this.config.storageKey);
    if (existing) {
      try {
        const parsed = JSON.parse(existing);
        data.mediaMarkers = parsed.mediaMarkers || {};
      } catch (e) {
        // Ignore parse errors
      }
    }
    
    // Update current media markers
    if (this.currentMediaId) {
      data.mediaMarkers[this.currentMediaId] = this.markers;
    }
    
    localStorage.setItem(this.config.storageKey, JSON.stringify(data));
  }
  
  loadFromStorage() {
    if (!this.config.storage) return;
    
    const data = localStorage.getItem(this.config.storageKey);
    if (data) {
      try {
        const parsed = JSON.parse(data);
        this.savedMarkers = parsed.mediaMarkers || {};
      } catch (e) {
        this.savedMarkers = {};
      }
    }
  }
  
  loadMarkersForMedia() {
    if (this.currentMediaId && this.savedMarkers) {
      this.markers = this.savedMarkers[this.currentMediaId] || [];
      this.renderMarkers();
    }
  }
  
  // ============================================
  // Import/Export
  // ============================================
  
  exportMarkers() {
    const data = {
      version: this.version,
      mediaId: this.currentMediaId,
      markers: this.markers,
      exportedAt: Date.now()
    };
    
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    
    const a = document.createElement('a');
    a.href = url;
    a.download = `markers-${this.currentMediaId || 'export'}.json`;
    a.click();
    
    URL.revokeObjectURL(url);
  }
  
  importMarkers() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    
    input.addEventListener('change', async (e) => {
      const file = e.target.files[0];
      if (!file) return;
      
      try {
        const text = await file.text();
        const data = JSON.parse(text);
        
        if (data.markers && Array.isArray(data.markers)) {
          this.markers = data.markers;
          this.renderMarkers();
          this.saveToStorage();
          
          this.showNotification(`Imported ${data.markers.length} markers`);
        }
      } catch (error) {
        console.error('[VideoMarkers] Import failed:', error);
        this.showNotification('Import failed: Invalid file format', 'error');
      }
    });
    
    input.click();
  }
  
  // ============================================
  // Utilities
  // ============================================
  
  generateId() {
    return 'marker-' + Date.now() + '-' + Math.random().toString(36).substr(2, 9);
  }
  
  formatTime(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  }
  
  showNotification(message, type = 'success') {
    const notification = document.createElement('div');
    notification.className = 'vantis-notification';
    notification.style.cssText = `
      position: absolute;
      bottom: 70px;
      left: 50%;
      transform: translateX(-50%);
      background: ${type === 'error' ? '#ff4444' : '#6366f1'};
      color: white;
      padding: 10px 20px;
      border-radius: 4px;
      font-size: 14px;
      z-index: 1000;
      animation: fadeInOut 2s ease;
    `;
    notification.textContent = message;
    
    this.player.getContainer().appendChild(notification);
    
    setTimeout(() => notification.remove(), 2000);
  }
  
  // ============================================
  // Public API
  // ============================================
  
  configure(config) {
    this.config = { ...this.config, ...config };
    this.renderMarkers();
  }
}
```

## Plugin Usage

```javascript
import VideoMarkersPlugin from './VideoMarkersPlugin.js';

const player = new VantisPlayer('#player', {
  plugins: [
    {
      plugin: VideoMarkersPlugin,
      config: {
        markerColor: '#6366f1',
        storage: true,
        maxMarkers: 50
      }
    }
  ]
});

// Access plugin methods
const markersPlugin = player.plugins.get('video-markers');

// Add marker programmatically
markersPlugin.addMarker(120, 'Important scene');

// Get all markers
const markers = markersPlugin.getMarkers();

// Jump to marker
markersPlugin.jumpToMarker(markers[0].id);

// Clear all markers
markersPlugin.clearMarkers();
```

## Plugin Testing

```javascript
// VideoMarkersPlugin.test.js

import { describe, it, expect, beforeEach } from 'vitest';
import VideoMarkersPlugin from './VideoMarkersPlugin.js';
import { createMockPlayer } from './test-utils.js';

describe('VideoMarkersPlugin', () => {
  let plugin;
  let mockPlayer;
  
  beforeEach(() => {
    mockPlayer = createMockPlayer();
    plugin = new VideoMarkersPlugin();
  });
  
  describe('lifecycle', () => {
    it('should initialize correctly', async () => {
      await plugin.onLoad(mockPlayer);
      
      expect(plugin.id).toBe('video-markers');
      expect(plugin.name).toBe('Video Markers');
    });
    
    it('should clean up on unload', async () => {
      await plugin.onLoad(mockPlayer);
      plugin.onUnload();
      
      expect(plugin.player).toBeNull();
    });
  });
  
  describe('marker management', () => {
    beforeEach(async () => {
      await plugin.onLoad(mockPlayer);
    });
    
    it('should add marker at current time', () => {
      mockPlayer.getCurrentTime = () => 60;
      
      const marker = plugin.addMarkerAtCurrentTime();
      
      expect(marker).toBeDefined();
      expect(marker.time).toBe(60);
    });
    
    it('should not add marker outside video duration', () => {
      mockPlayer.getDuration = () => 100;
      
      const marker = plugin.addMarker(200, 'Invalid');
      
      expect(marker).toBeNull();
    });
    
    it('should remove marker', () => {
      const marker = plugin.addMarker(30, 'Test');
      
      const removed = plugin.removeMarker(marker.id);
      
      expect(removed).toBe(true);
      expect(plugin.getMarkers()).toHaveLength(0);
    });
    
    it('should jump to marker', () => {
      const marker = plugin.addMarker(45, 'Jump target');
      
      plugin.jumpToMarker(marker.id);
      
      expect(mockPlayer.seek).toHaveBeenCalledWith(45);
    });
  });
  
  describe('limits', () => {
    it('should enforce max markers limit', async () => {
      plugin = new VideoMarkersPlugin({ maxMarkers: 2 });
      await plugin.onLoad(mockPlayer);
      mockPlayer.getDuration = () => 100;
      
      plugin.addMarker(10);
      plugin.addMarker(20);
      const third = plugin.addMarker(30);
      
      expect(third).toBeNull();
      expect(plugin.getMarkers()).toHaveLength(2);
    });
  });
});
```

## Best Practices Summary

1. **Always clean up** in `onUnload`
2. **Use unique IDs** for all UI elements
3. **Persist data** when appropriate
4. **Handle errors** gracefully
5. **Provide feedback** to users
6. **Document your API**
7. **Write tests** for critical functionality
8. **Use semantic versioning** for plugin versions

## Next Steps

- [Integration Examples](./integration) - Integrate with frameworks
- [Plugin API Reference](../plugins/plugin-api) - Full API documentation
- [WASM Plugins](../plugins/wasm-plugins) - High-performance plugins