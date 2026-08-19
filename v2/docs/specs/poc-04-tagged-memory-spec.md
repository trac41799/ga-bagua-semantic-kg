# SDD — POC-04: Interpretable-Tag Agent Memory

**Status:** Pre-registered POC | **Effort:** 2–4 weeks | **Priority:** 4 of 4
**Falsifiable question:** Does adding the 8 Bagua roles as an LLM-assigned TAG layer over standard retrieval deliver (a) high tag quality (≥80% agreement with human tags), (b) stable tags (≥80% re-tag consistency), (c) useful zero-token tag-based filtering (precision ≥ 0.5), while (d) never changing the underlying retrieval ranking?

## 1. Pre-registration (before any code)

| Item | Commitment |
|------|------------|
| Primary claim (tag quality) | LLM-assigned role tags (8 roles + strengths) agree with human-assigned tags on ≥ **80%** of dominant-role assignments (30 tagged items, single annotator + documented limitation) |
| Secondary claim (stability) | Re-tagging the same 30 items (2 runs, temperature 0) preserves the dominant role on ≥ **80%** |
| Tertiary claim (filtering) | Tag-based filtering (query → role → items with that role) achieves precision ≥ **0.5** on 10 filter queries with ground truth |
| Non-interference | Tag layer NEVER changes retrieval ranking: tagged vs untagged retrieval results identical (byte-level) |
| Retrieval accuracy | NOT claimed in POC phase — depends on real dense embeddings (documented dependency: install sentence-transformers or use LLM-judgment vectors ≥16 dims; BM25 is the interim stand-in) |
| Kill criterion | Tag quality or stability < 80% → POC-04 dies as a tag-layer claim |

## 2. Scope

**In:** tag protocol (8 roles + strengths, JSON, validated), tag encoder (LLM, Path D client pattern), tag stability runner, human-tag reference set (30 items from the Path D corpus), filter query set (10) + ground truth, non-interference check (rankings identical), reports, tests.
**Out:** No new embedding; no retrieval-accuracy claims; no token-economics claims (those died in D2 and stay dead).

## 3. Architecture

```
corpus.py        reuse Path D corpus (120 concepts; 30 tagged reference items)
tags.py          role definitions (8 roles, fixed order) + prompt + parse/validate
tagger.py        LLM: description → {role: strength} (JSON)
stability.py     re-tag consistency (dominant role match across runs)
filtering.py     role-filter precision/recall vs ground truth
noninterf.py     ranking invariance check (tagged vs untagged)
run_all.py       orchestrates, renders reports + verdicts
```

## 4. Interfaces

| Component | File | Interface |
|-----------|------|-----------|
| Tags | `tags.py` | `TAG_PROMPT(description)`, `parse_tags(text) -> dict[str,float]` (validates 8 roles, range) |
| Tagger | `tagger.py` | `tag(description) -> dict` (LLM, temperature 0) |
| Stability | `stability.py` | `dominant_role(tags) -> str`; `consistency(tag_sets) -> float` |
| Filtering | `filtering.py` | `filter_precision(query_role, items_with_roles, ground_truth) -> float` |
| Non-interference | `noninterf.py` | `rankings_identical(retrieval_fn, with_tags, without_tags) -> bool` |
| Runner | `run_all.py` | renders `output/tag_quality.md`, `stability.md`, `filtering.md`, `noninterference.md`, `gate_summary.md` |

## 5. Evaluation protocol

1. 30 reference items tagged by a human (single annotator, documented) and by the LLM (temperature 0, 2 runs).
2. Tag quality = fraction of items where LLM dominant role == human dominant role.
3. Stability = fraction of items where dominant role matches across the 2 LLM runs.
4. Filtering: 10 queries each mapped to a target role; ground-truth item sets authored; precision@retrieved.
5. Non-interference: retrieval ranking computed with and without tag fields — must be identical.
6. Determinism: temperature 0, cached responses; ledger appended.

## 6. Acceptance criteria (TDD: `docs/tdd/poc-04-tdd.md`)

- AC-04.1 parse_tags: valid → 8 roles in range; malformed → typed error
- AC-04.2 dominant_role correct on hand cases
- AC-04.3 consistency metric correct (hand case: 8/10 → 0.8)
- AC-04.4 filter precision metric correct (hand case)
- AC-04.5 non-interference check correct (identical rankings → True)
- AC-04.6 runner renders all reports + verdict rows
- AC-04.7 determinism (cached) + ledger + tests green
