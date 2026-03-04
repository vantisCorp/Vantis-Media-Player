// @ts-check
// `@type` annotation is closed for contributions
// This is the main Docusaurus configuration file for Vantis Media Player

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Vantis Media Player',
  tagline: 'The Last Interface - The World\'s Most Advanced Media Player',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://docs.vantis.media',
  // Set the /<baseUrl>/ pathname under which your site is served
  baseUrl: '/',

  // GitHub pages deployment config
  organizationName: 'vantisCorp',
  projectName: 'VantisMedia',

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'pl', 'de', 'zh', 'ru', 'ko', 'es', 'fr'],
    localeConfigs: {
      en: {
        label: 'English',
        direction: 'ltr',
        htmlLang: 'en-US',
      },
      pl: {
        label: 'Polski',
        direction: 'ltr',
        htmlLang: 'pl-PL',
      },
      de: {
        label: 'Deutsch',
        direction: 'ltr',
        htmlLang: 'de-DE',
      },
      zh: {
        label: '中文',
        direction: 'ltr',
        htmlLang: 'zh-CN',
      },
      ru: {
        label: 'Русский',
        direction: 'ltr',
        htmlLang: 'ru-RU',
      },
      ko: {
        label: '한국어',
        direction: 'ltr',
        htmlLang: 'ko-KR',
      },
      es: {
        label: 'Español',
        direction: 'ltr',
        htmlLang: 'es-ES',
      },
      fr: {
        label: 'Français',
        direction: 'ltr',
        htmlLang: 'fr-FR',
      },
    },
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo
          editUrl: 'https://github.com/vantisCorp/VantisMedia/tree/main/docs/',
          routeBasePath: 'docs',
          showLastUpdateAuthor: true,
          showLastUpdateTime: true,
        },
        blog: {
          showReadingTime: true,
          editUrl: 'https://github.com/vantisCorp/VantisMedia/tree/main/docs/blog/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
        gtag: {
          trackingID: 'G-XXXXXXXXXX',
          anonymizeIP: true,
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/social-card.png',
      navbar: {
        title: 'Vantis Media Player',
        logo: {
          alt: 'Vantis Media Player Logo',
          src: 'img/logo.svg',
          srcDark: 'img/logo-dark.svg',
        },
        items: [
          {
            type: 'doc',
            docId: 'getting-started/introduction',
            position: 'left',
            label: 'Documentation',
          },
          {
            to: '/api-reference/',
            label: 'API Reference',
            position: 'left',
          },
          {
            to: '/examples/',
            label: 'Examples',
            position: 'left',
          },
          {
            to: '/plugins/',
            label: 'Plugins',
            position: 'left',
          },
          {
            to: '/blog',
            label: 'Blog',
            position: 'left',
          },
          {
            type: 'docsVersionDropdown',
            position: 'right',
            dropdownActiveClassDisabled: true,
          },
          {
            type: 'localeDropdown',
            position: 'right',
          },
          {
            href: 'https://github.com/vantisCorp/VantisMedia',
            position: 'right',
            className: 'header-github-link',
            'aria-label': 'GitHub repository',
          },
          {
            href: 'https://discord.gg/A5MzwsRj7D',
            position: 'right',
            className: 'header-discord-link',
            'aria-label': 'Discord community',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Documentation',
            items: [
              {
                label: 'Getting Started',
                to: '/docs/getting-started/introduction',
              },
              {
                label: 'API Reference',
                to: '/api-reference/',
              },
              {
                label: 'Examples',
                to: '/examples/',
              },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'Discord',
                href: 'https://discord.gg/A5MzwsRj7D',
              },
              {
                label: 'Twitter',
                href: 'https://twitter.com/VantisMedia',
              },
              {
                label: 'Reddit',
                href: 'https://reddit.com/r/VantisMedia',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'Blog',
                to: '/blog',
              },
              {
                label: 'GitHub',
                href: 'https://github.com/vantisCorp/VantisMedia',
              },
              {
                label: 'Roadmap',
                to: '/docs/roadmap',
              },
            ],
          },
          {
            title: 'Legal',
            items: [
              {
                label: 'Privacy Policy',
                to: '/privacy',
              },
              {
                label: 'Terms of Service',
                to: '/terms',
              },
              {
                label: 'Security',
                to: '/security',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Vantis Media Player. Built with Docusaurus.`,
      },
      prism: {
        theme: require('prism-react-renderer/themes/github'),
        darkTheme: require('prism-react-renderer/themes/dracula'),
        additionalLanguages: ['rust', 'toml', 'bash', 'json', 'yaml', 'markdown', 'typescript', 'javascript', 'python', 'go', 'java', 'kotlin', 'swift'],
      },
      algolia: {
        appId: 'YOUR_APP_ID',
        apiKey: 'YOUR_API_KEY',
        indexName: 'vantis-media-player',
        contextualSearch: true,
      },
      announcementBar: {
        id: 'support_us',
        content: '⭐️ If you like Vantis Media Player, give it a star on <a target="_blank" rel="noopener noreferrer" href="https://github.com/vantisCorp/VantisMedia">GitHub</a>! ⭐️',
        backgroundColor: '#fafbfc',
        textColor: '#091E42',
        isCloseable: true,
      },
      colorMode: {
        defaultMode: 'dark',
        disableSwitch: false,
        respectPrefersColorScheme: true,
      },
      metadata: [
        { name: 'keywords', content: 'media player, video player, audio player, rust, ffmpeg, open source' },
        { name: 'description', content: 'Vantis Media Player - The Last Interface - The world\'s most advanced media player' },
        { name: 'twitter:card', content: 'summary_large_image' },
        { name: 'twitter:site', content: '@VantisMedia' },
        { name: 'twitter:title', content: 'Vantis Media Player' },
        { name: 'twitter:description', content: 'The Last Interface - The world\'s most advanced media player' },
        { property: 'og:title', content: 'Vantis Media Player' },
        { property: 'og:description', content: 'The Last Interface - The world\'s most advanced media player' },
        { property: 'og:type', content: 'website' },
        { property: 'og:url', content: 'https://docs.vantis.media' },
      ],
    }),

  plugins: [
    [
      '@docusaurus/plugin-pwa',
      {
        debug: true,
        offlineModeActivationStrategies: [
          'appInstalled',
          'standalone',
          'queryString',
        ],
        pwaHead: [
          {
            tagName: 'link',
            rel: 'icon',
            href: '/img/favicon.ico',
          },
          {
            tagName: 'link',
            rel: 'manifest',
            href: '/manifest.json',
          },
          {
            tagName: 'meta',
            name: 'theme-color',
            content: 'rgb(37, 194, 160)',
          },
          {
            tagName: 'meta',
            name: 'apple-mobile-web-app-capable',
            content: 'yes',
          },
          {
            tagName: 'meta',
            name: 'apple-mobile-web-app-status-bar-style',
            content: '#000',
          },
          {
            tagName: 'link',
            rel: 'apple-touch-icon',
            href: '/img/logo.png',
          },
          {
            tagName: 'meta',
            name: 'msapplication-TileImage',
            content: '/img/logo.png',
          },
          {
            tagName: 'meta',
            name: 'msapplication-TileColor',
            content: '#000',
          },
        ],
      },
    ],
    [
      '@docusaurus/plugin-content-docs',
      {
        id: 'api',
        path: 'api',
        routeBasePath: 'api-reference',
        sidebarPath: require.resolve('./sidebarsApi.js'),
      },
    ],
    [
      '@docusaurus/plugin-ideal-image',
      {
        quality: 70,
        max: 1030,
        min: 640,
        steps: 2,
        disableInDev: false,
      },
    ],
    [
      'docusaurus-plugin-typedoc',
      {
        entryPoints: ['../src/lib.rs'],
        tsconfig: '../tsconfig.json',
        readme: 'none',
      },
    ],
    require.resolve('docusaurus-plugin-image-zoom'),
    [
      '@orama/plugin-docusaurus',
      {
        oramaEndpoint: 'https://cloud.orama.run',
        oramaApiKey: 'YOUR_API_KEY',
      },
    ],
  ],

  themes: [
    [
      require.resolve('@easyops-cn/docusaurus-search-local'),
      {
        hashed: true,
        language: ['en', 'pl', 'de', 'zh', 'ru', 'ko', 'es', 'fr'],
        indexBlog: true,
        indexDocs: true,
        indexPages: true,
      },
    ],
  ],

  markdown: {
    mermaid: true,
  },

  themes: ['@docusaurus/theme-mermaid'],

  customFields: {
    discordUrl: 'https://discord.gg/A5MzwsRj7D',
    githubUrl: 'https://github.com/vantisCorp/VantisMedia',
    twitterUrl: 'https://twitter.com/VantisMedia',
    redditUrl: 'https://reddit.com/r/VantisMedia',
  },
};

module.exports = config;