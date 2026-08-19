# Lessons Ledger — v2 Portfolio

**Purpose:** The single source of truth for what the v1 project (ga-bagua-semantic-kg) got right and wrong. Every v2 probe MUST be auditable against this ledger. Any probe that regresses a lesson fails its QA gate.

## What v1 proved wrong (lessons to never repeat)

| # | Lesson | v1 evidence | v2 enforcement |
|---|--------|-------------|----------------|
| L1 | **Circular validation is worse than no validation.** Ground truth derived from the same encodings produces meaningless metrics. | 87% of benchmark relation labels == dominant role of concept A; WuXing cycle matched its own labels 7/37 times | Independent relational labels; circularity gate (report %labels==dom(A), FAIL >60%); inter-annotator protocol |
| L2 | **Trivial baselines must ship with every metric.** A "sophisticated" system is only as good as the baselines it beats. | Full classifier 52% test vs "label=dominant role of A" 80% vs majority 24% | Baseline wall: majority, dom(A), cosine kNN, random rendered in every eval table |
| L3 | **The mechanism must be the mechanism.** Claimed GA, actually lookup tables + vector reads. | Geometric-product feature (f4) changed 1/53 predictions; removing it changed nothing | Ablation gate: any feature changing <5% of predictions or <1pp accuracy is rejected |
| L4 | **Benchmarks must run the real production path.** Hand-authored coefficients masquerading as LLM output. | All benchmark encodings hand-tuned (`suggested_coefficients`) | Prod-path eval: real LLM API or documented stub; fixtures only for unit tests |
| L5 | **One source of truth for numbers.** | 2.9x vs 2.4x token savings; 59% vs 88.8% P@5; NIAH kept and removed; 99.8%@±5% vs ±10% | Claims ledger: every number → run artifact → baseline → date → owner; README renders from ledger |
| L6 | **Taxonomy is a hypothesis, not a first principle.** Cycles/opposites/transforms must be tested against data. | WuXing generate/control contradicted dataset labels 30/37 times | All structural claims treated as features to ablate, never as truth |
| L7 | **Scope before validation kills projects.** | 8 crates, 33 MCP tools, 5 README languages before the core claim survived a baseline | Portfolio of probes: 2-4 week timeboxes, minimal surface, decision gate before funding |
| L8 | **Marketing ahead of evidence.** | "Zero tokens forever", "48x denser than BERT", "unique to GA-Bagua" | Front page == internal assessment; claims ledger gate in CI |
| L9 | **Overfitting disguised as progress.** | Refinement 56.1% → calibrated CV 17.5% | Split mandate: no number without train/test/CV + fixed seed |
| L10 | **Multi-hop stability is float64 math, not semantics.** | "100-hop composition, zero drift" tested random rotors | No semantic claims derived from numerical stability |

## What v1 got right (keep)

| # | Asset | Why it's kept |
|---|-------|---------------|
| K1 | Cl(3) multivector implementation + geometric product table | Correct, tested; reused as the algebra core for probes A and B |
| K2 | SKILL.md rubric (LLM → 8 interpretable coefficients) | Genuinely good encoding protocol; reused verbatim as `encoding.RubricEncoder` |
| K3 | Pipeline pattern: encode once → deterministic ops → LLM verifies | Sound economics for multi-query agent memory; the real product insight |
| K4 | 3-bit↔blade isomorphism (line-flip = vector product, hexagram = blade product) | The one structurally sound piece of the Bagua↔Cl(3) mapping; foundation of Path C |
| K5 | Realism-assessment documentation practice | Kept, moved to the front page instead of a buried doc |

## Decision gates (what each probe must answer)

| Probe | Kill criterion | Timebox |
|-------|----------------|---------|
| A | Probe fails to beat "dominant role of A" by >10pp (p<0.05) on relational labels, OR grade-spectrum ablation is null | 2 weeks |
| B | GeomE ≤ RotatE on 2/3 public benchmarks AND no human-rated interpretability gain | 4 weeks |
| C | No adoption/learning signal in expert/user evaluation (pre-registered threshold) | 2 weeks |
