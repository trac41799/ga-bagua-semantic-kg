# Gate Report — v2 Portfolio (mechanical gates; promotion claims reported separately)

**Date:** 2026-08-15  **Run:** ddfb7c50 (status snapshot synchronized without rerunning `gates.py`)

## Recorded Mechanical Snapshot

The gate runner defines 64 execution checks, and the last synchronized snapshot recorded all 64 as `PASS`; this is not a current rerun. Claim, replication, and product statuses are sourced from [`promotion-report.md`](promotion-report.md); they are not inferred from this count.

| Gate | Probe | Status | Note |
|---|---|---|---|
| G1 | C-pytest | PASS | pytest 11 passed |
| G1 | C-js | PASS | -----------------------------------------------------------
RESULT: 44 assertions passed, 0 failed
ALL T-C1 TESTS GREEN
 |
| G1 | D-tests | PASS | pytest 28 passed |
| G7 | C | PASS | no-claims statement present |
| G7 | D | PASS | verdicts + limitations disclosed |
| G8 | C | PASS | thresholds pre-registered in template |
| G8 | D | PASS | real-run verdicts rendered |
| G8 | D-ledger | PASS | token ledger rendered |
| G8 | D-claims | PASS | ledger rows appended |
| G9 | C | PASS | --------------------------
OFFLINE VERIFICATION GREEN: single file, zero network, opens via file://
 |
| G12 | POC-01 | PASS | spec+tdd+readme complete |
| G12 | POC-02 | PASS | spec+tdd+readme complete |
| G12 | POC-03 | PASS | spec+tdd+readme complete |
| G12 | POC-04 | PASS | spec+tdd+readme complete |
| G12 | applications | PASS | portfolio doc present |
| G12 | archive | PASS | failure ledger present |
| G13 | poc01-tests | PASS | pytest 28 passed |
| G13 | poc01-verdict | PASS | verdict rendered |
| G13 | poc02-tests | PASS | pytest 26 passed |
| G13 | poc02-verdict | PASS | verdict rendered |
| G13 | poc03-tests | PASS | pytest 130 passed |
| G13 | poc03-verdict | PASS | verdict rendered |
| G13 | poc04-tests | PASS | pytest 30 passed |
| G13 | poc04-verdict | PASS | verdict rendered |
| G14 | poc05-tests | PASS | pytest 9 passed |
| G14 | poc05-verdict | PASS | verdict rendered |
| G14 | poc06-tests | PASS | pytest 7 passed |
| G14 | poc06-verdict | PASS | verdict rendered |
| G14 | poc07-tests | PASS | pytest 6 passed |
| G14 | poc07-verdict | PASS | verdict rendered |
| G14 | poc08-tests | PASS | pytest 8 passed |
| G14 | poc08-verdict | PASS | verdict rendered |
| G14 | poc09-tests | PASS | pytest 8 passed |
| G14 | poc09-verdict | PASS | verdict rendered |
| G14 | poc10-tests | PASS | pytest 7 passed |
| G14 | poc10-verdict | PASS | verdict rendered |
| G14 | yarrow-moved | PASS | extracted to D:\TRANSFER DATA\Coding\OpenCode\yarrow-factorial (see REPO_HANDOFF.md) |
| G15 | tools-tests | PASS | pytest 33 passed |
| G15 | tools-cli-coverage | PASS | cli sim green |
| G15 | tools-cli-reframe | PASS | cli sim green |
| G15 | tools-cli-statediff | PASS | cli sim green |
| G15 | yarrow-handoff | PASS | handoff doc present |
| G15 | assessment | PASS | assessment doc present |
| G16 | B1-coverage | PASS | delta >= 1.0, 0 defects |
| G16 | B2-reframe | PASS | diversity >= 0.403, coherence >= 3.5, 0 prod defects |
| G16 | B3-statediff | PASS | 20/20 compliance, coverage >= 0.95 |
| G16 | B4-smoke | PASS | CLI+MCP real mode, schema-valid |
| G16 | qc-report | PASS | independent QC verdict present |
| G16 | ledger-clean | PASS | no stale FAIL rows |
| G17 | itools-version | PASS | suite v0.2.0 |
| G17 | itools-reframe-flagship | PASS | description field present |
| G17 | suite-v2-tests | PASS | pytest 11 passed |
| G17 | agent-integration-doc | PASS | MCP tool-use example present |
| G17 | yarrow-extracted | PASS | standalone project present outside repo |
| G18 | cl3-tests | PASS | pytest 1141 passed |
| G18 | xai-tests | PASS | pytest 21 passed |
| G18 | rotor-tests | PASS | pytest 42 passed |
| G18 | mcp-sdk-compat | PASS | official SDK source probe passed |
| G18 | packaging-wheel | PASS | wheel built |
| G18 | packaging-ci | PASS | CI workflow present |
| G18 | skills | PASS | 3 SKILL.md files present |
| G18 | poc13-eval | PASS | coverage-eval validated |
| G18 | poc15-r05 | PASS | R-05 replicated |
| G18 | poc15-r08 | PASS | R-08 model-dependence recorded (honest FAIL) |

## Banked verdicts (evidence)

- **Path D (decisive experiment, real LLM):** D1 FAIL (rubric R@10 0.370 < 0.60), D2 FAIL (pipeline 47% of full-context recall, break-even 23), D3 PASS (+10.6pp over TF-IDF, +11.4pp over BM25).
- **Archived:** Path A (semantic index), Path B (rotor KG) — evidence in archive/experimentation/fails/README.md.

## POC thesis verdicts (2026-08-08, real runs)

- **POC-01 combinatorial scaffold: FAIL** — delta +2.0pp (bar +20pp). Decomposition: 37/50 scaffold failures = LLM protocol-format non-compliance; calculator execution failures 0. Protocol problem, not algebra.
- **POC-02 factorial explorer: PASS** — 2^3/2^4 contrast signs 22/22 exact vs independent brute force; Mobius decomposition max err 3.91e-14; Bagua names 19/19. The blade algebra IS 2^k factorial math.
- **POC-03 reframing engine: FAIL** — diversity delta +0.082 (bar +0.15); coherence 2.80 (bar 3.5). Direction correct, margins not met; naming protocol is the weak link.
- **POC-04 tagged memory: FAIL** — tag quality 66.7% (bar 80%), filter precision 0.46 (bar 0.50); stability 83.3% and non-interference PASS. The 8-role vocabulary is not discriminative enough as an audit layer.

## Adoption-batch verdicts (2026-08-08, real runs)

- **POC-05 coverage rubric: PASS** — 8-role checklist reduces missing aspects 2.40 -> 1.25 (delta +1.15, bar 1.0). The I-Ching roles work as a generative completeness checklist.
- **POC-06 state-diff: FAIL (bar), signal present** — rater delta +0.10 (ceiling 4.9/5.0); automated planted-delta coverage 1.00 vs 0.85: the 3-aspect template achieves perfect factual completeness.
- **POC-07 interaction XAI: math PASS / naming FAIL** — planted interactions recovered exactly (err 2.2e-16, no false positives); Bagua-named explanations +0.33 (bar 0.5), direction positive.
- **POC-08 reframing v2: MODEL_DEPENDENT** — DeepSeek baseline diversity +0.290/coherence 3.75 passes its validated protocol, but the gpt-4o-mini replication fails at diversity 0.296/coherence 3.10. Do not present the claim as model-general.
- **POC-09 situation labeling: FAIL (bar), non-inferiority** — delta +0.00 (4.80/4.80 ceiling); framing compliance 20/20; the hexagram scaffold costs nothing and adds no measurable proxy benefit.
- **POC-10 communication measurement: INCONCLUSIVE** — the historical objective QA signal and rater-calibration failure are retained, but the clean no-leakage rerun artifact is still pending.
- **POC-11/12/14 deterministic MCP tools: PASS mechanically, product PENDING** — package tests, official SDK compatibility, and fresh distribution checks are present; host/product fit is not evidenced.
- **Yarrow MVP: BUILT** — 16/16 verification tests, CLI green; product gates per PRODUCT_PLAN.md (external-user exit gates pending).

## Blockers

No mechanical gate failures are recorded in this snapshot. Promotion blockers remain: POC-08 model dependence, POC-10 clean rerun, and host/product-fit evidence for pending products.
