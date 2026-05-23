# OpenLess Local

OpenLess Local is a lightweight fork of OpenLess for local-first voice input. It keeps the core workflow: press a global hotkey, speak, transcribe, polish with your chosen model, and insert the result at the current cursor.

This fork intentionally removes upstream product, community, and hosted distribution surfaces. Network access is limited to the ASR/LLM providers and local model downloads that you explicitly configure.

[中文说明](README.zh.md)

## What Remains

- Global voice input with microphone recording, ASR, polishing, and cursor insertion.
- Local fallback to clipboard when direct insertion is unavailable.
- Built-in and custom writing styles, including local style pack import/export.
- Personal preferences for hotkeys, language, font size, provider credentials, insertion behavior, and local ASR settings.
- Local history, vocabulary, correction rules, ASR hotwords, and debug audio records.
- Windows local ASR paths for Foundry Local and Sherpa ONNX, plus the macOS vendored Qwen ASR path.

## Project Layout

- `openless-all/app`: the Tauri 2 desktop app.
- `openless-all/app/src`: React frontend.
- `openless-all/app/src-tauri`: Rust backend, native hotkeys, audio, ASR, LLM calls, insertion, persistence, and packaging config.
- `docs`: local setup notes and implementation plans.

## Local Setup

```powershell
cd openless-all\app
npm ci
npm run build
```

For interactive development:

```powershell
cd openless-all\app
npm run tauri dev
```

For the Windows native toolchain check:

```powershell
cd openless-all\app
powershell -ExecutionPolicy Bypass -File .\scripts\windows-preflight.ps1 -Toolchain msvc
```

For Rust verification:

```powershell
cd openless-all\app
cargo check --manifest-path src-tauri\Cargo.toml
```

## Provider Configuration

OpenLess Local does not ship a hosted backend. Configure your own ASR and LLM provider in Settings.

Common fields:

- ASR provider credentials, such as Volcengine App ID, access token, and resource ID.
- LLM provider endpoint, API key, and model ID.
- Local ASR backend choices, model path, and runtime settings when using a local recognizer.

See [docs/volcengine-setup.md](docs/volcengine-setup.md) for Volcengine ASR setup and [docs/windows-sherpa-onnx-asr-plan.md](docs/windows-sherpa-onnx-asr-plan.md) for the Windows Sherpa ONNX local ASR path.

## Local Data

Runtime data is stored by Tauri under the app data directory for the `com.openless.local` identifier. The exact base path depends on the operating system, but it contains user preferences, history, vocabulary, style packs, and local debug records. Credentials are stored through the platform keyring where supported.

## Packaging

The app can still be packaged locally with Tauri:

```powershell
cd openless-all\app
npm run tauri -- build
```

The fork does not produce hosted distribution metadata. Builds are treated as local artifacts unless you add your own distribution process later.

## License

This fork keeps the upstream open-source license. See [LICENSE](LICENSE).
