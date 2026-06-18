# MCP Servers Listing — Submission Checklist

Target: [awesome-mcp-servers](https://github.com/punkpeye/awesome-mcp-servers)  
Repo: `trac41799/ga-bagua-semantic-kg`  
Category: 🧠 **Knowledge & Memory**

---

## Status

- **PRs open**: 1,506
- **Backlog is massive** — PRs get auto-labeled by bots. Only "clean" PRs (no error labels) get fast-tracked.

- **Label** | **Required** | **Status**
  -----------|--------------|----------
  `valid-name` | Format: `owner/repo` | ✅ `trac41799/ga-bagua-semantic-kg`
  `has-emoji` | Language + scope + OS emoji icons | ⚠️ Missing (see below)
  `has-glama` | glama.ai score badge + link | ⚠️ Missing (see below)

Any PR missing these labels sits forever.

---

## 1. Pre-Flight: Repository Readiness

### 1.1 Fix the broken npm install (CRITICAL)

- [ ] **Publish npm v0.1.7** to fix `Cannot find module 'unzipper'` — already committed but needs `npm publish` from `npm/` directory
- [ ] **Create GitHub release v0.1.7** with platform binaries (same naming as v0.1.6). Otherwise `npm install ga-semantics-mcp` downloads 404.

### 1.2 Repository polish

- [ ] Add GitHub topics: `mcp`, `mcp-server`, `geometric-algebra`, `llm-memory`, `rust`, `knowledge-graph`, `claude-desktop`, `ai-agents`
- [ ] Verify the npm badge on README shows correctly (currently shows v0.1.7 after publish)
- [ ] Verify the crates.io core badge shows correctly

---

## 2. PR Content: The Exact Line to Add

The format used by every listing in the repo:

```
- [owner/repo](https://github.com/owner/repo) [![owner/repo MCP server](https://glama.ai/mcp/servers/owner/repo/badges/score.svg)](https://glama.ai/mcp/servers/owner/repo) 🦀 🏠 🍎 🪟 🐧 - Description. `npx ga-semantics-mcp`
```

### 2.1 Entry for our repo

```
- [trac41799/ga-bagua-semantic-kg](https://github.com/trac41799/ga-bagua-semantic-kg) [![trac41799/ga-bagua-semantic-kg MCP server](https://glama.ai/mcp/servers/trac41799/ga-bagua-semantic-kg/badges/score.svg)](https://glama.ai/mcp/servers/trac41799/ga-bagua-semantic-kg) 🦀 🏠 🍎 🪟 🐧 - Zero-token, 500ns LLM semantic memory layer using Cl(3) Geometric Algebra and Bagua trigrams. 64-byte concept vectors with algebraic retrieval, WuXing path traversal, and relation classification. `npx ga-semantics-mcp`
```

### 2.2 Emoji legend

- 🦀 = Rust codebase
- 🏠 = Local service (runs on-device as MCP server)
- 🍎 🪟 🐧 = macOS, Windows, Linux support

### 2.3 Placement

Insert alphabetically under `### 🧠 Knowledge & Memory` section. Find the correct position between neighboring entries. Search the README for `🧠` to locate the section.

---

## 3. The glama.ai Badge (REQUIRED for `has-glama` label)

The badge URL pattern is deterministic — it uses the GitHub `owner/repo` slug:

```
Badge: https://glama.ai/mcp/servers/trac41799/ga-bagua-semantic-kg/badges/score.svg
Link:  https://glama.ai/mcp/servers/trac41799/ga-bagua-semantic-kg
```

The badge will auto-generate once the repo is indexed by glama.ai. To trigger indexing:

- [ ] Go to https://glama.ai/mcp/servers and submit `trac41799/ga-bagua-semantic-kg`
- [ ] Verify the badge renders (may take a few hours)

> **If the badge returns 404 or shows an error at PR time**: the `missing-glama` label will be applied but PRs can still merge. However, merged PRs with `missing-glama` may rank lower.

---

## 4. The `🤖🤖🤖` Fast-Track

Per CONTRIBUTING.md:

> If you are an automated agent, we have a streamlined process for merging agent PRs. Just add `🤖🤖🤖` to the end of the PR title to opt-in.

- [ ] Add `🤖🤖🤖` to PR title: `Add trac41799/ga-bagua-semantic-kg to Knowledge & Memory 🤖🤖🤖`

This bypasses the human review queue and gets auto-merged if all labels pass.

---

## 5. Step-by-Step PR Process

- [ ] 1. Fork `punkpeye/awesome-mcp-servers`
- [ ] 2. Create branch `add-ga-bagua-semantic-kg`
- [ ] 3. Edit `README.md` — insert the line from Section 2.1 into the 🧠 Knowledge & Memory section in **alphabetical order**
- [ ] 4. Commit: `Add trac41799/ga-bagua-semantic-kg to Knowledge & Memory`
- [ ] 5. Push and open PR with title: `Add trac41799/ga-bagua-semantic-kg to Knowledge & Memory 🤖🤖🤖`
- [ ] 6. Wait for bot labels. If any error labels appear (`invalid-name`, `non-github-url`, `duplicate`), fix and push to the same branch.

---

## 6. Maintaining the Slot (Post-Merge)

Once merged, your listing lives in the README. Here's how to keep it:

### 6.1 Never let the badge break

- The glama.ai badge is dynamic. If the repo goes private, gets renamed, or the MCP server stops working, the badge will show an error.
- ✅ This project is public and unlikely to be renamed — low risk.

### 6.2 If the description needs updating

- [ ] Open a **new PR** to the same repo
- [ ] Find your entry in the README (it stays in the same category)
- [ ] Edit only that line
- [ ] Use the same `🤖🤖🤖` title suffix for fast-track

### 6.3 Watch for "cleanup" PRs

- The maintainers occasionally purge entries with broken badges or dead repos
- Keep the npm package published and functional (`npm install ga-semantics-mcp` must work)
- Keep the GitHub release assets available (don't delete old release binaries)

### 6.4 Category stability

- 🧠 Knowledge & Memory is the correct category. Do not request recategorization unless the project fundamentally changes scope.
- Moving categories requires a new PR and may lose the slot position.

---

## 7. Risk Factors

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| PR drowned in 1,500+ queue | High | Use `🤖🤖🤖` fast-track |
| glama.ai badge 404 | Medium | Submit to glama.ai first, wait for indexing |
| Name rejected (`invalid-name`) | Low | `trac41799/ga-bagua-semantic-kg` follows format |
| Duplicate detected | Low | No existing entry for this repo |
| npm install still broken | High | **Fix before submitting PR** — v0.1.7 must work |
> This checklist was generated based on live analysis of the repo's CONTRIBUTING.md, PR labels, and entry format as of 2026-06-18.
