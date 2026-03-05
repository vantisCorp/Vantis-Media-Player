/**
 * API Reference sidebar configuration for Vantis Media Player
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebarsApi = {
  apiSidebar: [
    {
      type: 'doc',
      id: 'api/overview',
      label: 'API Overview',
    },
    {
      type: 'category',
      label: 'Core API',
      collapsed: false,
      collapsible: true,
      items: [
        'api/player',
        'api/video',
        'api/audio',
        'api/subtitles',
      ],
    },
    {
      type: 'category',
      label: 'Events & Plugins',
      collapsed: true,
      collapsible: true,
      items: [
        'api/events',
        'api/plugins',
      ],
    },
  ],
};

module.exports = sidebarsApi;