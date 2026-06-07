#!/usr/bin/env node
"use strict";
// run.js — proxy to the platform binary

const { spawnSync } = require("child_process");
const { join } = require("path");

const platform = process.platform;
const arch = process.arch === "x64" ? "x86_64" : process.arch === "arm64" ? "aarch64" : null;

const TARGET_MAP = {
  "win32-x86_64": "x86_64-pc-windows-msvc",
  "darwin-x86_64": "x86_64-apple-darwin",
  "darwin-aarch64": "aarch64-apple-darwin",
  "linux-x86_64": "x86_64-unknown-linux-gnu",
  "linux-aarch64": "aarch64-unknown-linux-gnu",
};

const key = `${platform}-${arch}`;
const target = TARGET_MAP[key];
if (!target) {
  console.error(`Unsupported platform: ${key}`);
  process.exit(1);
}

const ext = platform === "win32" ? ".exe" : "";
const binaryPath = join(__dirname, "bin", `ga-semantics-mcp-${target}${ext}`);

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: "inherit",
});

process.exit(result.status ?? 1);
