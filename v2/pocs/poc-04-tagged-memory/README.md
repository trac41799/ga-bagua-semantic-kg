# POC-04 — Interpretable-Tag Agent Memory

**Status: BUILT AND VALIDATED — tag-layer claim FAILED (tag quality and filter precision below bars); stability and non-interference PASS**

## Verdict (2026-08-08, real LLM, 30 reference-tagged items, 10 filter queries)

| Gate | Value | Criterion | |
|------|-------|-----------|---|
| Tag quality (LLM dominant role == human) | **66.7%** | ≥ 80% | **FAIL — kill criterion fired** |
| Stability (dominant role across 2 runs) | **83.3%** | ≥ 80% | PASS |
| Filter precision | **0.46** | ≥ 0.50 | FAIL |
| Non-interference (tagged ≡ untagged ranking) | True | True | PASS |

**Honest interpretation:** the tag layer never interferes with retrieval (non-interference holds), and tags are stable across runs — but the LLM's dominant-role assignment agrees with the human annotator only 2/3 of the time, and role-based filtering barely misses its bar. The 8-role vocabulary is not discriminative enough for the audit-layer claim as pre-registered. Kill criterion fired. Salvageable: the stability + non-interference machinery; the tagging protocol could be revisited with few-shot exemplars under a NEW pre-registration.

## The coherent coupling this builds on