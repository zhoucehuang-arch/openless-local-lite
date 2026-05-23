# OpenLess Local Lite 计划

日期：2026-05-23  
工作分支：`local-lite`  
上游基线：`a38d6b5 chore(release): bump version to 1.3.4-13 (Beta) -- 设置界面重构 + 加入 Beta 渠道 / 麦克风 / 市场登录`  
应用目录：`openless-all/app`

本文档是后续执行依据。除非用户明确改变方向，后续实现、验证和收尾都按本文档推进。

## 目标

做一个更纯粹、轻量、本地优先的 OpenLess fork，定位为个人本地工具，而不是带运营、市场、社区和持续分发假设的产品。

保留核心能力：

- 语音输入主链路：全局快捷键、录音、ASR、润色、插入当前光标、失败时剪贴板兜底。
- 自动润色：保留 raw / light / structured / formal 以及自定义风格包。
- 风格切换：保留当前风格选择、快捷键切换、风格包编辑、导入 ZIP、导出 ZIP。
- 个性化：保留字体大小、语言、快捷键、麦克风、插入行为、模型/provider 配置。
- 历史：保留历史记录、清理、音频调试记录的本地管理。
- 词汇表：保留词汇表、纠错规则、ASR hotwords 和润色提示注入。
- 本地模型：保留本地 ASR 模型管理，尤其 Windows 侧 Foundry Local / Sherpa ONNX 路径。

移除或默认关闭：

- 应用内自动更新、手动检查更新、Beta 渠道、updater manifest。
- 风格市场、市场登录、市场上传/点赞/安装、GitHub device flow。
- 翻译页、翻译快捷键、录音中的翻译模式入口。
- 划词语音问答、QA 独立窗口、追问、多轮 QA 面板。
- 帮助中心、发布日志、反馈渠道、QQ 群、官网/社区入口。
- 非核心产品运营文案、赞助/贡献者/发布流程文档入口。

允许保留的网络行为：

- 用户自己配置的 ASR / LLM provider 请求。
- 用户主动触发的本地模型下载或模型运行时安装。
- 不保留任何后台市场、更新、反馈、社区、OAuth 相关网络请求。

## 调研结论

README 的产品方向本来强调“只做语音 -> 转写 -> 润色 -> 插入”，但当前代码已经加入较多产品化功能。轻量化需要同时处理 UI、前端 IPC、Tauri 配置和 Rust 命令面，不能只隐藏一个按钮。

已确认的核心入口：

- `src/App.tsx`：加载主窗口、胶囊窗口、QA 窗口，以及 `AutoUpdateGate`。
- `src/main.tsx`：通过 URL 参数区分 `window=capsule` / `window=qa`。
- `src/components/FloatingShell.tsx`：主导航，当前包含 Overview / History / Vocab / Style / Translation / SelectionAsk。
- `src/components/SettingsModal.tsx`：设置弹窗，当前含帮助中心和发布日志外链。
- `src/pages/settings/tabs.tsx`：设置页分组，当前 Services 含 Marketplace，Advanced 含 Beta。
- `src/pages/Style.tsx`：风格包核心 UI，同时混入风格市场入口、上传和发布。
- `src-tauri/tauri.conf.json`：包含 `qa` 窗口和 `plugins.updater` 配置。
- `src-tauri/src/lib.rs`：注册 updater/shell 插件，暴露 updater、marketplace、GitHub、QA、translation 命令，启动 QA/translation hotkey。
- `src-tauri/src/commands.rs`：updater、marketplace、GitHub device flow、QA/translation hotkey 命令集中在这里。
- `src-tauri/src/coordinator.rs` 和 `src-tauri/src/coordinator/qa.rs`：包含 translation hotkey、QA 状态和选区捕获路径。
- `src-tauri/src/types.rs`：用户配置里仍有 `update_channel`、`translation_hotkey`、`qa_hotkey`、`marketplace_base_url`、`marketplace_dev_login` 等字段。

已确认的外部/后台网络入口：

- updater：`@tauri-apps/plugin-updater`、`tauri-plugin-updater`、`AutoUpdateGate`、`CheckUpdateButton`、`app_check_update_with_channel`、`fetch_latest_beta_release`。
- marketplace：`https://apic.openless.top`、`Marketplace` 页面、`MarketplaceModal`、`MarketplaceSection`、`marketplace_*` Rust commands。
- GitHub OAuth：`GithubLoginModal`、`github_device_flow_start`、`github_device_flow_poll`。
- 外部链接：About / Settings 中的 GitHub、README、Issues、Release notes、QQ 群。
- Local model downloads：本地 ASR 需要，保留但必须保持用户主动触发。

## 裁剪原则

1. 先移除可见产品面，再删除后端能力。这样可以每阶段构建验证，不在一次大改里同时打断 UI 和 Rust。
2. 保留本地数据结构的向后兼容字段，除非确认迁移风险很低。第一版可以不立刻清理旧配置字段，后续再做 schema 瘦身。
3. 对网络调用做白名单思路：核心 provider 和本地模型下载允许，其余全部移除或不可达。
4. 删除功能时优先删除入口、IPC wrapper、invoke handler、插件依赖，再清理无引用文件和 i18n 文案。
5. 每阶段结束必须跑搜索审计和构建检查。不要只凭 TypeScript 未报错判断完成。

## 执行计划

### Phase 0：建立轻量 fork 身份

目标：让 fork 在命名、版本、路径上和上游产品分开，避免后续包、日志、凭据和自动更新混淆。

修改范围：

- `openless-all/app/package.json`
- `openless-all/app/package-lock.json`
- `openless-all/app/src-tauri/tauri.conf.json`
- `openless-all/app/src-tauri/Cargo.toml`
- `openless-all/app/src-tauri/Cargo.lock`
- 前端显示名称中硬编码的 `OpenLess`

建议值：

- 产品名：`OpenLess Local`
- npm 包名：`openless-local-app`
- Rust package：`openless-local`
- Tauri identifier：`com.openless.local`
- 初始版本：`1.0.0-local.0`

验收：

- 主窗口、胶囊窗口标题不再显示上游产品包身份。
- `rg -n "com.openless.app|openless-app|productName\": \"OpenLess\"" openless-all/app` 只剩兼容说明或历史注释。

### Phase 1：裁掉可见产品面

目标：用户打开应用后只看到核心功能：Overview / History / Vocab / Style / Settings。

修改范围：

- `src/App.tsx`
- `src/main.tsx`
- `src/components/FloatingShell.tsx`
- `src/state/useAppState.ts`
- `src/components/SettingsModal.tsx`
- `src/pages/settings/tabs.tsx`
- `src/pages/settings/AboutSection.tsx`
- `src/pages/settings/RecordingInputSection.tsx`
- `src/pages/settings/ShortcutsSection.tsx`
- `src/pages/Style.tsx`

具体动作：

- 移除 `AutoUpdateGate` 渲染。
- 移除 `QaPanel` 路由和 `isQa` 参数。
- 主导航删除 `translation` 和 `selectionAsk`。
- `AppTab` 收敛为 `overview | history | vocab | style`。
- 设置弹窗删除 Help center / Release notes 外链组。
- Services 只保留 provider 配置，移除 `MarketplaceSection`。
- Advanced 保留本地模型和本地调试工具，移除 `BetaChannelSection`。
- About 只保留本地版本信息和个性化字体设置，移除检查更新、GitHub、文档、反馈、QQ。
- Recording startup 组移除 `autoUpdateCheck` 开关。
- Shortcuts 移除 translation 和 selection ask hotkey，只保留录音、切换风格、打开 App、Esc/确认说明。
- Style 页移除风格市场按钮、`MarketplaceModal`、发布到市场按钮、上传逻辑、市场身份判断；保留本地导入/导出 ZIP。

验收：

- `npm run build` 通过。
- 搜索不到可见入口引用：
  - `rg -n "AutoUpdateGate|QaPanel|MarketplaceModal|MarketplaceSection|BetaChannelSection|CheckUpdateButton" openless-all/app/src`
  - `rg -n "helpCenter|releaseNotes|feedback|QQ|qq群|QQ群" openless-all/app/src`

### Phase 2：移除 updater 和外链插件能力

目标：没有后台更新检查，也不依赖 Tauri updater/shell 插件。

修改范围：

- `package.json`
- `package-lock.json`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`
- `src-tauri/tauri.conf.json`
- `src-tauri/capabilities/default.json`
- `src-tauri/src/lib.rs`
- `src-tauri/src/commands.rs`
- `src/lib/ipc.ts`
- `src/components/AutoUpdate.tsx`
- `src/components/AutoUpdateGate.tsx`
- `src/pages/settings/CheckUpdateButton.tsx`

具体动作：

- 删除 npm 依赖 `@tauri-apps/plugin-updater`。
- 如果 `openExternal` 和 GitHub login 已移除，删除 npm 依赖 `@tauri-apps/plugin-shell`。
- 删除 Rust 依赖 `tauri-plugin-updater`。
- 如果 shell 插件无剩余用途，删除 `tauri-plugin-shell`。
- 删除 `tauri.conf.json` 的 `plugins.updater`。
- 删除 capability 中 `updater:default`。
- 从 `lib.rs` 删除 updater/shell plugin 初始化。
- 从 invoke handler 删除：
  - `get_update_channel`
  - `set_update_channel`
  - `fetch_latest_beta_release`
  - `app_check_update_with_channel`
- 从 `commands.rs` 删除 updater/Beta 相关命令和只为它们服务的解析测试。
- 删除前端 updater 组件和 IPC wrappers。
- `UserPreferences` 里的 `autoUpdateCheck` / `updateChannel` 可先保留为兼容字段，但不再出现在 UI 或触发逻辑中；后续 Phase 5 再决定是否迁移清理。

验收：

- `npm run build` 通过。
- `cargo check --manifest-path openless-all/app/src-tauri/Cargo.toml` 通过。
- `rg -n "plugin-updater|AutoUpdate|CheckUpdateButton|app_check_update|fetch_latest_beta|update_channel|plugins.updater" openless-all/app` 只剩 lock 变更前残留或兼容字段说明；依赖清理后 lock 中也不应出现 updater crate。

### Phase 3：移除市场、GitHub 身份和社区外链

目标：风格只存在于本地，不存在任何云端市场、上传、点赞、登录或审核队列。

修改范围：

- `src/pages/Marketplace.tsx`
- `src/components/MarketplaceModal.tsx`
- `src/components/GithubLoginModal.tsx`
- `src/pages/settings/MarketplaceSection.tsx`
- `src/lib/ipc.ts`
- `src/lib/types.ts`
- `src/i18n/*.ts`
- `src-tauri/src/lib.rs`
- `src-tauri/src/commands.rs`
- `src-tauri/src/types.rs`
- `src-tauri/src/persistence.rs`

具体动作：

- 删除或断开 Marketplace 页面、Modal、Settings section、GitHub login modal。
- 删除前端 marketplace IPC wrappers 和 mock data。
- 从 invoke handler 删除：
  - `marketplace_list`
  - `marketplace_detail`
  - `marketplace_install`
  - `marketplace_upload`
  - `marketplace_like`
  - `marketplace_my_likes`
  - `marketplace_my_packs`
  - `marketplace_delete`
  - `github_device_flow_start`
  - `github_device_flow_poll`
- 从 `commands.rs` 删除市场和 GitHub device flow 实现。
- 清理 `MARKETPLACE_BASE_URL`、`marketplace_base_url`、`marketplace_dev_login` 的 UI 和新配置写入。
- 保留风格包本地 origin metadata 的读取兼容，避免已安装风格包损坏；UI 不再展示“衍生自 @login”。

验收：

- `npm run build` 通过。
- `cargo check --manifest-path openless-all/app/src-tauri/Cargo.toml` 通过。
- `rg -n "Marketplace|marketplace|GithubLogin|github_device|apic.openless.top|marketplaceDevLogin|marketplaceBaseUrl" openless-all/app/src openless-all/app/src-tauri/src` 只允许出现在迁移/兼容注释或历史数据结构兼容位置。

### Phase 4：移除翻译和划词 QA

目标：应用只做语音输入和文本润色，不做翻译热键、选中文本问答、追问或独立 QA 浮窗。

修改范围：

- `src/pages/Translation.tsx`
- `src/pages/SelectionAsk.tsx`
- `src/pages/QaPanel.tsx`
- `src/components/Capsule.tsx`
- `src/lib/ipc.ts`
- `src/lib/types.ts`
- `src/lib/capsuleLayout.ts`
- `src/i18n/*.ts`
- `src-tauri/tauri.conf.json`
- `src-tauri/src/lib.rs`
- `src-tauri/src/commands.rs`
- `src-tauri/src/coordinator.rs`
- `src-tauri/src/coordinator/qa.rs`
- `src-tauri/src/selection.rs`
- `src-tauri/src/qa_hotkey.rs`
- `src-tauri/src/types.rs`
- `src-tauri/src/linux_fcitx.rs`

具体动作：

- 删除 QA window 配置。
- 删除 QA 前端页面和 `window=qa` 路径。
- 删除 selection capture 模块和 QA coordinator 状态机，或先断开所有调用后再删文件。
- 删除 translation hotkey listener、translation hotkey settings、capsule translation indicator。
- 从 invoke handler 删除：
  - `set_qa_hotkey`
  - `get_qa_hotkey_label`
  - `set_translation_hotkey`
  - `qa_window_dismiss`
  - `qa_window_pin`
- 调整 hotkey 冲突检测：不再检测 dictation vs translation / qa，只保留 dictation / switch style / open app。
- `UserPreferences` 里的 `translation_hotkey`、`qa_hotkey`、`qa_save_history` 可先兼容读入但不使用；最终再做 schema 清理。

验收：

- `npm run build` 通过。
- `cargo check --manifest-path openless-all/app/src-tauri/Cargo.toml` 通过。
- `rg -n "Translation|translationHotkey|translation_hotkey|SelectionAsk|selectionAsk|QaPanel|qa_hotkey|qa_window|capture_selection|selection.rs" openless-all/app/src openless-all/app/src-tauri/src` 只允许出现在兼容字段或非功能性历史注释中。

### Phase 5：配置和数据模型瘦身

目标：减少旧功能对偏好结构、mock 数据和 i18n 的污染，但不牺牲升级兼容。

修改范围：

- `src/lib/types.ts`
- `src/lib/ipc.ts`
- `src/lib/mockData.ts`
- `src/i18n/*.ts`
- `src-tauri/src/types.rs`
- `src-tauri/src/persistence.rs`
- 相关测试

具体动作：

- 对旧字段分三类：
  - 运行必须：保留。
  - 读入兼容但不写出：标记 deprecated，默认值仍存在。
  - 完全无用：移除并更新序列化测试。
- 清理 marketplace / update / QA / translation 的 i18n 文案。
- 清理 mock 设置中的旧字段。
- 确认旧 `prefs.json` 含这些字段时仍可启动，或者提供一次性迁移。

验收：

- `npm run build` 通过。
- `cargo test --manifest-path openless-all/app/src-tauri/Cargo.toml` 通过，或记录无法在 Windows 当前环境完成的原因。
- 用包含旧字段的偏好 JSON 做一次反序列化测试。

### Phase 6：文档和发布脚本轻量化

目标：仓库文档只服务本地构建、安装、配置和故障排查，不再宣传官网、社区、市场、发布渠道。

修改范围：

- `README.md`
- `README.zh.md`
- `USAGE.md`
- `openless-all/README.md`
- `docs/volcengine-setup.md`
- `docs/windows-sherpa-onnx-asr-plan.md`
- `.github/workflows/*`
- `scripts/*`
- `openless-all/app/scripts/*`

具体动作：

- README 改成短入口：OpenLess Local 是本地优先语音输入工具。
- 移除官网、QQ、赞助、贡献者头像、发布流程、Beta/Stable 自动更新说明。
- 保留 Windows 构建说明、provider 配置、本地模型说明、数据路径说明。
- 发布脚本中 updater manifest 生成、release tag 规则、Cask 分发先标记为不使用；若会造成维护干扰再删除。

验收：

- `rg -n "openless.top|QQ|feedback|Release notes|Beta channel|marketplace|updater|Homebrew Cask|sponsor|赞助" README.md README.zh.md USAGE.md openless-all/README.md docs scripts openless-all/app/scripts` 无用户可见残留，除非是历史兼容说明。

### Phase 7：最终验证

目标：证明轻量版没有产品化后台能力，同时核心语音输入链路仍可构建。

必须执行：

```powershell
cd D:\hadan\typlss\openless-local\openless-all\app
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
```

推荐执行：

```powershell
npm run check:hotkey-injection
powershell -ExecutionPolicy Bypass -File .\scripts\windows-preflight.ps1 -Toolchain msvc
```

搜索审计：

```powershell
rg -n "AutoUpdate|CheckUpdate|plugin-updater|plugins.updater|BetaChannel|fetch_latest_beta|app_check_update" .
rg -n "Marketplace|marketplace|apic.openless.top|GithubLogin|github_device|OAuth" .
rg -n "Translation|translationHotkey|translation_hotkey|SelectionAsk|QaPanel|qa_window|qa_hotkey|capture_selection" .
rg -n "openExternal|helpCenter|releaseNotes|feedback|QQ|qq群|QQ群|openless.top" .
```

人工冒烟：

- 首次启动能看到主窗口。
- 设置页能打开并切换各 section。
- provider / credential 设置能保存。
- 录音快捷键设置能保存。
- 风格页能切换内置风格、创建本地风格、导入导出 ZIP。
- 词汇表能新增、启用、删除。
- 历史页能展示、删除、清理。
- 本地 ASR 设置页能打开，不自动下载模型。

## 风险和处理

- Rust 后端的 QA / translation 和 coordinator 绑定较深，Phase 4 可能是最大改动。处理策略是先从 UI 和 invoke handler 断开，再逐步删除模块。
- `UserPreferences` 字段直接删除可能破坏旧配置读取。处理策略是 Phase 5 再做 schema 瘦身，先保留兼容字段。
- 本地 ASR 下载也有外部网络，但属于用户明确触发的模型配置，不应和后台更新/市场混为一谈。
- 删除 shell 插件前必须确认 `openExternal` 没有剩余真实用途。
- i18n 文案很多，第一阶段允许残留未引用文案；最终阶段必须清理搜索结果。

## 执行状态

- [x] 调研上游文档和当前功能入口。
- [x] 确定保留/移除边界。
- [x] 写入本计划文档。
- [x] Phase 0：建立轻量 fork 身份。
- [x] Phase 1：裁掉可见产品面。
- [x] Phase 2：移除 updater 和外链插件能力。
- [x] Phase 3：移除市场、GitHub 身份和社区外链。
- [x] Phase 4：移除翻译和划词 QA。
- [x] Phase 5：配置和数据模型瘦身。
- [x] Phase 6：文档和发布脚本轻量化。
- [x] Phase 7：最终验证。

## 最终验证记录

日期：2026-05-23

- `npm run build`：通过。Vite 仍提示主 chunk 大于 500 kB，这是既有体积警告，不是构建失败。
- `cargo check --manifest-path src-tauri/Cargo.toml`：未进入编译阶段；当前 Windows 环境在 crates.io index 下载阶段失败，错误为 `schannel: AcquireCredentialsHandle failed: SEC_E_NO_CREDENTIALS (0x8009030E)`，阻塞在 `https://index.crates.io/config.json`。
- `npm run check:hotkey-injection`：同样被 Cargo crates.io 下载阶段阻塞，错误同上。
- `powershell -ExecutionPolicy Bypass -File .\scripts\windows-preflight.ps1 -Toolchain msvc`：Node/npm/rustc/cargo/rustup 与 Windows SDK `kernel32.lib` 可见；当前 shell 未加载 MSVC `link.exe`，需要 Developer PowerShell 或先调用 `vcvars64.bat`。
- 严格残留搜索：updater/Beta、市场/GitHub device flow、QA/翻译功能入口、社区/外链入口均无真实残留。仅保留旧配置兼容字段（`types.rs` 的 `_qa_hotkey`、`_translation_hotkey`、`marketplaceBaseUrl` 等）和 Codex OAuth 作为用户可自配的 LLM provider。
