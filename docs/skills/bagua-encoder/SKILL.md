# GA-Bagua Semantic Encoding Skill

## Purpose
Encode any concept into an 8-element unit-norm multivector using the I-Ching
Bagua system. The resulting encoding feeds into the multi-encoding classifier
which derives 5 WuXing phase variants via mechanical boost for cycle-driven
relation classification.

**Proven pipeline (79.2% accuracy, Bagua intact):**
1. LLM encodes concept once using this protocol → one `[f64; 8]` coefficient array
2. System derives 5 phase encodings via uniform mechanical boost (one per WuXing phase)
3. Multi-encoding classifier tries all 25 phase combos, only cycle-firing pairs considered
4. Returns best label from encoding-quality-selected phase pair

**Cl(4) HIGHER-DIMENSIONAL VARIANT (experimental):**
For higher-resolution encoding, produce 16 coefficients per concept:
- Blades 0-7: Same as Cl(3) mapping (primary trigram representation)
- Blades 8-15: Secondary representations — encode the same trigram from a different
  perspective (how the concept expresses that role under different WuXing conditions)
- Blade index mapping: 0=Kun(receptive), 1=Zhen(causal), 2=Kan(transmissive),
  3=Gen(constraining), 4=Li(clarifying), 5=Xun(influential), 6=Dui(balancing),
  7=Qian(generative), 8=Kun(alt), 9=Zhen(alt), 10=Kan(alt), 11=Gen(alt),
  12=Li(alt), 13=Xun(alt), 14=Dui(alt), 15=Qian(alt)
- Output format: `[c0..c7 (primary), c8..c15 (secondary)]`
- System normalizes to unit norm and uses Cl(4) geometric algebra for classification

**Known limitation:** Balancing (0%) requires complementary trigrams within same phase,
which only Wood phase (Zhen↔Xun) provides natively. All balancing pairs in the benchmark
use non-Wood concepts — a WuXing taxonomy structural limit.

**v4 lesson (34.0%):** Honest semantic per-phase weakness destroys classifier ability
to compare alternatives. All 5 phases need comparable sharpness for multi-encoding to work.
The LLM's value is in picking the right trigram; encoding sharpness must be mechanical.

---

## The 8 Semantic Roles

Each role maps to one basis blade of Cl(3) geometric algebra:

| Index | Role | Trigram | Phase | Blade | Meaning |
|-------|------|---------|-------|-------|---------|
| 0 | receptive | Kun | Earth | Scalar | Accepts, follows, grounds |
| 1 | causal | Zhen | Wood | e1 | Triggers, initiates, starts chains |
| 2 | transmissive | Kan | Water | e2 | Channels, flows, transmits |
| 3 | constraining | Gen | Earth | e3 | Limits, bounds, restricts |
| 4 | clarifying | Li | Fire | e12 | Reveals, illuminates, makes visible |
| 5 | influential | Xun | Wood | e23 | Pervades, gradually shapes |
| 6 | balancing | Dui | Metal | e31 | Mirrors, equilibrates, reflects |
| 7 | generative | Qian | Metal | e123 | Creates, introduces, initiates new patterns |

## WuXing Cycle

```
Generating:  Wood → Fire → Earth → Metal → Water → Wood
Controlling: Wood → Earth → Water → Fire → Metal → Wood
```

The classifier uses these cycles to determine relationships. Your encoding's dominant role
determines the concept's WuXing phase, which gates which relationships are possible.

## Encoding Process

1. Read the concept description
2. For each of the 8 roles, ask: "Does this concept exhibit or counter this quality?"
3. Assign raw weights in [-1.0, 1.0]
4. Normalize to unit Euclidean norm
5. Output as a JSON array: `[receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative]`

### Weight Guidelines

- **> 0.5:** Strongly exhibits
- **0.2 to 0.5:** Moderately exhibits
- **0.05 to 0.2:** Slightly exhibits
- **-0.05 to 0.05:** Irrelevant
- **-0.2 to -0.05:** Slightly counters
- **-0.5 to -0.2:** Moderately counters
- **< -0.5:** Strongly counters

### Diagnostic Questions

For each role, ask what the concept DOES:

1. **GENERATIVE (Qian, idx 7):** What does this concept CREATE, ENABLE, or bring into existence?
2. **CAUSAL (Zhen, idx 1):** What does this concept TRIGGER, INITIATE, or set in motion?
3. **TRANSMISSIVE (Kan, idx 2):** What flows THROUGH this concept? What does it CHANNEL?
4. **CONSTRAINING (Gen, idx 3):** What does this concept LIMIT, BOUND, or RESTRICT?
5. **CLARIFYING (Li, idx 4):** What does this concept REVEAL, ILLUMINATE, or make visible?
6. **INFLUENTIAL (Xun, idx 5):** What does this concept GRADUALLY SHAPE or PERVADE?
7. **BALANCING (Dui, idx 6):** What does this concept MIRROR, EQUILIBRATE, or REFLECT?
8. **RECEPTIVE (Kun, idx 0):** What does this concept ACCEPT, FOLLOW, or GROUND itself in?

## Output Format

```json
[receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative]
```

Where each value is a float in [-1.0, 1.0], normalized to unit norm.

## Examples

### Rate Limiter
Restricts the number of requests a client can make in a time window.
- receptive: +0.05 | causal: -0.15 | transmissive: -0.55 | constraining: +0.85
- clarifying: +0.30 | influential: -0.20 | balancing: +0.20 | generative: -0.35

### Message Queue
Transmits events between services asynchronously with guaranteed delivery.
- receptive: +0.15 | causal: +0.25 | transmissive: +0.85 | constraining: -0.15
- clarifying: -0.20 | influential: +0.10 | balancing: +0.30 | generative: +0.05
