# 🔒 Security Guide for Vantis Media Player

This document outlines the security architecture, best practices, and security considerations for Vantis Media Player.

## Table of Contents

1. [Security Overview](#security-overview)
2. [Security Architecture](#security-architecture)
3. [WASM Sandbox](#wasm-sandbox)
4. [File System Security](#file-system-security)
5. [Network Security](#network-security)
6. [Plugin Security](#plugin-security)
7. [Data Privacy](#data-privacy)
8. [Security Best Practices](#security-best-practices)
9. [Vulnerability Reporting](#vulnerability-reporting)
10. [Security Audits](#security-audits)

## Security Overview

Vantis Media Player implements defense-in-depth security principles:

- **WASM Sandbox**: All plugins run in isolated WASM environments
- **Principle of Least Privilege**: Minimal permissions for all components
- **Memory Safety**: Rust's memory safety guarantees
- **Input Validation**: All inputs are validated and sanitized
- **Secure by Default**: Secure configurations out of the box

### Threat Model

| Threat | Impact | Mitigation |
|--------|--------|------------|
| Malicious plugins | System compromise | WASM sandbox, API restrictions |
| Video file exploits | Code execution | Input validation, sandboxed decoders |
| Network attacks | Data theft | TLS, input validation |
| Supply chain attacks | Backdoor injection | Dependency verification, reproducible builds |
| Side-channel attacks | Data leakage | Constant-time algorithms, ASLR |

## Security Architecture

### Layered Security

```
┌─────────────────────────────────────┐
│         User Interface Layer        │
│  (Validation, sanitization)         │
├─────────────────────────────────────┤
│       Application Layer             │
│  (Memory safety, bounds checking)   │
├─────────────────────────────────────┤
│      Plugin Layer (WASM)            │
│  (Sandboxed execution)              │
├─────────────────────────────────────┤
│       Core Services Layer           │
│  (Privilege separation)             │
├─────────────────────────────────────┤
│         OS/Driver Layer             │
│  (Kernel protections)               │
└─────────────────────────────────────┘
```

### Security Guarantees

#### Memory Safety

All code is written in Rust, providing:
- No buffer overflows
- No null pointer dereferences
- No data races
- Automatic memory management

#### Type Safety

Strong type system prevents:
- Type confusion attacks
- Integer overflows
- Use-after-free bugs

#### Concurrency Safety

Safe concurrency primitives:
- No data races
- Thread-safe communication channels
- Atomic operations

## WASM Sandbox

### Sandbox Architecture

Plugins run in WASM environments using Wasmtime:

```rust
// WASM sandbox configuration
let mut config = Config::new();
config.wasm_simd(true);
config.wasm_bulk_memory(true);
config.wasm_reference_types(true);
config.max_wasm_stack(1024 * 1024); // 1MB stack limit
config.wasm_threads(false); // Disable threading for security

// Memory limits
let mut store = Store::new(&engine, HostState::new());
store.limiter(|memory| {
    // Limit memory allocation
    memory.grow(0).ok(); // No growth allowed
});
```

### Security Features

#### 1. Memory Isolation

- Plugins cannot access host memory directly
- All data transfer through defined APIs
- Memory sandbox prevents pointer escape

#### 2. API Restrictions

Only safe host functions are exposed:

```rust
// Safe API exports
extern "C" {
    fn vantis_log(level: u32, message_ptr: u32, len: u32);
    fn vantis_get_media_info() -> MediaInfo;
    fn vantis_play();
    fn vantis_pause();
    fn vantis_seek(time_ms: u32);
}

// NOT exposed (dangerous):
// fn vantis_execute_command(cmd: *const u8);
// fn vantis_read_file(path: *const u8);
// fn vantis_write_file(path: *const u8, data: *const u8);
```

#### 3. Resource Limits

```toml
[advanced.wasm]
max_memory_mb = 128        # Per-plugin memory limit
max_execution_time_ms = 10000  # Timeout per function call
max_file_size_mb = 10      # File I/O limit
max_network_requests = 5   # Network request limit
```

#### 4. No Direct System Access

Plugins cannot:
- Execute shell commands
- Access file system directly
- Open network sockets
- Access hardware devices
- Load native libraries

### Host Function Security

All host functions implement:

```rust
// Example: Safe logging function
#[no_mangle]
pub unsafe extern "C" fn vantis_log(
    level: u32,
    message_ptr: u32,
    len: u32
) {
    // Validate inputs
    if level > 3 {
        return; // Invalid log level
    }
    
    // Get memory reference
    let memory = get_plugin_memory();
    let memory_size = memory.size() as usize;
    
    // Check bounds
    let message_offset = message_ptr as usize;
    if message_offset + (len as usize) > memory_size {
        return; // Out of bounds
    }
    
    // Safe string extraction
    let message = extract_string(memory, message_ptr, len);
    
    // Sanitize output
    let sanitized = sanitize_log_message(&message);
    
    // Log safely
    log::log(level_to_log_level(level), "{}", sanitized);
}

// Sanitize log message to prevent log injection
fn sanitize_log_message(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii() || c.is_alphanumeric())
        .collect()
}
```

### Plugin Validation

#### 1. Signature Verification

```rust
// Verify plugin signature before loading
pub fn verify_plugin_signature(plugin_path: &Path) -> Result<bool> {
    let signature = extract_signature(plugin_path)?;
    let public_key = load_public_key()?;
    
    public_key.verify(&signature)
}
```

#### 2. Code Analysis

```rust
// Scan for dangerous patterns
pub fn scan_plugin(wasm_bytes: &[u8]) -> Result<ScanResult> {
    let patterns = vec![
        // Known dangerous function calls
        b"web_sys::window()",
        b"std::process::Command",
        b"std::fs::File",
    ];
    
    for pattern in patterns {
        if wasm_bytes.windows(pattern.len()).any(|w| w == pattern) {
            return Ok(ScanResult::Dangerous(pattern.to_vec()));
        }
    }
    
    Ok(ScanResult::Safe)
}
```

## File System Security

### Access Control

#### Configuration Directory

```
~/.vantis/
├── config.toml          # 600 (rw-------)
├── cache/               # 755 (rwxr-xr-x)
│   ├── thumbnails/      # 644 (rw-r--r--)
│   └── subtitles/       # 644 (rw-r--r--)
├── plugins/             # 755 (rwxr-xr-x)
│   └── *.wasm           # 644 (rw-r--r--)
└── logs/                # 755 (rwxr-xr-x)
    └── *.log            # 644 (rw-r--r--)
```

#### Permission Enforcement

```rust
// Set secure permissions
pub fn set_secure_permissions(path: &Path, mode: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(mode);
    std::fs::set_permissions(path, perms)?;
    
    Ok(())
}

// Apply on startup
set_secure_permissions(&config_dir, 0o700)?;
set_secure_permissions(&config_file, 0o600)?;
```

### Path Traversal Prevention

```rust
// Validate file paths
pub fn validate_path(path: &Path, allowed_dir: &Path) -> Result<PathBuf> {
    let canonical = path.canonicalize()?;
    let allowed = allowed_dir.canonicalize()?;
    
    if !canonical.starts_with(&allowed) {
        return Err(SecurityError::PathTraversal);
    }
    
    Ok(canonical)
}

// Usage in media loading
let media_path = validate_path(&user_input, &allowed_media_dir)?;
```

### Symbolic Link Protection

```rust
// Prevent symlink attacks
pub fn safe_resolve_path(path: &Path) -> Result<PathBuf> {
    let metadata = path.symlink_metadata()?;
    
    if metadata.file_type().is_symlink() {
        return Err(SecurityError::SymlinkNotAllowed);
    }
    
    Ok(path.to_path_buf())
}
```

## Network Security

### TLS Configuration

All network traffic uses TLS 1.3:

```rust
// Secure TLS configuration
pub fn create_tls_config() -> rustls::ClientConfig {
    let mut root_store = rustls::RootCertStore::empty();
    
    // Load system certificates
    for cert in rustls_native_certs::load_native_certs()
        .expect("could not load platform certs")
    {
        root_store.add(&rustls::Certificate(cert.0)).unwrap();
    }
    
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    
    config
}
```

### Certificate Verification

```rust
// Strict certificate verification
let config = rustls::ClientConfig::builder()
    .with_safe_defaults()  // Rejects weak ciphers, TLS 1.2-
    .with_root_certificates(root_store)
    .with_no_client_auth();

// Enable certificate pinning for known services
config.enable_sni = true;
config.max_version = rustls::ProtocolVersion::TLSv1_3;
config.min_version = rustls::ProtocolVersion::TLSv1_3;
```

### Request Validation

```rust
// Validate URLs before requesting
pub fn validate_url(url: &str) -> Result<()> {
    let parsed = Url::parse(url)?;
    
    // Only allow HTTPS
    if parsed.scheme() != "https" {
        return Err(SecurityError::InsecureScheme);
    }
    
    // Block localhost and private networks
    match parsed.host() {
        Some(host) => {
            if host.is_loopback() {
                return Err(SecurityError::LocalhostNotAllowed);
            }
            if host.is_private() {
                return Err(SecurityError::PrivateNetworkNotAllowed);
            }
        }
        None => return Err(SecurityError::InvalidHost),
    }
    
    Ok(())
}
```

### Rate Limiting

```rust
// Rate limit network requests
use governor::{Quota, RateLimiter};

pub struct NetworkLimiter {
    limiter: RateLimiter<direct::NotKeyed, state::InMemoryState>,
}

impl NetworkLimiter {
    pub fn new() -> Self {
        let quota = Quota::per_second(NonZeroU32::new(10).unwrap());
        let limiter = RateLimiter::direct(quota);
        
        Self { limiter }
    }
    
    pub fn check(&self) -> Result<()> {
        self.limiter.check().map_err(|_| SecurityError::RateLimitExceeded)
    }
}
```

## Plugin Security

### Plugin Manifest

Each plugin requires a manifest:

```toml
# plugin.toml
name = "example-plugin"
version = "1.0.0"
author = "Vantis Team"
description = "Example plugin"

[permissions]
# Required permissions (all denied by default)
access_media_info = false
control_playback = false
read_subtitles = false

[security]
signature_required = true
max_memory_mb = 128
max_execution_time_ms = 10000
```

### Permission System

```rust
// Define permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    AccessMediaInfo,
    ControlPlayback,
    ReadSubtitles,
    WriteSubtitles,
    AccessNetwork,
}

// Check permission before API call
pub fn check_permission(plugin_id: &str, perm: Permission) -> Result<bool> {
    let manifest = load_plugin_manifest(plugin_id)?;
    
    Ok(manifest.permissions.contains(&perm))
}
```

### Plugin Isolation

```rust
// Isolated plugin instance
pub struct IsolatedPlugin {
    id: String,
    store: Store<HostState>,
    instance: Instance,
    permissions: Vec<Permission>,
}

impl IsolatedPlugin {
    pub fn call_function(&mut self, name: &str, args: &[Val]) -> Result<Vec<Val>> {
        // Check if function is allowed
        if !is_function_allowed(name, &self.permissions) {
            return Err(SecurityError::PermissionDenied);
        }
        
        // Execute with timeout
        let start = Instant::now();
        let result = self.instance.get_func(&self.store, name)
            .ok_or(SecurityError::FunctionNotFound)?
            .call(&mut self.store, args);
        
        // Check execution time
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(self.max_execution_time_ms) {
            return Err(SecurityError::ExecutionTimeout);
        }
        
        result.map_err(|e| SecurityError::ExecutionFailed(e.to_string()))
    }
}
```

## Data Privacy

### Data Collection

Vantis Media Player does NOT collect:
- User viewing habits
- Media library contents
- User location data
- Personal information

#### What IS Collected (Optional)

If user opts in:
- Anonymous crash reports
- Performance metrics (FPS, CPU usage)
- Feature usage statistics

```toml
[privacy]
analytics_enabled = false       # Default: false
crash_reports_enabled = true   # Default: true
share_plugin_list = false      # Default: false
```

### Data Storage

#### Configuration

```toml
[privacy]
config_dir = "~/.vantis"
encrypt_config = false         # Optional: encrypt sensitive config
```

#### Cache Management

```bash
# Clear all cache
vantis cache clear

# Clear specific cache
vantis cache clear thumbnails
vantis cache clear subtitles
```

### Third-Party Services

#### TMDB Integration

- Only public API calls (no authentication)
- Read-only access
- No user data sent

#### Subtitle Services

- Hash-based matching (no content upload)
- Anonymous requests
- No user accounts required

## Security Best Practices

### For Users

#### 1. Keep Updated

```bash
# Check for updates
vantis --check-updates

# Update
vantis --update
```

#### 2. Use Secure Configuration

```toml
[privacy]
analytics_enabled = false
encrypt_config = true

[advanced]
wasm_sandbox = true
ipc_guard = true
```

#### 3. Plugin Safety

- Only install plugins from trusted sources
- Verify plugin signatures
- Review plugin permissions
- Keep plugins updated

#### 4. Network Safety

- Only use HTTPS subtitle services
- Disable auto-download from unknown sources
- Review network access logs

#### 5. File Permissions

```bash
# Secure config directory
chmod 700 ~/.vantis
chmod 600 ~/.vantis/config.toml
```

### For Developers

#### 1. Code Reviews

- All code must be reviewed
- Security-focused review for sensitive code
- Use static analysis tools

#### 2. Testing

```bash
# Security tests
cargo test security

# Fuzz testing
cargo fuzz run subtitle_parser

# Dependency audit
cargo audit
cargo deny check
```

#### 3. Dependencies

```toml
# Cargo.toml - pin dependencies
[dependencies]
wasmtime = "=14.0.0"   # Pin major versions
tokio = "=1.35.0"
rustls = "=0.23.0"
```

#### 4. Secrets Management

```rust
// Never hardcode secrets
// ❌ BAD
let api_key = "sk_live_1234567890abcdef";

// ✅ GOOD
let api_key = std::env::var("API_KEY")?;
```

## Vulnerability Reporting

### Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Email**: security@vantis-os.org
2. **Subject**: [SECURITY] Vulnerability Report
3. **Include**:
   - Detailed description
   - Steps to reproduce
   - Impact assessment
   - Proposed fix (optional)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Detailed Review**: Within 7 days
- **Fix Timeline**: Based on severity
- **Public Disclosure**: After fix is released

### Severity Levels

| Severity | Response Time | Examples |
|----------|--------------|----------|
| Critical | 48 hours | RCE, data breach |
| High | 7 days | Privilege escalation |
| Medium | 14 days | Information disclosure |
| Low | 30 days | Minor issues |

## Security Audits

### Automated Security Checks

#### CI/CD Pipeline

```yaml
# .github/workflows/security.yml
name: Security Audit

on: [push, pull_request]

jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Run cargo-audit
        run: cargo audit
      
      - name: Run cargo-deny
        run: cargo deny check
      
      - name: Run security tests
        run: cargo test security
      
      - name: Fuzz testing
        run: cargo fuzz run subtitle_parser -- -max_total_time=300
```

### Dependency Scanning

```bash
# Check for known vulnerabilities
cargo audit

# Check license compliance
cargo deny check

# Check for outdated dependencies
cargo outdated
```

### Static Analysis

```bash
# Clippy linting
cargo clippy --all-targets --all-features -- -D warnings

# Format check
cargo fmt --all -- --check
```

### Penetration Testing

Regular penetration testing includes:
- Fuzz testing of parsers
- Memory leak detection
- Race condition testing
- Integer overflow checks
- Boundary testing

## Security Checklist

### Pre-Release Checklist

- [ ] All dependencies audited
- [ ] No known vulnerabilities
- [ ] Security tests passing
- [ ] Code reviewed
- [ ] Fuzz testing completed
- [ ] Memory leak detection clean
- [ ] Permissions properly set
- [ ] Configuration defaults secure
- [ ] Documentation updated

### Ongoing Security

- [ ] Dependencies monitored
- [ ] Security advisories tracked
- [ ] CI/CD security checks enabled
- [ ] Automated testing running
- [ ] Vulnerability reporting process in place

---

## Conclusion

Vantis Media Player implements comprehensive security measures to protect users while providing powerful features. The WASM sandbox ensures plugin safety, and the Rust language provides memory safety guarantees.

For security questions or to report vulnerabilities, contact us at security@vantis-os.org.

For more information, see:
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
- [PLUGIN_DEVELOPMENT.md](PLUGIN_DEVELOPMENT.md) - Plugin security
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) - Security issues