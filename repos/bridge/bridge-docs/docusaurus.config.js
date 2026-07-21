// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Sygma',
  tagline: '',
  url: 'https://docs.buildwithsygma.com',
  baseUrl: '/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'throw',
  favicon: 'img/favicon.ico',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'sygmaprotocol', // Usually your GitHub org/user name.
  projectName: 'docs', // Usually your epository name.

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          routeBasePath: '/',
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl:
            'https://github.com/sygmaprotocol/docs/tree/main',
        },
        blog: false,
        debug: true,
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      navbar: {
        title: 'Sygma',
        logo: {
          alt: 'Sygma Logo',
          src: 'img/logo.png',
        },
        items: [
          {
            href: 'https://github.com/sygmaprotocol/',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
          title: `Docs`,
          items: [
            {
              label: `What Is Sygma?`,
              to: `/readme/vision`,
            },
            {
              label: `Sygma SDK`,
              to: `/sdk`,
            },
            {
              label: `Environments`,
              to: `/resources/environments`,
            },
            {
              label: `Audits`,
              to: `/resources/audits`,
            },
          ],
        },
        {
          title: `Community`,
          items: [
          {
            label: `Twitter`,
            href: `https://twitter.com/buildwithsygma`,
          },
          {
            label: `Discord`,
            href: `https://discord.gg/Qdf6GyNB5J`,
          },
          {
            label: `Telegram`,
            href: `https://t.me/buildwithsygma`,
          },
        ]
        },
        {
          title: `More`,
          items: [
          {
            label: `Website`,
            href: `https://buildwithsygma.com`,
          },
          {
            label: `Blog`,
            href: `https://blog.buildwithsygma.com`,
          },
          {
            label: `YouTube`,
            href: `https://youtube.com/@buildwithsygma`,
          }
        ]
          }
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Sygma. Built with Docusaurus.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
    }),
    plugins: [
      [
        require.resolve("@cmfcmf/docusaurus-search-local"),
        {
          indexBlog: false,
          indexPages: false,
          indexDocs: true,
        },
      ],
    ],
};

module.exports = config;
