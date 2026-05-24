import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const scriptsDir = dirname(fileURLToPath(import.meta.url));
const appRoot = join(scriptsDir, "..");
const repoRoot = join(appRoot, "..", "..");
const tauriConfigPath = join(appRoot, "src-tauri", "tauri.conf.json");
const releaseWorkflowPath = join(repoRoot, ".github", "workflows", "windows-release.yml");

const tauriConfig = JSON.parse(readFileSync(tauriConfigPath, "utf8"));
const releaseWorkflow = readFileSync(releaseWorkflowPath, "utf8");

assert.equal(
  tauriConfig.bundle.windows.nsis.installMode,
  "currentUser",
  "Lite Windows installer should not require machine-wide TSF registration"
);
assert.equal(
  tauriConfig.bundle.windows.nsis.installerHooks,
  undefined,
  "Lite Windows installer must not register a TSF input method"
);
assert.equal(
  tauriConfig.bundle.windows.wix,
  undefined,
  "Lite release builds only the NSIS setup exe and must not include TSF WiX fragments"
);
assert.doesNotMatch(
  releaseWorkflow,
  /windows-ime-build\.ps1|OPENLESS_IME_DLL|openless-ime-hooks|openless-ime\.wxs/,
  "Lite release workflow must not build, bundle, or register OpenLessIme.dll"
);
assert.match(
  releaseWorkflow,
  /--bundles nsis/,
  "Lite release workflow should still build the Windows setup exe"
);
