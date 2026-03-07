# Monorepo Migration Progress

This document tracks the migration to a Turborepo-based monorepo structure with Feature-Sliced Design (FSD).

## Migration Status

### Phase 1: Structure Setup ✅
- [x] Create `apps/` directory
- [x] Create `packages/` directory
- [x] Create FSD layer directories (`src/app`, `src/shared`, etc.)
- [x] Add `turbo.json` configuration
- [x] Add root `package.json` with workspaces
- [x] Update `Cargo.toml` with new package structure

### Phase 2: Package Creation 🔄
- [x] Create `packages/core/` with core primitives
  - [x] Error types
  - [x] Core types (MediaId, MediaInfo, MediaFormat)
  - [x] Event system
  - [x] Configuration management
  - [x] State management
- [ ] Create `packages/media/audio/`
- [ ] Create `packages/media/video/`
- [ ] Create `packages/media/subtitles/`
- [ ] Create `packages/media/streaming/`
- [ ] Create `packages/ui/`
- [ ] Create `packages/plugins/`
- [ ] Create `packages/hwaccel/`

### Phase 3: Documentation 📝
- [x] Create `apps/README.md`
- [x] Create `packages/README.md`
- [x] Create `src/README.md` (FSD overview)
- [x] Create layer READMEs:
  - [x] `src/app/README.md`
  - [x] `src/shared/README.md`
  - [x] `src/entities/README.md`
  - [x] `src/features/README.md`
  - [x] `src/widgets/README.md`
  - [x] `src/pages/README.md`
  - [x] `src/processes/README.md`

### Phase 4: Duplicate Analysis 📊
- [ ] Analyze `vantis-player/` directory
- [ ] Analyze `V-Streaming/` directory
- [ ] Compare with `Vantis-Media-Player/`
- [ ] Identify unique code to migrate
- [ ] Create migration plan for duplicates

### Phase 5: Legacy Consolidation 🔄
- [ ] Migrate `advanced_*` modules to proper packages
- [ ] Consolidate `core/` into `packages/core/`
- [ ] Consolidate `audio/` into `packages/media/audio/`
- [ ] Consolidate `video/` into `packages/media/video/`
- [ ] Consolidate `ui/` into `packages/ui/`
- [ ] Consolidate `plugins/` into `packages/plugins/`

### Phase 6: Testing 🧪
- [ ] Verify all packages build
- [ ] Run test suite
- [ ] Verify CI/CD pipeline
- [ ] Update documentation

## Directory Structure

```
Vantis-Media-Player/
├── apps/                    # Applications
│   ├── cli/                # CLI application
│   ├── desktop/            # Desktop app (Tauri/Electron)
│   ├── web/                # Web app (WASM)
│   └── mobile/             # Mobile app (React Native)
├── packages/               # Shared packages
│   ├── core/               # Core primitives
│   ├── media/              # Media processing
│   │   ├── audio/
│   │   ├── video/
│   │   ├── subtitles/
│   │   └── streaming/
│   ├── ui/                 # UI components
│   ├── plugins/            # Plugin system
│   ├── hwaccel/            # Hardware acceleration
│   ├── integrations/       # External integrations
│   ├── ai/                 # AI features
│   ├── marketplace/        # Plugin marketplace
│   └── devtools/           # Developer tools
├── src/                    # FSD structure (TypeScript/React)
│   ├── app/
│   ├── processes/
│   ├── pages/
│   ├── widgets/
│   ├── features/
│   ├── entities/
│   └── shared/
├── core/                   # Legacy Rust crate
├── audio/                  # Legacy Rust crate
├── video/                  # Legacy Rust crate
├── ... (other legacy crates)
└── turbo.json              # Turborepo configuration
```

## Import Changes

### Before (Legacy)
```rust
use vantis_core::types::MediaId;
use vantis_audio::decoder::AudioDecoder;
```

### After (New Structure)
```rust
// From new packages
use vantis_core_new::{MediaId, MediaInfo, Config};
use vantis_audio_new::decoder::AudioDecoder;
```

## Next Steps

1. Complete package creation for media, ui, plugins
2. Run `cargo build --workspace` to verify
3. Update CI/CD for new structure
4. Archive legacy directories
5. Update documentation

## Notes

- Legacy crates remain functional during migration
- New packages use `-new` suffix temporarily
- Full migration will remove legacy code
- CI/CD will test both old and new structures

---

*Last updated: 2026-03-07*