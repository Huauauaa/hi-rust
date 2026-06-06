import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'hi-rust',
  description: 'hi-rust documentation site.',
  base: '/hi-rust/',
  cleanUrls: true,
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/getting-started' }
    ],
    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Getting Started', link: '/guide/getting-started' }
        ]
      }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Huauauaa/hi-rust' }
    ]
  }
})
