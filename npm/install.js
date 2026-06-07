#!/usr/bin/env node
"use strict";

const { mkdirSync, existsSync, createWriteStream, chmodSync, copyFileSync } = require("fs");
const { join } = require("path");
const { get } = require("https");
const { createGunzip } = require("zlib");
const { Extract } = require("unzipper");
const { pipeline } = require("stream");
const { promisify } = require("util");

const pipelineAsync = promisify(pipeline);

const OWNER = "YOUR_ORG";
const REPO = "ga-bagua-semantic-kg";
const VERSION = "0.1.0";

const PLATFORM_MAP = {
  "win32-x64": "x86_64-pc-windows-msvc",
  "darwin-x64": "x86_64-apple-darwin",
  "darwin-arm64": "aarch64-apple-darwin",
  "linux-x64": "x86_64-unknown-linux-gnu",
  "linux-arm64": "aarch64-unknown-linux-gnu",
};

function getTarget() {
  const platform = process.platform;
  const arch = process.arch === "x64" ? "x64" : process.arch === "arm64" ? "arm64" : null;
  if (!arch) throw new Error(`Unsupported architecture: ${process.arch}`);
  const key = `${platform}-${arch}`;
  const target = PLATFORM_MAP[key];
  if (!target) throw new Error(`Unsupported platform: ${key}`);
  return target;
}

function getBinaryName(target) {
  const base = `ga-semantics-mcp-${target}`;
  return process.platform === "win32" ? `${base}.exe` : base;
}

function getArchiveName(target) {
  const base = `ga-semantics-mcp-${target}`;
  return process.platform === "win32" ? `${base}.zip` : `${base}.tar.gz`;
}

async function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = createWriteStream(dest);
    get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        return download(response.headers.location, dest).then(resolve).catch(reject);
      }
      if (response.statusCode !== 200) {
        return reject(new Error(`Download failed: ${response.statusCode}`));
      }
      pipeline(response, file, (err) => {
        if (err) reject(err);
        else resolve();
      });
    }).on("error", reject);
  });
}

async function extract(archive, destDir, isZip) {
  if (isZip) {
    const unzipper = require("unzipper");
    const fs = require("fs");
    return new Promise((resolve, reject) => {
      fs.createReadStream(archive)
        .pipe(unzipper.Extract({ path: destDir }))
        .on("close", resolve)
        .on("error", reject);
    });
  } else {
    const tar = require("tar");
    await tar.extract({ file: archive, cwd: destDir });
  }
}

async function main() {
  const target = getTarget();
  const binaryName = getBinaryName(target);
  const archiveName = getArchiveName(target);
  const binDir = join(__dirname, "bin");
  const binaryPath = join(binDir, binaryName);

  // Skip if already installed with the right version marker
  const markerFile = join(binDir, `.installed-${VERSION}`);
  if (existsSync(markerFile) && existsSync(binaryPath)) {
    console.log(`[ga-semantics-mcp] v${VERSION} already installed`);
    return;
  }

  console.log(`[ga-semantics-mcp] Installing v${VERSION} for ${target}...`);

  mkdirSync(binDir, { recursive: true });

  const url = `https://github.com/${OWNER}/${REPO}/releases/download/v${VERSION}/${archiveName}`;

  try {
    const archivePath = join(__dirname, archiveName);
    console.log(`  Downloading ${url}...`);
    await download(url, archivePath);

    console.log(`  Extracting...`);
    await extract(archivePath, binDir, process.platform === "win32");

    // The binary might be nested. Find it.
    const { readdirSync, renameSync, unlinkSync } = require("fs");
    const entries = readdirSync(binDir);
    for (const entry of entries) {
      if (entry.endsWith(".exe") || (!entry.includes(".") && entry.startsWith("ga-semantics-mcp-"))) {
        const found = join(binDir, entry);
        if (found !== binaryPath) {
          renameSync(found, binaryPath);
        }
        break;
      }
    }

    // Make executable on Unix
    if (process.platform !== "win32") {
      chmodSync(binaryPath, 0o755);
    }

    // Clean up archive
    try { unlinkSync(archivePath); } catch (_) {}

    // Write marker
    const { writeFileSync } = require("fs");
    writeFileSync(markerFile, VERSION);

    console.log(`  Installed: ${binaryPath}`);
  } catch (err) {
    console.error(`[ga-semantics-mcp] Install failed: ${err.message}`);
    console.error("  Falling back to building from source via: cargo install ga-semantics-mcp");
    process.exit(1);
  }
}

main();
