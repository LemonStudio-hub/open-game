# OpenGame Engine - 实施计划

## 项目概述

使用 Rust 开发一个功能完整的 Web 游戏引擎，编译为 WebAssembly 在浏览器中运行。引擎将提供 2D 游戏开发所需的全部核心功能，包括渲染、输入、音频、物理、ECS、场景管理等。

## 技术选型

| 模块 | 技术方案 | 说明 |
|------|---------|------|
| 渲染 | `glow` (WebGL2) | 轻量级 OpenGL/WebGL2 抽象层，适合 2D 引擎 |
| ECS | 自实现 | 基于 `generational-arena` 的自定义 ECS，教育性强 |
| 物理 | 自实现 2D 物理 | 内置 AABB/圆形碰撞检测、刚体模拟 |
| 数学 | `glam` | 高性能游戏数学库 (Vec2, Mat3, Quat) |
| 音频 | `web-sys` Web Audio API | 直接调用浏览器音频接口 |
| 输入 | `web-sys` 事件系统 | 键盘/鼠标/触摸/手柄 |
| Web桥接 | `wasm-bindgen` + `web-sys` + `js-sys` | Rust <-> JS 互操作 |
| 构建 | `trunk` | WebAssembly 构建工具 + 开发服务器 |
| 异步 | `wasm-bindgen-futures` | 将 JS Promise 桥接为 Rust Future |

## 项目结构

```
/home/dev/projects/
├── Cargo.toml                          # Workspace 根配置
├── Trunk.toml                          # Trunk 构建配置
├── index.html                          # 主入口 HTML
├── crates/
│   ├── engine/                         # 主引擎库 (cdylib + rlib)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # 引擎入口，导出所有公共模块
│   │       ├── app.rs                  # 应用框架：游戏循环、生命周期管理
│   │       ├── ecs/
│   │       │   ├── mod.rs              # ECS 总入口
│   │       │   ├── world.rs            # World：实体和组件的容器
│   │       │   ├── entity.rs           # Entity 定义和 GenerationalArena
│   │       │   ├── component.rs        # Component trait
│   │       │   ├── system.rs           # System trait 和调度器
│   │       │   ├── query.rs            # 查询系统：按组件类型查询实体
│   │       │   └── builder.rs          # EntityBuilder 模式
│   │       ├── renderer/
│   │       │   ├── mod.rs              # 渲染器总入口
│   │       │   ├── gl_backend.rs       # WebGL2 后端 (glow 封装)
│   │       │   ├── shader.rs           # 着色器编译和管理
│   │       │   ├── sprite.rs           # 精灵渲染器 (SpriteBatch)
│   │       │   ├── shape.rs            # 几何形状渲染 (矩形/圆形/线段)
│   │       │   ├── texture.rs          # 纹理加载和管理
│   │       │   ├── camera.rs           # 2D 相机 (正交投影、缩放、旋转)
│   │       │   ├── color.rs            # 颜色类型和工具
│   │       │   └── text.rs             # 文本渲染 (Bitmap font)
│   │       ├── input/
│   │       │   ├── mod.rs              # 输入系统总入口
│   │       │   ├── keyboard.rs         # 键盘输入管理
│   │       │   ├── mouse.rs            # 鼠标输入管理
│   │       │   ├── touch.rs            # 触摸输入管理
│   │       │   ├── gamepad.rs          # 手柄输入管理
│   │       │   └── keys.rs             # 按键码定义
│   │       ├── audio/
│   │       │   ├── mod.rs              # 音频系统总入口
│   │       │   ├── engine.rs           # Web Audio API 封装
│   │       │   ├── source.rs           # 音频源 (音效/音乐)
│   │       │   └── mixer.rs            # 音频混音器
│   │       ├── physics/
│   │       │   ├── mod.rs              # 物理系统总入口
│   │       │   ├── rigid_body.rs       # 刚体组件
│   │       │   ├── collider.rs         # 碰撞体 (AABB/圆形/多边形)
│   │       │   ├── collision.rs        # 碰撞检测算法
│   │       │   ├── solver.rs           # 碰撞响应求解器
│   │       │   └── spatial.rs          # 空间分区 (网格/四叉树)
│   │       ├── asset/
│   │       │   ├── mod.rs              # 资源管理总入口
│   │       │   ├── loader.rs           # 资源加载器
│   │       │   ├── cache.rs            # 资源缓存
│   │       │   └── image.rs            # 图片加载
│   │       ├── scene/
│   │       │   ├── mod.rs              # 场景管理总入口
│   │       │   ├── manager.rs          # 场景管理器
│   │       │   └── transition.rs       # 场景过渡效果
│   │       ├── math.rs                 # 数学工具扩展 (glam wrapper)
│   │       ├── time.rs                 # 时间管理 (Delta time, Timer)
│   │       ├── event.rs                # 事件系统
│   │       ├── transform.rs            # 2D 变换组件 (位置/旋转/缩放)
│   │       ├── sprite_component.rs     # 精灵组件
│   │       └── log.rs                  # 日志 (console_log)
│   ├── engine-macros/                  # 过程宏 (可选)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── examples/
│       ├── pong/                       # Pong 示例游戏
│       │   ├── Cargo.toml
│       │   ├── index.html
│       │   └── src/
│       │       └── lib.rs
│       └── platformer/                 # 平台跳跃示例游戏
│           ├── Cargo.toml
│           ├── index.html
│           └── src/
│               └── lib.rs
├── assets/
│   ├── textures/                       # 纹理资源
│   ├── audio/                          # 音频资源
│   └── fonts/                          # 字体资源
└── web/
    └── style.css                       # 全局样式
```

## 实施步骤

### 步骤 1：项目基础设施搭建
- 创建 Cargo workspace 根 `Cargo.toml`
- 创建 `Trunk.toml` 构建配置
- 创建 `index.html` 主入口页面
- 创建引擎 crate 的 `Cargo.toml`（配置依赖和 crate-type）
- 配置 `lib.rs` 模块结构
- 配置 `.cargo/config.toml` 设置 wasm32 目标

### 步骤 2：核心模块 - 数学、时间和日志
- 实现 `math.rs`：扩展 glam 的工具函数（角度转换、lerp、clamp 等）
- 实现 `time.rs`：DeltaTime 计算、Timer 组件、固定时间步长管理
- 实现 `log.rs`：console_log 宏封装
- 实现 `color.rs`：Color 类型（RGBA），预定义颜色常量
- 实现 `transform.rs`：Transform2D 组件（position, rotation, scale）

### 步骤 3：ECS 系统
- 实现 `entity.rs`：基于 `generational-arena` 的 Entity ID
- 实现 `component.rs`：Component trait（要求 `'static + Any`）
- 实现 `world.rs`：World 结构体，管理多个 ComponentStorage
- 实现 `query.rs`：类型安全的组件查询（单组件、双组件、多组件）
- 实现 `system.rs`：System trait 和 SystemScheduler
- 实现 `builder.rs`：EntityBuilder 链式 API

### 步骤 4：事件系统
- 实现 `event.rs`：事件总线 EventBus，支持发布/订阅模式
- 定义引擎事件类型：Resize, KeyDown, KeyUp, MouseMove, Collision 等

### 步骤 5：渲染系统
- 实现 `gl_backend.rs`：初始化 WebGL2 上下文（通过 glow）
- 实现 `shader.rs`：着色器编译、链接、Uniform 设置
- 实现 `texture.rs`：从 HTMLImageElement 创建 WebGL 纹理
- 实现 `camera.rs`：2D 正交相机
- 实现 `sprite.rs`：SpriteBatch 批量渲染精灵（纹理四边形）
- 实现 `shape.rs`：绘制矩形、圆形、线段等基本形状
- 实现 `text.rs`：BitmapFont 文本渲染

### 步骤 6：输入系统
- 实现 `keyboard.rs`：键盘事件监听、按键状态追踪（down/pressed/released）
- 实现 `mouse.rs`：鼠标事件、位置、按钮状态
- 实现 `touch.rs`：触摸事件、多点触控
- 实现 `gamepad.rs`：Gamepad API 轮询
- 实现 `keys.rs`：KeyCode 和 MouseButton 枚举定义

### 步骤 7：音频系统
- 实现 `engine.rs`：Web Audio API 初始化（处理自动播放策略）
- 实现 `source.rs`：AudioSource（音效/音乐加载和播放）
- 实现 `mixer.rs`：AudioMixer（音量控制、声道管理）

### 步骤 8：物理系统
- 实现 `collider.rs`：Collider 组件（AABB、圆形）
- 实现 `rigid_body.rs`：RigidBody 组件（质量、速度、加速度、力）
- 实现 `collision.rs`：碰撞检测算法（AABB-AABB、Circle-Circle、AABB-Circle）
- 实现 `solver.rs`：碰撞响应（弹性碰撞、摩擦力）
- 实现 `spatial.rs`：简单网格空间分区

### 步骤 9：资源管理
- 实现 `loader.rs`：异步资源加载（fetch API）
- 实现 `cache.rs`：资源缓存（HashMap 持有已加载资源）
- 实现 `image.rs`：图片加载（fetch -> blob -> HTMLImageElement）

### 步骤 10：场景管理
- 实现 `manager.rs`：SceneManager（push/pop/switch 场景）
- 实现 `transition.rs`：场景过渡效果（淡入淡出）
- 定义 Scene trait：on_enter, on_exit, update, render

### 步骤 11：应用框架和游戏循环
- 实现 `app.rs`：
  - `AppBuilder` 模式配置引擎
  - requestAnimationFrame 游戏循环
  - 固定时间步长物理更新 + 可变帧率渲染
  - 系统执行顺序管理
  - 引擎初始化和清理

### 步骤 12：lib.rs 整合
- 导出所有公共模块
- 提供 prelude 模块（最常用类型集合）
- 定义 `#[wasm_bindgen]` 入口函数

### 步骤 13：Pong 示例游戏
- 创建 Pong 游戏 crate
- 实现：球拍控制、球运动、碰撞检测、分数显示
- 创建对应 index.html

### 步骤 14：平台跳跃示例游戏
- 创建平台跳跃游戏 crate
- 实现：角色控制、重力、平台碰撞、摄像机跟随
- 展示 ECS 和物理系统的使用

### 步骤 15：Web 页面和构建优化
- 优化 `index.html`（加载动画、Canvas 全屏）
- 配置 wasm-opt 优化
- 配置 release profile（opt-level="z", LTO, strip）
- 添加 console_error_panic_hook 用于调试

## 核心 API 设计示例

```rust
// 游戏开发者使用引擎的方式
use opengame_engine::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugin(RenderPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(ball_movement)
        .add_system(collision_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Transform2D::new(Vec2::new(100.0, 300.0)),
        Sprite::new("paddle.png"),
        Player { speed: 300.0 },
        RigidBody::kinematic(),
        Collider::rectangle(20.0, 100.0),
    ));
}

fn player_movement(
    input: Res<InputState>,
    mut query: Query<(&mut Transform2D, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in query.iter_mut() {
        if input.is_key_down(KeyCode::ArrowUp) {
            transform.position.y -= player.speed * time.delta();
        }
        if input.is_key_down(KeyCode::ArrowDown) {
            transform.position.y += player.speed * time.delta();
        }
    }
}
```

## 关键依赖版本

```toml
[dependencies]
wasm-bindgen = "0.2"
web-sys = "0.3"
js-sys = "0.3"
glow = "0.16"
glam = "0.29"
generational-arena = "0.2"
log = "0.4"
console_log = "1"
console_error_panic_hook = "0.1"
wasm-bindgen-futures = "0.4"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

## 验证方式

1. **编译检查**：`trunk build` 成功编译为 WASM
2. **Pong 游戏**：可在浏览器中运行，球拍可控制，球正确反弹，碰撞检测正常
3. **平台跳跃游戏**：角色可移动和跳跃，重力正常，平台碰撞正常
4. **ECS 验证**：组件添加/移除/查询正确
5. **输入验证**：键盘和鼠标输入正确响应
6. **渲染验证**：精灵、形状、文本正确渲染
