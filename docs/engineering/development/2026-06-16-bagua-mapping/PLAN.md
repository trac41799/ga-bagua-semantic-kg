# Phase 2: Bagua Mapping Layer

**Date Range:** 2026-06-16 → 2026-06-25
**Status:** ⬜ Pending
**Epic:** Epic 2 — Bagua Relationship Taxonomy
**Depends On:** Phase 1 (Core Algebra Engine)

---

## Objective

Map Cl(3) basis blades to Bagua trigrams and implement the complete relationship taxonomy with WuXing transformation cycles.

---

## Task Breakdown

### Day 1-2: Trigrams Enum (June 16-17)

| Task | Est. | Status |
|------|------|--------|
| Define `Trigrams` enum with 8 variants | 1h | ⬜ |
| Implement `Trigram::blade() -> Blade` mapping | 2h | ⬜ |
| Implement `Blade::trigram() -> Trigram` reverse mapping | 1h | ⬜ |
| Add Chinese names: `name() -> &str` | 1h | ⬜ |
| Add English translations: `translation() -> &str` | 1h | ⬜ |
| Add binary encoding: `binary() -> [bool; 3]` | 1h | ⬜ |
| Add semantic descriptions: `description() -> &str` | 1h | ⬜ |
| Write bidirectional mapping tests | 2h | ⬜ |

**Deliverable:** `src/bagua.rs` — Trigram enum with full metadata

### Day 3-4: Hexagram Type (June 18-19)

| Task | Est. | Status |
|------|------|--------|
| Define `Hexagram` struct with upper/lower trigrams | 1h | ⬜ |
| Implement `Hexagram::new(upper, lower)` | 1h | ⬜ |
| Implement `Hexagram::upper()` and `Hexagram::lower()` | 0.5h | ⬜ |
| Implement `hexagram_number() -> u8` (1-64 traditional numbering) | 2h | ⬜ |
| Implement `hexagram_name() -> &str` (traditional name) | 2h | ⬜ |
| Implement `Hexagram::from_multivector_pair(a, b)` | 2h | ⬜ |
| Write all 64 hexagram construction tests | 2h | ⬜ |

**Deliverable:** `src/bagua.rs` — Hexagram type with 64 states

### Day 5: WuXing Cycles (June 20)

| Task | Est. | Status |
|------|------|--------|
| Define `WuXing` enum with 5 phases | 0.5h | ⬜ |
| Implement `WuXing::generate() -> WuXing` (generating cycle) | 1h | ⬜ |
| Implement `WuXing::control() -> WuXing` (controlling cycle) | 1h | ⬜ |
| Implement `Trigram::wuxing_phase() -> WuXing` | 1h | ⬜ |
| Implement `WuXing::trigrams() -> &[Trigram]` | 0.5h | ⬜ |
| Write cycle verification tests | 1h | ⬜ |

**Deliverable:** `src/wuxing.rs` — Five Elements transformation cycles

### Day 6-7: Trigram Transformations (June 21-22)

| Task | Est. | Status |
|------|------|--------|
| Implement `Trigram::transform_line(line: usize) -> Trigram` | 2h | ⬜ |
| Implement `Trigram::all_transforms() -> [Trigram; 3]` | 1h | ⬜ |
| Implement `Trigram::complementary() -> Trigram` (yin↔yang flip) | 1h | ⬜ |
| Map line changes to basis vector multiplication | 2h | ⬜ |
| Write transformation rule tests | 2h | ⬜ |

**Deliverable:** Trigram line-change transformation operations

### Day 8: Semantic Integration (June 23)

| Task | Est. | Status |
|------|------|--------|
| Implement `Multivector::dominant_trigram() -> Trigram` | 2h | ⬜ |
| Implement `Multivector::relationship_to(&self, other: &Self) -> Hexagram` | 3h | ⬜ |
| Implement `Multivector::trigram_weights() -> [f64; 8]` | 1h | ⬜ |
| Implement `Trigram::as_rotor(theta: f64) -> Rotor` | 2h | ⬜ |

**Deliverable:** Integration between multivector operations and Bagua taxonomy

### Day 9-10: Tests & Documentation (June 24-25)

| Task | Est. | Status |
|------|------|--------|
| Write Bagua mapping validation tests | 3h | ⬜ |
| Write hexagram construction tests (all 64) | 2h | ⬜ |
| Write WuXing cycle tests | 1h | ⬜ |
| Write trigram transformation tests | 2h | ⬜ |
| Update rustdoc for all public items | 2h | ⬜ |
| Run `cargo clippy` and fix warnings | 1h | ⬜ |

**Deliverable:** `tests/bagua_tests.rs` — complete Bagua test suite

---

## Verification Checklist

- [ ] 8 trigrams ↔ 8 basis blades bidirectional mapping correct
- [ ] 64 hexagrams constructible from all trigram pairs
- [ ] WuXing generating cycle: Wood→Fire→Earth→Metal→Water→Wood
- [ ] WuXing controlling cycle: Wood→Earth→Water→Fire→Metal→Wood
- [ ] `dominant_trigram()` returns correct trigram for single-blade multivectors
- [ ] Line-change transformations produce correct results
- [ ] All public items have rustdoc documentation

---

## Bagua Reference Table

| Trigram | Blade | Grade | Binary | Chinese | English | WuXing |
|---------|-------|-------|--------|---------|---------|--------|
| ☷ Kun | Scalar | 0 | 000 | 坤 | Earth | Earth |
| ☶ Gen | E3 | 1 | 001 | 艮 | Mountain | Earth |
| ☵ Kan | E2 | 1 | 010 | 坎 | Water | Water |
| ☴ Xun | E23 | 2 | 011 | 巽 | Wind | Wood |
| ☳ Zhen | E1 | 1 | 100 | 震 | Thunder | Wood |
| ☲ Li | E12 | 2 | 101 | 離 | Fire | Fire |
| ☱ Dui | E31 | 2 | 110 | 兌 | Lake | Metal |
| ☰ Qian | E123 | 3 | 111 | 乾 | Heaven | Metal |
