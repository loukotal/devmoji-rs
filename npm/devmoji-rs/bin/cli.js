#!/usr/bin/env node

const { execFileSync } = require("child_process");
const path = require("path");

const PLATFORMS = {
  "darwin-arm64": "devmoji-rs-darwin-arm64",
  "darwin-x64": "devmoji-rs-darwin-x64",
  "linux-x64-gnu": "devmoji-rs-linux-x64-gnu",
  "linux-x64-musl": "devmoji-rs-linux-x64-musl",
  "linux-arm64-gnu": "devmoji-rs-linux-arm64-gnu",
  "win32-x64": "devmoji-rs-win32-x64",
};

function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;
  const binaryName = platform === "win32" ? "devmoji.exe" : "devmoji";

  // Try platform-specific package first
  if (platform === "linux") {
    // Try musl first for Alpine/musl-based systems, fall back to gnu
    const candidates = [`${platform}-${arch}-musl`, `${platform}-${arch}-gnu`];
    for (const candidate of candidates) {
      const pkg = PLATFORMS[candidate];
      if (pkg) {
        try {
          return path.join(
            path.dirname(require.resolve(`${pkg}/package.json`)),
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
        return path.join(
          path.dirname(require.resolve(`${pkg}/package.json`)),
          "bin",
          binaryName
        );
      } catch {}
    }
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
  if (err.status !== undefined) {
    process.exit(err.status);
  }
  console.error(err.message);
  process.exit(1);
}
