# THESIS — I-Ching × Clifford × AI (Versioned Working Thesis)

**Version:** v1.0 | **Date:** 2026-08-15 | **Status:** Historical thesis evidence is retained; current claim/product promotion status is governed by `qa/promotion-report.md`

## The thesis (v1.0)

> **I-Ching structure benefits AI where it scaffolds (a) exact computation over combinatorial structures and (b) LLM generation protocols. It does not benefit discrimination. Communication: structured state-diffs help (objective); framing layers neither help nor harm factually (measured null); subjective clarity/trust claims remain human-gate-pending (LLM raters proven unfit by calibration).**

The Clifford-algebra half of the original vision is retired (see `archive/experimentation/fails/`); this thesis governs what remains.

## Evidence map

| Claim | Evidence | Status |
|-------|----------|--------|
| (a) Exact computation over combinatorial structures | POC-02 (contrasts 22/22, Möbius 3.91e-14); POC-07 (interaction recovery err 2.2e-16); calculator in POC-01 (0 execution failures) | **SUPPORTED** (2 independent exact replications) |
| (b) Generation protocols | POC-05 (coverage checklist: missing aspects 2.40→1.25, Δ+1.15; R-05 replicated at Δ+1.80); POC-08 (few-shot-named cube reframes: diversity Δ+0.290, coherence 3.75; R-08 failed at 0.296/3.10) | **BOUNDED** — POC-05 replicated; POC-08 is **MODEL_DEPENDENT**, not model-general |
| (c) Discrimination (tags, retrieval, classification) | POC-04 (quality 66.7%), D1/D2 (R@10 0.37, 47% recall), A-v1/v2 | **FALSIFIED** (5 independent attempts) |
| (d) Communication/comprehension | **HISTORICAL POC-10 SIGNAL:** objective comprehension-QA: structured summaries 1.000 vs free-form 0.900 (Δ+0.100 = at bar); conveyance 0.518 vs 0.597 (Δ−0.078). Rater calibration FAILS to discriminate known-good from known-bad (Δ≤0.5 vs bar 1.0). The clean no-leakage rerun is still pending. | **INCONCLUSIVE for promotion; calibration finding retained** |

## The boundary question — PROVISIONAL (POC-10 clean rerun pending)

The open question was: is the failure boundary *generation vs communication* or *objective vs subjective metrics*?
**Historical answer: both effects may exist, but the instrument and planted-ground-truth protocol remain confounds until the clean rerun.**
- POC-06's structured summaries showed an objective QA signal at the bar in the historical run; the current promotion status is **INCONCLUSIVE** pending a clean no-leakage rerun.
- POC-09's hexagram framing genuinely does not help (objective conveyance negative) — a **true null**.
- The LLM 1–5 rater is unfit for these measurements (calibration failed) — subjective clarity/trust claims require the human gate; no LLM-rater verdict is interpretable without passing calibration first.
- The metric-design lesson is now empirically proven: **objective instruments measure; subjective proxies saturated and uncalibrated mislead.**

## The metric-design lesson (the pattern behind the failures)

- **Every success used objective, parse-based metrics** (exact match, counts, coverage, error bounds).
- **Every failure used LLM-rater 1–5 scales** (saturate at 4.8–5.0 for competent outputs; noise dominates small effects).
- **Calibration is now mandatory**: POC-10 proved the rater cannot discriminate known-good from known-bad (Δ≤0.5). Any future 1–5 claim must pass calibration first; otherwise it is declared uninterpretable.
- House rule: subjective proxies require calibration before pre-registration; human gates are scheduled INSIDE experiments, not deferred.

## Open boundary question (RESOLVED — POC-10, 2026-08-08)

~~Is the boundary "generation vs communication" or "objective vs subjective metrics"?~~ **Provisionally narrowed:** the historical instrument failed calibration, and the clean no-leakage protocol still needs a rerun. Remaining open items are the clean POC-10 rerun, human-gate measurements, and cross-model replication.

## Replication queue (before promotion)

1. ~~POC-05 and POC-08 on a second LLM (claude/gpt) — same frozen protocols, same bars.~~ **RESOLVED (POC-15, gpt-4o-mini):** POC-05 **REPLICATED** (Δ+1.80 ≥ 1.0 — model-robust). POC-08 **NOT REPLICATED** (diversity 0.296 < 0.403, coherence 3.10 < 3.5 — **model-dependent**); the reframe claim carries a model-dependence caveat and needs a model-specific naming protocol under a new pre-registration before any promotion.
2. POC-10's comprehension-QA finding (Δ+0.100, exactly at bar) — replicate with a second reader model and more transitions before treating as robust.
3. Yarrow Phase-1 exit gates (external users, task-time study) — product promotion path.

## Rules that keep the thesis convergent

1. New experiments test the thesis boundary, not adjacent novelty.
2. Replicate before extending; failure analysis before counting a FAIL as a phenomenon failure.
3. No new LLM-rater-1–5 primary claims without calibration.
4. Promotion requires: validated + replicated + host-product fit (Yarrow qualifies; POC-05/08 are capabilities awaiting hosts).
