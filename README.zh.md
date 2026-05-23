# OpenLess Local

OpenLess Local 是 OpenLess 的轻量本地优先 fork。它只保留核心语音输入链路：按全局快捷键，说话，转写，用你配置的模型润色，然后把结果插入当前光标位置。

这个 fork 有意移除了上游偏产品化、社区化和托管分发的功能。网络访问只保留你主动配置的 ASR/LLM provider 请求，以及你主动触发的本地模型下载。

[English](README.md)

## 保留功能

- 全局语音输入：录音、ASR、润色、插入当前输入框。
- 直接插入失败时自动回退到剪贴板。
- 内置和自定义写作风格，支持本地风格包导入/导出。
- 个性化设置：快捷键、语言、字号、provider 凭据、插入行为和本地 ASR 设置。
- 本地历史、词汇表、纠错规则、ASR 热词和调试音频记录。
- Windows 的 Foundry Local / Sherpa ONNX 本地 ASR 路径，以及 macOS vendored Qwen ASR 路径。

## 项目结构

- `openless-all/app`：Tauri 2 桌面应用。
- `openless-all/app/src`：React 前端。
- `openless-all/app/src-tauri`：Rust 后端，包含原生快捷键、音频、ASR、LLM 调用、文本插入、持久化和打包配置。
- `docs`：本地配置说明和实现计划。

## 本地开发

```powershell
cd openless-all\app
npm ci
npm run build
```

交互式开发：

```powershell
cd openless-all\app
npm run tauri dev
```

Windows 原生工具链检查：

```powershell
cd openless-all\app
powershell -ExecutionPolicy Bypass -File .\scripts\windows-preflight.ps1 -Toolchain msvc
```

Rust 检查：

```powershell
cd openless-all\app
cargo check --manifest-path src-tauri\Cargo.toml
```

## Provider 配置

OpenLess Local 不提供托管后端。请在设置中配置自己的 ASR 和 LLM provider。

常见字段：

- ASR provider 凭据，例如火山引擎 App ID、Access Token 和 Resource ID。
- LLM provider 的 endpoint、API Key 和模型 ID。
- 使用本地识别器时的本地 ASR 后端、模型路径和运行参数。

火山 ASR 配置见 [docs/volcengine-setup.md](docs/volcengine-setup.md)。Windows Sherpa ONNX 本地 ASR 路径见 [docs/windows-sherpa-onnx-asr-plan.md](docs/windows-sherpa-onnx-asr-plan.md)。

## 本地数据

运行数据由 Tauri 存放在 `com.openless.local` 标识对应的应用数据目录中。具体基础路径取决于操作系统，里面包含用户偏好、历史记录、词汇表、风格包和本地调试记录。支持的平台会把凭据存到系统 keyring。

## 本地打包

仍可用 Tauri 生成本地安装包：

```powershell
cd openless-all\app
npm run tauri -- build
```

这个 fork 不生成托管分发元数据。构建产物默认只视为本地 artifact；如果以后需要分发，可以再单独建立自己的流程。

## 许可证

本 fork 保留上游开源许可证。见 [LICENSE](LICENSE)。
