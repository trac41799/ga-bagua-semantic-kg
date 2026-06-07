# Phase 3: Semantic Operations API

**Date Range:** 2026-06-26 → 2026-07-02
**Status:** ⬜ Pending
**Epic:** Epic 3 — Semantic Operations API
**Depends On:** Phase 1 (Core Algebra) + Phase 2 (Bagua Mapping)

---

## Objective

Provide high-level semantic operations built on the algebraic and Bagua foundations, enabling practical knowledge graph relationship reasoning.

---

## Task Breakdown

### Day 1-2: Similarity & Difference (June 26-27)

| Task | Est. | Status |
|------|------|--------|
| Implement `semantic_similarity(&self, other: &Self) -> f64` | 2h | — |
| — Scalar part of geometric product, normalized to [-1, 1] | | |
| Implement `semantic_difference(&self, other: &Self) -> f64` | 2h | — |
| — Bivector magnitude, normalized to [0, 1] | | |
| Implement `angular_relationship(&self, other: &Self) -> f64` | 1h | — |
| — Angle between multivectors in full algebra | | |
| Write tests verifying metric properties | 2h | — |
| — Symmetry: sim(a,b) == sim(b,a) | | |
| — Non-negativity: diff(a,b) >= 0 | | |
| — Identity: sim(a,a) == 1 | | |

**Deliverable:** Similarity and difference metrics

### Day 3: Relation Classification (June 28)

| Task | Est. | Status |
|------|------|--------|
| Implement `semantic_relation(&self, other: &Self) -> Trigram` | 3h | — |
| — Classify relationship type via dominant blade of geometric product | | |
| Implement `relation_strength(&self, other: &Self) -> f64` | 1h | — |
| — Magnitude of the geometric product | | |
| Implement `is_contradictory(&self, other: &Self, threshold: f64) -> bool` | 2h | — |
| — High bivector magnitude indicates contradiction | | |
| Write relation classification tests | 2h | — |

**Deliverable:** Relation classification and contradiction detection

### Day 4: Analogy Computation (June 29)

| Task | Est. | Status |
|------|------|--------|
| Implement `analogy(a, b, c) -> Multivector` | 3h | — |
| — Compute `(a⁻¹b) * c` — rotor from A→B applied to C | | |
| Implement `analogy_confidence(a, b, c, expected) -> f64` | 2h | — |
| — Measure how close analogy result is to expected | | |
| Implement `analogy_batch(pairs: &[(A,B,C,D)]) -> f64` | 2h | — |
| — Average confidence across multiple analogy test cases | | |
| Write analogy test cases (king-man-woman-queen pattern) | 2h | — |

**Deliverable:** Analogical reasoning API

### Day 5: Relation Composition (June 30)

| Task | Est. | Status |
|------|------|--------|
| Implement `compose_relations(r1, r2) -> Rotor` | 3h | — |
| — Rotor multiplication: r2 * r1 (apply r1 then r2) | | |
| Implement `compose_chain(relations: &[Rotor]) -> Rotor` | 2h | — |
| — Fold composition across a chain of relations | | |
| Implement `inverse_relation(r: &Rotor) -> Rotor` | 1h | — |
| — Reverse the rotor to invert the relationship | | |
| Verify associativity: compose(compose(a,b),c) == compose(a,compose(b,c)) | 2h | — |
| Write composition tests | 2h | — |

**Deliverable:** Relation composition API

### Day 6: Context Transformations (July 1)

| Task | Est. | Status |
|------|------|--------|
| Implement `Context` struct holding a rotor | 1h | — |
| Implement `Context::apply(&self, mv: &Multivector) -> Multivector` | 1h | — |
| Implement `Context::compose(&self, other: &Context) -> Context` | 1h | — |
| Implement `Context::identity() -> Context` | 0.5h | — |
| Implement `Context::from_trigram_transform(from, to) -> Context` | 2h | — |
| Write context transformation tests | 2h | — |

**Deliverable:** Context transformation API

### Day 7: Integration Tests & Examples (July 2)

| Task | Est. | Status |
|------|------|--------|
| Write `analogy.rs` example (king-man-woman-queen) | 2h | — |
| Write `relation_compose.rs` example (multi-hop reasoning) | 2h | — |
| Write `cycle_demo.rs` example (Wuxing cycle traversal) | 2h | — |
| Write integration tests combining all operations | 2h | — |
| Update rustdoc for all new public items | 1h | — |

**Deliverable:** Examples and integration tests

---

## Verification Checklist

- [ ] `semantic_similarity(a, a) ≈ 1.0` for all non-zero multivectors
- [ ] `semantic_difference(a, a) ≈ 0.0` for all multivectors
- [ ] `semantic_relation(a, b)` returns valid trigram for all pairs
- [ ] Analogy: `analogy(king, man, woman) ≈ queen` (conceptually)
- [ ] Composition is associative: `compose(compose(a,b),c) == compose(a,compose(b,c))`
- [ ] Contradiction detection identifies known contradictory pairs
- [ ] All examples compile and run successfully

---

## API Reference

### Semantic Metrics

```rust
impl Multivector {
    /// Degree of alignment with another concept [-1, 1]
    pub fn semantic_similarity(&self, other: &Self) -> f64;

    /// Degree of difference/orthogonality [0, 1]
    pub fn semantic_difference(&self, other: &Self) -> f64;

    /// Classify relationship type as Bagua trigram
    pub fn semantic_relation(&self, other: &Self) -> Trigram;

    /// Full relational signature (geometric product)
    pub fn relational_signature(&self, other: &Self) -> Multivector;

    /// Check if two concepts are contradictory
    pub fn is_contradictory(&self, other: &Self, threshold: f64) -> bool;
}
```

### Analogy

```rust
/// Compute "A is to B as C is to ?"
/// Returns: (A⁻¹ * B) * C
pub fn analogy(a: &Multivector, b: &Multivector, c: &Multivector) -> Multivector;

/// Measure analogy confidence given expected result
pub fn analogy_confidence(
    a: &Multivector, b: &Multivector,
    c: &Multivector, expected: &Multivector
) -> f64;
```

### Composition

```rust
/// Compose two relations (apply r1 then r2)
pub fn compose_relations(r1: &Rotor, r2: &Rotor) -> Rotor;

/// Compose a chain of relations
pub fn compose_chain(relations: &[Rotor]) -> Rotor;

/// Invert a relation
pub fn inverse_relation(r: &Rotor) -> Rotor;
```
