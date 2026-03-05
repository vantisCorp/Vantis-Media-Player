const config = {
  title: 'Vantis Media Player',
  tagline: 'The World\'s Most Advanced Media Player',
  favicon: 'img/favicon.ico',

  url: 'https://docs.vantis.media',
  baseUrl: '/',

  organizationName: 'vantisCorp',
  projectName: 'VantisMedia',

  onBrokenLinks: 'warn',
  onBrokenMarkdownLinks: 'warn',

  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'pl', 'de', 'zh', 'ru', 'ko', 'es', 'fr'],
  },

  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          path: 'content/docs',
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://github.com/vantisCorp/VantisMedia/tree/main/docs/',
          routeBasePath: 'docs',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],

  themeConfig: {
    navbar: {
      title: 'Vantis Media Player',
      logo: {
        alt: 'Vantis Logo',
        src: 'img/logo.svg',
      },
      items: [
        {to: '/docs/introduction', label: 'Docs', position: 'left'},
        {to: '/docs/api-reference/overview', label: 'API', position: 'left'},
        {href: 'https://github.com/vantisCorp/VantisMedia', label: 'GitHub', position: 'right'},
      ],
    },
    footer: {
      style: 'dark',
      copyright: `Copyright © ${new Date().getFullYear()} Vantis Media Player.`,
    },
    prism: {
      theme: require('prism-react-renderer').themes.github,
      darkTheme: require('prism-react-renderer').themes.dracula,
    },
  },
};

module.exports = config;
