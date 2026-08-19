# Path C — 20-Minute Guided Lesson Plan

**Tool:** `index.html` (double-click to open, works offline via `file://`)
**Session structure:** 5-min pre-quiz → 20-min guided walkthrough → 5-min post-quiz → 5-min Likert feedback.
**Total session time:** ~35 minutes (the walkthrough itself is the 20-minute block).

---

## Phase 0 — Pre-quiz (5 minutes)

1. Open `index.html` in the learner's browser.
2. Scroll to the **Quiz** module (nav chip "7 · Quiz").
3. Ensure the mode reads "Mode: pre — before the lesson" (button **Pre-quiz** active).
4. Ask the learner to answer all 5 questions without help. Record the score.

---

## Phase 1 — Guided walkthrough (20 minutes)

Work top-to-bottom through the modules. For each: demonstrate once, then let the learner click.

### 1. Trigram gallery (2 min) — `gallery`
- Show the 8 panels: 坤 → 乾, ordered by grade.
- **Teaching point:** reading bottom-up, each **yang** line selects a basis vector (e1, e2, e3). A blade is the *product of the selected vectors*; the grade is the *number of yang lines* (0 → scalar, 1 → line, 2 → plane, 3 → volume).
- **Teaching point:** 离 (☲) is the interesting one: bottom and top yang → e1·e3 = e13 = **−e31**. Orientation signs are real.

### 2. Blade viewer (2.5 min) — `blade-view`
- Click trigrams in the gallery and watch the isometric view change: point → arrow → plane → cube.
- **Teaching point:** "grade" = dimension of the geometric object, not a semantic category.

### 3. Line flip (3 min) — `line-flip`
- Pick 震 (e1). Click the middle line: e1 → e1·e2 = e12. The trigram becomes 兑.
- **Teaching point:** flipping line *i* = multiplying the blade by e_{i+1} (bit XOR). Watch the sign when the result flips orientation (e.g. flip line 3 of 乾: e3·e123 = e12 with sign +1, but flip other lines and watch signs appear).

### 4. Product table (4 min) — `product-table`
- Click 兑 (e12) row × 震 (e1) column: e12·e1 = −e2 (red cell).
- **Teaching point:** green = sign +1, red = sign −1. **Order matters**: e1·e2 = e12 but e2·e1 = −e12. And e12·e12 = −1: squares of bivectors are negative scalars.
- Ask the learner to find three red cells and predict the sign before clicking.

### 5. Hexagram stack (2.5 min) — `hexagram-stack`
- Set upper = 坎 (water), lower = 离 (fire) → 水火既济 #63. Try the Random button.
- **Teaching point:** a hexagram is just (upper, lower) — a stacking rule. Names/glosses are traditional and presentational; this tool makes no interpretive claims.

### 6. Rotor demo (4 min) — `rotor-demo`
- Rotate e1 in the e12 plane. Drag θ to π: e1 → −e1. Set θ to π/2: e1 → e2. Press **Animate**.
- **Teaching point:** the rotor R = cos(θ/2) − sin(θ/2)·B and the sandwich a′ = R·a·R̃. The readout shows the *actual computed* result at every angle, and |a′| stays 1.

### 7. Quiz (1 min, optional preview)
- Point out the 5 questions map 1:1 to the modules above.

### 8. Glossary (1 min) — `glossary`
- Recap the full role ↔ trigram ↔ blade ↔ grade mapping in one glance.

---

## Phase 2 — Post-quiz (5 minutes)

1. Back in the **Quiz** module, press the **Post-quiz** button (mode label changes).
2. The learner answers the same 5 questions again — answers are reset and scored fresh.
3. Record the score next to the pre score.

---

## Phase 3 — Likert feedback (5 minutes)

Read each item aloud; learner rates 1 (strongly disagree) – 5 (strongly agree). Verbatim items (one source of truth: also in `output/human-gate-report.md`):

1. "The visualization helped me understand what a blade is and what its grade means." (1–5)
2. "The product table made the geometric product (signs and blade results) clear." (1–5)
3. "The line-flip module helped me see how flipping a line relates to multiplying by a basis vector." (1–5)
4. "The rotor demo helped me understand the sandwich product a' = R a R~." (1–5)
5. "Overall, I would recommend this tool to someone learning geometric algebra." (1–5)

Plus free text: "What was most confusing? What would you change?"

---

## Recording & gate

Fill `output/human-gate-report.md` with: session log, pre/post score table, Likert results, free text.

**Pre-registered thresholds:** ≥60% of learners improve pre→post; ≥70% rate ≥4/5 on Likert. If the first feedback session shows no learning signal, Path C is killed (decision gate from `v2/LESSONS.md`).
