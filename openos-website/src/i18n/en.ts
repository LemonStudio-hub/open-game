export default {
  // Navigation
  nav: {
    home: 'Home',
    features: 'Features',
    architecture: 'Architecture',
    code: 'Code',
    docs: 'Documentation',
    github: 'GitHub',
  },

  // Hero Section
  hero: {
    title: 'OpenOS',
    subtitle: 'A Modern Operating System for the Future',
    description: 'Built with Rust, designed for performance, security, and developer experience. OpenOS is the next generation operating system that combines modern architecture with elegant design.',
    getStarted: 'Get Started',
    viewGithub: 'View on GitHub',
    terminal: {
      booting: 'Booting OpenOS...',
      loading: 'Loading kernel modules...',
      ready: 'System ready.',
      welcome: 'Welcome to OpenOS',
    },
  },

  // Features Section
  features: {
    title: 'Why OpenOS?',
    subtitle: 'Built for the modern era with cutting-edge technology',
    items: [
      {
        title: 'Blazing Fast',
        description: 'Optimized Rust-based kernel with zero-cost abstractions and minimal overhead.',
        icon: 'bolt',
      },
      {
        title: 'Secure by Design',
        description: 'Memory safety guaranteed at compile time. No buffer overflows, no data races.',
        icon: 'shield',
      },
      {
        title: 'Modern Architecture',
        description: 'Microkernel design with message-passing IPC and modular driver framework.',
        icon: 'cpu',
      },
      {
        title: 'Developer Friendly',
        description: 'Comprehensive tooling, excellent documentation, and a welcoming community.',
        icon: 'code',
      },
      {
        title: 'Cross Platform',
        description: 'Run on x86_64, ARM64, and RISC-V architectures with consistent behavior.',
        icon: 'globe',
      },
      {
        title: 'Open Source',
        description: 'MIT licensed. Contribute, fork, and build upon OpenOS freely.',
        icon: 'heart',
      },
    ],
  },

  // Architecture Section
  architecture: {
    title: 'Architecture',
    subtitle: 'A clean, modular design that scales',
    description: 'OpenOS uses a microkernel architecture that separates core services into isolated modules, providing better security, reliability, and maintainability.',
    layers: [
      {
        name: 'User Applications',
        description: 'Your applications run in isolated sandboxes',
      },
      {
        name: 'System Services',
        description: 'File system, networking, and device management',
      },
      {
        name: 'Kernel Core',
        description: 'Memory management, scheduling, and IPC',
      },
      {
        name: 'Hardware Abstraction',
        description: 'Unified interface for diverse hardware',
      },
    ],
  },

  // Code Example Section
  code: {
    title: 'Simple & Powerful',
    subtitle: 'Write system code with confidence',
    description: 'OpenOS provides clean, safe APIs that make systems programming accessible without sacrificing performance.',
    examples: [
      {
        title: 'Hello World',
        language: 'rust',
        code: `use openos::prelude::*;

fn main() {
    println!("Hello from OpenOS!");

    // Access system services safely
    let fs = FileSystem::mount("/dev/sda1")?;
    let contents = fs.read_to_string("/etc/hostname")?;
    println!("Hostname: {}", contents);
}`,
      },
      {
        title: 'Process Management',
        language: 'rust',
        code: `use openos::process::{Process, Priority};

fn spawn_workers() -> Result<()> {
    // Spawn processes with isolated memory
    let worker = Process::new("worker")?
        .priority(Priority::High)
        .memory_limit(256 * 1024 * 1024)
        .spawn()?;

    // Communicate via message passing
    worker.send(Message::Data(payload))?;
    let response = worker.recv()?;

    Ok(())
}`,
      },
    ],
  },

  // Footer
  footer: {
    description: 'Building the future of operating systems, one commit at a time.',
    links: {
      product: {
        title: 'Product',
        items: [
          { label: 'Features', href: '#features' },
          { label: 'Architecture', href: '#architecture' },
          { label: 'Roadmap', href: '#' },
          { label: 'Changelog', href: '#' },
        ],
      },
      resources: {
        title: 'Resources',
        items: [
          { label: 'Documentation', href: '#' },
          { label: 'API Reference', href: '#' },
          { label: 'Tutorials', href: '#' },
          { label: 'Blog', href: '#' },
        ],
      },
      community: {
        title: 'Community',
        items: [
          { label: 'GitHub', href: 'https://github.com' },
          { label: 'Discord', href: '#' },
          { label: 'Forum', href: '#' },
          { label: 'Twitter', href: '#' },
        ],
      },
    },
    copyright: '© 2024 OpenOS. All rights reserved.',
    madeWith: 'Made with ❤️ by the OpenOS Team',
  },
} as const;
