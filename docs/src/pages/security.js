import React from 'react';
import Layout from '@theme/Layout';

export default function SecurityPage() {
  return (
    <Layout title="Security" description="Vantis Media Player Security Information">
      <main>
        <div className="container margin-vert--xl">
          <h1>Security</h1>
          
          <h2>Reporting Vulnerabilities</h2>
          <p>If you discover a security vulnerability, please report it responsibly:</p>
          <ul>
            <li>Email: security@vantis.media</li>
            <li>Do not create public issues for security vulnerabilities</li>
            <li>Include detailed information about the vulnerability</li>
            <li>Allow us time to fix the issue before disclosure</li>
          </ul>

          <h2>Security Features</h2>
          <p>Vantis Media Player includes several security features:</p>
          <ul>
            <li>HTTPS enforcement for all communications</li>
            <li>Content Security Policy (CSP) support</li>
            <li>DRM support for protected content</li>
            <li>Zero Trust Architecture principles</li>
            <li>Regular security audits</li>
          </ul>

          <h2>Best Practices</h2>
          <p>To ensure secure usage of Vantis Media Player:</p>
          <ul>
            <li>Keep dependencies up to date</li>
            <li>Use HTTPS for all video sources</li>
            <li>Implement proper authentication and authorization</li>
            <li>Validate all user inputs</li>
            <li>Use secure token-based authentication</li>
            <li>Enable CORS restrictions appropriately</li>
          </ul>

          <h2>Dependency Security</h2>
          <p>We regularly audit and update our dependencies for security vulnerabilities:</p>
          <ul>
            <li>Automated security scanning on every commit</li>
            <li>Monthly dependency audits</li>
            <li>Rapid response to known vulnerabilities</li>
            <li>Security advisories for all patches</li>
          </ul>

          <h2>Compliance</h2>
          <p>Vantis Media Player helps you comply with security standards:</p>
          <ul>
            <li>GDPR compliance tools</li>
            <li>COPPA compliance mode</li>
            <li>Security audit trails</li>
            <li>Data encryption support</li>
          </ul>

          <h2>Recent Security Updates</h2>
          <p>Check our <a href="/docs/reference/changelog">changelog</a> for recent security updates and patches.</p>

          <h2>Contact</h2>
          <p>For security-related questions or concerns:</p>
          <ul>
            <li>Security Team: security@vantis.media</li>
            <li>GitHub Security Advisories: https://github.com/vantisCorp/VantisMedia/security/advisories</li>
            <li>Discord Security Channel: https://discord.gg/vantis-security</li>
          </ul>
        </div>
      </main>
    </Layout>
  );
}