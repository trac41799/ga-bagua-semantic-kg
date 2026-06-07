# Incident: Code Does Not Implement Corrected Interface Naming

**Date:** 2026-06-04
**Status:** 🔴 Open
**Severity:** High — all user-facing interfaces return opaque Bagua trigram names
**Depends On:** `2026-06-04-interface-naming/PLAN.md` (doc corrections applied)
**Test Results:** 82/82 pass (algebra correct, interface wrong)

---

## 1. Root Cause

The code was written before the interface naming correction plan. The docs now describe semantic role labels (`RelationType` enum: generative, causal, constraining, etc.) as the public interface, with Bagua trigrams as internal canonical naming. The code still returns raw `Trigram` enum variants (Kun, Gen, Li, etc.) at every boundary. The `RelationType` enum does not exist in any source file.

## 2. Gap Matrix

| Source | Line(s) | Current Behavior | Required Behavior |
|--------|---------|------------------|-------------------|
| `core/src/semantics.rs` | 34 | `fn semantic_relation() -> Trigram` | `fn semantic_relation() -> RelationType` |
| `core/src/multivector.rs` | 169 | `fn dominant_trigram() -> Trigram` | `fn dominant_role() -> RelationType` (public); keep `dominant_trigram()` as `pub(crate)` |
| `core/src/multivector.rs` | 182 | `fn trigram_weights() -> [f64; 8]` | `fn role_weights() -> [f64; 8]` |
| `core/src/bagua.rs` | 218-223 | `Hexagram::from_multivector_pair()` returns `Hexagram` using `dominant_trigram()` | Keep internal; add `RelationType::from_pair(a, b)` as public alternative |
| `core/src/lib.rs` | 12,17 | Re-exports `Bagua` types as primary API | Re-export `RelationType` as primary; `Trigram` gated behind `advanced` module |
| `mcp/src/main.rs` | 82-89 | `classify_relation` returns `{trigram, hexagram, upper, lower}` | Return `{relation_type, confidence}` |
| `mcp/src/main.rs` | 105-115 | Tool is `trigram_info`, accepts bagua names only | Tool is `relation_type_info`, accepts role labels + bagua names |
| `mcp/src/main.rs` | 170-176 | `parse_trigram()` accepts "kun", "qian" only | Accept role labels AND bagua names; warn if bagua name used |
| `mcp/src/main.rs` | 210 | Description: "Classify relation ... as a Bagua trigram" | "Classify relation ... returning a semantic role label" |
| `cli/src/main.rs` | 28 | `Command::Trigram(TrigramArgs)` | `Command::RelationType(RelationTypeArgs)` |
| `cli/src/main.rs` | 167-179 | `Trigram` subcommand — prints bagua names only | `RelationType` subcommand — prints role def, bagua, wuxing |
| `cli/src/main.rs` | — | No `--json`, `--csv`, `--file`, `--stdin` | Flags needed (see corrected functionalities.md §9.2) |
| `mcp/src/main.rs` | 138-141 | `batch_process` is a stub | Implement actual batch dispatch |
| **(missing)** | — | No `RelationType` enum exists | Needs 8 variants + `.bagua()` + `Display` + `FromStr` |

## 3. New Type to Introduce

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RelationType {
    Generative,     // ☰ Qian — introduces, creates
    Receptive,      // ☷ Kun — accepts, follows
    Causal,         // ☳ Zhen — triggers, starts
    Transmissive,   // ☵ Kan — channels, flows
    Constraining,   // ☶ Gen — limits, bounds
    Influential,    // ☴ Xun — pervades, affects
    Clarifying,     // ☲ Li — reveals, illuminates
    Balancing,      // ☱ Dui — mirrors, equilibrates
}

impl RelationType {
    pub fn bagua(&self) -> Trigram;
    pub fn name(&self) -> &'static str;       // "generative"
    pub fn description(&self) -> &'static str; // "Introduces, creates, initiates new patterns"
    pub fn from_trigram(t: Trigram) -> Self;
    pub fn from_pair(a: &Multivector, b: &Multivector) -> (Self, f64); // type + confidence
}
```

## 4. Files to Modify

| File | Changes |
|------|---------|
| `core/src/relation_type.rs` | **New file.** `RelationType` enum and all methods. |
| `core/src/lib.rs` | Add `pub mod relation_type;`; re-export `RelationType` in prelude; gate `Trigram`/`Hexagram` behind `advanced` feature or `#[doc(hidden)]` |
| `core/src/semantics.rs` | Change `semantic_relation()` return type to `RelationType`; update test `relation_type_returns_trigram` |
| `core/src/multivector.rs` | Add `dominant_role()`; mark `dominant_trigram()` as `pub(crate)`; rename `trigram_weights()` → `role_weights()` |
| `mcp/src/main.rs` | Fix all tool output formats, rename `trigram_info` → `relation_type_info`, update `parse_trigram()`, fix descriptions |
| `cli/src/main.rs` | Rename `Trigram` → `RelationType` command, add `--json`/`--csv` flags, add colored output |
| `core/tests/algebra_tests.rs` | Update if any test asserts on `Trigram` from public API |

## 5. Verification

- [ ] `cargo test -p ga-semantics-core` — all 82 tests still pass after rename
- [ ] `semantic_relation(a, b)` returns `RelationType::Clarifying` when a=e1, b=e2
- [ ] MCP `classify_relation` returns `{"relation_type": "clarifying", "confidence": 1.0}`
- [ ] MCP `relation_type_info` accepts both "causal" and "zhen" as input
- [ ] CLI `ga-semantics relation-type causal` prints role definition triples
- [ ] CLI `ga-semantics classify --json '[...]' '[...]'` prints `{"relation_type":"causal","confidence":0.87}`
- [ ] Bagua internals accessible via `ga_semantics_core::advanced::Trigram::Li`

## 6. Risk

Low. The algebra is unchanged. Only the type returned at the API boundary changes. All internal logic (geometric product, blade mapping, rotor operations) stays identical. The conversion is a pure function: `RelationType::from_trigram(t)`.
