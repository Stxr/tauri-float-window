# 开发记录 Dev Log

> 用于快速回顾项目关键变更、决策和命令。新增记录请按日期倒序追加。

## 2025-09-20
- 功能：在 Rust 侧监听窗口 `Resized` 事件并根据初始宽高比强制调整尺寸，保持固定纵横比（`src-tauri/src/lib.rs:15`, `src-tauri/src/lib.rs:23`）。
- 命令：新增 `set_always_on_top`，前端按钮一键置顶/取消置顶（`src-tauri/src/lib.rs:9`, `src/App.tsx:16`，按钮在 `src/App.tsx:56`）。
- 前端：通过 `@tauri-apps/api` 的 `invoke` 调用 `greet` 与 `set_always_on_top`（`src/App.tsx:11`, `src/App.tsx:16`）。
- 配置：Vite 开发端口为 1420（`vite.config.ts:16-18`）；Tauri 窗口尺寸限制（`src-tauri/tauri.conf.json:16-23`）。
- 文档：新增 `AGENTS.md`（贡献/协作规范）。
- 常用命令：`pnpm dev`、`pnpm run tauri dev`、`pnpm run tauri build`。

### 手机模拟器（加载指定 URL）
- UI：新增简单“手机外观”与刘海样式，内嵌 iframe 加载目标 URL，默认 `https://baidu.com`（`src/App.tsx:9`, `src/App.tsx:33`, `src/App.css:130`）。
- 工具栏：地址栏 `Load` 按钮、窗口置顶开关、`Open Externally` 外部打开（`src/App.tsx:40`, `src/App.tsx:41`, `src/App.tsx:44`）。
  - 新增 `Open In-App Window` 在应用内以整窗打开目标站点，绕过 iframe 的 `frame-ancestors` 限制（`src/App.tsx:46`, `src/App.tsx:58`）。
- 样式：圆角边框、阴影、刘海与全屏内容区域（`src/App.css:130`, `src/App.css:140`, `src/App.css:153`）。
- 比例：窗口初始尺寸与最小/最大尺寸调整为 19.5:9 近似（`src-tauri/tauri.conf.json:18`）。Rust 侧依旧根据初始宽高比锁定比例（`src-tauri/src/lib.rs:15`）。

### 无边框窗口（iOS 模拟器风格）
- 配置：窗口去装饰、透明背景与无阴影（`src-tauri/tauri.conf.json:23-26`）。
- 样式：全局背景透明；去除设备边框与刘海，仅保留圆角屏幕（`src/App.css:8-15`, `src/App.css:120-129`, `src/App.css:138-147`）。
- 可拖拽：工具栏区域支持拖拽（`-webkit-app-region: drag`），交互控件设为 `no-drag`（`src/App.css:123-124`）。

### 工具栏默认隐藏 + 快捷键显示
- 逻辑：新增 `toolbarVisible` 状态，默认隐藏；`Cmd/Ctrl+L` 显示/隐藏工具栏，`Esc` 隐藏（`src/App.tsx:11`, `src/App.tsx:18-32`）。
- 拖拽：工具栏隐藏时显示顶部 `drag-strip` 供拖拽移动窗口（`src/App.tsx:48`, `src/App.css:149-157`）。

## 2025-05-21
- 初始化：创建 Tauri v2 + React 18 + Vite 6 项目，包管理使用 pnpm；添加 `@tauri-apps/cli` 与 `@tauri-apps/plugin-opener`（`package.json`）。
- 目录：前端在 `src/`，Tauri 在 `src-tauri/`；入口 `src-tauri/src/main.rs`，主要逻辑 `src-tauri/src/lib.rs`。
- 基础命令：`pnpm tauri dev` 启动开发，`pnpm tauri build` 打包。

---

## 如何追加新记录（模板）

### YYYY-MM-DD 简要标题
- 变更：简述做了什么和为什么（如修复、重构、配置调整）。
- 代码：列出关键文件/位置（例如：`src/path/file.ts:12`）。
- 命令：记录调试/构建/迁移用到的关键命令。
- 备注：遇到的问题与决策取舍，后续待办。
