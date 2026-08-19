# Path D — The Decisive Experiment (real LLM, real baselines, real token ledger)

**Status: EXECUTED with real LLM (deepseek-chat). Verdicts: D1 FAIL, D2 FAIL, D3 PASS — the original vision is falsified as specified, with one genuine positive.**

## What was run (2026-08-08, live API, temperature 0, all numbers from `usage.total_tokens`)

- 120 concepts encoded via the SKILL.md rubric (one-time: **42,801 tokens**)
- 24 queries, single-annotator ground truth (documented limitation)
- Retrieval: rubric-cosine vs TF-IDF vs BM25 vs random (seeded)
- Pipeline: rubric top-10 + LLM verification (123.5 tokens/query) vs full-context LLM baseline (2,041.5 tokens/query)

## Results

| Verdict | Criterion | Result | |
|---------|-----------|--------|---|
| **D1** | rubric R@10 ≥ 60% | **0.370** (random floor 0.065) | **FAIL** |
| **D2** | pipeline recall ≥ 95% of full-context AND break-even ≤ 10 | pipeline **0.331** vs full-context **0.702** (47%); break-even **23 queries** | **FAIL** |
| **D3** | rubric ≥ TF-IDF +10pp AND ≥ BM25 +10pp R@10 | 0.370 vs 0.264 (TF-IDF) vs 0.256 (BM25) → **+10.6pp / +11.4pp** | **PASS** |

| Method | R@5 | R@10 | MRR |
|--------|-----|------|-----|
| rubric-cosine (real LLM encodings) | 0.234 | **0.370** | 0.527 |
| TF-IDF | 0.194 | 0.264 | 0.419 |
| BM25 | 0.185 | 0.256 | 0.439 |
| random | 0.031 | 0.065 | 0.087 |

## Honest interpretation

1. **The original vision — as specified (8-role rubric + compact index + LLM verify) — is falsified.** The rubric encoding cannot reach the 60% retrieval bar (D1), and even where it retrieves, the pipeline preserves only 47% of full-context accuracy while requiring 23 queries to break even on tokens (D2). The v1-era token-savings claims were projections; this is the first measured ledger, and it says the encoding cost dominates at every realistic query count.

2. **The one real positive (D3): LLM-judgment encoding beats lexical IR by >10pp.** The *mechanism* — an LLM reading descriptions and producing semantic encodings — carries signal that TF-IDF/BM25 cannot reach. The bottleneck is the **8-dim role rubric's capacity**, not the LLM-judgment approach. This is the first clean empirical evidence that the v1 "semantic encoding" instinct had something real in it — and it points to the fix: higher-dimensional, less lossy encodings (i.e., ordinary dense embeddings, or a wider rubric).

3. **What this means for the vision:** as a *product claim* ("64-byte semantic memory that preserves accuracy and compresses context"), the vision is dead — the pre-registered kill criteria fired. As a *research finding*, the D3 result says the surviving hypothesis is: "LLM-judgment semantic encodings help retrieval; the 8-role space is too small to exploit them." The practical descendant is standard RAG (LLM or learned embeddings + vector DB + LLM verify) — which is exactly the conclusion the evidence has been pointing to since Path A.

## How to reproduce

```
python -m pytest tests/ -q            # 28 tests (SimulatedLLM only)
python run_all.py                     # real run (cached: encodings.json, verify.json, full_context.json)
python run_all.py --offline           # cached/report-only, no network
```

Reports: `output/retrieval_metrics.md`, `output/pipeline.md`, `output/token_economics.md`, `output/gate_summary.md`, `output/claims_ledger.csv`. Caches in `data/cache/` — re-runs do not re-spend API tokens (verified by test).

## Limitations (disclosed)

- Single-annotator ground truth (24 queries); description-corpus is authored, not sampled from production.
- Full-context accuracy uses substring name-match (documented; 0.702 — the LLM itself misses concepts even with all descriptions).
- One LLM (deepseek-chat), temperature 0, one prompt version. Different models/prompts could shift numbers; the pre-registered bars were calibrated to the vision, not to any model.
