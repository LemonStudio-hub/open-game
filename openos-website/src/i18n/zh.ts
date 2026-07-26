export default {
  // 导航
  nav: {
    home: '首页',
    features: '特性',
    architecture: '架构',
    code: '代码',
    docs: '文档',
    github: 'GitHub',
  },

  // 主视觉区域
  hero: {
    title: 'OpenOS',
    subtitle: '面向未来的现代操作系统',
    description: '基于 Rust 构建，专为性能、安全性和开发者体验而设计。OpenOS 是下一代操作系统，将现代架构与优雅设计完美结合。',
    getStarted: '快速开始',
    viewGithub: '查看 GitHub',
    terminal: {
      booting: '正在启动 OpenOS...',
      loading: '加载内核模块...',
      ready: '系统就绪。',
      welcome: '欢迎使用 OpenOS',
    },
  },

  // 特性区域
  features: {
    title: '为什么选择 OpenOS？',
    subtitle: '为现代时代打造，采用前沿技术',
    items: [
      {
        title: '极致性能',
        description: '基于 Rust 优化的内核，零成本抽象，最小化开销。',
        icon: 'bolt',
      },
      {
        title: '安全设计',
        description: '编译时保证内存安全，无缓冲区溢出，无数据竞争。',
        icon: 'shield',
      },
      {
        title: '现代架构',
        description: '微内核设计，消息传递 IPC，模块化驱动框架。',
        icon: 'cpu',
      },
      {
        title: '开发者友好',
        description: '完善的工具链，优秀的文档，友好的社区。',
        icon: 'code',
      },
      {
        title: '跨平台',
        description: '支持 x86_64、ARM64 和 RISC-V 架构，行为一致。',
        icon: 'globe',
      },
      {
        title: '开源免费',
        description: 'MIT 许可证，自由贡献、分叉和构建。',
        icon: 'heart',
      },
    ],
  },

  // 架构区域
  architecture: {
    title: '系统架构',
    subtitle: '清晰、可扩展的模块化设计',
    description: 'OpenOS 采用微内核架构，将核心服务分离为独立模块，提供更好的安全性、可靠性和可维护性。',
    layers: [
      {
        name: '用户应用层',
        description: '您的应用在隔离沙箱中运行',
      },
      {
        name: '系统服务层',
        description: '文件系统、网络和设备管理',
      },
      {
        name: '内核核心层',
        description: '内存管理、调度和进程间通信',
      },
      {
        name: '硬件抽象层',
        description: '统一的多样化硬件接口',
      },
    ],
  },

  // 代码示例区域
  code: {
    title: '简洁而强大',
    subtitle: '自信地编写系统代码',
    description: 'OpenOS 提供清晰、安全的 API，让系统编程变得简单，同时不牺牲性能。',
    examples: [
      {
        title: 'Hello World',
        language: 'rust',
        code: `use openos::prelude::*;

fn main() {
    println!("Hello from OpenOS!");

    // 安全访问系统服务
    let fs = FileSystem::mount("/dev/sda1")?;
    let contents = fs.read_to_string("/etc/hostname")?;
    println!("Hostname: {}", contents);
}`,
      },
      {
        title: '进程管理',
        language: 'rust',
        code: `use openos::process::{Process, Priority};

fn spawn_workers() -> Result<()> {
    // 使用隔离内存生成进程
    let worker = Process::new("worker")?
        .priority(Priority::High)
        .memory_limit(256 * 1024 * 1024)
        .spawn()?;

    // 通过消息传递通信
    worker.send(Message::Data(payload))?;
    let response = worker.recv()?;

    Ok(())
}`,
      },
    ],
  },

  // 页脚
  footer: {
    description: '构建操作系统未来，一次提交一个进步。',
    links: {
      product: {
        title: '产品',
        items: [
          { label: '特性', href: '#features' },
          { label: '架构', href: '#architecture' },
          { label: '路线图', href: '#' },
          { label: '更新日志', href: '#' },
        ],
      },
      resources: {
        title: '资源',
        items: [
          { label: '文档', href: '#' },
          { label: 'API 参考', href: '#' },
          { label: '教程', href: '#' },
          { label: '博客', href: '#' },
        ],
      },
      community: {
        title: '社区',
        items: [
          { label: 'GitHub', href: 'https://github.com' },
          { label: 'Discord', href: '#' },
          { label: '论坛', href: '#' },
          { label: 'Twitter', href: '#' },
        ],
      },
    },
    copyright: '© 2024 OpenOS. 保留所有权利。',
    madeWith: '由 OpenOS 团队用心打造 ❤️',
  },
} as const;
