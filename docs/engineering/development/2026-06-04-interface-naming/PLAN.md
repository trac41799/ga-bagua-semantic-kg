# Correction Plan: Interface Naming — Semantic Role Labels over Raw Bagua Terminology

**Date:** 2026-06-04
**Status:** 🟢 Applied (2026-06-04)
**Scope:** All public-facing API surfaces (core lib, MCP tools, CLI commands, docs)
**Problem:** The current design returns Bagua trigram names (Qian, Kun, Dui, etc.) to users and LLMs — opaque terms that require a decoding step to be useful.

---

## 1. Root Cause

The product docs treat the Bagua trigrams as *the interface* rather than an *implementation isomorphism*. While the Cl(3) ↔ Bagua mapping is mathematically elegant, raw trigram names at the API boundary create friction:

- LLMs have no training priors on "Xun" or "Gen" as relationship categories
- Users must memorize or reference an 8-term taxonomy to use the tool
- The novelty ("we use I-Ching!") distracts from the actual value (algebraic relationship composition)

## 2. Correct Design

**Bagua is the canonical naming convention for basis blades internally.** The public interface translates to semantic role labels that describe the relationship's *function* in terms any LLM or developer can act on without a decoding step.

### 2.1 The Mapping

| Bagua (Internal) | Semantic Role Label | Meaning | When Used |
|------------------|-------------------|---------|-----------|
| Qian (☰) | `generative` | Introduces, creates, initiates new patterns | Innovation, origin, root cause |
| Kun (☷) | `receptive` | Accepts, follows, grounds | Convention adoption, dependency acceptance |
| Zhen (☳) | `causal` | Triggers, starts a chain reaction | Event-driven relationships |
| Kan (☵) | `transmissive` | Channels, flows, transmits | Data pipelines, communication channels |
| Gen (☶) | `constraining` | Limits, bounds, restricts | Rate limits, permissions, capacity |
| Xun (☴) | `influential` | Pervades, gradually affects | Cultural influence, convention spreading |
| Li (☲) | `clarifying` | Reveals, illuminates, makes visible | Debugging, introspection, documentation |
| Dui (☱) | `balancing` | Mirrors, equilibrates, reflects | Redundancy, feedback loops, mutual dependencies |

### 2.2 Changes by Layer

| Layer | Current | Corrected |
|-------|---------|-----------|
| Rust `semantic_relation()` return | `Trigram` enum | `RelationType` enum (with semantic labels + optional bagua metadata) |
| MCP `classify_relation` response | `{trigram: "Zhen"}` | `{relation_type: "causal", confidence: 0.87, taxonomy: {bagua: "Zhen", wuxing: "Wood"}}` |
| CLI output | `Trigram: Zhen` | `Relation: causal (confidence: 0.87)` |
| CLI `trigram` command | Bagua-only | Rename to `relation-type` — shows definition + bagua + wuxing + semantic label |
| User-facing docs | "8 Bagua trigram categories replace opaque relation labels" | "8 semantic relationship categories grounded in geometric algebra" |
| Bagua reference | Primary documentation | Advanced reference — decoupled from quickstart |

### 2.3 What Stays Internal

- The `Trigram` enum with all Bagua metadata (Chinese names, binary encodings, line transforms)
- The Cl(3) ↔ Bagua blade mapping
- The WuXing cycle system
- Hexagram construction

All of this powers the algebra internally. It's just no longer the *output format*.

### 2.4 Bagua Role in Documentation

| Document | Bagua Coverage |
|----------|---------------|
| README quickstart | None — start with semantic labels |
| Product README | One paragraph explaining the isomorphism exists |
| Mathematical background (math.md) | Full treatment of Cl(3) ↔ Bagua |
| api.md | Mention Bagua as the canonical naming scheme for power users |
| bagua_reference.md | Unchanged — complete reference for those who want it |

---

## 3. Files to Modify

| File | Changes |
|------|---------|
| `docs/product/README.md` | Replace "8 Bagua trigram categories" with "8 semantic role categories" in key value props, problem statement, and competitive position. Add explanation that Bagua is the underlying mathematical mapping. |
| `docs/product/functionalities.md` | §2 (Bagua Taxonomy) → rename "Relation classification" output to `RelationType` with semantic labels. Add note explaining Bagua as internal taxonomy. Update MCP tools table in §8, CLI commands in §9. |
| `docs/product/epics.md` | Epic 2 (Bagua Taxonomy) → add `RelationType` enum as the public type, `Trigram` as internal. Epic 7 (MCP) → update tool response schemas. Epic 8 (CLI) → rename `trigram` command to `relation-type`, update examples. |
| `docs/engineering/technical-stack.md` | Add `RelationType` enum to core data structures. Update API examples to show semantic labels. |
| `docs/engineering/roadmap.md` | No structural changes — the mapping work shifts from "expose Bagua" to "expose semantic labels with Bagua as canonical internal mapping." |

---

## 4. Resolved Decisions

| # | Question | Decision | Rationale |
|---|----------|----------|-----------|
| 1 | Return `RelationType` directly or a richer struct? | **Directly.** Flat string enum at the API boundary. Bagua metadata is opt-in via `?verbose`. | The whole point is zero decode steps for the LLM. |
| 2 | CLI taxonomy command accept semantic labels? | **Yes, and rename to `relation-types`.** Accepts semantic labels as input, shows Bagua/WuXing as secondary info. Symmetric lookup. | The primary interface should be fully bidirectional on its own terms. |
| 3 | `causal` vs `causative`? | **`causal`.** No LLM has ever been confused by that word. | Most common, most intuitive. |
| 4 | WuXing semantic labels? | **Keep as-is.** Five-element names (Wood, Fire, Earth, Metal, Water) are inherently intuitive. WuXing is an internal algebraic driver. | Secondary feature; the phase names already do their job without translation. |

---

## 5. Proposed Implementation Order

1. Introduce `RelationType` enum with 8 semantic labels + `.bagua()` conversion method
2. Update `semantic_relation()` return type from `Trigram` to `RelationType`
3. Update all internal consumers (MCP handlers, CLI commands) to use `RelationType`
4. Rename CLI `trigram` → `relation-type` subcommand
5. Update MCP tool response schemas
6. Rewrite doc sections to lead with semantic labels
7. Add Bagua-as-reference section to advanced docs
