/**
 * API Reference sidebar configuration for Vantis Media Player
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebarsApi = {
  apiSidebar: [
    {
      type: 'doc',
      id: 'overview',
      label: 'API Overview',
    },
    {
      type: 'category',
      label: 'Core API',
      collapsed: false,
      collapsible: true,
      items: [
        'player',
        'video',
        'audio',
        'subtitles',
      ],
    },
    {
      type: 'category',
      label: 'Events & Plugins',
      collapsed: true,
      collapsible: true,
      items: [
        'events',
        'plugins',
      ],
    },
  ],
};

module.exports = sidebarsApi;