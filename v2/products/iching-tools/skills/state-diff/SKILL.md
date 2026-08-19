---
name: state-diff
description: Summarize a state change as exactly 3 aspect lines ('aspect: before -> after'). Historical objective POC-10 QA was 1.00 vs 0.90, but the clean no-leakage rerun is pending; production B3 is a separate mechanical compliance check. Use for agent memory updates, event summaries, change reporting.
---

# State Diff (3-aspect summary)

## When to use
When an agent records a state change (memory update, event, system transition) and needs a complete, structured, parseable summary.

## Prompt (strict format)

```
Summarize the change between the two states using EXACTLY 3 aspect lines:
<aspect>: <before> -> <after>
Cover the three most important changes, one per line, no prose beyond the 3 lines.

Before: <before state>
After: <after state>
```

## Tool form (MCP)
`state_diff(before, after)` → `{before, after, aspects: [{aspect, before, after} x3]}`

## Evidence
- Historical POC-10 objective comprehension-QA: structured 1.00 vs free-form 0.90 (delta +0.100, at bar); rater calibration failed and the clean no-leakage rerun is pending.
- Production benchmark B3: 20/20 compliance, planted-delta coverage 1.0000, 0 defects; this is mechanical output evidence, not a replicated communication claim.

## House rule
The 3-line structure is a communication template — the content is the agent's factual summary.
