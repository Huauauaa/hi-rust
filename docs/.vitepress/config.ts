import { defineConfig } from 'vitepress'

const sidebarItems = [
  {
    text: '语法',
    items: [
      { text: 'Hello, World', link: '/guide/hello_world' },
      { text: '定义变量', link: '/guide/variables' },
      { text: '元组', link: '/guide/tuples' },
      { text: '数组', link: '/guide/arrays' },
      { text: '字符串', link: '/guide/strings' },
      { text: '所有权和移动', link: '/guide/ownership' },
      { text: '借用', link: '/guide/borrowing' },
      { text: '切片', link: '/guide/slices' },
      { text: '结构体', link: '/guide/structs' },
      { text: '枚举', link: '/guide/enums' },
      { text: '集合', link: '/guide/collections' },
      { text: '泛型', link: '/guide/generics' },
      { text: '运算符', link: '/guide/operators' },
      { text: '条件判断', link: '/guide/conditionals' },
      { text: '循环', link: '/guide/loops' },
      { text: '函数', link: '/guide/functions' }
    ]
  },
  {
    text: '规范',
    items: [
      { text: '命名规范', link: '/spec/naming' }
    ]
  },
  {
    text: 'Q&A',
    items: [
      { text: '内存', link: '/guide/memory' },
      { text: '函数与宏的区别', link: '/guide/function_vs_macro' },
      { text: '单引号与双引号', link: '/guide/single_double_quotes' }
    ]
  }
]

export default defineConfig({
  title: 'hi-rust',
  description: 'hi-rust documentation site.',
  base: '/hi-rust/',
  cleanUrls: true,
  themeConfig: {
    logo: {
      light: '/logo-light.svg',
      dark: '/logo-dark.svg',
      alt: 'hi-rust'
    },
    siteTitle: false,
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Issues', link: 'https://github.com/Huauauaa/hi-rust/issues' }
    ],
    sidebar: {
      '/guide/': sidebarItems,
      '/spec/': sidebarItems
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Huauauaa/hi-rust' }
    ]
  }
})
