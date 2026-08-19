# Verdict -- POC-03 reframing engine

- run_id: `20260808T154020Z`
- engine: REAL (deepseek-chat, cached in D:\TRANSFER DATA\Coding\OpenCode\ga-bagua-semantic-kg\v2\pocs\poc-03-reframing-engine\data\cache)

## Proxy claim (pre-registered)

Mean pairwise semantic distance of the 8 algebra-grounded views >= LLM free-form reframes + **0.15** (cosine on 8-dim rubric vectors, same encoder both arms), AND mean coherence (LLM-judged, 1-5) >= 3.5.

| quantity | Arm A | Arm B | delta | threshold | condition |
|----------|-------|-------|-------|-----------|-----------|
| mean diversity | 0.2781 | 0.3601 | 0.0819 | +0.15 | FAIL |
| mean coherence | 4.3000 | 2.8000 | -1.5000 | >= 3.5 | FAIL |

## Proxy claim: **FAIL**

Kill criterion: the proxy claim failing kills POC-03 as a claim (the moves remain useful only pedagogically -> Path C, not a product).

Human gate (pre-registered, runs after the proxy passes): >=70% of raters (n>=5) rate the algebra-grounded set 'more systematically complete' than the free-form set. See SDD: ../../docs/specs/poc-03-reframing-engine-spec.md (section 1, Pre-registration).

- total tokens (live calls): 129756
- total protocol failures (all arms): 2
