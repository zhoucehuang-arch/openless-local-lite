# OpenLess Local Workspace

This directory contains the runnable cross-platform Tauri app.

## App Directory

The desktop app lives in `app/`.

```bash
cd app
npm ci
npm run tauri dev
```

The macOS build links a vendored C ASR engine under `app/src-tauri/vendor/qwen-asr/`, so initialize submodules after a fresh clone:

```bash
git submodule update --init --recursive
```

## Windows Build

Run preflight first so missing MSVC, Windows SDK, or MinGW tools fail with clear messages.

```powershell
cd app
powershell -ExecutionPolicy Bypass -File .\scripts\windows-preflight.ps1 -Toolchain msvc
npm ci
npm run tauri -- build
```

If MSVC is unavailable and you intentionally use the GNU route:

```powershell
cd app
scoop install rustup mingw
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-gnu
powershell -ExecutionPolicy Bypass -File .\scripts\windows-preflight.ps1 -Toolchain gnu
powershell -ExecutionPolicy Bypass -File .\scripts\windows-build-gnu.ps1
```

## macOS Build

Use the helper script from `app/`:

```bash
cd app
INSTALL=0 ./scripts/build-mac.sh
```

For local install during development:

```bash
cd app
./scripts/build-mac.sh
```

The script builds normal local artifacts only. It does not generate update metadata.

## Verification

```powershell
cd app
npm run build
cargo check --manifest-path src-tauri\Cargo.toml
npm run check:hotkey-injection
```

## Local Distribution Boundary

OpenLess Local keeps packaging scripts for local builds and manual artifacts. It does not maintain hosted distribution metadata or cloud publishing flows.
