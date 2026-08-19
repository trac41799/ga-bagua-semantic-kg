# Path C — GA × Bagua Teaching / Visualization Tool

A single-file, offline, zero-dependency interactive visualization of the **8-blade ↔ 8-trigram isomorphism** (the K4 lesson from the v1 project): the geometric product, line-flip dynamics, hexagram stacking, and rotor sandwich products.

## How to open

1. Navigate to `v2\probes\path-c\`.
2. **Double-click `index.html`** — it opens via `file://` with no build step, no frameworks, no CDN, and no network requests of any kind. The entire tool (styles, SVG, math core, interactivity) is one self-contained file.

Verified by `tests/verify_offline.py` and enforced in CI by `tests/test_tool.py`.

## Module guide (all in `index.html`)

| # | Module | DOM id | What it teaches |
|---|--------|--------|-----------------|
| 1 | Trigram gallery | `gallery` | 8 trigrams ↔ 8 blades (1, e1, e2, e3, e12, e23, e31, e123) + grades |
| 2 | Blade viewer | `blade-view` | Blade as geometric object: point / line / plane / volume (isometric SVG) |
| 3 | Line flip | `line-flip` | Flipping a line = multiplying the blade by the corresponding basis vector (bit XOR) |
| 4 | Product table | `product-table` | Interactive 8×8 geometric-product table with sign coloring |
| 5 | Hexagram stack | `hexagram-stack` | Upper-over-lower stacking → all 64 King Wen names (presentational) |
| 6 | Rotor demo | `rotor-demo` | R = cos(θ/2) − sin(θ/2)·B, sandwich a′ = R·a·R̃, animated θ slider |
| 7 | Quiz | `quiz` | 5 questions, instant scoring, pre/post mode |
| 8 | Glossary | `glossary` | Roles ↔ trigrams ↔ blades ↔ grades |

## Math core (verified against v1)

`window.BaguaTool` exposes the pure functions used by the UI (also what the tests assert against):

- `product(a, b)` → `{blade, sign}` — Cl(3) geometric product, exactly the verified `PROD_TABLE` from v1 `multivector.rs`.
- `flipLine(trigramMask, lineIdx)` → new mask — bit XOR with `1 << lineIdx`.
- `hexagram(upperMask, lowerMask)` → `{number, name, pinyin, gloss}` — the 64 King Wen names (e.g. 坎 over 离 = 水火既济 #63).
- `rotorApply(theta, plane, blade)` → `{blade, sign, coeff, norm}` — full multivector sandwich.
- `quizScore(answers)` → 0–5.

Run the math-core test suite with `node tests/run_js_tests.js`.

## Lesson plan

`docs/lesson-plan.md` — a 20-minute guided session: 5-min pre-quiz → module-by-module walkthrough with teaching points → 5-min post-quiz → 5-question Likert feedback form (verbatim questions listed).

## Human-gate protocol (pre-registered)

After the tool ships, run **at least one documented session (target n ≥ 5 learners)**:

1. **Pre-quiz** (5 min): learner answers the 5 questions in the quiz module in *pre* mode.
2. **Guided walkthrough** (20 min): follow `docs/lesson-plan.md`.
3. **Post-quiz** (5 min): same 5 questions in *post* mode.
4. **Feedback** (5 min): 5-question Likert (1–5) + free text (verbatim items in the lesson plan).

**Pre-registered thresholds** (written before any session, one source of truth — also in `output/human-gate-report.md`):

- **Primary claim:** ≥60% of learners improve their quiz score pre→post.
- **Secondary claim:** ≥70% of learners rate the tool ≥4/5 on the Likert items.
- **Kill criterion:** no learning signal in the first feedback session → Path C dies.

Report results in `output/human-gate-report.md` (template provided). **No quantitative claim of any kind may be made before this report exists.**

## Honesty statement

This tool teaches the **isomorphism** — the one structurally sound piece of the Bagua↔Cl(3) mapping (K4). **It makes NO semantic claims**: it does not claim that hexagrams "mean" anything, that the algebra predicts anything, or that Bagua semantics can be derived from Cl(3). All hexagram names and glosses are traditional, presentational content with no interpretive claims. All numerical results come from the verified v1 product table; there are no baselines, no benchmarks, and no fake numbers here.
