---
sidebar_position: 5
---

# Plugin System

The Vantis Media Player plugin system provides a powerful and flexible way to extend player functionality. This document describes the plugin architecture, lifecycle, and development patterns.

## Plugin System Overview

The plugin system follows a modular architecture that allows plugins to be loaded, configured, and managed at runtime. Plugins have access to the player API and can extend functionality in various ways.

```
┌─────────────────────────────────────────────┐
│                 Player Core                  │
└──────────────────────┬──────────────────────┘
                       │
┌──────────────────────▼──────────────────────┐
│              Plugin Manager                  │
├─────────────────────────────────────────────┤
│  - Plugin Registration                       │
│  - Lifecycle Management                      │
│  - API Exposure                              │
│  - Communication                             │
└──────────────────────┬──────────────────────┘
                       │
    ┌──────────────────┼──────────────────┐
    ▼                  ▼                  ▼
┌─────────┐      ┌─────────┐      ┌─────────┐
│ Plugin A│      │ Plugin B│      │ Plugin C│
└─────────┘      └─────────┘      └─────────┘
```

## Core Concepts

### Plugin Interface

Every plugin must implement the base Plugin interface:

```typescript
interface Plugin {
  // Metadata
  id: string;
  name: string;
  version: string;
  description?: string;
  author?: string;
  
  // Lifecycle
  onLoad?(player: Player): void | Promise<void>;
  onUnload?(): void | Promise<void>;
  
  // Optional
  config?: PluginConfig;
  api?: Record<string, any>;
}
```

### Plugin Metadata

Metadata provides information about the plugin:

```typescript
interface PluginMetadata {
  id: string;           // Unique identifier
  name: string;         // Human-readable name
  version: string;      // Semantic version
  description?: string; // Plugin description
  author?: string;      // Author name
  homepage?: string;    // URL to plugin homepage
  repository?: string;  // URL to source repository
  license?: string;     // License identifier
  keywords?: string[];  // Search keywords
}
```

### Plugin Configuration

Plugins can accept configuration through a schema:

```typescript
interface PluginConfig {
  schema: Record<string, ConfigField>;
  defaults?: Record<string, any>;
  validate?: (config: Record<string, any>) => boolean;
}

interface ConfigField {
  type: 'string' | 'number' | 'boolean' | 'object' | 'array';
  required?: boolean;
  default?: any;
  description?: string;
  enum?: any[];
  min?: number;
  max?: number;
  pattern?: string;
}
```

## Plugin Lifecycle

### Loading Sequence

```
Plugin Registration
       ↓
Plugin Instantiation
       ↓
Plugin.onLoad(player)
       ↓
Plugin Ready
       ↓
[Plugin Active]
       ↓
Plugin.onUnload()
       ↓
Plugin Destroyed
```

### Lifecycle Hooks

Plugins can hook into various player lifecycle events:

```typescript
class MyPlugin implements Plugin {
  id = 'my-plugin';
  name = 'My Plugin';
  version = '1.0.0';
  
  // Called when plugin is loaded
  onLoad(player: Player) {
    console.log('Plugin loaded');
    this.setupEventHandlers(player);
    this.createUI(player);
  }
  
  // Called when plugin is unloaded
  onUnload() {
    console.log('Plugin unloaded');
    this.cleanup();
  }
  
  // Called before plugin is loaded (validation)
  onBeforeLoad(player: Player): boolean {
    if (!this.checkRequirements(player)) {
      return false; // Abort loading
    }
    return true;
  }
  
  // Called after plugin is fully loaded
  onAfterLoad(player: Player) {
    console.log('Plugin is fully loaded');
  }
}
```

## Plugin Manager

The Plugin Manager handles plugin lifecycle and communication.

### Interface

```typescript
class PluginManager {
  private player: Player;
  private plugins: Map<string, Plugin>;
  private pluginAPI: PluginAPI;
  
  // Registration
  async register(plugin: Plugin): Promise<void>;
  async unregister(pluginId: string): Promise<void>;
  
  // Plugin access
  getPlugin(pluginId: string): Plugin | null;
  getPlugins(): Plugin[];
  hasPlugin(pluginId: string): boolean;
  
  // Lifecycle
  async loadPlugins(configs: PluginConfig[]): Promise<void>;
  async unloadAll(): Promise<void>;
  async reload(pluginId: string): Promise<void>;
  
  // Communication
  sendTo(pluginId: string, message: any): void;
  broadcast(message: any): void;
  
  // Configuration
  configure(pluginId: string, config: any): void;
  getConfiguration(pluginId: string): any;
}
```

### Implementation

```typescript
class PluginManagerImpl implements PluginManager {
  private player: Player;
  private plugins: Map<string, Plugin> = new Map();
  private pluginAPI: PluginAPI;
  
  constructor(player: Player) {
    this.player = player;
    this.pluginAPI = new PluginAPI(player);
  }
  
  async register(plugin: Plugin): Promise<void> {
    if (this.plugins.has(plugin.id)) {
      throw new Error(`Plugin ${plugin.id} is already registered`);
    }
    
    // Validate plugin
    if (!this.validatePlugin(plugin)) {
      throw new Error(`Invalid plugin: ${plugin.id}`);
    }
    
    // Check dependencies
    await this.checkDependencies(plugin);
    
    // Initialize plugin
    try {
      await plugin.onLoad?.(this.pluginAPI);
      this.plugins.set(plugin.id, plugin);
      
      this.player.emit('plugin:registered', {
        id: plugin.id,
        name: plugin.name,
        version: plugin.version
      });
    } catch (error) {
      console.error(`Failed to load plugin ${plugin.id}:`, error);
      throw error;
    }
  }
  
  async unregister(pluginId: string): Promise<void> {
    const plugin = this.plugins.get(pluginId);
    if (!plugin) {
      throw new Error(`Plugin ${pluginId} not found`);
    }
    
    // Unload plugin
    await plugin.onUnload?.();
    
    // Remove from registry
    this.plugins.delete(pluginId);
    
    this.player.emit('plugin:unregistered', { id: pluginId });
  }
  
  getPlugin(pluginId: string): Plugin | null {
    return this.plugins.get(pluginId) || null;
  }
  
  getPlugins(): Plugin[] {
    return Array.from(this.plugins.values());
  }
  
  hasPlugin(pluginId: string): boolean {
    return this.plugins.has(pluginId);
  }
  
  async loadPlugins(configs: PluginConfig[]): Promise<void> {
    for (const config of configs) {
      await this.register(config.plugin);
    }
  }
  
  async unloadAll(): Promise<void> {
    const pluginIds = Array.from(this.plugins.keys());
    for (const id of pluginIds) {
      await this.unregister(id);
    }
  }
  
  sendTo(pluginId: string, message: any): void {
    const plugin = this.plugins.get(pluginId);
    if (plugin?.onMessage) {
      plugin.onMessage(message);
    }
  }
  
  broadcast(message: any): void {
    this.plugins.forEach(plugin => {
      if (plugin.onMessage) {
        plugin.onMessage(message);
      }
    });
  }
  
  private validatePlugin(plugin: Plugin): boolean {
    // Check required properties
    if (!plugin.id || typeof plugin.id !== 'string') {
      return false;
    }
    if (!plugin.name || typeof plugin.name !== 'string') {
      return false;
    }
    if (!plugin.version || typeof plugin.version !== 'string') {
      return false;
    }
    return true;
  }
  
  private async checkDependencies(plugin: Plugin): Promise<void> {
    if (!plugin.dependencies) return;
    
    for (const dep of plugin.dependencies) {
      if (!this.hasPlugin(dep.id)) {
        throw new Error(`Missing dependency: ${dep.id}`);
      }
    }
  }
}
```

## Plugin API

The Plugin API exposes player functionality to plugins.

### Interface

```typescript
interface PluginAPI {
  // Player access
  getPlayer(): Player;
  
  // Media control
  play(): Promise<void>;
  pause(): Promise<void>;
  seek(time: number): Promise<void>;
  
  // State access
  getState(): PlayerState;
  getCurrentTime(): number;
  getDuration(): number;
  
  // Event handling
  on(event: string, callback: Function): void;
  off(event: string, callback: Function): void;
  emit(event: string, data?: any): void;
  
  // UI access
  getContainer(): HTMLElement;
  getUIManager(): UIManager;
  registerComponent(component: UIComponent): void;
  unregisterComponent(componentId: string): void;
  
  // Storage
  storage: StorageAPI;
  
  // Network
  network: NetworkAPI;
  
  // Plugin communication
  plugins: PluginCommunicationAPI;
}
```

### Implementation

```typescript
class PluginAPIImpl implements PluginAPI {
  private player: Player;
  private storageAPI: StorageAPI;
  private networkAPI: NetworkAPI;
  private pluginCommunicationAPI: PluginCommunicationAPI;
  
  constructor(player: Player) {
    this.player = player;
    this.storageAPI = new StorageAPI(player);
    this.networkAPI = new NetworkAPI(player);
    this.pluginCommunicationAPI = new PluginCommunicationAPI(player);
  }
  
  getPlayer(): Player {
    return this.player;
  }
  
  play(): Promise<void> {
    return this.player.play();
  }
  
  pause(): Promise<void> {
    return this.player.pause();
  }
  
  seek(time: number): Promise<void> {
    return this.player.seek(time);
  }
  
  getState(): PlayerState {
    return this.player.getState();
  }
  
  getCurrentTime(): number {
    return this.player.getCurrentTime();
  }
  
  getDuration(): number {
    return this.player.getDuration();
  }
  
  on(event: string, callback: Function): void {
    this.player.on(event, callback);
  }
  
  off(event: string, callback: Function): void {
    this.player.off(event, callback);
  }
  
  emit(event: string, data?: any): void {
    this.player.emit(event, data);
  }
  
  getContainer(): HTMLElement {
    return this.player.getContainer();
  }
  
  getUIManager(): UIManager {
    return this.player.getUIManager();
  }
  
  registerComponent(component: UIComponent): void {
    this.player.getUIManager().registerComponent(component);
  }
  
  unregisterComponent(componentId: string): void {
    this.player.getUIManager().unregisterComponent(componentId);
  }
  
  get storage(): StorageAPI {
    return this.storageAPI;
  }
  
  get network(): NetworkAPI {
    return this.networkAPI;
  }
  
  get plugins(): PluginCommunicationAPI {
    return this.pluginCommunicationAPI;
  }
}
```

## Plugin Types

Vantis Media Player supports multiple plugin types:

### JavaScript Plugins

Standard JavaScript/TypeScript plugins:

```typescript
class JavaScriptPlugin implements Plugin {
  id = 'js-plugin';
  name = 'JavaScript Plugin';
  version = '1.0.0';
  
  onLoad(player: Player) {
    console.log('JavaScript plugin loaded');
  }
}
```

### WASM Plugins

High-performance WebAssembly plugins:

```typescript
import init, { WasmModule } from './pkg/my_plugin.js';

class WasmPlugin implements Plugin {
  id = 'wasm-plugin';
  name = 'WASM Plugin';
  version = '1.0.0';
  
  private wasmModule: WasmModule | null = null;
  
  async onLoad(player: Player) {
    await init();
    this.wasmModule = new WasmModule();
    console.log('WASM plugin loaded');
  }
  
  processVideoFrame(frame: VideoFrame): VideoFrame {
    return this.wasmModule?.processFrame(frame) || frame;
  }
}
```

### Native Plugins

System-level native plugins:

```typescript
class NativePlugin implements Plugin {
  id = 'native-plugin';
  name = 'Native Plugin';
  version = '1.0.0';
  type = 'native';
  
  async onLoad(player: Player) {
    const nativeModule = await import('./native/my_plugin.node');
    console.log('Native plugin loaded');
  }
}
```

## Plugin Communication

Plugins can communicate with each other:

### Direct Communication

```typescript
// Plugin A sends message to Plugin B
player.plugins.sendTo('plugin-b', {
  type: 'greeting',
  message: 'Hello from Plugin A!'
});

// Plugin B listens for messages
class PluginB implements Plugin {
  id = 'plugin-b';
  name = 'Plugin B';
  version = '1.0.0';
  
  onMessage(message: any) {
    console.log('Received:', message);
  }
}
```

### Broadcast Communication

```typescript
// Broadcast to all plugins
player.plugins.broadcast({
  type: 'update',
  data: { some: 'data' }
});
```

### Shared State

```typescript
// Plugin A sets shared state
player.plugins.setSharedState('analytics', {
  trackingId: 'UA-XXXXX-Y'
});

// Plugin B gets shared state
const analyticsConfig = player.plugins.getSharedState('analytics');
```

## Plugin Dependencies

Plugins can declare dependencies on other plugins:

```typescript
interface PluginDependency {
  id: string;
  version?: string;      // Version constraint
  required?: boolean;    // Is dependency required?
}

class AnalyticsPlugin implements Plugin {
  id = 'analytics';
  name = 'Analytics';
  version = '1.0.0';
  
  dependencies: PluginDependency[] = [
    { id: 'logger', version: '^1.0.0', required: true },
    { id: 'tracking', version: '^2.0.0', required: false }
  ];
  
  onLoad(player: Player) {
    const logger = player.plugins.getPlugin('logger');
    logger?.log('Analytics plugin loaded');
  }
}
```

## Plugin Security

### Permission System

Plugins can request specific permissions:

```typescript
interface PluginPermissions {
  media?: boolean;      // Access to media
  storage?: boolean;    // Access to storage
  network?: boolean;    // Access to network
  ui?: boolean;         // Access to UI
  plugins?: boolean;    // Access to other plugins
}

class SecurePlugin implements Plugin {
  id = 'secure-plugin';
  name = 'Secure Plugin';
  version = '1.0.0';
  
  permissions: PluginPermissions = {
    media: true,
    storage: true,
    network: false
  };
}
```

### Sandboxing

Plugins can be sandboxed for security:

```typescript
class SandboxedPluginLoader {
  async load(plugin: Plugin): Promise<void> {
    // Create sandbox
    const sandbox = this.createSandbox(plugin.permissions);
    
    // Load plugin in sandbox
    await sandbox.execute(async () => {
      await plugin.onLoad(this.player);
    });
  }
  
  private createSandbox(permissions: PluginPermissions): PluginSandbox {
    return new PluginSandbox(permissions);
  }
}
```

## Plugin Distribution

### NPM Package

```json
{
  "name": "@vantis/plugin-my-plugin",
  "version": "1.0.0",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "files": ["dist", "README.md"],
  "peerDependencies": {
    "@vantis/player": "^3.0.0"
  }
}
```

### Plugin Registry

```bash
# Publish to Vantis Plugin Registry
vantis plugin publish my-plugin

# Install plugin
vantis plugin install @vantis/plugin-analytics
```

## Best Practices

1. **Keep plugins small**: Each plugin should do one thing well
2. **Document your plugin**: Provide clear documentation and examples
3. **Handle errors gracefully**: Don't crash the player
4. **Clean up resources**: Always clean up in `onUnload`
5. **Use semantic versioning**: Follow semver for version numbers
6. **Test your plugin**: Write comprehensive tests
7. **Consider performance**: Optimize for performance
8. **Provide configuration**: Allow customization through config

## Related Documentation

- [Overview](./overview) - High-level architecture
- [Components](./components) - Component details
- [Event System](./event-system) - Event system details
- [State Management](./state-management) - State management patterns
- [Plugin Development](../plugins/introduction) - Plugin development guide