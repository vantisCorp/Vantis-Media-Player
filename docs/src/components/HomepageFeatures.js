import React from 'react';
import clsx from 'clsx';
import styles from './HomepageFeatures.module.css';

const FeatureList = [
  {
    title: 'Easy to Use',
    description: (
      <>
        Vantis Media Player was designed from the ground up to be easily installed and
        used to get your media player up and running quickly.
      </>
    ),
  },
  {
    title: 'Focus on What Matters',
    description: (
      <>
        Vantis Media Player lets you focus on your content, not the player implementation.
        Build amazing media experiences with minimal effort.
      </>
    ),
  },
  {
    title: 'Powered by Modern Technology',
    description: (
      <>
        Built with React and TypeScript, Vantis Media Player delivers 
        exceptional performance with hardware acceleration and efficient resource management.
      </>
    ),
  },
];

function Feature({ title, description }) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}