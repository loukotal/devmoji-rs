#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { existsSync } from "node:fs";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = dirname(fileURLToPath(import.meta.url));

const PLATFORMS = {
  "darwin-arm64": "@loukotal/devmoji-rs-darwin-arm64",
  "darwin-x64": "@loukotal/devmoji-rs-darwin-x64",
  "linux-x64-gnu": "@loukotal/devmoji-rs-linux-x64-gnu",
  "linux-x64-musl": "@loukotal/devmoji-rs-linux-x64-musl",
  "linux-arm64-gnu": "@loukotal/devmoji-rs-linux-arm64-gnu",
  "win32-x64": "@loukotal/devmoji-rs-win32-x64",
};

function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;
  const binaryName = platform === "win32" ? "devmoji.exe" : "devmoji";

  // Try platform-specific package first (normal npm install)
  if (platform === "linux") {
    const candidates = [`${platform}-${arch}-musl`, `${platform}-${arch}-gnu`];
    for (const candidate of candidates) {
      const pkg = PLATFORMS[candidate];
      if (pkg) {
        try {
          return join(
            dirname(require.resolve(`${pkg}/package.json`)),
            "bin",
            binaryName
          );
        } catch {}
      }
    }
  } else {
    const key = `${platform}-${arch}`;
    const pkg = PLATFORMS[key];
    if (pkg) {
      try {
        return join(
          dirname(require.resolve(`${pkg}/package.json`)),
          "bin",
          binaryName
        );
      } catch {}
    }
  }

  // Fallback: binary in the same bin/ directory (local dev / pnpm link)
  const localBinary = join(__dirname, binaryName);
  if (existsSync(localBinary)) {
    return localBinary;
  }

  throw new Error(
    `Unsupported platform: ${platform}-${arch}. ` +
      `devmoji-rs does not have a prebuilt binary for your system.`
  );
}

try {
  const binary = getBinaryPath();
  execFileSync(binary, process.argv.slice(2), { stdio: "inherit" });
} catch (err) {
  if (typeof err.status === "number") {
    process.exit(err.status);
  }
  console.error(err.message);
  process.exit(1);
}
