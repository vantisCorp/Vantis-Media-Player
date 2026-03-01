# ⚖️ Legal Information for Vantis Media Player

This document contains legal information, licenses, and compliance details for Vantis Media Player.

## Table of Contents

1. [License](#license)
2. [Third-Party Licenses](#third-party-licenses)
3. [Patent Notice](#patent-notice)
4. [Trademark Notice](#trademark-notice)
5. [Privacy Policy](#privacy-policy)
6. [Terms of Service](#terms-of-service)
7. [Compliance](#compliance)
8. [Contributor License Agreement](#contributor-license-agreement)
9. [Export Control](#export-control)
10. [Contact](#contact)

## License

### MIT License

Copyright (c) 2024 Vantis OS Team

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

### License Summary

- ✅ **Commercial Use**: You can use Vantis Media Player for commercial purposes
- ✅ **Modification**: You can modify the source code
- ✅ **Distribution**: You can distribute the software
- ✅ **Private Use**: You can use the software privately
- ⚠️ **Liability**: The software is provided "as is" without warranty
- ⚠️ **Attribution**: The copyright notice must be included

## Third-Party Licenses

Vantis Media Player uses the following third-party libraries and components:

### Core Dependencies

#### Rust Crates

| Crate | License | Version |
|-------|---------|---------|
| tokio | MIT | 1.35.0 |
| async-std | Apache-2.0/MIT | 1.12.0 |
| wgpu | Apache-2.0/MIT | 0.17.0 |
| symphonia | MPL-2.0 | 0.5.3 |
| cpal | Apache-2.0/MIT | 0.15.2 |
| wasmtime | Apache-2.0 | 14.0.0 |
| iced | MIT | 0.10.0 |
| serde | MIT/Apache-2.0 | 1.0.193 |
| toml | MIT/Apache-2.0 | 0.8.8 |
| reqwest | MIT/Apache-2.0 | 0.11.22 |
| hyper | MIT/Apache-2.0 | 0.14.27 |
| log | MIT/Apache-2.0 | 0.4.20 |
| env_logger | MIT/Apache-2.0 | 0.10.1 |

#### System Libraries

| Library | License | Purpose |
|---------|---------|---------|
| FFmpeg | GPL-2.0/LGPL-2.1 | Video/audio decoding |
| OpenSSL | Apache-2.0 | Cryptography |
| Vulkan | Apache-2.0 | GPU rendering |
| DirectX | Microsoft EULA | GPU rendering (Windows) |
| Metal | Apple EULA | GPU rendering (macOS) |

### License Compatibility

All third-party licenses are compatible with the MIT license. The following licenses are used:

- **MIT**: Permissive, compatible with commercial use
- **Apache-2.0**: Permissive, compatible with commercial use
- **MPL-2.0**: Weak copyleft, compatible with commercial use
- **GPL-2.0/LGPL-2.1**: Copyleft, used for FFmpeg (linked dynamically)

### Dynamic Linking

FFmpeg is linked dynamically to comply with GPL requirements. Users can replace FFmpeg with their own version if needed.

## Patent Notice

### Patent Grant

To the extent that any patent rights are held by Vantis OS Team, we grant a non-exclusive, worldwide, royalty-free patent license under our patent rights to use, modify, and distribute Vantis Media Player.

### Patent Retaliation

If you initiate patent litigation against Vantis OS Team or any contributor alleging that Vantis Media Player infringes your patents, any patent licenses granted to you shall terminate.

### No Warranty

Vantis OS Team makes no representations or warranties regarding patent infringement. Users are responsible for ensuring their use complies with applicable patent laws.

## Trademark Notice

### Trademarks

The following are trademarks of Vantis OS Team:

- **Vantis Media Player**
- **Vantis**
- **Vantis OS**
- **Vantis Babel**
- **Liquid Glass**

### Trademark Usage

You may use the Vantis trademarks to:
- Refer to Vantis Media Player in factual descriptions
- Indicate compatibility with Vantis Media Player
- Link to the official Vantis Media Player website

You may NOT use the Vantis trademarks to:
- Imply endorsement or sponsorship
- Create confusion about the source of your product
- Use in a way that dilutes the trademark

### Logo Usage

The Vantis logo is a trademark and may only be used with permission. Contact trademarks@vantis-os.org for permission.

## Privacy Policy

### Data Collection

Vantis Media Player does NOT collect:
- Personal information
- Usage statistics
- Viewing history
- Media library contents
- Location data

### Optional Data

With user consent, Vantis Media Player may collect:
- Anonymous crash reports
- Performance metrics
- Feature usage statistics

### Data Storage

All data is stored locally on the user's device:
- Configuration files: `~/.vantis/config.toml`
- Cache files: `~/.vantis/cache/`
- Logs: `~/.vantis/logs/`

### Data Sharing

Vantis Media Player does NOT share user data with third parties, except:
- Crash reports (with user consent)
- Optional analytics (with user consent)

### Data Retention

- Crash reports: Retained for 30 days
- Analytics: Retained for 90 days
- Local data: Retained until user deletes it

### User Rights

Users have the right to:
- Access their data
- Delete their data
- Opt-out of data collection
- Export their data

### GDPR Compliance

Vantis Media Player is designed to comply with GDPR:
- Minimal data collection
- Clear consent mechanisms
- Data portability
- Right to deletion
- Data protection by design

## Terms of Service

### Acceptance of Terms

By using Vantis Media Player, you agree to these Terms of Service.

### Permitted Uses

You may:
- Use Vantis Media Player for personal or commercial purposes
- Modify the source code
- Distribute modified versions (with attribution)
- Create plugins and extensions

### Prohibited Uses

You may NOT:
- Remove copyright notices
- Claim ownership of Vantis Media Player
- Use Vantis Media Player for illegal purposes
- Distribute malware or harmful code
- Violate applicable laws or regulations

### Disclaimer of Warranties

Vantis Media Player is provided "AS IS" without warranties of any kind, express or implied, including but not limited to warranties of merchantability, fitness for a particular purpose, and non-infringement.

### Limitation of Liability

In no event shall Vantis OS Team be liable for any indirect, incidental, special, consequential, or punitive damages, including without limitation, loss of profits, data, use, goodwill, or other intangible losses.

### Indemnification

You agree to indemnify and hold harmless Vantis OS Team from any claims arising from your use of Vantis Media Player.

### Termination

These Terms of Service continue until terminated. Your rights under these Terms will terminate automatically if you fail to comply with any term.

### Governing Law

These Terms of Service are governed by the laws of the jurisdiction where Vantis OS Team is located.

## Compliance

### Export Control

Vantis Media Player may be subject to export control laws. Users are responsible for complying with applicable export regulations.

### Encryption

Vantis Media Player uses encryption for network communications (TLS 1.3). Users are responsible for complying with local encryption laws.

### Accessibility

Vantis Media Player is designed to comply with WCAG 2.1 AA accessibility standards.

### Security

Vantis Media Player implements security best practices:
- Memory safety (Rust)
- WASM sandbox for plugins
- Input validation
- Secure by default

## Contributor License Agreement (CLA)

### Grant of License

By contributing to Vantis Media Player, you grant Vantis OS Team a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable copyright license to reproduce, prepare derivative works of, publicly display, publicly perform, sublicense, and distribute your contributions.

### Grant of Patent License

You also grant a patent license to Vantis OS Team to make, have made, use, sell, offer for sale, import, and otherwise transfer your contributions.

### Representations

You represent that:
- You have the legal right to make the contribution
- Your contribution does not violate any third-party rights
- Your contribution is not subject to any license restrictions

### No Obligation

Vantis OS Team is under no obligation to use your contribution.

## Export Control

### Export Classification

Vantis Media Player is classified as:
- **ECCN**: 5D002 (software)
- **License Exception**: TSU (technology and software under unrestricted)

### Restricted Countries

Vantis Media Player may not be exported to:
- Cuba
- Iran
- North Korea
- Sudan
- Syria
- Crimea region of Ukraine

### Compliance

Users are responsible for ensuring their use complies with applicable export control laws.

## Contact

### Legal Inquiries

For legal inquiries, contact:
- **Email**: legal@vantis-os.org
- **Address**: [Vantis OS Team Address]

### Trademark Inquiries

For trademark inquiries, contact:
- **Email**: trademarks@vantis-os.org

### Privacy Inquiries

For privacy inquiries, contact:
- **Email**: privacy@vantis-os.org

### Security Inquiries

For security inquiries, contact:
- **Email**: security@vantis-os.org

### General Inquiries

For general inquiries, contact:
- **Email**: info@vantis-os.org
- **Website**: https://vantis-os.org

## Updates

This legal information may be updated from time to time. Users will be notified of significant changes.

---

## Disclaimer

This document is provided for informational purposes only and does not constitute legal advice. Consult with a qualified attorney for specific legal advice.

**Last Updated**: January 2024