import { defineConfig } from 'vitepress'

const syntaxItems = [
  { text: 'Hello, World', link: '/syntax/hello_world' },
  { text: '定义变量', link: '/syntax/variables' },
  { text: '变量绑定', link: '/syntax/bindings' },
  { text: '元组', link: '/syntax/tuples' },
  { text: '数组', link: '/syntax/arrays' },
  { text: '字符串', link: '/syntax/strings' },
  { text: '类型系统', link: '/syntax/type_system' },
  { text: '类型转换', link: '/syntax/type_conversion' },
  { text: '所有权和移动', link: '/syntax/ownership' },
  { text: '借用', link: '/syntax/borrowing' },
  { text: '生命周期', link: '/syntax/lifetimes' },
  { text: '智能指针', link: '/syntax/smart_pointers' },
  { text: '切片', link: '/syntax/slices' },
  { text: '结构体', link: '/syntax/structs' },
  { text: '枚举', link: '/syntax/enums' },
  { text: 'Debug', link: '/syntax/debug' },
  { text: '自定义 Debug', link: '/syntax/debug_custom' },
  { text: 'Display', link: '/syntax/display' },
  { text: '错误处理', link: '/syntax/error_handling' },
  { text: '集合', link: '/syntax/collections' },
  { text: '迭代器', link: '/syntax/iterators' },
  { text: '泛型', link: '/syntax/generics' },
  { text: '运算符', link: '/syntax/operators' },
  { text: '条件判断', link: '/syntax/conditionals' },
  { text: 'match 匹配', link: '/syntax/match_patterns' },
  { text: 'if let 和 while let', link: '/syntax/if_while_let' },
  { text: '循环', link: '/syntax/loops' },
  { text: '函数', link: '/syntax/functions' },
  { text: '闭包', link: '/syntax/closures' },
  { text: '闭包的使用', link: '/syntax/closure_usage' },
  { text: '多线程', link: '/syntax/threads' },
  { text: 'async / await', link: '/syntax/async_await' },
  { text: 'IO', link: '/syntax/io' },
  { text: '文件操作', link: '/syntax/file_ops' },
  { text: '包管理', link: '/syntax/package_management' },
  { text: '模块', link: '/syntax/modules' }
]

const specItems = [{ text: '命名规范', link: '/spec/naming' }]

const qaItems = [
  { text: '内存', link: '/qa/memory' },
  { text: '函数与宏的区别', link: '/qa/function_vs_macro' },
  { text: '单引号与双引号', link: '/qa/single_double_quotes' },
  { text: 'ref 解构有什么用', link: '/qa/ref_destructuring' }
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
