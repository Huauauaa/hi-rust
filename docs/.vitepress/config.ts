import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'hi-rust',
  description: 'hi-rust documentation site.',
  base: '/hi-rust/',
  cleanUrls: true,
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: '语法', link: '/guide/hello_world' },
      { text: '原理', link: '/guide/memory' }
    ],
    sidebar: {
      '/guide/': [
        {
          text: '语法',
          items: [
            { text: 'Hello, World', link: '/guide/hello_world' },
            { text: '定义变量', link: '/guide/variables' },
            { text: '字符串', link: '/guide/strings' }
          ]
        },
        {
          text: '原理',
          items: [
            { text: '内存', link: '/guide/memory' }
          ]
        }
      ]
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Huauauaa/hi-rust' }
    ]
  }
})
