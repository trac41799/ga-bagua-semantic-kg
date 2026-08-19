---
name: bagua-reframe
description: Generate exactly 8 algebra-grounded reframes of a statement — origin, 3 single-line flips, 3 double-flips, complement (Hodge dual). Each position is an explainable structural move. Model-dependent evidence: DeepSeek diversity 0.453 / coherence 3.75 (POC-08); the gpt-4o-mini replication fails. Use for ideation, perspective-taking, argument exploration with the model caveat.
---

# Reframe (8 positions of the cube)

## When to use
When a statement, decision, or claim needs structurally complete alternative framings — ideation, stakeholder perspectives, counter-positions.

## Method
Generate exactly 8 reframes, each as a distinct structural move on the trigram cube:
1. **origin** — the statement as-is
2. **flip0 / flip1 / flip2** — change ONE aspect (bottom/middle/top line)
3. **double_flip01 / double_flip02 / double_flip12** — change TWO aspects
4. **complement** — the antipode (Hodge dual ·e123): the opposite pole

For each position, name the move and write one reframe sentence expressing the statement from that position.

## Tool form (MCP)
`reframe(statement)` → `{statement, positions: [{move, state, reframe, description} x8]}`

## Evidence
- POC-08 baseline (few-shot naming, deepseek-chat, 20 frozen statements): diversity 0.453, coherence 3.75.
- POC-15 R-08 replication (gpt-4o-mini): diversity 0.296 and coherence 3.10, both below bar; the claim is **MODEL_DEPENDENT**, not model-general.
- Production benchmark B2: 0.441 / 3.75 on the validated model, 0 production defects; this does not remove the model-dependence caveat.

## House rule
The moves are exact operators; the reframes are structural views — no predictive meaning. Do not present the POC-08 quality result as cross-model or product validation.
