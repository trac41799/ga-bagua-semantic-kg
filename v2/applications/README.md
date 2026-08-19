# Applications Portfolio — Clifford/Bagua Principle (Evidence-Graded)

**Date:** 2026-08-15 | **Status:** Evidence-graded builds and products from the surviving principle
**The principle that survived all falsification attempts:** *GA computes (flip ·ei, complement ·e123, wedge, grade, product); Bagua names (8 trigram states, 64 hexagram states). Never the reverse.*

## Active strategic documents
- **[Yarrow — Product Spec](products/yarrow-factorial/PRODUCT_SPEC.md)** — industry-standard PRD for the validated POC-02 application (interaction algebra workbench)
- **[Yarrow — Product Plan](products/yarrow-factorial/PRODUCT_PLAN.md)** — phased delivery with grow/kill gates
- **[I-Ching × AI Adoption Strategy](iching-ai-adoption-strategy.md)** — evidence-graded applications with and without GA; everything unproven is pre-registered before build

## PROMOTION STATUS

The source of truth for claim status is [`../qa/promotion-report.md`](../qa/promotion-report.md), not the mechanical execution-gate count.

- **POC-05:** `PASS` and R-05 `PASS` on the frozen second-model replication; host-product fit remains `PENDING`.
- **POC-08:** `MODEL_DEPENDENT`; R-08 replication `FAIL` on `gpt-4o-mini`, so the claim is not model-general or product-green.
- **POC-10:** `INCONCLUSIVE`; historical objective and calibration findings are retained, but the clean no-leakage rerun is `PENDING`.
- **POC-11/12/14:** deterministic claim, execution, and evidence statuses `PASS`; product status `PENDING` until host/product fit is evidenced.
- **ICHING-TOOLS:** package/MCP/SDK mechanics `PASS`; product status `PENDING` until host/product fit is evidenced.
- **Yarrow:** `TECHNICAL_BETA` recommendation only; external-user and task-time product validation is `PENDING`.

---

## HISTORICAL FOUNDATIONAL POC RESULTS (2026-08-08 — POCs 01-04)

| POC | Claim | Result | Verdict |
|-----|-------|--------|---------|
| **01** Combinatorial scaffold | Scaffold beats LLM-alone ≥ +20pp on 50 exact problems | **+2.0pp** (scaffold 0.120, alone 0.100); decomposition: 37/50 failures = LLM JSON-format non-compliance, **0 calculator failures** | **FAIL** — protocol problem, not algebra; calculator proven exact wherever it ran |
| **02** Factorial explorer | Blade algebra reproduces 2^k factorial math at 100% | 2³/2⁴ contrast signs **22/22 exact** vs independent brute force; Möbius decomposition err **3.91e-14**; names 19/19 | **PASS — the thesis holds fully here** |
| **03** Reframing engine | Algebra-grounded reframes ≥ +0.15 diversity, ≥3.5 coherence | Diversity Δ **+0.082** (Arm B 0.360 > Arm A 0.278); coherence **2.80** | **FAIL** — direction correct, margins not met; naming protocol is the weak link |
| **04** Tagged memory | Tag quality ≥80%, precision ≥0.5, stability, non-interference | Quality **66.7%**, precision **0.46**, stability **83.3%**, non-interference **True** | **FAIL** — vocabulary not discriminative enough as an audit layer |

**The consolidated thesis verdict (across ALL experiments to date — A, B, D, POC-01..04):**

> **GA computes — perfectly. Bagua names — inconsistently.**
> Everywhere the algebra was asked to *compute* (POC-02; the calculator inside POC-01; the Path D ledger), it was exact. Everywhere the Bagua/LLM layer was asked to produce *semantics* (roles as labels, per-position naming, tag vocabulary), it failed to reach pre-registered bars. The single fully validated application is the one where the algebra does the work and Bagua only labels: **POC-02**.

**Salvageable directions (each needs a NEW pre-registration, per the failure ledger rule):**
- POC-01 v2: few-shot JSON format enforcement + canonical answer format for both arms (the calculator is already proven exact).
- POC-03 v2: stronger naming protocol (few-shot position descriptions) and a re-derived margin.
- POC-04 v2: fewer/more discriminative roles or learned tag axes (the stability and non-interference machinery already passes).

---


Every application below is rated by **structural success probability** (how likely the construction is to work as specified, given it uses the coherent coupling only) and **competitive impact**. Ratings are honest, not promotional. Anything that failed has been archived to `archive/experimentation/fails/` with its evidence.

## The coherent coupling (what all POCs build on)

1. **3-bit ↔ blade**: a trigram's three lines ↔ exponents of (e1,e2,e3); grade = number of yang lines; the natural blade = product of selected basis vectors.
2. **Line-flip = multiply by ei**; **complement (antipode of the cube) = Hodge dual ·e123** — under the natural convention, complementary trigrams (Kan↔Li, Gen↔Dui, …) are the algebra's duality, up to orientation sign.
3. **Hexagram = 6-bit state** (Cl(6) blade / 64-vertex hypercube); upper/lower = 3⊕3 split.
4. **Grade = interaction order** — blades are interaction terms; the product table is the sign bookkeeping of ordered combinations.

**Hard rules (from the failure ledger):** no semantic labels derived from products; no WuXing as dynamics; never fewer than ~16 effective dimensions for retrieval; every claim pre-registered with baselines.

---

## RECOMMENDED APPLICATIONS (build order)

### POC-01 — Combinatorial Reasoning Scaffold for LLMs ⭐ highest priority
| Dimension | Rating |
|-----------|--------|
| Structural success probability | **~65%** (algebra exact; risk = LLM translation in/out, mitigated by strict JSON protocol + validation) |
| Novelty | High (no standard tool grounds LLM combinatorial reasoning in a blade algebra with a cultural state vocabulary) |
| Competitive impact | **High if it works** — LLM exact-reasoning failure is a live, costly problem (parity, De Morgan, interaction counting, sign errors) |
| LLM needed | Yes (translator/interpreter); the calculator is deterministic |

**Idea:** LLMs are reliably wrong at exact combinatorial reasoning. POC-01 wraps a deterministic Cl(3) calculator (flip, complement, wedge, grade, product) behind an LLM translation protocol: LLM encodes a problem statement into blade operations (JSON), the algebra computes exactly, the LLM interprets the result. The Bagua vocabulary (8 trigrams, 64 hexagrams) gives the states human-readable names. Verdict benchmark: 50 problems across 5 categories (parity/bit ops, complement/De Morgan, interaction counting, sign bookkeeping, hexagram composition), scaffold vs LLM-alone, pre-registered ≥20pp improvement, p<0.05.
**Reuses:** verified Cl(3) core (K1); rubric JSON protocol (Path D); baseline/gate machinery.

### POC-02 — Factorial Interaction Design Explorer
| Dimension | Rating |
|-----------|--------|
| Structural success probability | **~90%** (the algebra IS the domain math — blades are interaction terms; correctness is by construction) |
| Novelty | Moderate (the math is classical 2^k factorial design; the Bagua-mnemonic interface + interaction decomposition tooling is the new package) |
| Competitive impact | Niche (experiment design / feature-interaction analysis tooling) |
| LLM needed | Optional (later: natural-language experiment descriptions → design) |

**Idea:** A library + report generator for 2^k factorial designs where interaction terms ARE blades: grade = interaction order, the geometric product table reproduces the contrast signs of ANOVA, Möbius inversion on the subset lattice gives interaction-effect decomposition. The Bagua layer is a mnemonic UI: a trigram IS a 3-factor interaction term; hexagrams stack designs. Success = 100% agreement with textbook contrast tables and brute-force interaction decomposition.
**Reuses:** verified Cl(3) core; nothing else.

### POC-03 — Dialectical Reframing Engine ("8 positions of the cube")
| Dimension | Rating |
|-----------|--------|
| Structural success probability | **~75%** (the moves are real operators; risk = LLM naming quality per position) |
| Novelty | High (algebra-grounded reframing: every view is a specific, explainable algebraic move) |
| Competitive impact | Moderate (creative/consulting/ideation tooling; LLM-agent self-review) |
| LLM needed | Yes (names each position; evaluates diversity/coherence) |

**Idea:** For any concept or statement, generate its 8 cube positions: 3 single-line flips (edge neighbors), 3 double-flips, and the complement (antipode via ·e123). Each move is a real operator — the "reframes" are not free-form paraphrases but a closed set of structurally defined views. LLM assigns language per position. Automated proxy metrics first (diversity = mean pairwise embedding distance; coherence = LLM-judged), human gate later.
**Reuses:** flip/complement operators; LLM JSON protocol; Path C module patterns.

### POC-04 — Interpretable-Tag Agent Memory (tag layer over standard retrieval)
| Dimension | Rating |
|-----------|--------|
| Structural success probability | **~55%** (mechanism D3-validated; the specific tag-layer construction is new) |
| Novelty | Moderate (agent-memory auditability via role tags is a real 2026 gap) |
| Competitive impact | High if it works (memory governance + explanation for LLM agents) |
| LLM needed | Yes (assigns tags; verifies) |

**Idea:** The D3-validated mechanism (LLM-judgment semantic encoding beats lexical IR) applied at the right capacity: standard retrieval (BM25 now; dense embeddings when installable) + the 8 Bagua roles as an **audit/explanation tag layer** — every memory item carries role tags ("constrains," "transmits," "balances") that (a) explain why an item was surfaced, (b) enable zero-token tag-based filtering, (c) never alter the ranking. Phase-1 verdicts pre-registered around TAG QUALITY (LLM-tag vs human-tag agreement ≥ 80%; tag stability; tag-filter precision) — NOT retrieval accuracy, which depends on real embeddings (documented dependency).
**Lesson encoded:** never make the 8 roles the embedding (D1/D2); tags sit on top.

---

## ALREADY BUILT (keep — not new POCs)

| Item | Status | Next step |
|------|--------|-----------|
| **Path C** — GA×Bagua teaching tool | All gates green | Add the complement = Hodge-dual module (the coherent identity) + run the human gate |
| **Path D** — decisive experiment harness | Executed, verdicts banked | Reusable as the LLM-eval substrate for POCs 01/03/04 |

## REJECTED (evidence in `archive/experimentation/fails/README.md`)

| Application | Reason |
|-------------|--------|
| Semantic KG / relation classification from Bagua roles | A-v1/A-v2: learned models lost to trivial baselines; labels couldn't be derived |
| Context compression via 8-dim index | D2: break-even 23 queries, 47% of full-context recall |
| Reasoning enhancement via index-fed pipelines | Cascade from D1/D2 retrieval loss |
| GA-KG embeddings as a product | B-v2: TransE beat rotors on CI; bagua init harmful |
| WuXing-cycle dynamics / role-derived semantics | No algebraic realization exists (structural) |

---

## Success-rate summary

| POC | Structural success | Impact if it works | Effort | Order |
|-----|--------------------|--------------------|--------|-------|
| 01 Combinatorial scaffold | ~65% | High | S (1–2 wks) | 1 |
| 02 Factorial explorer | ~90% | Niche | S (1 wk) | 2 |
| 03 Reframing engine | ~75% | Moderate | S (1–2 wks) | 3 |
| 04 Tagged agent memory | ~55% | High | M (2–4 wks) | 4 |
