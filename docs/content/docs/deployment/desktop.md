---
sidebar_position: 1
---

# Desktop Deployment

This guide covers deploying Vantis Media Player as a desktop application on Windows, macOS, and Linux platforms.

## Overview

Vantis Media Player can be deployed as a native desktop application using either Electron or Tauri frameworks. This provides users with a familiar native application experience with full system integration.

## Platform Support

| Platform | Architecture | Installer | Notes |
|----------|-------------|-----------|-------|
| Windows  | x64, ARM64  | NSIS, MSI, AppX | Windows 10+ |
| macOS    | x64, ARM64 (Apple Silicon) | DMG, PKG | macOS 10.15+ |
| Linux    | x64, ARM64  | DEB, RPM, AppImage, Snap | Ubuntu 18.04+, Fedora 30+ |

## Electron Deployment

### Project Setup

```bash
# Create Electron project
npm create electron-app@latest vantis-player-desktop

cd vantis-player-desktop

# Install Vantis Player
npm install @vantis/player
```

### Electron Configuration

```javascript
// electron.main.js

const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('path');

let mainWindow;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1280,
    height: 720,
    minWidth: 640,
    minHeight: 480,
    frame: false,
    titleBarStyle: 'hiddenInset',
    backgroundColor: '#1a1a2e',
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
      enableBlinkFeatures: 'HardwareMediaKeyHandling,MediaSession'
    }
  });
  
  // Load app
  if (process.env.NODE_ENV === 'development') {
    mainWindow.loadURL('http://localhost:3000');
    mainWindow.webContents.openDevTools();
  } else {
    mainWindow.loadFile(path.join(__dirname, 'dist', 'index.html'));
  }
  
  // Hardware acceleration
  app.commandLine.appendSwitch('enable-features', 'VaapiVideoDecoder');
  app.commandLine.appendSwitch('enable-gpu-rasterization');
  app.commandLine.appendSwitch('enable-zero-copy');
}

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});
```

### Preload Script

```javascript
// electron.preload.js

const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electronAPI', {
  // File operations
  openFile: () => ipcRenderer.invoke('dialog:openFile'),
  saveFile: (data) => ipcRenderer.invoke('dialog:saveFile', data),
  
  // System integration
  setThumbarButtons: (buttons) => ipcRenderer.send('window:setThumbarButtons', buttons),
  setProgressBar: (progress) => ipcRenderer.send('window:setProgressBar', progress),
  
  // Media keys
  onMediaKey: (callback) => ipcRenderer.on('media:key', (event, key) => callback(key)),
  
  // System info
  getSystemInfo: () => ipcRenderer.invoke('system:getInfo'),
  
  // Native menus
  showContextMenu: (menu) => ipcRenderer.invoke('menu:context', menu)
});
```

### IPC Handlers

```javascript
// electron.ipc.js

const { dialog, systemPreferences, nativeTheme } = require('electron');
const ipcMain = require('electron').ipcMain;

// File dialogs
ipcMain.handle('dialog:openFile', async () => {
  const result = await dialog.showOpenDialog({
    properties: ['openFile'],
    filters: [
      { name: 'Media Files', extensions: ['mp4', 'mkv', 'avi', 'mov', 'mp3', 'flac', 'wav'] },
      { name: 'All Files', extensions: ['*'] }
    ]
  });
  
  if (!result.canceled && result.filePaths.length > 0) {
    return result.filePaths[0];
  }
  return null;
});

ipcMain.handle('dialog:saveFile', async (event, data) => {
  const result = await dialog.showSaveDialog({
    defaultPath: data.defaultPath,
    filters: data.filters
  });
  
  return result.filePath;
});

// System info
ipcMain.handle('system:getInfo', () => {
  return {
    platform: process.platform,
    arch: process.arch,
    version: process.getSystemVersion(),
    hardwareConcurrency: require('os').cpus().length,
    totalMemory: require('os').totalmem(),
    darkMode: nativeTheme.shouldUseDarkColors
  };
});

// Progress bar
ipcMain.on('window:setProgressBar', (event, progress) => {
  const window = BrowserWindow.fromWebContents(event.sender);
  window.setProgressBar(progress);
});

// Thumbnail toolbar (Windows)
ipcMain.on('window:setThumbarButtons', (event, buttons) => {
  const window = BrowserWindow.fromWebContents(event.sender);
  window.setThumbarButtons(buttons);
});
```

### Build Configuration

```json
// package.json

{
  "name": "vantis-media-player",
  "version": "1.0.0",
  "main": "electron/main.js",
  "scripts": {
    "start": "electron .",
    "build": "electron-builder",
    "build:win": "electron-builder --win",
    "build:mac": "electron-builder --mac",
    "build:linux": "electron-builder --linux"
  },
  "build": {
    "appId": "media.vantis.player",
    "productName": "Vantis Media Player",
    "copyright": "Copyright © 2024 Vantis Media",
    
    "directories": {
      "output": "release",
      "buildResources": "build"
    },
    
    "files": [
      "dist/**/*",
      "electron/**/*",
      "package.json"
    ],
    
    "win": {
      "target": [
        { "target": "nsis", "arch": ["x64", "arm64"] },
        { "target": "portable", "arch": ["x64"] },
        { "target": "msi", "arch": ["x64"] }
      ],
      "icon": "build/icon.ico",
      "artifactName": "${productName}-${version}-${arch}.${ext}"
    },
    
    "nsis": {
      "oneClick": false,
      "perMachine": false,
      "allowToChangeInstallationDirectory": true,
      "installerIcon": "build/icon.ico",
      "uninstallerIcon": "build/icon.ico",
      "installerHeaderIcon": "build/icon.ico",
      "createDesktopShortcut": true,
      "createStartMenuShortcut": true,
      "shortcutName": "Vantis Media Player"
    },
    
    "mac": {
      "target": [
        { "target": "dmg", "arch": ["x64", "arm64"] },
        { "target": "zip", "arch": ["x64", "arm64"] }
      ],
      "icon": "build/icon.icns",
      "category": "public.app-category.video",
      "artifactName": "${productName}-${version}-${arch}.${ext}",
      "hardenedRuntime": true,
      "gatekeeperAssess": false,
      "entitlements": "build/entitlements.mac.plist",
      "entitlementsInherit": "build/entitlements.mac.plist"
    },
    
    "dmg": {
      "contents": [
        { "x": 130, "y": 220 },
        { "x": 410, "y": 220, "type": "link", "path": "/Applications" }
      ]
    },
    
    "linux": {
      "target": [
        { "target": "AppImage", "arch": ["x64", "arm64"] },
        { "target": "deb", "arch": ["x64", "arm64"] },
        { "target": "rpm", "arch": ["x64", "arm64"] },
        { "target": "snap", "arch": ["x64"] }
      ],
      "icon": "build/icons",
      "category": "AudioVideo;Video;Player;",
      "maintainer": "Vantis Media <support@vantis.media>",
      "artifactName": "${productName}-${version}-${arch}.${ext}",
      "desktop": {
        "Name": "Vantis Media Player",
        "Comment": "Modern media player for Windows, macOS, and Linux",
        "Categories": "AudioVideo;Video;Player;",
        "Keywords": "media;player;video;audio;streaming;",
        "StartupWMClass": "vantis-media-player"
      }
    },
    
    "deb": {
      "depends": ["libgtk-3-0", "libnotify4", "libnss3", "libxss1", "libxtst6", "xdg-utils"]
    },
    
    "snap": {
      "plugs": ["audio-playback", "video", "removable-media", "desktop", "desktop-legacy"]
    },
    
    "publish": {
      "provider": "github",
      "owner": "vantisCorp",
      "repo": "Vantis-Media-Player"
    }
  }
}
```

## Tauri Deployment

### Project Setup

```bash
# Create Tauri project
npm create tauri-app@latest vantis-player-tauri

cd vantis-player-tauri

# Install dependencies
npm install @vantis/player
```

### Tauri Configuration

```json
// tauri.conf.json

{
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devPath": "http://localhost:3000",
    "distDir": "../dist"
  },
  "package": {
    "productName": "Vantis Media Player",
    "version": "1.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": true,
        "scope": ["$HOME/**", "$DOCUMENT/**", "$DOWNLOAD/**"]
      },
      "dialog": {
        "all": true,
        "open": true,
        "save": true
      },
      "shell": {
        "open": true
      },
      "window": {
        "all": true
      },
      "http": {
        "all": true,
        "request": true
      },
      "path": {
        "all": true
      },
      "os": {
        "all": true
      },
      "globalShortcut": {
        "all": true
      }
    },
    "bundle": {
      "active": true,
      "category": "Video",
      "copyright": "Copyright © 2024 Vantis Media",
      "deb": {
        "depends": ["libgtk-3-0", "libwebkit2gtk-4.0-37"]
      },
      "externalBin": [],
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "identifier": "media.vantis.player",
      "longDescription": "Vantis Media Player is a modern, high-performance media player supporting a wide range of video and audio formats.",
      "macOS": {
        "entitlements": null,
        "exceptionDomain": "",
        "frameworks": [],
        "providerShortName": null,
        "signingIdentity": null
      },
      "resources": [],
      "shortDescription": "Modern media player",
      "targets": "all",
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": "",
        "webviewInstallMode": {
          "type": "embedBootstrapper"
        }
      }
    },
    "security": {
      "csp": "default-src 'self'; media-src 'self' *; img-src 'self' * data:; style-src 'self' 'unsafe-inline'"
    },
    "updater": {
      "active": true,
      "dialog": true,
      "endpoints": [
        "https://update.vantis.media/{{target}}/{{arch}}/{{current_version}}"
      ],
      "pubkey": "..."
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 720,
        "minHeight": 480,
        "minWidth": 640,
        "resizable": true,
        "title": "Vantis Media Player",
        "width": 1280,
        "center": true,
        "decorations": true,
        "transparent": false
      }
    ]
  }
}
```

### Rust Backend

```rust
// src-tauri/src/main.rs

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
async fn open_file(window: tauri::Window) -> Option<String> {
    let file_path = window.dialog()
        .file()
        .add_filter("Media Files", &["mp4", "mkv", "avi", "mov", "mp3", "flac", "wav"])
        .blocking_pick_file();
    
    file_path.map(|p| p.to_string())
}

#[tauri::command]
async fn get_system_info() -> SystemInfo {
    SystemInfo {
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        version: os_info::get().version().to_string(),
        cpu_cores: num_cpus::get(),
        total_memory: sys_info::mem_info().map(|m| m.total).unwrap_or(0),
    }
}

#[derive(serde::Serialize)]
struct SystemInfo {
    platform: String,
    arch: String,
    version: String,
    cpu_cores: usize,
    total_memory: u64,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::init())
        .invoke_handler(tauri::generate_handler![open_file, get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Code Signing

### Windows

```powershell
# Sign with SignTool
signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com release/Vantis Media Player-1.0.0-x64.exe
```

### macOS

```bash
# Sign with codesign
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name (XXXXXXXXXX)" "release/Vantis Media Player.app"

# Notarize
xcrun notarytool submit "release/Vantis Media Player-1.0.0-x64.dmg" --apple-id "your@email.com" --password "app-specific-password" --team-id "XXXXXXXXXX" --wait
```

## Auto-Update

### Implementation

```javascript
// Auto-update with electron-updater

const { autoUpdater } = require('electron-updater');
const { dialog } = require('electron');

autoUpdater.checkForUpdatesAndNotify();

autoUpdater.on('update-downloaded', async () => {
  const result = await dialog.showMessageBox({
    type: 'info',
    title: 'Update Available',
    message: 'A new version of Vantis Media Player is available. Would you like to install it now?',
    buttons: ['Install Now', 'Later']
  });
  
  if (result.response === 0) {
    autoUpdater.quitAndInstall();
  }
});
```

## Best Practices

1. **Test on all platforms**: Test on Windows, macOS, and Linux
2. **Use CI/CD**: Automate builds with GitHub Actions
3. **Code signing**: Sign all builds for trust
4. **Auto-update**: Implement auto-update functionality
5. **Error reporting**: Include error reporting (Sentry, etc.)
6. **Hardware acceleration**: Enable GPU acceleration
7. **Resource optimization**: Optimize bundle size
8. **Documentation**: Include user documentation

## Related Documentation

- [Web Deployment](./web) - Deploy as web application
- [Mobile Deployment](./mobile) - Deploy to mobile platforms
- [Docker Deployment](./docker) - Deploy with Docker
- [Cloud Deployment](./cloud) - Deploy to cloud infrastructure