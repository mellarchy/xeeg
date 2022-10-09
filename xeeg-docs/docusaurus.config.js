// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
	title: 'Xeeg - Template File Generator',
	tagline: 'An advanced and universal template file generator that puts you in charge',
	url: 'https://mellarchy.github.io',
	baseUrl: '/xeeg/',
	onBrokenLinks: 'throw',
	onBrokenMarkdownLinks: 'warn',
	favicon: 'favicon.ico',

	// GitHub pages deployment config.
	// If you aren't using GitHub pages, you don't need these.
	organizationName: 'Mellarchy', // Usually your GitHub org/user name.
	projectName: 'xeeg', // Usually your repo name.

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
					sidebarPath: require.resolve('./sidebars.js'),
					// Please change this to your repo.
					// Remove this to remove the "edit this page" links.
					editUrl:
					'https://github.com/mellarchy/xeeg/website/',
				},
				theme: {
					customCss: require.resolve('./src/css/custom.css'),
				},
				sitemap: {
					changefreq: 'weekly',
					priority: 0.5,
					ignorePatterns: ['/tags/**'],
					filename: 'sitemap.xml',
				},
			}),
		],
	],

	themeConfig:
	/** @type {import('@docusaurus/preset-classic').ThemeConfig} */
	({
		navbar: {
			title: 'Xeeg',
			logo: {
				alt: 'Xeeg Logo',
				src: 'img/xeeg-logo.png',
			},
			items: [
				{
					type: 'doc',
					docId: 'intro',
					position: 'left',
					label: 'Quick Start',
				},
				{
					href: 'https://github.com/mellarchy/xeeg/tree/main/xeeg-cli/TODO.md',
					position: 'left',
					label: 'Early TODOs',
				},
				{
					href: 'https://github.com/mellarchy/xeeg',
					label: 'GitHub',
					position: 'right',
				},
			],
		},
		footer: {
			style: 'dark',
			links: [
				{
					title: 'Docs',
					items: [
						{
							label: 'Quick Start',
							to: '/docs/intro',
						},
						{
							label: 'License',
							to: '/docs/extras/license',
						},
						{
							label: 'Downloads',
							to: '/docs/extras/downloads',
						},
						{
							label: 'Credits',
							to: '/docs/extras/credits',
						},
					],
				},
				{
					title: 'Community',
					items: [
						{
							label: 'Stack Overflow',
							href: 'https://stackoverflow.com/questions/tagged/xeeg',
						},
						// {
						// 	label: 'Discord',
						// 	href: 'https://discordapp.com/invite/xeeg',
						// },
						// {
							//   label: 'Twitter',
							//   href: 'https://twitter.com/docusaurus',
							// },
						],
					},
					{
						title: 'More',
						items: [
							{
								label: 'GitHub',
								href: 'https://github.com/mellarchy/xeeg',
							},
						],
					},
				],
				copyright: `Copyright © ${new Date().getFullYear()} Mellarchy.`,
			},
			prism: {
				theme: lightCodeTheme,
				darkTheme: darkCodeTheme,
			},
		}),
	};

	module.exports = config;
