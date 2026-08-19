# POC-15 — Cross-Model Replication (gpt-4o-mini via OpenRouter)

**Status: EXECUTED — one claim replicates, one does not (honest, model-dependence established).**

## Results (same frozen protocols, same prompts, same bars; model = openai/gpt-4o-mini, temperature 0)

| Claim | deepseek-chat (validated) | gpt-4o-mini | Bar | Verdict |
|-------|---------------------------|-------------|-----|---------|
| **R-05** coverage checklist Δ (A−B missing aspects) | +1.15 | **+1.80** (audited mean 0.15) | ≥ +1.0 | **PASS — model-robust** |
| **R-08** reframe diversity | 0.453 | **0.296** | ≥ 0.403 | **FAIL — model-dependent** |
| **R-08** reframe coherence | 3.75 | **3.10** | ≥ 3.5 | **FAIL — model-dependent** |

Tokens: 112,013; calls: 420; all cached and reproducible.

## Interpretation (pre-registered honesty rule applied)

1. **The coverage-checklist claim is model-robust**: gpt-4o-mini reproduces (and exceeds) the deepseek effect (Δ+1.80). This capability can be promoted with cross-model confidence.
2. **The reframe claim is model-dependent**: the few-shot naming protocol validated on deepseek-chat does not transfer to gpt-4o-mini — naming quality degrades (coherence 3.10), and reframe diversity collapses toward free-form levels (0.296). The reframe flagship must either (a) be documented as model-specific, (b) gain a model-specific naming protocol under a NEW pre-registration, or (c) require a stronger model. None of these is a silent fix; the claim as pre-registered does NOT generalize.
3. **THESIS impact**: replication queue item 1 is now partially resolved — POC-05 replicated, POC-08 not. The reframe tool's README claims must carry the model-dependence caveat.

## Run
```
python run_all.py --which coverage   # R-05 (80 calls)
python run_all.py --which reframe    # R-08 (340 calls)
```
Cached in `data/cache/`; re-runs deterministic (temperature 0).
