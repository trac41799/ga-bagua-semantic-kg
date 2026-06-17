#!/usr/bin/env node
"use strict";

const fs = require("fs");
const fsp = require("fs/promises");
const path = require("path");
const https = require("https");
const { pipeline } = require("stream");
const { promisify } = require("util");
const pipelineAsync = promisify(pipeline);

const OWNER = "trac41799";
const REPO = "ga-bagua-semantic-kg";
const VERSION = "0.1.7";

const RED = "\x1b[31m";
const GREEN = "\x1b[32m";
const YELLOW = "\x1b[33m";
const BLUE = "\x1b[34m";
const CYAN = "\x1b[36m";
const BOLD = "\x1b[1m";
const DIM = "\x1b[2m";
const RESET = "\x1b[0m";

const PLATFORM_MAP = {
  "win32-x64": "x86_64-pc-windows-msvc",
  "darwin-x64": "x86_64-apple-darwin",
  "darwin-arm64": "aarch64-apple-darwin",
  "linux-x64": "x86_64-unknown-linux-gnu",
  "linux-arm64": "aarch64-unknown-linux-gnu",
};

let quiet = false;
let yes = false;

function log(...args) {
  if (!quiet) console.log(...args);
}
function info(...args) {
  console.log(`${BLUE}${BOLD}[info]${RESET}`, ...args);
}
function success(...args) {
  console.log(`${GREEN}${BOLD}[ok]${RESET}`, ...args);
}
function warn(...args) {
  console.log(`${YELLOW}${BOLD}[warn]${RESET}`, ...args);
}
function err(...args) {
  console.error(`${RED}${BOLD}[error]${RESET}`, ...args);
}
function header(text) {
  console.log(`\n${BOLD}${CYAN}${"=".repeat(60)}${RESET}`);
  console.log(`${BOLD}${CYAN}  ${text}${RESET}`);
  console.log(`${BOLD}${CYAN}${"=".repeat(60)}${RESET}\n`);
}

function getHome() {
  return process.env.HOME || process.env.USERPROFILE || process.env.HOMEDRIVE + process.env.HOMEPATH || require("os").homedir();
}

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
  const base = `ga-semantics-${target}`;
  return process.platform === "win32" ? `${base}.zip` : `${base}.tar.gz`;
}

async function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    let downloaded = 0;
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        file.close();
        fs.unlinkSync(dest);
        return downloadFile(response.headers.location, dest).then(resolve).catch(reject);
      }
      if (response.statusCode !== 200) {
        file.close();
        fs.unlinkSync(dest);
        return reject(new Error(`Download failed: HTTP ${response.statusCode}`));
      }
      const total = parseInt(response.headers["content-length"], 10) || 0;
      response.on("data", (chunk) => {
        downloaded += chunk.length;
        if (total > 0 && !quiet) {
          const pct = Math.round((downloaded / total) * 100);
          process.stdout.write(`\r  Downloading... ${pct}%`);
        }
      });
      pipeline(response, file, (e) => {
        if (!quiet) process.stdout.write("\r" + " ".repeat(30) + "\r");
        if (e) {
          try { fs.unlinkSync(dest); } catch (_) {}
          reject(e);
        } else {
          resolve();
        }
      });
    }).on("error", reject);
  });
}

async function extractArchive(archivePath, destDir) {
  return new Promise((resolve, reject) => {
    if (process.platform === "win32" && archivePath.endsWith(".zip")) {
      const unzipper = require("unzipper");
      fs.createReadStream(archivePath)
        .pipe(unzipper.Extract({ path: destDir }))
        .on("close", resolve)
        .on("error", reject);
    } else {
      const zlib = require("zlib");
      const tar = require("tar");
      fs.createReadStream(archivePath)
        .pipe(zlib.createGunzip())
        .pipe(tar.extract({ cwd: destDir }))
        .on("finish", resolve)
        .on("error", reject);
    }
  });
}

async function downloadBinary(target, installDir) {
  const binaryName = getBinaryName(target);
  const binaryPath = path.join(installDir, binaryName);

  const markerFile = path.join(installDir, `.installed-${VERSION}`);
  if (fs.existsSync(markerFile) && fs.existsSync(binaryPath)) {
    info(`Binary v${VERSION} already installed at ${binaryPath}`);
    return binaryPath;
  }

  info(`Downloading GA-Bagua MCP server v${VERSION} for ${target}...`);

  const archiveName = getArchiveName(target);
  const url = `https://github.com/${OWNER}/${REPO}/releases/download/v${VERSION}/${archiveName}`;
  const tmpDir = path.join(installDir, ".tmp");
  const archivePath = path.join(tmpDir, archiveName);

  fs.mkdirSync(tmpDir, { recursive: true });

  log(`  From: ${url}`);
  await downloadFile(url, archivePath);

  log("  Extracting...");
  fs.mkdirSync(installDir, { recursive: true });
  await extractArchive(archivePath, installDir);

  const entries = fs.readdirSync(installDir);
  for (const entry of entries) {
    const full = path.join(installDir, entry);
    if (full === binaryPath) continue;
    const stat = fs.statSync(full);
    if (stat.isDirectory()) continue;
    if (entry.endsWith(".exe") || (!entry.includes(".") && entry.startsWith("ga-semantics-mcp-"))) {
      if (full !== binaryPath) {
        fs.renameSync(full, binaryPath);
      }
      break;
    }
  }

  if (process.platform !== "win32") {
    try { fs.chmodSync(binaryPath, 0o755); } catch (_) {}
  }

  try { fs.rmSync(tmpDir, { recursive: true, force: true }); } catch (_) {}
  fs.writeFileSync(markerFile, VERSION);

  success(`Installed: ${binaryPath}`);
  return binaryPath;
}

async function downloadSkillFile(destDir) {
  const destPath = path.join(destDir, "bagua-encoder", "SKILL.md");
  if (fs.existsSync(destPath)) {
    return null;
  }

  const url = `https://raw.githubusercontent.com/${OWNER}/${REPO}/refs/tags/v${VERSION}/docs/skills/bagua-encoder/SKILL.md`;
  log(`  Downloading encoding skill...`);

  return new Promise((resolve, reject) => {
    https.get(url, (response) => {
      if (response.statusCode !== 200) {
        return reject(new Error(`Skill download failed: HTTP ${response.statusCode}`));
      }
      let data = "";
      response.on("data", (chunk) => { data += chunk; });
      response.on("end", () => {
        fs.mkdirSync(path.dirname(destPath), { recursive: true });
        fs.writeFileSync(destPath, data, "utf8");
        resolve(destPath);
      });
    }).on("error", reject);
  });
}

const AGENT_DEFS = {
  "claude-desktop": {
    label: "Claude Desktop",
    detects(home) {
      if (process.platform === "win32") {
        const p = path.join(process.env.APPDATA || path.join(home, "AppData", "Roaming"), "Claude", "claude_desktop_config.json");
        return { configPath: p, found: fs.existsSync(p) };
      } else if (process.platform === "darwin") {
        const p = path.join(home, "Library", "Application Support", "Claude", "claude_desktop_config.json");
        return { configPath: p, found: fs.existsSync(p) };
      } else {
        const p = path.join(home, ".config", "Claude", "claude_desktop_config.json");
        return { configPath: p, found: fs.existsSync(p) };
      }
    },
    configKey: "mcpServers",
    configFormat: "json",
    skillDirs(home) {
      return []; // No standardized skill dir for Claude Desktop
    },
  },
  "claude-code": {
    label: "Claude Code (CLI)",
    detects(home) {
      const mcpPath = path.join(home, ".claude", "mcp.json");
      const legacyPath = path.join(home, ".claude", "claude_desktop_config.json");
      const exists = fs.existsSync(path.join(home, ".claude"));
      if (fs.existsSync(mcpPath)) return { configPath: mcpPath, found: true };
      if (fs.existsSync(legacyPath)) return { configPath: legacyPath, found: true };
      return { configPath: mcpPath, found: exists }; // dir exists but no config yet
    },
    configKey: "mcpServers",
    configFormat: "json",
    skillDirs(home) {
      const dirs = [];
      const global = path.join(home, ".claude", "skills");
      const project = path.join(process.cwd(), ".claude", "skills");
      try { if (fs.statSync(path.join(home, ".claude")).isDirectory()) dirs.push(global); } catch (_) {}
      dirs.push(project);
      return dirs;
    },
    claudeMdPath(home) {
      return path.join(process.cwd(), "CLAUDE.md");
    },
  },
  opencode: {
    label: "OpenCode",
    detects(home) {
      const globalPath = path.join(home, ".config", "opencode", "opencode.json");
      const projectPath = path.join(process.cwd(), ".opencode", "opencode.json");
      const globalDir = path.join(home, ".config", "opencode");
      const projectDir = path.join(process.cwd(), ".opencode");
      if (fs.existsSync(globalPath)) return { configPath: globalPath, found: true };
      if (fs.existsSync(projectPath)) return { configPath: projectPath, found: true };
      if (fs.existsSync(globalDir)) return { configPath: globalPath, found: true };
      if (fs.existsSync(projectDir)) return { configPath: projectPath, found: true };
      return { configPath: globalPath, found: false };
    },
    configKey: "mcpServers",
    configFormat: "json",
    skillDirs(home) {
      const dirs = [];
      const global = path.join(home, ".config", "opencode", "skills");
      const project = path.join(process.cwd(), ".opencode", "skills");
      if (fs.existsSync(path.join(home, ".config", "opencode"))) dirs.push(global);
      dirs.push(project);
      return dirs;
    },
  },
  cursor: {
    label: "Cursor",
    detects(home) {
      const globalPath = path.join(home, ".cursor", "mcp.json");
      const projectPath = path.join(process.cwd(), ".cursor", "mcp.json");
      const globalDir = path.join(home, ".cursor");
      const projectDir = path.join(process.cwd(), ".cursor");
      if (fs.existsSync(globalPath)) return { configPath: globalPath, found: true };
      if (fs.existsSync(projectPath)) return { configPath: projectPath, found: true };
      if (fs.existsSync(globalDir)) return { configPath: globalPath, found: true };
      if (fs.existsSync(projectDir)) return { configPath: projectPath, found: true };
      return { configPath: globalPath, found: false };
    },
    configKey: "mcpServers",
    configFormat: "json",
    skillDirs(home) {
      const dirs = [];
      const global = path.join(home, ".cursor", "skills");
      const project = path.join(process.cwd(), ".cursor", "skills");
      if (fs.existsSync(path.join(home, ".cursor"))) dirs.push(global);
      dirs.push(project);
      return dirs;
    },
  },
  continue: {
    label: "Continue.dev",
    detects(home) {
      const globalPath = path.join(home, ".continue", "config.json");
      const exists = fs.existsSync(globalPath);
      return { configPath: globalPath, found: exists };
    },
    configKey: "experimental.modelContextProtocolServers",
    configFormat: "json",
    skillDirs(home) {
      const dirs = [];
      const global = path.join(home, ".continue", "skills");
      if (fs.existsSync(path.join(home, ".continue"))) dirs.push(global);
      return dirs;
    },
    mcpEntry(binaryPath) {
      return { name: "ga-bagua-semantic-kg", command: binaryPath };
    },
    mergeConfig(existing, binaryPath) {
      const config = existing || {};
      if (!config.experimental) config.experimental = {};
      if (!config.experimental.modelContextProtocolServers) config.experimental.modelContextProtocolServers = [];
      const servers = config.experimental.modelContextProtocolServers;
      const exists = servers.some((s) => s.name === "ga-bagua-semantic-kg");
      if (!exists) {
        servers.push({ name: "ga-bagua-semantic-kg", command: binaryPath });
      }
      return config;
    },
  },
  windsurf: {
    label: "Windsurf",
    detects(home) {
      const globalPath = path.join(home, ".windsurf", "mcp_config.json");
      const exists = fs.existsSync(globalPath) || fs.existsSync(path.join(home, ".windsurf"));
      return { configPath: globalPath, found: exists };
    },
    configKey: "mcpServers",
    configFormat: "json",
    skillDirs(home) {
      const dirs = [];
      const global = path.join(home, ".windsurf", "skills");
      if (fs.existsSync(path.join(home, ".windsurf"))) dirs.push(global);
      return dirs;
    },
  },
  aider: {
    label: "Aider",
    detects(home) {
      const globalPath = path.join(home, ".aider.conf.yml");
      const projectPath = path.join(process.cwd(), ".aider.conf.yml");
      if (fs.existsSync(globalPath)) return { configPath: globalPath, found: true };
      if (fs.existsSync(projectPath)) return { configPath: projectPath, found: true };
      return { configPath: globalPath, found: false };
    },
    configKey: null,
    configFormat: "yaml",
    skillDirs(home) {
      return [];
    },
  },
  cline: {
    label: "Cline (VS Code)",
    detects(home) {
      const extDir = path.join(home, ".vscode", "extensions");
      if (!fs.existsSync(extDir)) return { configPath: null, found: false };
      const entries = fs.readdirSync(extDir);
      const found = entries.some((e) => e.toLowerCase().includes("cline"));
      return { configPath: null, found };
    },
    configKey: null,
    configFormat: "manual",
    skillDirs(home) {
      const dirs = [];
      const project = path.join(process.cwd(), ".cline", "skills");
      dirs.push(project);
      return dirs;
    },
  },
  codex: {
    label: "Codex (OpenAI)",
    detects(home) {
      const codexDir = process.platform === "win32" ? path.join(home, ".config", "codex") : path.join(home, ".config", "codex");
      return { configPath: null, found: fs.existsSync(codexDir) };
    },
    configKey: null,
    configFormat: "manual",
    skillDirs(home) {
      return [];
    },
  },
};

function detectAllAgents(home) {
  const detected = [];
  for (const [id, def] of Object.entries(AGENT_DEFS)) {
    try {
      const result = def.detects(home);
      if (result.found || (result.configPath && fs.existsSync(result.configPath)) || def.label.includes("(")) {
        detected.push({ id, ...result, def });
      }
    } catch (_) {
      /* skip agent if detection fails */
    }
  }

  // Always include OpenCode and Cursor project-level options even if not detected
  for (const id of ["opencode", "cursor", "claude-code"]) {
    if (!detected.some((d) => d.id === id)) {
      const def = AGENT_DEFS[id];
      try {
        const result = def.detects(home);
        if (result.found) detected.push({ id, ...result, def });
      } catch (_) {}
    }
  }

  return detected;
}

function readJsonConfig(configPath) {
  try {
    if (fs.existsSync(configPath)) {
      const raw = fs.readFileSync(configPath, "utf8");
      return JSON.parse(raw);
    }
  } catch (_) {}
  return null;
}

function installJsonMCPConfig(configPath, serverName, binaryPath, configKey) {
  let config = readJsonConfig(configPath) || {};

  if (configKey === "mcpServers") {
    if (!config.mcpServers) config.mcpServers = {};
    if (config.mcpServers[serverName]) {
      return null; // Already configured
    }
    config.mcpServers[serverName] = { command: binaryPath };
  }

  const dir = path.dirname(configPath);
  fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(configPath, JSON.stringify(config, null, 2) + "\n", "utf8");
  return configPath;
}

function installContinueMCPConfig(configPath, binaryPath) {
  const def = AGENT_DEFS["continue"];
  let existing;
  try { existing = readJsonConfig(configPath); } catch (_) { existing = null; }
  const config = def.mergeConfig(existing, binaryPath);
  const dir = path.dirname(configPath);
  fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(configPath, JSON.stringify(config, null, 2) + "\n", "utf8");
  return configPath;
}

async function installSkillForAgent(agentId, home) {
  const def = AGENT_DEFS[agentId];
  if (!def || !def.skillDirs) return [];

  const skillDirs = def.skillDirs(home);
  const installed = [];

  for (const skillDir of skillDirs) {
    if (!skillDir) continue;
    try {
      const skillPath = path.join(skillDir, "bagua-encoder", "SKILL.md");
      if (fs.existsSync(skillPath)) {
        log(`    ${DIM}(skill already exists: ${skillPath})${RESET}`);
        installed.push(skillPath);
        continue;
      }
      await downloadSkillFile(skillDir);
      if (fs.existsSync(skillPath)) {
        log(`    ${GREEN}Skill installed:${RESET} ${skillPath}`);
        installed.push(skillPath);
      }
    } catch (e) {
      warn(`  Could not install skill to ${skillDir}: ${e.message}`);
    }
  }

  return installed;
}

function showHelp() {
  console.log(`
${BOLD}GA-Bagua Semantic KG — Setup${RESET}

${CYAN}Usage:${RESET}
  node setup.js [options]
  npx ga-semantics-setup [options]

${CYAN}One-liner (no npm needed):${RESET}
  ${DIM}# Unix / macOS${RESET}
  curl -fsSL https://raw.githubusercontent.com/${OWNER}/${REPO}/main/npm/setup.js | node -

  ${DIM}# Windows PowerShell${RESET}
  Invoke-WebRequest -Uri "https://raw.githubusercontent.com/${OWNER}/${REPO}/main/npm/setup.js" -OutFile "$env:TEMP\\ga-setup.js"; node "$env:TEMP\\ga-setup.js"

${CYAN}Options:${RESET}
  --help              Show this help
  --quiet             Minimal output
  --yes               Auto-confirm all prompts
  --path <dir>        Binary install directory (default: ~/.ga-semantics/bin)
  --skip-binary       Skip binary download (use existing)
  --skip-config       Skip MCP config installation
  --skip-skills       Skip skill file installation
  --agents <list>     Only configure specific agents (comma-separated)
  --list-agents       List detected agents without installing

${CYAN}What it does:${RESET}
  1. Downloads the correct GA-Bagua MCP binary for your platform
  2. Auto-detects installed coding agent harnesses
  3. Installs MCP server config for each detected agent
  4. Installs the Bagua encoder skill into agent skill directories

${CYAN}Supported agents:${RESET}
  Claude Desktop, Claude Code (CLI), OpenCode, Cursor, Continue.dev,
  Windsurf, Aider, Cline (VS Code), Codex (OpenAI)
`);
}

async function main() {
  const args = process.argv.slice(2);

  if (args.includes("--help") || args.includes("-h")) {
    showHelp();
    return;
  }

  if (args.includes("--list-agents")) {
    header("Detected Coding Agent Harnesses");
    const home = getHome();
    const detected = detectAllAgents(home);
    for (const d of detected) {
      const icon = d.found ? `${GREEN}[DETECTED]${RESET}` : `${DIM}[not found]${RESET}`;
      console.log(`  ${icon} ${d.def.label}`);
      const cfgPath = d.configPath || (d.def.detects ? d.def.detects(home).configPath : null);
      if (cfgPath) console.log(`    Config: ${DIM}${cfgPath}${RESET}`);
      const skillDirs = d.def.skillDirs ? d.def.skillDirs(home) : [];
      for (const sd of skillDirs) {
        console.log(`    Skills: ${DIM}${sd}${RESET}`);
      }
    }
    return;
  }

  quiet = args.includes("--quiet");
  yes = args.includes("--yes");

  const skipBinary = args.includes("--skip-binary");
  const skipConfig = args.includes("--skip-config");
  const skipSkills = args.includes("--skip-skills");

  const agentsArg = args.find((a, i) => (a === "--agents" || a === "-a") && i + 1 < args.length);
  const agentsFilter = agentsArg ? args[args.indexOf(agentsArg) + 1].split(",").map((s) => s.trim().toLowerCase()) : null;

  const pathArg = args.find((a, i) => (a === "--path" || a === "-p") && i + 1 < args.length);
  const customPath = pathArg ? args[args.indexOf(pathArg) + 1] : null;

  const home = getHome();
  const target = getTarget();
  const installDir = customPath || path.join(home, ".ga-semantics", "bin");

  header("GA-Bagua Semantic KG Setup");

  // Step 1: Download binary
  let binaryPath = null;
  if (!skipBinary) {
    try {
      binaryPath = await downloadBinary(target, installDir);
    } catch (e) {
      err(`Binary download failed: ${e.message}`);
      console.log(`${YELLOW}  You can download manually from:${RESET}`);
      console.log(`  https://github.com/${OWNER}/${REPO}/releases/latest`);
      if (!skipConfig && !skipSkills) {
        console.log(`  Then re-run with --skip-binary to configure agents only.`);
      }
      binaryPath = null;
    }
  } else {
    binaryPath = getBinaryName(target);
    // Try to find existing binary
    const candidates = [
      path.join(installDir, binaryPath),
      path.join(home, ".ga-semantics", "bin", binaryPath),
      path.join(__dirname, "bin", binaryPath),
    ];
    for (const c of candidates) {
      if (fs.existsSync(c)) {
        binaryPath = c;
        break;
      }
    }
    if (fs.existsSync(binaryPath)) {
      info(`Using existing binary: ${binaryPath}`);
    } else {
      warn("No existing binary found. MCP configs will use command name 'ga-semantics-mcp'.");
      binaryPath = "ga-semantics-mcp";
    }
  }

  if (skipConfig && skipSkills) {
    if (binaryPath && binaryPath !== "ga-semantics-mcp") success(`Binary ready: ${binaryPath}`);
    return;
  }

  // Step 2: Detect agents
  header("Detecting Coding Agent Harnesses");
  let detected = detectAllAgents(home);

  if (agentsFilter) {
    detected = detected.filter((d) => agentsFilter.includes(d.id));
    if (detected.length === 0) {
      warn(`No detected agents match filter: ${agentsFilter.join(", ")}`);
      console.log(`  Available: ${Object.keys(AGENT_DEFS).join(", ")}`);
    }
  }

  if (detected.length === 0) {
    warn("No coding agent harnesses detected on this system.");
    console.log("  Skipping MCP config and skill installation.");
    return;
  }

  for (const d of detected) {
    const icon = d.found ? `${GREEN}✓${RESET}` : `${YELLOW}?${RESET}`;
    console.log(`  ${icon} ${d.def.label} ${d.found ? "" : DIM + "(not detected, will create config)" + RESET}`);
  }

  // Step 3: Install MCP config
  if (!skipConfig) {
    header("Installing MCP Server Configuration");

    const serverName = "ga-bagua-semantic-kg";
    let configured = 0;
    let skipped = 0;
    const manualAgents = [];

    for (const d of detected) {
      const label = d.def.label;

      if (d.def.configFormat === "manual" || !d.configPath || !d.def.configKey) {
        if (d.def.configFormat === "manual") {
          manualAgents.push(d);
        }
        continue;
      }

      try {
        if (d.def.configKey === "experimental.modelContextProtocolServers") {
          const result = installContinueMCPConfig(d.configPath, binaryPath);
          if (result) {
            success(`${label}: ${result}`);
            configured++;
          } else {
            log(`  ${YELLOW}⊘${RESET} ${label}: Already configured`);
            skipped++;
          }
        } else {
          const result = installJsonMCPConfig(d.configPath, serverName, binaryPath, d.def.configKey);
          if (result) {
            success(`${label}: ${result}`);
            configured++;
          } else if (d.found) {
            log(`  ${YELLOW}⊘${RESET} ${label}: Already configured`);
            skipped++;
          } else {
            configured++;
          }
        }
      } catch (e) {
        err(`${label}: ${e.message}`);
      }
    }

    // Handle Aider (YAML config)
    const aiderDetected = detected.filter((d) => d.id === "aider" && d.found);
    for (const d of aiderDetected) {
      console.log(`\n  ${YELLOW}[aider]${RESET} YAML config detected. Add manually to ${d.configPath}:`);
      console.log(`${DIM}    mcp_servers:${RESET}`);
      console.log(`${DIM}      - name: ga-bagua-semantic-kg${RESET}`);
      console.log(`${DIM}        command: ${binaryPath}${RESET}`);
    }

    // Handle Cline (VS Code manual)
    for (const d of manualAgents) {
      if (d.id === "cline") {
        console.log(`\n  ${YELLOW}[cline]${RESET} MCP config must be added via VS Code.`);
        console.log(`    Open VS Code > Cline > MCP Servers > Add server:`);
        console.log(`${DIM}    Name: ga-bagua-semantic-kg${RESET}`);
        console.log(`${DIM}    Command: ${binaryPath}${RESET}`);
        manualAgents.length = 0;
      }
      if (d.id === "codex") {
        console.log(`\n  ${YELLOW}[codex]${RESET} Codex does not natively support MCP.`);
        console.log(`    Use the CLI tool directly:`);
        console.log(`${DIM}    npx ga-semantics-cli encode <coefficients>${RESET}`);
      }
    }

    if (configured > 0) console.log(`\n  ${GREEN}Configured: ${configured}, Skipped: ${skipped}${RESET}`);
  }

  // Step 4: Install encoding skill
  if (!skipSkills) {
    header("Installing Bagua Encoder Skill");

    let skillsInstalled = 0;
    for (const d of detected) {
      if (d.def.skillDirs && d.def.skillDirs.length > 0) {
        log(`  ${d.def.label}:`);
        try {
          const installed = await installSkillForAgent(d.id, home);
          skillsInstalled += installed.length;
        } catch (e) {
          err(`  ${d.def.label}: ${e.message}`);
        }
      }
    }

    if (skillsInstalled > 0) {
      console.log(`\n  ${GREEN}Skills installed: ${skillsInstalled}${RESET}`);
    }
  }

  // Step 5: Summary
  header("Setup Complete");
  console.log(`  Binary: ${GREEN}${binaryPath}${RESET}`);
  console.log(`  Version: v${VERSION}`);
  console.log(`  Platform: ${target}`);
  console.log();
  console.log(`  To test: echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ${binaryPath}`);
  console.log();
  console.log(`  ${BOLD}Next steps:${RESET}`);
  console.log(`  1. Restart your coding agent (Claude Desktop, Cursor, etc.)`);
  console.log(`  2. Verify the hammer icon (MCP tools) appears`);
  console.log(`  3. Ask your LLM: "Read the bagua-encoder skill and encode concept X"`);
}

main().catch((e) => {
  err(`Setup failed: ${e.message}`);
  if (!quiet) console.error(e.stack);
  process.exit(1);
});
