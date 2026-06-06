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
          { text: 'Hello, World', link: '/guide/hello_world' },
          { text: '定义变量', link: '/guide/variables' },
          { text: '字符串', link: '/guide/strings' }
        ]
      }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Huauauaa/hi-rust' }
    ]
  }
})
