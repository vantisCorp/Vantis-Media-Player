import React from 'react';
import Layout from '@theme/Layout';

export default function PrivacyPolicy() {
  return (
    <Layout title="Privacy Policy" description="Vantis Media Player Privacy Policy">
      <main>
        <div className="container margin-vert--xl">
          <h1>Privacy Policy</h1>
          <p>Last updated: January 2024</p>
          
          <h2>Information We Collect</h2>
          <p>Vantis Media Player collects minimal information necessary to provide and improve our services:</p>
          <ul>
            <li>Usage analytics (optional, with your consent)</li>
            <li>Error reports and crash data (optional, with your consent)</li>
            <li>Performance metrics (optional, with your consent)</li>
          </ul>

          <h2>How We Use Your Information</h2>
          <p>We use collected information to:</p>
          <ul>
            <li>Improve product performance and user experience</li>
            <li>Fix bugs and issues</li>
            <li>Develop new features</li>
          </ul>

          <h2>Data Protection</h2>
          <p>Vantis Media Player is committed to protecting your privacy:</p>
          <ul>
            <li>No personal data is collected without your explicit consent</li>
            <li>All data collection is opt-in</li>
            <li>We do not sell your data to third parties</li>
            <li>We follow industry-standard security practices</li>
          </ul>

          <h2>Your Rights</h2>
          <p>You have the right to:</p>
          <ul>
            <li>Opt out of data collection at any time</li>
            <li>Request deletion of your data</li>
            <li>Access data we have collected about you</li>
            <li>Export your data</li>
          </ul>

          <h2>COPPA Compliance</h2>
          <p>Vantis Media Player offers a COPPA-compliant mode that disables all data collection and tracking for applications targeting children under 13.</p>

          <h2>GDPR Compliance</h2>
          <p>Vantis Media Player is designed to help you comply with GDPR requirements, including:</p>
          <ul>
            <li>Data minimization</li>
            <li>Consent management</li>
            <li>Right to deletion</li>
            <li>Data portability</li>
          </ul>

          <h2>Contact Us</h2>
          <p>For privacy-related questions or concerns:</p>
          <ul>
            <li>Email: privacy@vantis.media</li>
            <li>GitHub: https://github.com/vantisCorp/VantisMedia</li>
          </ul>
        </div>
      </main>
    </Layout>
  );
}