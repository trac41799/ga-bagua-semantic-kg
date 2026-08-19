# Spec C — GA×Bagua Teaching / Visualization Tool (Path C)

**Status:** Pre-registered probe | **Timebox:** 2 weeks | **Owner:** Path C agent
**Falsifiable question:** Can a single-file, offline, dependency-free interactive visualization make the 8-blade↔8-trigram isomorphism, the geometric product, and line-flip dynamics comprehensible to a learner — measured by a pre/post quiz and learner feedback (human gate, pre-registered thresholds)?

## 1. Pre-registration (written before any code)

| Item | Commitment |
|------|------------|
| Primary claim | ≥60% of learners improve quiz score pre→post (min n=5, documented session) |
| Secondary claim | ≥70% of learners rate the visualization "helped understand GA" (Likert ≥4/5) |
| Human gate | Manual step after probe ships; tool must be runnable by a non-technical person (double-click → works offline) |
| Scope guard | NO semantic claims. The tool teaches the *isomorphism* (K4 lesson), not "Bagua semantics." |
| Kill criterion | No learning signal in first feedback session → Path C dies |

## 2. Scope

**In:** single self-contained `index.html` (vanilla JS + SVG, no build, no CDN, no network), interactive modules (below), embedded 5-question pre/post quiz with scoring, lesson plan + evaluation protocol doc, structural tests.
**Out:** No backend, no bundler, no framework, no fake benchmarks, no claims beyond pedagogy.

## 3. Interactive modules (all in one file, stable DOM IDs for automated checks)

| Module | DOM id | Content |
|--------|--------|---------|
| Trigram gallery | `gallery` | 8 trigrams (Kun..Qian) as 3-line patterns + blade labels (1, e1, e2, e3, e12, e23, e31, e123) + grades |
| Blade viewer | `blade-view` | Selected trigram as oriented geometric object (isometric SVG: line/plane/volume for grade 1/2/3), selectable |
| Line flip | `line-flip` | Click a line → bit flips → shows resulting trigram AND the blade-label change (e1·flip semantics) |
| Product table | `product-table` | Interactive blade × blade grid; click two → shows geometric product (blade + sign), e.g., e1·e2=e12, e2·e1=−e12, e12²=−1 |
| Hexagram stack | `hexagram-stack` | Upper/lower trigram stacking → hexagram name + interpretation (purely presentational) |
| Rotor demo | `rotor-demo` | Slider θ → rotor R=e^(−θB/2) applied to a blade via sandwich product; animated |
| Quiz | `quiz` | 5 questions (blade grades, product signs, line-flip semantics, hexagram stacking, rotor effect), instant scoring, pre/post mode |
| Glossary | `glossary` | Role names ↔ trigrams ↔ blades ↔ grades |

## 4. Components & interfaces

| Component | File | Interface |
|-----------|------|-----------|
| Tool | `index.html` | single file; `window.BaguaTool` exposes `product(a,b)`, `flipLine(t,i)`, `hexagram(u,l)` for tests |
| Lesson plan | `docs/lesson-plan.md` | 20-min guided session, pre/post protocol, feedback form |
| Tests | `tests/test_tool.py` | structural checks (HTML parses, required IDs present, product/flip logic correct) |

## 5. Math core (the ONLY truthful content — K4)

- Trigram line bits ↔ exponent of (e1,e2,e3); blade = product of selected vectors; grade = number of yang lines.
- `product(a,b)` uses the standard Cl(3) table (ported from v1 `multivector.rs`, which was verified correct).
- Line flip = multiplication by the corresponding basis vector (bit XOR).
- Hexagram = (upper, lower) pair — presentational only, no interpretive claims.
- Rotor: R = cos(θ/2) − sin(θ/2)·B, sandwich a' = R a R̃.

## 6. Evaluation protocol (human gate — manual, documented)

1. Session: 5-min pre-quiz → 20-min guided walkthrough (lesson plan) → 5-min post-quiz → 5-question Likert feedback.
2. Record: pre/post scores, Likert items, free-text. Target n≥5.
3. Gate: ≥60% improve; ≥70% rate ≥4/5. Report goes into `output/human-gate-report.md` (template provided; to be filled after real sessions).
4. No quantitative claim of any kind before this report exists.

## 7. Acceptance criteria (see `../tdd/path-c-tdd.md` for test cases)

- AC-C1 `index.html` is single-file, zero external requests (verified: no `http`/`src=`/`link href` to network; opens via `file://`)
- AC-C2 All 8 trigram panels render with correct binary lines + blade labels
- AC-C3 `window.BaguaTool.product` correct on ≥12 hand-checked pairs (incl. e1e2=e12, e2e1=−e12, e12²=−1, e1e1=1)
- AC-C4 `flipLine` correct on all 8×3 line flips (bit semantics)
- AC-C5 `hexagram(u,l)` returns the 64-name table correctly (spot-check ≥8)
- AC-C6 Rotor demo: sandbox application matches analytic result (θ=π in e12 maps e1→−e1; θ=π/2 maps e1→e2)
- AC-C7 Quiz present with 5 questions, scoring works (answering all correct → 5/5)
- AC-C8 HTML parses (Python html.parser, no errors); inline JS syntax valid (`node --check` on extracted script)
- AC-C9 Stable DOM IDs present (gallery, blade-view, line-flip, product-table, hexagram-stack, rotor-demo, quiz, glossary)
- AC-C10 `tests/test_tool.py` green; README documents open instructions + human-gate protocol
