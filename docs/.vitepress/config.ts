import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'hi-rust',
  description: 'hi-rust documentation site.',
  base: '/hi-rust/',
  cleanUrls: true,
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/hello_world' }
    ],
    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Hello, World', link: '/guide/hello_world' }
        ]
      }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Huauauaa/hi-rust' }
    ]
  }
})
