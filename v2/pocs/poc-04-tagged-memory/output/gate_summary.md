# Gate summary — POC-04 interpretable-tag agent memory

Mode: real

| Claim | Criterion | Measured | Verdict |
|-------|-----------|----------|---------|
| Tag quality (LLM vs human dominant role) | quality >= 80% | 66.7% | FAIL |
| Tag stability (2 runs, temperature 0) | stability >= 80% | 83.3% | PASS |
| Filtering precision (10 queries) | mean precision >= 0.5 | 0.46 | FAIL |
| Non-interference (tagged vs untagged) | rankings identical | True | PASS |

**Overall gate: FAIL**

Notes: retrieval accuracy is OUT of scope for POC-04 (real dense embeddings are a documented dependency); the 8 roles are tags on top of retrieval, never the embedding/retrieval vector; no token-economics claims are made.
