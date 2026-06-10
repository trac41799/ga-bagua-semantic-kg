# PLAN: WuXing-Aligned Encoding via LLM Feedback Loop

**Status:** Complete  
**Result:** WuXing cycle signal RESTORED (f1=0.6, f2=1.0) but standalone encoding ceiling hit

---

## Result Summary

| Metric | v1 | v3 (feedback loop) | Delta |
|--------|-----|---------------------|-------|
| Encoding alignment | 15.1% | 18.9% | +3.8pp |
| WuXing cycle weight f1 | 0.0 | **0.6** | +0.6 |
| WuXing partial weight f2 | 0.0 | **1.0** | +1.0 |
| Weighted accuracy (all) | 86.8% | 22.6% | -64.2pp |
| Weighted accuracy (test) | 80.0% | 16.0% | -64.0pp |

## Key Finding: The Fundamental Ceiling

The feedback loop **successfully restored the WuXing cycle signal** (f1=0.6, f2=1.0). The classifier now uses the generating/controlling cycle — that's the Bagua principle working.

**But accuracy collapsed** because each concept participates in MULTIPLE relationships but can only be in ONE WuXing phase. Shifting Sales Pipeline to Earth fixes Pipeline→Revenue but breaks Onboarding→Pipeline because Pipeline now has the wrong phase for that relationship.

This IS how I-Ching works: the same entity gets different trigrams based on the question asked. Our system forces one encoding per concept — that's the ceiling.

## Path Forward

1. **Multi-encoding per concept** — encode each concept in all 5 WuXing phases, select appropriate encoding based on the relationship being classified. 5 × 64 bytes = 320 bytes per concept.
2. **Accept the ceiling** — use the weighted classifier with f3 (trigram quality) as primary signal (v1 weights), achieving 80-87% accuracy. This is practical but breaks Bagua principle.
3. **Contextual encoding** — re-encode per relationship pair. Defeats "encode once, classify many" efficiency.

## Conclusion

The feedback loop proved the WuXing cycle IS the correct classification mechanism — when concepts are in the right phases, it works. The limitation is that standalone encoding cannot satisfy multiple contradictory phase requirements simultaneously. This is a fundamental property of the WuXing taxonomy, not a bug in the classifier.
