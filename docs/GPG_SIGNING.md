# GPG Commit Signing Guide

This guide explains how to set up GPG commit signing for Vantis Media Player development.

## Why Sign Commits?

- **Security**: Ensures commits are from verified authors
- **Integrity**: Detects if commits were tampered with
- **Trust**: Builds trust in the codebase
- **Post-Quantum Ready**: First step towards quantum-safe cryptography

## Prerequisites

- GPG (GnuPG) 2.2+
- Git 2.0+

## Setup Instructions

### 1. Install GPG

**Linux (Debian/Ubuntu):**
```bash
sudo apt-get install gnupg
```

**Linux (Fedora):**
```bash
sudo dnf install gnupg2
```

**macOS:**
```bash
brew install gnupg
```

**Windows:**
Download from https://gpg4win.org/

### 2. Generate a GPG Key

```bash
# Generate a new key (use 4096 bits for security)
gpg --full-generate-key

# Select:
# - RSA and RSA
# - 4096 bits
# - Key expires in 1y (or longer)
# - Enter your name and email (must match Git config)
```

### 3. Get Your Key ID

```bash
gpg --list-secret-keys --keyid-format=long

# Output example:
# sec   rsa4096/3AA5C34371567BD2 2024-01-01 [SC]
#       ^^^^^^^^^^^^^^^^^^^^^^^^ This is your key ID
```

### 4. Export Your Public Key

```bash
gpg --armor --export YOUR_KEY_ID
```

### 5. Add Key to GitHub

1. Copy the public key output
2. Go to GitHub Settings > SSH and GPG keys
3. Click "New GPG key"
4. Paste your key and save

### 6. Configure Git

```bash
# Set your GPG key
git config --global user.signingkey YOUR_KEY_ID

# Enable commit signing
git config --global commit.gpgsign true

# Enable tag signing
git config --global tag.gpgsign true

# Trust your own key
gpg --edit-key YOUR_KEY_ID trust
# Select "5" (I trust ultimately) and quit
```

### 7. Test Signing

```bash
# Create a signed commit
git commit -S -m "test: verify GPG signing works"

# Verify signature
git log --show-signature
```

## Troubleshooting

### "No secret key" Error

```bash
# Check if GPG can find your key
gpg --list-secret-keys

# If empty, generate a new key (step 2)
```

### "gpg failed to sign the data"

```bash
# Add to your shell config (~/.bashrc or ~/.zshrc)
export GPG_TTY=$(tty)

# Restart your terminal
```

### macOS-specific Issues

```bash
# Install pinentry for passphrase entry
brew install pinentry-mac

# Update GPG config
echo "pinentry-program /usr/local/bin/pinentry-mac" >> ~/.gnupg/gpg-agent.conf
gpgconf --kill gpg-agent
```

## Post-Quantum Cryptography

Vantis Media Player is preparing for post-quantum cryptography:

1. **Current**: RSA-4096 or ECC (Curve25519)
2. **Planned**: Kyber-1024 + Dilithium5 hybrid signatures

To use post-quantum algorithms (experimental):

```bash
# Using Open Quantum Safe's fork
git clone https://github.com/open-quantum-safe/oqs-demos
```

## Verification in CI/CD

All commits to `main` branch are automatically verified:

- Unsigned commits will fail CI checks
- Commits with invalid signatures are rejected
- Security policy requires all maintainer commits signed

## Resources

- [GitHub GPG Guide](https://docs.github.com/en/authentication/managing-commit-signature-verification)
- [GnuPG Documentation](https://www.gnupg.org/documentation/)
- [NIST Post-Quantum Standards](https://csrc.nist.gov/projects/post-quantum-cryptography)