.PHONY: help dev build release test lint fmt clippy doc clean check all cli cli-install

help: ## 显示帮助信息
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

dev: ## 启动开发服务器 (热重载)
	trunk serve

build: ## 构建 WASM 包 (开发模式)
	trunk build

release: ## 构建 WASM 包 (发布模式, 优化体积)
	trunk build --release

test: ## 运行所有测试
	cargo test -p opengame-engine --target x86_64-unknown-linux-gnu
	cargo test -p opengame-engine --doc --target x86_64-unknown-linux-gnu

test-verbose: ## 运行所有测试 (详细输出)
	cargo test -p opengame-engine --target x86_64-unknown-linux-gnu -- --nocapture

lint: fmt-check clippy ## 运行所有 lint 检查

fmt: ## 格式化代码
	cargo fmt --all

fmt-check: ## 检查代码格式
	cargo fmt --all -- --check

clippy: ## 运行 clippy 静态分析
	cargo clippy --all-targets -- -D warnings

doc: ## 生成 API 文档
	cargo doc --no-deps --open

doc-check: ## 检查文档是否能正确生成
	cargo doc --no-deps

clean: ## 清理构建产物
	cargo clean
	rm -rf dist

check: ## 快速检查编译是否通过
	cargo check --all-targets

all: fmt clippy test build ## 运行完整流水线 (格式化 + lint + 测试 + 构建)

size: release ## 显示构建产物大小
	@echo "=== 构建产物 ==="
	@ls -lh dist/*.wasm 2>/dev/null || echo "未找到 WASM 文件"
	@du -sh dist/ 2>/dev/null || echo "未找到 dist 目录"

install-tools: ## 安装推荐的开发工具
	rustup target add wasm32-unknown-unknown
	rustup component add clippy rustfmt
	cargo install trunk
	cargo install wasm-bindgen-cli

cli: ## 构建 CLI 工具
	cargo build --target x86_64-unknown-linux-gnu -p opengame-cli

cli-release: ## 构建 CLI 工具 (发布模式)
	cargo build --target x86_64-unknown-linux-gnu --release -p opengame-cli

cli-install: cli-release ## 安装 CLI 工具到 PATH
	@mkdir -p ~/.local/bin
	cp target/x86_64-unknown-linux-gnu/release/og ~/.local/bin/og
	@echo "Installed 'og' to ~/.local/bin/og"
	@echo "Add ~/.local/bin to your PATH if not already done"
