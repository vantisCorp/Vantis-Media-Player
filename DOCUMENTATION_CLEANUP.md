# Documentation Cleanup Plan

## Analysis

### Duplicate Directories
1. `/workspace/vantis-player/` - Old duplicate of Vantis-Media-Player
2. `/workspace/V-Streaming/` - Separate TypeScript/Tauri project

### Root-Level Documentation (To Consolidate)

**Keep (Authoritative):**
- `README.md` - Main project README
- `CHANGELOG.md` - Version history
- `ROADMAP.md` - Future plans
- `SECURITY.md` - Security policy
- `CODE_OF_CONDUCT.md` - Community guidelines
- `CONTRIBUTING.md` - Contribution guide
- `CONTRIBUTORS.md` - Contributor list
- `LICENSE` - License file
- `CITATION.cff` - Citation info

**Keep (Project-Specific):**
- `ARCHITECTURE.md` - System architecture
- `BUILD_SYSTEM.md` - Build documentation
- `PLUGIN_SYSTEM.md` - Plugin documentation
- `MASTER_TODO.md` - Project tracking

**Archive (Historical/Session):**
- `SESSION_SUMMARY_PRIORITY_A.md` → `archive/reports/`
- `PROGRESS_SUMMARY.md` → `archive/reports/`
- `MONOREPO_ANALYSIS.md` → `archive/reports/`
- `VIDEOLAN_ANALYSIS.md` → `archive/reports/`
- `SIMD_OPTIMIZATIONS.md` → `archive/reports/`

**Remove (Outdated/Duplicate):**
- `QUICKSTART.md` (merge into README.md)
- `GETTING_STARTED.md` (merge into docs/)
- `FAQ.md` (merge into docs/content/docs/reference/faq.md)
- `PERFORMANCE_GUIDE.md` (merge into docs/)
- `DEPLOYMENT.md` (merge into docs/content/docs/deployment/)
- `DOCKER.md` (merge into docs/content/docs/deployment/docker.md)
- `INSTALLERS.md` (merge into docs/)
- `PROJECT_MAINTENANCE.md` (outdated)
- `PROJECT_STRUCTURE.md` (outdated, replaced by MONOREPO_MIGRATION.md)
- `LEGAL.md` (merge into LICENSE or CONTRIBUTING.md)
- `BRANDING.md` (merge into docs/)
- `SUPPORT.md` (merge into README.md)

### Docs Directory Structure

**Keep:**
- `docs/content/docs/` - Docusaurus content (authoritative)
- `docs/API_REFERENCE.md` - API documentation
- `docs/PLUGIN_DEVELOPMENT.md` - Plugin guide

**Archive:**
- `docs/progress/` → `archive/reports/progress/`
- `docs/implementation/` → `archive/reports/implementation/`
- `docs/modules/` → `archive/reports/modules/`
- `docs/features/` → `archive/reports/features/`

### Actions

1. **Archive historical reports** - Move to `archive/reports/`
2. **Merge overlapping documentation** - Combine into single files
3. **Update Docusaurus** - Ensure all content is in `docs/content/docs/`
4. **Remove duplicates from other directories** - vantis-player/, V-Streaming/
5. **Create single source of truth** for each document type

## Files to Remove/Archive

### Move to Archive
```
SESSION_SUMMARY_PRIORITY_A.md → archive/reports/
PROGRESS_SUMMARY.md → archive/reports/
MONOREPO_ANALYSIS.md → archive/reports/
VIDEOLAN_ANALYSIS.md → archive/reports/
SIMD_OPTIMIZATIONS.md → archive/reports/
docs/progress/* → archive/reports/progress/
docs/implementation/* → archive/reports/implementation/
docs/modules/* → archive/reports/modules/
docs/features/* → archive/reports/features/
```

### Remove (Already in Docusaurus)
```
FAQ.md (exists as docs/content/docs/reference/faq.md)
ROADMAP.md in docs/ (duplicate of root)
```

### Consolidate
```
QUICKSTART.md → Merge into README.md
GETTING_STARTED.md → Merge into docs/content/docs/getting-started/
```

## Next Steps

1. Create archive directories
2. Move historical files
3. Merge duplicate content
4. Update internal links
5. Verify Docusaurus build
6. Update CI/CD for docs

---

*Last updated: 2026-03-07*