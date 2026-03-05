/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each docs of a group
 - provide on/next navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  // Main documentation sidebar
  docsSidebar: [
    {
      type: 'doc',
      id: 'getting-started/introduction',
      label: 'Introduction',
    },
    {
      type: 'category',
      label: 'Getting Started',
      collapsed: false,
      collapsible: true,
      items: [
        'getting-started/introduction',
        'getting-started/installation',
        'getting-started/quickstart',
        'getting-started/configuration',
        'getting-started/first-project',
      ],
    },
    {
      type: 'category',
      label: 'Core Features',
      collapsed: false,
      collapsible: true,
      items: [
        'core-features/video-playback',
        'core-features/audio-playback',
        'core-features/subtitles',
        'core-features/playlists',
        'core-features/media-library',
      ],
    },
    {
      type: 'category',
      label: 'Advanced Features',
      collapsed: true,
      collapsible: true,
      items: [
        'advanced-features/hardware-acceleration',
        'advanced-features/streaming',
        'advanced-features/plugins',
        'advanced-features/custom-ui',
        'advanced-features/performance',
      ],
    },
    {
      type: 'category',
      label: 'API Reference',
      collapsed: true,
      collapsible: true,
      items: [
        'api/overview',
        'api/player',
        'api/audio',
        'api/video',
        'api/subtitles',
        'api/events',
        'api/plugins',
      ],
    },
    {
      type: 'category',
      label: 'Plugin Development',
      collapsed: true,
      collapsible: true,
      items: [
        'plugins/introduction',
        'plugins/getting-started',
        'plugins/plugin-api',
        'plugins/wasm-plugins',
        'plugins/native-plugins',
        'plugins/examples',
      ],
    },
    {
      type: 'category',
      label: 'Examples',
      collapsed: true,
      collapsible: true,
      items: [
        'examples/simple-player',
        'examples/advanced-player',
        'examples/custom-controls',
        'examples/plugin-development',
        'examples/integration',
      ],
    },
    {
      type: 'category',
      label: 'Architecture',
      collapsed: true,
      collapsible: true,
      items: [
        'architecture/overview',
        'architecture/components',
        'architecture/event-system',
        'architecture/state-management',
        'architecture/plugin-system',
      ],
    },
    {
      type: 'category',
      label: 'Deployment',
      collapsed: true,
      collapsible: true,
      items: [
        'deployment/desktop',
        'deployment/web',
        'deployment/mobile',
        'deployment/docker',
        'deployment/cloud',
      ],
    },
    {
      type: 'category',
      label: 'Development',
      collapsed: true,
      collapsible: true,
      items: [
        'development/setup',
        'development/building',
        'development/testing',
        'development/debugging',
        'development/contributing',
      ],
    },
    {
      type: 'category',
      label: 'Reference',
      collapsed: true,
      collapsible: true,
      items: [
        'reference/glossary',
        'reference/troubleshooting',
        'reference/faq',
        'reference/changelog',
        'reference/roadmap',
        'reference/license',
        'reference/security',
      ],
    },
  ],
};

module.exports = sidebars;