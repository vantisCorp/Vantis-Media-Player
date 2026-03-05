---
sidebar_position: 2
title: Building
sidebar_label: Building
---

# Building

Learn how to build Vantis Media Player for different platforms and environments.

## Overview

Vantis Media Player uses Turborepo for efficient, monorepo-aware builds. The build system supports multiple platforms including web, desktop, and mobile, with optimized production bundles.

## Build System

### Turborepo Configuration

The build system is configured in `turbo.json`:

```json
{
  "$schema": "https://turbo.build/schema.json",
  "globalDependencies": ["**/.env.*local"],
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": [".next/**", "!.next/cache/**", "dist/**"]
    },
    "dev": {
      "cache": false,
      "persistent": true
    },
    "lint": {
      "outputs": []
    },
    "test": {
      "dependsOn": ["build"],
      "outputs": ["coverage/**"]
    }
  }
}
```

### Build Commands

```bash
# Build all packages
pnpm build

# Build specific package
pnpm build --filter @vantis/player

# Build with caching disabled
pnpm build --force

# Build in parallel
pnpm build --parallel
```

## Web Build

### Development Build

```bash
# Build for development
pnpm build --filter @vantis/web-app

# Build with development mode
NODE_ENV=development pnpm build
```

### Production Build

```bash
# Build for production
pnpm build --filter @vantis/web-app

# Build with optimization
NODE_ENV=production pnpm build

# Build with source maps
pnpm build --filter @vantis/web-app -- --sourcemap
```

### Build Output

The web build outputs to:

```
apps/web/.next/
├── static/
│   ├── chunks/
│   ├── css/
│   └── media/
├── server/
└── BUILD_ID
```

### Environment-Specific Builds

```bash
# Staging build
STAGE=staging pnpm build

# Production build
STAGE=production pnpm build

# Custom environment
NODE_ENV=production \
API_URL=https://api.vantis.com \
pnpm build
```

## Desktop Build

### Electron Build

```bash
# Build Electron app for development
pnpm build:electron:dev

# Build for all platforms
pnpm build:electron:all

# Build for specific platform
pnpm build:electron:mac
pnpm build:electron:windows
pnpm build:electron:linux

# Build with code signing
pnpm build:electron:signed
```

### Electron Configuration

Configure Electron builder in `apps/desktop/electron-builder.json`:

```json
{
  "appId": "com.vantis.player",
  "productName": "Vantis Media Player",
  "directories": {
    "output": "dist/electron",
    "buildResources": "resources"
  },
  "files": [
    "dist/**/*",
    "package.json"
  ],
  "mac": {
    "category": "public.app-category.video",
    "hardenedRuntime": true,
    "gatekeeperAssess": false,
    "entitlements": "build/entitlements.mac.plist",
    "entitlementsInherit": "build/entitlements.mac.plist",
    "target": [
      {
        "target": "dmg",
        "arch": ["x64", "arm64"]
      }
    ],
    "icon": "build/icon.icns"
  },
  "win": {
    "target": [
      {
        "target": "nsis",
        "arch": ["x64", "ia32"]
      }
    ],
    "icon": "build/icon.ico"
  },
  "linux": {
    "target": [
      "AppImage",
      "deb",
      "rpm"
    ],
    "category": "Video",
    "icon": "build/icon.png"
  }
}
```

### Tauri Build

```bash
# Build Tauri app for development
pnpm build:tauri:dev

# Build for production
pnpm build:tauri:prod

# Build for specific platform
pnpm build:tauri:mac
pnpm build:tauri:windows
pnpm build:tauri:linux
```

### Tauri Configuration

Configure Tauri in `apps/desktop/src-tauri/tauri.conf.json`:

```json
{
  "build": {
    "distDir": "../dist",
    "devPath": "http://localhost:3000",
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build"
  },
  "tauri": {
    "bundle": {
      "identifier": "com.vantis.player",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "targets": ["dmg", "msi", "appimage", "deb"],
      "category": "Video Player",
      "shortDescription": "Modern media player",
      "longDescription": "Vantis Media Player - A modern, high-performance media player built with cutting-edge technology.",
      "macOS": {
        "minimumSystemVersion": "10.15",
        "entitlements": null,
        "exceptionDomain": "",
        "frameworks": [],
        "providerShortName": null,
        "signingIdentity": null
      }
    }
  }
}
```

## Mobile Build

### React Native Build

```bash
# iOS Build
cd apps/mobile/ios
pod install
# In Xcode: Product > Archive

# Android Build
cd apps/mobile/android
./gradlew assembleRelease
./gradlew bundleRelease
```

### React Native Configuration

Configure Android in `apps/mobile/android/app/build.gradle`:

```gradle
android {
    compileSdkVersion 33

    defaultConfig {
        applicationId "com.vantis.player"
        minSdkVersion 21
        targetSdkVersion 33
        versionCode 1
        versionName "1.0.0"

        ndk {
            abiFilters 'armeabi-v7a', 'arm64-v8a', 'x86', 'x86_64'
        }
    }

    buildTypes {
        release {
            minifyEnabled true
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
            signingConfig signingConfigs.release
        }
        debug {
            minifyEnabled false
            debuggable true
        }
    }

    splits {
        abi {
            enable true
            reset()
            include 'armeabi-v7a', 'arm64-v8a', 'x86', 'x86_64'
            universalApk false
        }
    }
}
```

Configure iOS in `apps/mobile/ios/Podfile`:

```ruby
platform :ios, '13.0'

target 'VantisPlayer' do
  config = use_native_modules!

  # Flags change depending on the env values.
  flags = get_default_flags()

  use_react_native!(
    :path => config[:reactNativePath],
    :hermes_enabled => true,
    :fabric_enabled => false,
    :app_path => "#{Pod::Config.instance.installation_root}/.."
  )

  target 'VantisPlayerTests' do
    inherit! :complete
  end

  post_install do |installer|
    react_native_post_install(
      installer,
      config[:reactNativePath],
      :mac_catalyst_enabled => false
    )
    __apply_Xcode_12_5_M1_post_install_workaround(installer)
  end
end
```

### Capacitor Build

```bash
# Sync native projects
pnpm cap sync

# iOS Build
pnpm cap open ios
# In Xcode: Product > Archive

# Android Build
pnpm cap open android
# In Android Studio: Build > Generate Signed Bundle / APK
```

### Flutter Build

```bash
# iOS Build
flutter build ios --release

# Android Build
flutter build apk --release
flutter build appbundle --release

# Web Build
flutter build web --release
```

## Build Optimization

### Code Splitting

Configure code splitting in `apps/web/next.config.js`:

```javascript
module.exports = {
  webpack: (config) => {
    config.optimization.splitChunks = {
      chunks: 'all',
      cacheGroups: {
        default: false,
        vendors: false,
        vendor: {
          name: 'vendor',
          chunks: 'all',
          test: /node_modules/,
          priority: 20
        },
        common: {
          name: 'common',
          minChunks: 2,
          chunks: 'all',
          priority: 10,
          reuseExistingChunk: true,
          enforce: true
        }
      }
    }
    return config
  }
}
```

### Tree Shaking

Configure tree shaking in `package.json`:

```json
{
  "sideEffects": [
    "*.css",
    "*.scss",
    "*.svg"
  ]
}
```

### Compression

Enable compression in Next.js:

```javascript
module.exports = {
  compress: true,
  swcMinify: true
}
```

## Build Caching

### Turborepo Cache

```bash
# Enable caching
pnpm build --cache-dir=./.turbo

# Clear cache
pnpm build --force

# Check cache status
pnpm build --dry-run
```

### Docker Build Cache

```dockerfile
# Optimize Docker layer caching
FROM node:20-alpine AS builder
WORKDIR /app

# Copy package files first (better cache utilization)
COPY package*.json pnpm-lock.yaml ./
COPY pnpm-workspace.yaml ./

# Install dependencies
RUN npm install -g pnpm && pnpm install --frozen-lockfile

# Copy source code
COPY . .

# Build application
RUN pnpm build
```

## Build Verification

### Check Build Output

```bash
# Check build artifacts
ls -la dist/
ls -la apps/web/.next/

# Verify build size
du -sh dist/
du -sh apps/web/.next/

# Check bundle size
pnpm analyze
```

### Bundle Analysis

```bash
# Analyze bundle size
pnpm build -- --analyze

# Generate bundle report
pnpm build -- --profile

# Visualize dependencies
pnpm build -- --stats
```

## Build Troubleshooting

### Common Issues

**Build fails with TypeScript errors:**
```bash
# Clean TypeScript build cache
rm -rf packages/*/tsconfig.tsbuildinfo

# Rebuild TypeScript
pnpm build -- --clean
```

**Build fails with memory errors:**
```bash
# Increase Node memory limit
NODE_OPTIONS="--max-old-space-size=8192" pnpm build
```

**Build fails with dependency errors:**
```bash
# Clean dependencies
rm -rf node_modules packages/*/node_modules
rm pnpm-lock.yaml

# Reinstall
pnpm install
```

## Build Best Practices

1. **Use build caching** to speed up subsequent builds
2. **Optimize bundle size** with code splitting and tree shaking
3. **Enable compression** for smaller production bundles
4. **Use environment variables** for different build configurations
5. **Test builds locally** before deploying
6. **Monitor build times** and optimize slow builds
7. **Use source maps** for debugging production issues
8. **Enable strict TypeScript** for better type safety
9. **Run linting** before building
10. **Test production builds** thoroughly

## Next Steps

- [ ] Configure build scripts for your workflow
- [ ] Set up build optimization
- [ ] Configure continuous integration builds
- [ ] Set up build monitoring
- [ ] Document custom build configurations