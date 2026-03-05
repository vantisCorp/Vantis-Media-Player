---
sidebar_position: 3
title: Mobile Deployment
sidebar_label: Mobile
---

# Mobile Deployment

Deploy Vantis Media Player to iOS and Android platforms with native performance and cross-platform compatibility.

## Overview

Vantis Media Player supports mobile deployment through multiple frameworks:

- **React Native** - Native mobile apps with JavaScript
- **Capacitor** - Web-based apps with native capabilities
- **Flutter** - Cross-platform with native rendering
- **Kotlin Multiplatform** - Shared code across mobile platforms

## React Native Deployment

### Setup

Install React Native dependencies:

```bash
npm install react-native-vantis-player
# or
yarn add react-native-vantis-player
```

### iOS Implementation

```typescript
// App.tsx
import React, { useRef } from 'react';
import { View, StyleSheet } from 'react-native';
import VantisPlayer from 'react-native-vantis-player';

export default function App() {
  const playerRef = useRef<VantisPlayer>(null);

  return (
    <View style={styles.container}>
      <VantisPlayer
        ref={playerRef}
        source={{ uri: 'https://example.com/video.mp4' }}
        style={styles.player}
        autoplay
        controls
        onReady={() => console.log('Player ready')}
        onError={(error) => console.error(error)}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#000',
  },
  player: {
    flex: 1,
  },
});
```

### Android Implementation

Configure `android/app/build.gradle`:

```gradle
android {
    compileSdkVersion 33

    defaultConfig {
        minSdkVersion 21
        targetSdkVersion 33
    }

    compileOptions {
        sourceCompatibility JavaVersion.VERSION_1_8
        targetCompatibility JavaVersion.VERSION_1_8
    }
}

dependencies {
    implementation 'com.vantis:player:1.0.0'
}
```

Add permissions to `AndroidManifest.xml`:

```xml
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
    
    <uses-feature android:name="android.hardware.camera.autofocus" />
    <uses-feature android:glEsVersion="0x00020000" android:required="true" />
</manifest>
```

## Capacitor Deployment

### Setup

```bash
npm install @capacitor/core @capacitor/cli
npm install @capacitor/android @capacitor/ios
npx cap init VantisPlayer com.vantis.player
npx cap add android
npx cap add ios
```

### Configuration

```typescript
// capacitor.config.ts
import { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'com.vantis.player',
  appName: 'Vantis Player',
  webDir: 'dist',
  bundledWebRuntime: false,
  server: {
    androidScheme: 'https',
    iosScheme: 'https',
    cleartext: true,
  },
  plugins: {
    SplashScreen: {
      launchShowDuration: 2000,
      launchAutoHide: true,
    },
  },
};

export default config;
```

### Build Commands

```bash
# Build web assets
npm run build

# Sync to native platforms
npx cap sync

# Build for iOS
npx cap open ios
# In Xcode: Product > Archive

# Build for Android
npx cap open android
# In Android Studio: Build > Generate Signed Bundle / APK
```

## Flutter Deployment

### Setup

Add to `pubspec.yaml`:

```yaml
dependencies:
  vantis_player: ^1.0.0
```

### iOS Implementation

```dart
import 'package:flutter/material.dart';
import 'package:vantis_player/vantis_player.dart';

class VideoPlayerScreen extends StatefulWidget {
  @override
  _VideoPlayerScreenState createState() => _VideoPlayerScreenState();
}

class _VideoPlayerScreenState extends State<VideoPlayerScreen> {
  VantisPlayerController? _controller;

  @override
  void initState() {
    super.initState();
    _controller = VantisPlayerController.network(
      'https://example.com/video.mp4',
      autoPlay: true,
    );
    _controller!.initialize();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: _controller != null && _controller!.value.isInitialized
            ? VantisPlayer(controller: _controller!)
            : CircularProgressIndicator(),
      ),
    );
  }

  @override
  void dispose() {
    _controller?.dispose();
    super.dispose();
  }
}
```

### Android Implementation

Configure `android/app/build.gradle`:

```gradle
android {
    compileSdkVersion 33

    defaultConfig {
        minSdkVersion 21
        targetSdkVersion 33
    }
}

dependencies {
    implementation 'com.vantis:flutter-player:1.0.0'
}
```

## Native iOS Deployment

### Swift Implementation

```swift
// ViewController.swift
import UIKit
import VantisPlayerSDK

class ViewController: UIViewController {
    var player: VantisPlayer!
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        player = VantisPlayer(frame: view.bounds)
        player.autoPlay = true
        player.source = URL(string: "https://example.com/video.mp4")
        
        player.onReady = { [weak self] in
            print("Player ready")
        }
        
        player.onError = { error in
            print("Error: \(error)")
        }
        
        view.addSubview(player)
    }
    
    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        player.frame = view.bounds
    }
}
```

### Info.plist Configuration

```xml
<key>NSAppTransportSecurity</key>
<dict>
    <key>NSAllowsArbitraryLoads</key>
    <true/>
    <key>NSAllowsLocalNetworking</key>
    <true/>
</dict>
<key>UIRequiresFullScreen</key>
<false/>
<key>UIStatusBarHidden</key>
<false/>
```

## Native Android Deployment

### Kotlin Implementation

```kotlin
// MainActivity.kt
package com.vantis.player

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.vantis.player.VantisPlayer

class MainActivity : AppCompatActivity() {
    private lateinit var player: VantisPlayer
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        
        player = VantisPlayer(this)
        player.setSource("https://example.com/video.mp4")
        player.setAutoplay(true)
        
        player.setOnReadyListener {
            println("Player ready")
        }
        
        player.setOnErrorListener { error ->
            println("Error: $error")
        }
        
        val container = findViewById<FrameLayout>(R.id.player_container)
        container.addView(player)
    }
    
    override fun onPause() {
        super.onPause()
        player.pause()
    }
    
    override fun onResume() {
        super.onResume()
        player.resume()
    }
    
    override fun onDestroy() {
        super.onDestroy()
        player.release()
    }
}
```

### Permissions Configuration

```xml
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.WAKE_LOCK" />
    
    <uses-feature
        android:glEsVersion="0x00020000"
        android:required="true" />
</manifest>
```

## Performance Optimization

### iOS Optimization

Enable hardware acceleration:

```swift
// In your view controller
player.allowsVideoPlaybackAccelerated = true
player.allowsVideoPlaybackGPUAcceleration = true
player.preferredForwardBufferDuration = 30.0
```

### Android Optimization

Configure for hardware decoding:

```kotlin
player.setVideoDecoderType(VantisPlayer.DecoderType.HARDWARE)
player.setBufferConfig(
    minBufferMs = 15000,
    maxBufferMs = 50000,
    bufferForPlaybackMs = 2500,
    bufferForPlaybackAfterRebufferMs = 5000
)
```

### React Native Optimization

```typescript
<VantisPlayer
  resizeMode="contain"
  progressUpdateInterval={250}
  minLoadRetryCount={3}
  maxBitRate={5000000}
  preferredForwardBufferDuration={30}
  playInBackground={false}
  preventsDisplaySleepDuringVideoPlayback={true}
/>
```

## Build and Distribution

### iOS App Store

```bash
# Archive app
npx cap open ios
# In Xcode: Product > Archive > Distribute App

# Command line
xcodebuild -workspace App.xcworkspace \
  -scheme App \
  -configuration Release \
  -archivePath build/App.xcarchive \
  archive
```

### Android Play Store

```bash
# Generate APK
cd android
./gradlew assembleRelease

# Generate App Bundle
./gradlew bundleRelease

# Sign APK
jarsigner -verbose -sigalg SHA1withRSA -digestalg SHA1 \
  -keystore my-release-key.keystore \
  app-release-unsigned.apk alias_name
```

## Troubleshooting

### Common iOS Issues

**Video not playing:**
- Verify Info.plist has proper ATS settings
- Check network permissions
- Ensure video format is supported (H.264, H.265)

**App crashes on launch:**
- Verify all frameworks are properly linked
- Check deployment target compatibility
- Review crash logs in Xcode

### Common Android Issues

**Video not loading:**
- Check Internet permissions in manifest
- Verify network security configuration
- Ensure minSdkVersion is 21+

**Playback stutters:**
- Enable hardware decoding
- Adjust buffer settings
- Check available memory

## Best Practices

1. **Handle orientation changes** properly in both platforms
2. **Implement background playback** only for music apps
3. **Use adaptive streaming** for better network performance
4. **Optimize for battery life** with proper resource management
5. **Test on multiple devices** with different screen sizes
6. **Follow platform guidelines** for UI/UX consistency
7. **Implement crash reporting** for production apps
8. **Use analytics** to track performance metrics

## Next Steps

- [ ] Configure build pipelines for automated releases
- [ ] Implement proper error handling
- [ ] Add analytics and crash reporting
- [ ] Set up testing infrastructure
- [ ] Create app store screenshots and descriptions