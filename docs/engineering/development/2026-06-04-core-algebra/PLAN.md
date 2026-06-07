# Phase 1: Core Algebra Engine

**Date Range:** 2026-06-04 → 2026-06-15
**Status:** 🟡 In Planning
**Epic:** Epic 1 — Core Algebra Engine

---

## Objective

Implement a mathematically correct Cl(3) Clifford algebra engine in Rust with zero external dependencies.

---

## Task Breakdown

### Day 1-2: Multivector Type (June 4-5)

| Task | Est. | Status |
|------|------|--------|
| Define `Multivector` struct with `[f64; 8]` | 2h | ⬜ |
| Implement `Multivector::new()` constructor | 1h | ⬜ |
| Implement `Multivector::zero()` and `Multivector::one()` | 0.5h | ⬜ |
| Implement `Add`, `Sub`, `Mul<f64>`, `Neg` traits | 3h | ⬜ |
| Implement `fmt::Display` for human-readable output | 1h | ⬜ |
| Implement `approx_eq` with tolerance for float comparison | 1h | ⬜ |

**Deliverable:** `src/multivector.rs` — basic multivector arithmetic

### Day 3: Basis Blades (June 6)

| Task | Est. | Status |
|------|------|--------|
| Define `Blade` enum with 8 variants | 1h | ⬜ |
| Define grade constants and lookup table | 1h | ⬜ |
| Implement blade-to-index mapping | 1h | ⬜ |
| Implement `Multivector::from_blade(blade, coeff)` | 1h | ⬜ |
| Implement `Multivector::blade_coefficient(blade)` | 1h | ⬜ |

**Deliverable:** `src/blades.rs` — blade constants and helpers

### Day 4-5: Geometric Product (June 7-9)

| Task | Est. | Status |
|------|------|--------|
| Implement Cl(3) Cayley table as `const` array | 3h | ⬜ |
| Implement `geo_product(&self, other: &Self) -> Self` | 4h | ⬜ |
| Implement `inner_product(&self, other: &Self) -> f64` | 2h | ⬜ |
| Implement `wedge_product(&self, other: &Self) -> Self` | 2h | ⬜ |
| Implement `reverse(&self) -> Self` | 1h | ⬜ |
| Implement `grade_projection(&self, k: usize) -> Self` | 2h | ⬜ |

**Deliverable:** `src/multivector.rs` — complete algebraic operations

### Day 6: Inverse & Norm (June 10)

| Task | Est. | Status |
|------|------|--------|
| Implement `norm_squared(&self) -> f64` | 1h | ⬜ |
| Implement `norm(&self) -> f64` | 0.5h | ⬜ |
| Implement `inverse(&self) -> Option<Self>` | 3h | ⬜ |
| Implement `dualize(&self) -> Self` (pseudoscalar multiply) | 1h | ⬜ |
| Handle degenerate cases (zero norm) | 1h | ⬜ |

**Deliverable:** Inverse and dualization operations

### Day 7: Rotors (June 11)

| Task | Est. | Status |
|------|------|--------|
| Define `Rotor` struct wrapping `Multivector` | 1h | ⬜ |
| Implement `Rotor::new(theta, bivector_plane)` | 2h | ⬜ |
| Implement `Rotor::apply(&self, mv: &Multivector) -> Multivector` | 2h | ⬜ |
| Implement `Rotor::compose(&self, other: &Rotor) -> Rotor` | 1h | ⬜ |
| Verify unitarity: `R * R̃ = 1` | 1h | ⬜ |

**Deliverable:** `src/rotor.rs` — rotor construction and application

### Day 8-9: Tests (June 12-13)

| Task | Est. | Status |
|------|------|--------|
| Write Cayley table verification tests | 3h | ⬜ |
| Write inverse property tests (property-based) | 2h | ⬜ |
| Write rotor unitarity tests | 2h | ⬜ |
| Write grade consistency tests | 2h | ⬜ |
| Write dualization tests (`dualize² = -1`) | 1h | ⬜ |
| Run `cargo clippy` and fix warnings | 1h | ⬜ |

**Deliverable:** `tests/algebra_tests.rs` — comprehensive test suite

### Day 10: Serde & Benchmarks (June 14-15)

| Task | Est. | Status |
|------|------|--------|
| Implement `Serialize`/`Deserialize` for `Multivector` and `Blade` | 2h | ⬜ |
| Set up criterion benchmarks | 1h | ⬜ |
| Write `product_bench` (geometric product) | 2h | ⬜ |
| Write `rotor_bench` (rotor application) | 2h | ⬜ |
| Verify <50ns geometric product target | 1h | ⬜ |

**Deliverable:** Serde support + criterion benchmarks

---

## Verification Checklist

- [ ] `cargo test` passes all tests
- [ ] `cargo clippy --all-features` produces zero warnings
- [ ] `cargo bench` shows <50ns for geometric product
- [ ] Cayley table matches known Cl(3) multiplication table
- [ ] All property-based tests pass with ≥1000 random inputs
- [ ] `Multivector` implements `Clone`, `Copy`, `Debug`, `PartialEq`

---

## Technical Notes

### Cl(3) Cayley Table

The geometric product of basis blades follows:

```
e_i * e_i = 1       (for all i)
e_i * e_j = e_ij     (for i < j)
e_i * e_j = -e_ji    (for i > j)
```

### Coefficient Order

```
[0] = scalar     (1)
[1] = e1         (Thunder / Zhèn)
[2] = e2         (Water / Kǎn)
[3] = e3         (Mountain / Gèn)
[4] = e12        (Fire / Lí)
[5] = e23        (Wind / Xùn)
[6] = e31        (Lake / Duì)
[7] = e123       (Heaven / Qián)
```

### Inverse Formula

For a non-degenerate multivector `A`:
```
A⁻¹ = A̅ / (A * A̅)
```
where `A̅` is the reverse of `A`, and `A * A̅` is a scalar (the norm squared).
