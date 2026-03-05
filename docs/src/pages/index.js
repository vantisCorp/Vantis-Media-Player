import React from 'react';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';

export default function Home() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title={`Welcome to ${siteConfig.title}`}
      description="The World's Most Advanced Media Player">
      <main>
        <div className="hero">
          <div className="container">
            <h1 className="hero__title">{siteConfig.title}</h1>
            <p className="hero__subtitle">{siteConfig.tagline}</p>
            <div className="hero__buttons">
              <Link
                className="button button--primary button--lg"
                to="/docs/getting-started/introduction">
                Get Started
              </Link>
              <Link
                className="button button--secondary button--lg"
                to="/docs/">
                Documentation
              </Link>
            </div>
          </div>
        </div>
        <div className="container margin-vert--xl">
          <div className="row">
            <div className="col col--4">
              <h3>🚀 Fast & Efficient</h3>
              <p>Optimized for performance with hardware acceleration and efficient resource management.</p>
            </div>
            <div className="col col--4">
              <h3>🔌 Extensible Plugin System</h3>
              <p>Build custom functionality with our powerful plugin architecture supporting JavaScript, WASM, and native plugins.</p>
            </div>
            <div className="col col--4">
              <h3>🌐 Cross-Platform</h3>
              <p>Deploy to web, desktop, and mobile with a single codebase and unified API.</p>
            </div>
          </div>
        </div>
      </main>
    </Layout>
  );
}