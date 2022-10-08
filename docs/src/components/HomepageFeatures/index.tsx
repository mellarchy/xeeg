import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Easy to Use',
    Svg: require('@site/static/img/drew_3.svg').default,
    description: (
      <>
        Zeeg was designed from the ground up to be easily configured and
        used to dynamically generate static files for your projects.
      </>
    ),
  },
  {
    title: 'Focus on What Matters',
    Svg: require('@site/static/img/drew_2.svg').default,
    description: (
      <>
        Much too often, package/framework developers scare away end-user developers with their bulky
				syntax. By integrating Xeeg into your source, you enjoy the benefit of an awesome markup file generator
				which can be customised to fit your brand.
      </>
    ),
  },
  {
    title: 'Cross Platform',
    Svg: require('@site/static/img/drew_1.svg').default,
    description: (
      <>
        Extensible to all platforms, Xeeg can be packaged and executed anywhere.
				Xeeg aims to remove OS and code stack barriers when it comes to template file
				generation.
      </>
    ),
  },
];

function Feature({title, Svg, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" style={{height: '120px', marginBottom: 20}} />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
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
