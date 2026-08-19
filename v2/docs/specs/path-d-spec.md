# Spec D — The Decisive Experiment: LLM-Encoded Semantic Index vs. Real Baselines (Path D)

**Status:** Pre-registered probe | **Timebox:** 1 week | **Owner:** Path D
**Falsifiable question (the original vision, finally tested end-to-end):** With REAL LLM encodings (SKILL.md rubric), plain cosine retrieval, and LLM verification — does the index deliver (a) retrieval accuracy ≥ 60% R@10, (b) pipeline accuracy ≥ 95% of full-context accuracy, (c) token savings with break-even ≤ 10 queries, and (d) a ≥10pp R@10 advantage over standard IR (TF-IDF, BM25)?

This is the one experiment every prior benchmark avoided: it runs the actual LLM end-to-end and measures against real baselines with a real token ledger. No Clifford algebra, no learned models — Path A showed both are the weak link. The vision's value chain is: LLM judgment → compact vector → cheap retrieval → LLM verification.

## 1. Pre-registration (written before any code)

| Item | Commitment |
|------|------------|
| Corpus | 120 concepts, 1–2 sentence descriptions, 4 domains (software systems, business ops, biological systems, governance) — authored, single-annotator ground truth (documented limitation) |
| Queries | 24 queries, 1–3 ground-truth relevant concepts each, answerable from descriptions |
| Encoding | REAL LLM (deepseek-chat, temperature 0) via SKILL.md rubric prompt → 8 coefficients, JSON-validated, normalized; cached to disk (re-runs don't re-spend) |
| Retrieval | cosine top-K on the 8 coefficients (NO learned model) |
| Baselines | TF-IDF (numpy), BM25 (pure python), random (seeded), full-context LLM (all descriptions in context, name-match accuracy) |
| Verdicts | **D1:** rubric-cosine R@10 ≥ 60%. **D2:** pipeline accuracy ≥ 95% of full-context accuracy AND break-even ≤ 10 queries. **D3:** rubric-cosine R@10 ≥ TF-IDF by +10pp AND ≥ BM25 by +10pp. All-or-nothing on each verdict |
| Token ledger | API `usage.total_tokens` for every call: encoding (one-time), verification per query, full-context per query. Savings curves at 10/50/200 queries |
| Kill criteria | D1 fails → the 8-role rubric cannot carry semantics; vision falsified (close honestly). D2 fails → economic claim dead even if retrieval works. D3 fails → standard IR beats it trivially; vision reduces to ordinary RAG |
| Honesty rules | LLM-simulator used ONLY in unit tests (L4). If the API is unavailable mid-run → status PENDING, no claims. Every number links to its ledger row. Single-annotator ground truth and substring name-match evaluation disclosed as limitations |

## 2. Scope

**In:** corpus + queries + ground truth, rubric encoder client (real API, cached), SimulatedLLM (tests only), cosine retrieval, TF-IDF/BM25/random baselines, pipeline evaluator (LLM verifier on top-K), token ledger, reports (retrieval_metrics.md, pipeline.md, token_economics.md, gate_summary.md, claims_ledger.csv), tests.
**Out:** No GA, no rotors, no WuXing, no learned probes, no new taxonomy claims.

## 3. Architecture

```
corpus.py (120 concepts + 24 queries + ground truth)
    │
    ├── llm_client.py ── encode (rubric prompt, JSON, normalize) ──► encodings.json (cached)
    │                └── verify(query, candidates) → JSON ids
    │                └── full_context(query, all descriptions) → answer text
    │
    ├── retrieval.py      cosine top-K
    ├── baselines.py      TF-IDF, BM25, random
    │
    └── eval.py → output/ retrieval_metrics.md, pipeline.md, token_economics.md,
                  gate_summary.md, claims_ledger.csv
```

## 4. Interfaces

| Component | File | Interface |
|-----------|------|-----------|
| Corpus | `corpus.py` | `CONCEPTS: list[(id, name, domain, description)]`; `QUERIES: list[(id, text, [relevant concept ids])]` |
| Rubric | `rubric.py` | `encode_prompt(description) -> messages`; `parse_encoding(text) -> (np.ndarray[8], err)` (strict JSON) |
| LLM client | `llm_client.py` | `LLMClient(base_url, key, model)`; `chat(messages) -> (text, usage)`; `SimulatedLLM` (tests only, deterministic) |
| Retrieval | `retrieval.py` | `cosine_topk(encodings, query_vec, k) -> [(id, score)]` |
| Baselines | `baselines.py` | `tfidf_scores(descriptions, query)`; `bm25_scores(...)`; `random_topk(seed)` |
| Eval | `eval.py` | metrics (R@5/R@10, MRR), pipeline F1/recall, token ledger, verdicts |
| Runner | `run_all.py` | orchestrates, caches, writes reports; `--offline` runs baselines-only with cached encodings |

## 5. Evaluation protocol

1. Encode all 120 concepts (temperature 0, 2 retries, budget cap 200K tokens). Cache.
2. For each of 24 queries: rubric-cosine top-10, TF-IDF top-10, BM25 top-10, random top-10 → R@5/R@10/MRR vs ground truth.
3. Pipeline: rubric-cosine top-10 → LLM verifier returns relevant ids (JSON) → recall/F1 vs ground truth.
4. Full-context baseline: LLM answers each query with all descriptions in context → name-match accuracy (substring, case-insensitive, documented).
5. Token ledger: per-call usage; break-even = encoding_tokens / (full_context_per_query − verify_per_query); savings at 10/50/200 queries.
6. Determinism: seeds for random baseline; encodings cached; ledger immutable per run hash.

## 6. Acceptance criteria (see `../tdd/path-d-tdd.md`)

- AC-D1 rubric JSON parse: valid output → 8 floats; malformed → error, no crash
- AC-D2 SimulatedLLM deterministic; real client sends auth + parses usage (mock-tested)
- AC-D3 TF-IDF/BM25/cosine math correct (hand-computed cases)
- AC-D4 retrieval metrics correct (R@5/R@10/MRR hand-computed)
- AC-D5 token ledger arithmetic correct (break-even, savings)
- AC-D6 cache: re-run with cache → no new API calls (mock counter)
- AC-D7 reports render all verdict rows (D1/D2/D3) with PASS/FAIL/PENDING
- AC-D8 offline mode: baselines + cached encodings run with no network
- AC-D9 tests green; claims ledger rows appended
