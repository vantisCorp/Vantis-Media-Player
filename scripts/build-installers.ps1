# Build Installers Script for Vantis Media Player (Windows)
# This script builds installers for Windows

param(
    [string]$Version = "1.0.0",
    [string]$OutputDir = "installers"
)

# Colors
$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Blue
Write-Host "Vantis Media Player - Build Installers" -ForegroundColor Blue
Write-Host "========================================" -ForegroundColor Blue
Write-Host ""

# Create output directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
}

# Build release
Write-Host "Building release..." -ForegroundColor Yellow
cargo build --release

# Create installer directory
$InstallerDir = "$OutputDir\vantis-player-windows-x64"
if (-not (Test-Path $InstallerDir)) {
    New-Item -ItemType Directory -Path $InstallerDir | Out-Null
}

# Copy files
Write-Host "Copying files..." -ForegroundColor Yellow
Copy-Item "target\release\vantis.exe" -Destination "$InstallerDir\vantis.exe"
Copy-Item "README.md" -Destination "$InstallerDir\README.md"
Copy-Item "LICENSE" -Destination "$InstallerDir\LICENSE"
Copy-Item "CHANGELOG.md" -Destination "$InstallerDir\CHANGELOG.md"

# Create ZIP archive
Write-Host "Creating ZIP archive..." -ForegroundColor Yellow
$ZipPath = "$OutputDir\vantis-player-windows-x64-$Version.zip"
Compress-Archive -Path "$InstallerDir\*" -DestinationPath $ZipPath -Force

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Build complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "Installer: $ZipPath" -ForegroundColor Blue