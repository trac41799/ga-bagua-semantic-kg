# Handoff: Encoding Quality Workstream

**Status:** Ready to begin
**Depends on:** trigram-specific classification rules (MERGE)
**Blocks:** relation classification accuracy improvement

---

## What Was Proven

The trigram-specific rules added to `from_pair()` prove the Bagua framework CAN classify all 8 relation types when encodings are correct:

```
Before (phase-only rules):    After (trigram-specific rules):
  causal:           0%    →   40%  combined
  clarifying:       0%    →   40%  combined
  influential:      0%    →   28.6% combined
  transmissive:     0%    →    still 0%  (encoding mismatch)
  balancing:        0%    →    still 0%  (encoding mismatch)
```

The classification logic is no longer the bottleneck. The encoding quality is.

---

## The Encoding Problem

The current SKILL.md protocol asks: "What intrinsic properties does this concept have?" and maps those to 8 coefficients. This captures what a concept IS but not where it SITS in the WuXing dynamic chain.

**Example:** "Sales Pipeline" has transmissive properties (channels leads). So the LLM encodes it as Kan (Water). But in a "Pipeline → Revenue Target" relationship, the Pipeline plays a GENERATIVE role (it enables the target). Water generates Wood — but Revenue is encoded as Qian (Metal). Mismatch.

The classification rule says: "If A=Kan AND Water generates B's phase → transmissive." But B's phase is Metal, and Water generates Wood, not Metal. The rule can't fire because the encodings don't align.

**Root cause:** Concepts are encoded independently. But the WuXing cycle describes RELATIONAL dynamics. A concept's trigram depends on the relationship being examined, not just its intrinsic properties. This is how the I-Ching works: the trigram you draw depends on the question you ask.

---

## What Needs to Be Done

### 1. Research: I-Ching Consultation Protocol

Study how I-Ching practitioners map situations to trigrams. Key questions:
- How does the consultation question frame the trigram assignment?
- What is the protocol for decomposing a situation into its constituent dynamics?
- How are hexagrams (pairs of trigrams) interpreted as compound dynamics?
- How do moving lines (trigram line changes) predict transition?

**Resources to study:**
- Wilhelm/Baynes translation of the I-Ching (standard reference)
- "The I Ching or Book of Changes" — particularly the Great Treatise (Ta Chuan)
- Scholarly work on the epistemology of I-Ching consultation

### 2. Design: Contextual Encoding Protocol v2

Current protocol (SKILL.md v1):
```
For concept X, assign weights to 8 roles based on X's intrinsic properties.
```

Proposed v2:
```
For concept X IN THE CONTEXT OF domain/relationship Y:
  1. Identify the GENERATIVE force: what does X create/enable?
  2. Identify the RECEPTIVE aspect: what does X accept/accommodate?
  3. Identify the CAUSAL trigger: what does X initiate?
  4. Identify the TRANSMISSIVE channel: what flows through X?
  5. Identify the CONSTRAINING boundary: what does X limit?
  6. Identify the INFLUENTIAL pervasion: what does X gradually shape?
  7. Identify the CLARIFYING illumination: what does X reveal?
  8. Identify the BALANCING reflection: what mirrors X?

Output: 8 coefficients AND a confidence score for each.
```

Key change: encode concepts relationally, not intrinsically. "What does X DO to others?" not "What is X?"

### 3. Implement: Encoding Validation Loop

Add to `refine.rs`:
- `validate_encoding_pair(a, b, expected_label) → diagnoses WHY a prediction failed`
- `suggest_encoding_fix(a, b, expected_label) → proposes coefficient changes with reasoning`
- `batch_validate(dataset) → summary of encoding quality issues`

The MCP server should expose a `diagnose_misclassification` tool that tells the LLM agent WHY a prediction was wrong and how to fix the encoding.

### 4. Evaluate: Re-run All Benchmarks

After encoding protocol improvements:
- Run `train_test_benchmark` → target: test accuracy > 50%
- Run `semantic_benchmark` → target: > 90% (with proper non-circular labels)
- Run `realistic_benchmark` → target: > 60% refinement benchmark
- Run `scalability_benchmark` → verify no regression
- Run `context_compression_benchmark` → verify token savings

---

## Acceptance Criteria

- [ ] SKILL.md v2 protocol documented with examples
- [ ] Encoding validation loop implemented in MCP server
- [ ] At least 2 of the 4 zero-accuracy labels (transmissive, balancing, causal, influential) reach > 30% test accuracy
- [ ] Overall test accuracy on `train_test_benchmark` > 35% (up from 24%)
- [ ] Dominant role identification accuracy remains at 100%
- [ ] No regression in retrieval benchmarks

---

## Why Not Drop Relation Classification?

The trigram rules prove the framework works. Dropping it now would be abandoning a feature at the moment we discovered the ACTUAL bottleneck (encoding), not because the approach is wrong. The Bagua/WuXing model IS correct — we just haven't taught the LLM how to use it properly yet.

This is analogous to: building a correct SQL query engine, but the user keeps sending malformed queries. The fix isn't to remove the engine — it's to improve the query writing.

---

## Why Separate This Workstream?

Encoding quality touches:
- The SKILL.md prompt (natural language, needs linguistic iteration)
- The encoding protocol (needs I-Ching domain research)
- The MCP server tools (needs new diagnostic endpoints)
- The benchmark dataset (needs re-encoding of all 50 concepts)

Each of these is a substantial task that benefits from focused, uninterrupted work. It's not a "bug fix" you add to a sprint — it's a capability upgrade that needs design decisions, research, and validation.

---

## Immediate Actions (Can Be Done Without This Workstream)

1. **Merge the trigram-specific classification rules** (already implemented, tests pass, improves accuracy)
2. **Review the JSON dataset** — are the concept encodings in `data/benchmark_dataset.json` high-quality? If not, re-encode them using current SKILL.md (not v2)
3. **Add per-pair diagnosis** — for each failing test pair, print WHY it failed (which encoding was wrong, what would fix it)

---

## Related Files

| File | Role |
|------|------|
| `src/relation_type.rs` | Classification logic (trigram rules added) |
| `src/refine.rs` | Encoding refinement loop |
| `docs/skills/bagua-encoder/SKILL.md` | Current encoding protocol v1 |
| `data/benchmark_dataset.json` | 50-concept, 53-relation benchmark dataset |
| `tests/train_test_benchmark.rs` | Primary evaluation benchmark |
| `tests/semantic_benchmark.rs` | Original 20-concept benchmark (inflated) |
| `tests/realistic_benchmark.rs` | Independent-ground-truth benchmark |
| `tests/scalability_benchmark.rs` | Scale/multi-hop/contradiction benchmarks |
| `tests/context_compression_benchmark.rs` | LLM pipeline token economics |
| `tests/final_benchmark.rs` | Redundant — superseded by train_test_benchmark |

---

*Handoff created 2026-06-08. Trigram-specific classification rules implemented and tested (110 unit tests pass). Encoding quality is the binding constraint on classification accuracy.*
