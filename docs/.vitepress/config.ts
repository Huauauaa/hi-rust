import { defineConfig } from 'vitepress'

const syntaxItems = [
  { text: 'Hello, World', link: '/syntax/hello_world' },
  { text: '定义变量', link: '/syntax/variables' },
  { text: '元组', link: '/syntax/tuples' },
  { text: '数组', link: '/syntax/arrays' },
  { text: '字符串', link: '/syntax/strings' },
  { text: '所有权和移动', link: '/syntax/ownership' },
  { text: '借用', link: '/syntax/borrowing' },
  { text: '切片', link: '/syntax/slices' },
  { text: '结构体', link: '/syntax/structs' },
  { text: '枚举', link: '/syntax/enums' },
  { text: '集合', link: '/syntax/collections' },
  { text: '泛型', link: '/syntax/generics' },
  { text: '运算符', link: '/syntax/operators' },
  { text: '条件判断', link: '/syntax/conditionals' },
  { text: '循环', link: '/syntax/loops' },
  { text: '函数', link: '/syntax/functions' }
]

const specItems = [{ text: '命名规范', link: '/spec/naming' }]

const qaItems = [
  { text: '内存', link: '/qa/memory' },
  { text: '函数与宏的区别', link: '/qa/function_vs_macro' },
  { text: '单引号与双引号', link: '/qa/single_double_quotes' }
]

const sidebarItems = [
  { text: '语法', items: syntaxItems },
  { text: '规范', items: specItems },
  { text: 'Q&A', items: qaItems }
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
      { text: 'Issues', link: 'https://github.com/Huauauaa/hi-rust/issues' }
    ],
    sidebar: {
      '/syntax/': sidebarItems,
      '/spec/': sidebarItems,
      '/qa/': sidebarItems
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Huauauaa/hi-rust' }
    ]
  }
})
