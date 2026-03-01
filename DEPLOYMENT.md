# 🚀 Deployment Guide for Vantis Media Player

This guide covers deployment strategies and procedures for Vantis Media Player across different environments.

## Table of Contents

1. [Deployment Overview](#deployment-overview)
2. [Pre-Deployment Checklist](#pre-deployment-checklist)
3. [Building for Production](#building-for-production)
4. [Deployment Methods](#deployment-methods)
5. [Platform-Specific Deployment](#platform-specific-deployment)
6. [Docker Deployment](#docker-deployment)
7. [Cloud Deployment](#cloud-deployment)
8. [Monitoring and Maintenance](#monitoring-and-maintenance)
9. [Rollback Procedures](#rollback-procedures)
10. [Troubleshooting](#troubleshooting)

## Deployment Overview

Vantis Media Player supports multiple deployment methods:

- **Native Binaries**: Direct installation on target systems
- **Docker Containers**: Containerized deployment
- **Cloud Services**: Cloud-native deployment
- **Package Managers**: System package distribution

### Deployment Architecture

```
┌─────────────────────────────────────┐
│         Deployment Target           │
├─────────────────────────────────────┤
│  Native Binary  │  Docker  │  Cloud │
└─────────────────────────────────────┘
           ↓              ↓          ↓
    ┌────────────────────────────────┐
    │   Vantis Media Player Core     │
    ├────────────────────────────────┤
    │  Video │ Audio │ Subtitles     │
    │  UI    │ Plugins│ Integrations │
    └────────────────────────────────┘
```

## Pre-Deployment Checklist

Before deploying, ensure:

- [ ] All tests pass (`cargo test`)
- [ ] Benchmarks meet performance targets (`cargo bench`)
- [ ] Security audit passes (`cargo audit`)
- [ ] Documentation is up to date
- [ ] Version number is updated
- [ ] CHANGELOG.md is updated
- [ ] Release notes are prepared
- [ ] Dependencies are up to date
- [ ] No deprecated features are used
- [ ] Configuration defaults are appropriate

## Building for Production

### Release Build

```bash
# Clean previous builds
cargo clean

# Build release version
cargo build --release

# Verify binary
./target/release/vantis --version

# Run tests on release build
cargo test --release
```

### Optimized Build

```bash
# Build with optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Strip debug symbols
strip target/release/vantis

# Compress binary
upx --best target/release/vantis
```

### Cross-Platform Builds

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# Windows x86_64
cargo build --release --target x86_64-pc-windows-gnu

# macOS x86_64
cargo build --release --target x86_64-apple-darwin

# macOS ARM64 (Apple Silicon)
cargo build --release --target aarch64-apple-darwin
```

## Deployment Methods

### Method 1: Native Binary Deployment

#### Linux

```bash
# Copy binary to system
sudo cp target/release/vantis /usr/local/bin/

# Set permissions
sudo chmod +x /usr/local/bin/vantis

# Create desktop entry
sudo cat > /usr/share/applications/vantis-player.desktop << EOF
[Desktop Entry]
Name=Vantis Media Player
Comment=Advanced media player
Exec=/usr/local/bin/vantis %F
Icon=vantis-player
Type=Application
Categories=AudioVideo;Player;
MimeType=video/*;audio/*;
EOF

# Install icon
sudo cp assets/icon.png /usr/share/icons/hicolor/256x256/apps/vantis-player.png
```

#### Windows

```bash
# Copy binary to Program Files
copy target\release\vantis.exe "C:\Program Files\Vantis Media Player&quot;

# Add to PATH
setx PATH "%PATH%;C:\Program Files\Vantis Media Player"

# Create desktop shortcut
powershell -Command "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('$HOME\Desktop\Vantis Media Player.lnk'); $Shortcut.TargetPath = 'C:\Program Files\Vantis Media Player\vantis.exe'; $Shortcut.Save()"
```

#### macOS

```bash
# Create app bundle
mkdir -p VantisPlayer.app/Contents/{MacOS,Resources}
cp target/release/vantis VantisPlayer.app/Contents/MacOS/
cat > VantisPlayer.app/Contents/Info.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>vantis</string>
    <key>CFBundleIdentifier</key>
    <string>com.vantis.player</string>
    <key>CFBundleName</key>
    <string>Vantis Media Player</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
</dict>
</plist>
EOF

# Copy to Applications
sudo cp -R VantisPlayer.app /Applications/
```

### Method 2: Package Manager Deployment

#### Debian/Ubuntu (.deb)

```bash
# Create package directory
mkdir -p vantis-player_1.0.0/DEBIAN
mkdir -p vantis-player_1.0.0/usr/local/bin
mkdir -p vantis-player_1.0.0/usr/share/applications
mkdir -p vantis-player_1.0.0/usr/share/icons/hicolor/256x256/apps

# Copy files
cp target/release/vantis vantis-player_1.0.0/usr/local/bin/
cp assets/vantis-player.desktop vantis-player_1.0.0/usr/share/applications/
cp assets/icon.png vantis-player_1.0.0/usr/share/icons/hicolor/256x256/apps/vantis-player.png

# Create control file
cat > vantis-player_1.0.0/DEBIAN/control << EOF
Package: vantis-player
Version: 1.0.0
Architecture: amd64
Maintainer: Vantis Team <team@vantis-os.org>
Depends: libavcodec-dev, libavformat-dev, libssl-dev
Description: Advanced media player with GPU acceleration
 Vantis Media Player is a high-performance media player featuring
 GPU acceleration, AI-powered features, and a WASM plugin system.
EOF

# Build package
dpkg-deb --build vantis-player_1.0.0

# Install
sudo dpkg -i vantis-player_1.0.0.deb
```

#### Red Hat/Fedora (.rpm)

```bash
# Create spec file
cat > vantis-player.spec << EOF
Name: vantis-player
Version: 1.0.0
Release: 1%{?dist}
Summary: Advanced media player

License: MIT
URL: https://github.com/vantis-os/vantis-player
Source0: %{name}-%{version}.tar.gz

BuildRequires: cargo, rust, ffmpeg-devel, openssl-devel
Requires: ffmpeg-libs, openssl-libs

%description
Vantis Media Player is a high-performance media player featuring
GPU acceleration, AI-powered features, and a WASM plugin system.

%prep
%setup -q

%build
cargo build --release

%install
install -D -m 755 target/release/vantis %{buildroot}%{_bindir}/vantis

%files
%{_bindir}/vantis

%changelog
* $(date +'%a %b %d %Y') Vantis Team <team@vantis-os.org> - 1.0.0-1
- Initial release
EOF

# Build package
rpmbuild -ba vantis-player.spec

# Install
sudo rpm -i ~/rpmbuild/RPMS/x86_64/vantis-player-1.0.0-1.x86_64.rpm
```

#### Arch Linux (AUR)

```bash
# Create PKGBUILD
cat > PKGBUILD << EOF
pkgname=vantis-player
pkgver=1.0.0
pkgrel=1
pkgdesc="Advanced media player with GPU acceleration"
arch=('x86_64')
url="https://github.com/vantis-os/vantis-player"
license=('MIT')
depends=('ffmpeg' 'openssl')
makedepends=('cargo' 'rust')
source=("\$pkgname-\$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "\$pkgname-\$pkgver"
  cargo build --release
}

package() {
  cd "\$pkgname-\$pkgver"
  install -Dm755 target/release/vantis "\$pkgdir/usr/bin/vantis"
}
EOF

# Build package
makepkg -si
```

## Platform-Specific Deployment

### Linux Deployment

#### Systemd Service

```bash
# Create service file
sudo cat > /etc/systemd/system/vantis-player.service << EOF
[Unit]
Description=Vantis Media Player
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/vantis --daemon
Restart=on-failure
User=vantis
Group=vantis

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable vantis-player
sudo systemctl start vantis-player
```

#### SELinux Configuration

```bash
# Create SELinux policy
cat > vantis-player.te << EOF
module vantis-player 1.0;

require {
    type unconfined_t;
    type etc_t;
    type bin_t;
    class file { read open execute };
}

allow unconfined_t bin_t:file { read open execute };
allow unconfined_t etc_t:file { read open };
EOF

# Compile and load policy
checkmodule -M -m -o vantis-player.mod vantis-player.te
semodule_package -o vantis-player.pp -m vantis-player.mod
semodule -i vantis-player.pp
```

### Windows Deployment

#### Installer (NSIS)

```nsis
; VantisPlayer.nsi
!define APPNAME "Vantis Media Player"
!define COMPANYNAME "Vantis OS"
!define DESCRIPTION "Advanced media player"
!define VERSIONMAJOR 1
!define VERSIONMINOR 0
!define VERSIONBUILD 0

RequestExecutionLevel admin

InstallDir "$PROGRAMFILES\Vantis Media Player"

Page directory
Page instfiles

Section "install"
    SetOutPath $INSTDIR
    File "target\release\vantis.exe"
    File "assets\icon.png"
    
    WriteUninstaller "$INSTDIR\uninstall.exe"
    
    CreateDirectory "$SMPROGRAMS\Vantis Media Player"
    CreateShortcut "$SMPROGRAMS\Vantis Media Player\Vantis Media Player.lnk" "$INSTDIR\vantis.exe"
    CreateShortcut "$SMPROGRAMS\Vantis Media Player\Uninstall.lnk" "$INSTDIR\uninstall.exe"
SectionEnd

Section "uninstall"
    Delete $INSTDIR\vantis.exe
    Delete $INSTDIR\icon.png
    Delete $INSTDIR\uninstall.exe
    RMDir $INSTDIR
    
    Delete "$SMPROGRAMS\Vantis Media Player\Vantis Media Player.lnk"
    Delete "$SMPROGRAMS\Vantis Media Player\Uninstall.lnk"
    RMDir "$SMPROGRAMS\Vantis Media Player"
SectionEnd
```

Build installer:
```bash
makensis VantisPlayer.nsi
```

### macOS Deployment

#### Homebrew Formula

```ruby
# Formula/vantis-player.rb
class VantisPlayer < Formula
  desc "Advanced media player with GPU acceleration"
  homepage "https://github.com/vantis-os/vantis-player"
  url "https://github.com/vantis-os/vantis-player/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "..."
  license "MIT"

  depends_on "rust" => :build
  depends_on "ffmpeg"
  depends_on "openssl"

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/vantis"
  end

  test do
    system "#{bin}/vantis", "--version"
  end
end
```

Install via Homebrew:
```bash
brew tap vantis-os/vantis
brew install vantis-player
```

## Docker Deployment

### Building Docker Image

```bash
# Build image
docker build -t vantis-player:latest .

# Tag image
docker tag vantis-player:latest vantis-player:1.0.0

# Push to registry
docker push vantis-player:latest
docker push vantis-player:1.0.0
```

### Running Docker Container

```bash
# Basic run
docker run -it --rm \
    -v /path/to/videos:/media \
    --device /dev/snd \
    vantis-player:latest

# With GUI support
docker run -it --rm \
    -e DISPLAY=$DISPLAY \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    -v /path/to/videos:/media \
    --device /dev/snd \
    vantis-player:latest

# Daemon mode
docker run -d \
    --name vantis-player \
    -v /path/to/videos:/media \
    --device /dev/snd \
    vantis-player:latest
```

### Docker Compose

```yaml
version: '3.8'

services:
  vantis-player:
    image: vantis-player:latest
    container_name: vantis-player
    volumes:
      - /path/to/videos:/media
      - /path/to/config:/root/.vantis
    devices:
      - /dev/snd
    environment:
      - DISPLAY=${DISPLAY}
    ports:
      - "8080:8080"
    restart: unless-stopped
```

Run with Docker Compose:
```bash
docker-compose up -d
```

## Cloud Deployment

### AWS Deployment

#### EC2 Instance

```bash
# Launch EC2 instance
aws ec2 run-instances \
    --image-id ami-0abcdef1234567890 \
    --instance-type t3.medium \
    --key-name my-key-pair \
    --security-group-ids sg-12345678 \
    --user-data file://user-data.sh

# user-data.sh
#!/bin/bash
yum update -y
yum install -y docker
systemctl start docker
docker run -d --name vantis-player vantis-player:latest
```

#### ECS Deployment

```json
{
  "family": "vantis-player",
  "containerDefinitions": [
    {
      "name": "vantis-player",
      "image": "vantis-player:latest",
      "memory": 2048,
      "cpu": 512,
      "essential": true,
      "portMappings": [
        {
          "containerPort": 8080,
          "protocol": "tcp"
        }
      ]
    }
  ]
}
```

### Google Cloud Deployment

#### GKE Deployment

```yaml
# deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: vantis-player
spec:
  replicas: 2
  selector:
    matchLabels:
      app: vantis-player
  template:
    metadata:
      labels:
        app: vantis-player
    spec:
      containers:
      - name: vantis-player
        image: vantis-player:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "2Gi"
            cpu: "500m"
          limits:
            memory: "4Gi"
            cpu: "1000m"
```

Deploy:
```bash
kubectl apply -f deployment.yaml
```

### Azure Deployment

#### Azure Container Instances

```bash
# Create container group
az container create \
    --resource-group myResourceGroup \
    --name vantis-player \
    --image vantis-player:latest \
    --cpu 1 \
    --memory 2 \
    --ports 8080
```

## Monitoring and Maintenance

### Health Checks

```bash
# Check if player is running
pgrep -f vantis-player

# Check version
vantis --version

# Check dependencies
ldd /usr/local/bin/vantis

# Check logs
journalctl -u vantis-player -f
```

### Performance Monitoring

```bash
# Monitor CPU usage
top -p $(pgrep vantis-player)

# Monitor memory usage
ps aux | grep vantis-player

# Monitor GPU usage
nvidia-smi dmon -s u -c 100

# Monitor network connections
netstat -an | grep $(pgrep vantis-player)
```

### Log Management

```bash
# Configure log rotation
sudo cat > /etc/logrotate.d/vantis-player << EOF
/var/log/vantis-player/*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    create 0640 vantis vantis
}
EOF
```

## Rollback Procedures

### Binary Rollback

```bash
# Stop current version
sudo systemctl stop vantis-player

# Restore previous version
sudo cp /usr/local/bin/vantis.backup /usr/local/bin/vantis

# Start service
sudo systemctl start vantis-player
```

### Docker Rollback

```bash
# Stop current container
docker stop vantis-player

# Remove current container
docker rm vantis-player

# Run previous version
docker run -d --name vantis-player vantis-player:1.0.0
```

### Package Rollback

```bash
# Debian/Ubuntu
sudo apt-get install vantis-player=0.9.0

# Red Hat/Fedora
sudo yum downgrade vantis-player-0.9.0

# Arch Linux
sudo pacman -U /var/cache/pacman/pkg/vantis-player-0.9.0-1-x86_64.pkg.tar.zst
```

## Troubleshooting

### Common Issues

#### Binary Not Found

```bash
# Check PATH
echo $PATH

# Verify installation
which vantis

# Reinstall
sudo cp target/release/vantis /usr/local/bin/
```

#### Permission Denied

```bash
# Set executable permissions
sudo chmod +x /usr/local/bin/vantis

# Check ownership
ls -l /usr/local/bin/vantis
```

#### Missing Dependencies

```bash
# Check dependencies
ldd /usr/local/bin/vantis

# Install missing dependencies
sudo apt-get install -f  # Debian/Ubuntu
sudo yum install -y     # Red Hat/Fedora
```

#### GPU Not Detected

```bash
# Check GPU
vantis --check-gpu

# Install GPU drivers
sudo apt-get install nvidia-driver-535  # NVIDIA
sudo apt-get install mesa-vulkan-drivers  # AMD/Intel
```

## Security Considerations

### File Permissions

```bash
# Set secure permissions
sudo chmod 755 /usr/local/bin/vantis
sudo chmod 644 /usr/share/applications/vantis-player.desktop
sudo chmod 644 /usr/share/icons/hicolor/256x256/apps/vantis-player.png
```

### Firewall Configuration

```bash
# Allow necessary ports
sudo ufw allow 8080/tcp  # Web interface
sudo ufw allow 8081/tcp  # Streaming
```

### Updates

```bash
# Check for updates
vantis --check-updates

# Update
vantis --update
```

---

## Conclusion

This deployment guide covers all major deployment methods for Vantis Media Player. Choose the method that best fits your environment and requirements.

For additional support, see:
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md)
- [SECURITY.md](SECURITY.md)
- [CONTRIBUTING.md](CONTRIBUTING.md)