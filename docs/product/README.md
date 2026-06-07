# ga-semantics — Product Documentation

**Geometric Algebra Semantic Layer for Knowledge Graphs**

---

## Quick Start

```bash
# Install via npm (recommended)
npm install -g ga-semantics-mcp

# Or via Cargo
cargo install ga-semantics-cli

# Encode a concept
ga-semantics-cli encode 0.04 -0.09 -0.51 0.68 0.21 -0.26 0.17 -0.34

# See full delivery guide
open docs/DELIVERY.md
```

---

## 1. Executive Summary

**ga-semantics** is a lightweight Rust library that represents semantic relationships as geometric algebra operations, with 8 intuitive role labels — generative, receptive, causal, transmissive, constraining, influential, clarifying, balancing — providing an interpretable relationship taxonomy.

The library maps the 8 basis blades of Cl(3) (3D Clifford algebra) to the 8 trigrams of the I-Ching Bagua system as its canonical naming convention, creating a mathematically rigorous framework for knowledge graph relationship modeling. The Bagua mapping is surfaced in advanced documentation; the primary interface uses semantic role labels that require no decoding.

### Key Value Propositions

| Capability | Description |
|-----------|-------------|
| **Interpretable Relationships** | 8 semantic role labels (generative, causal, constraining, etc.) replace opaque relation labels |
| **Algebraic Composition** | Geometric product enables reversible, composable relationship operations |
| **Asymmetric & Cyclical** | Non-commutative algebra naturally models directed and cyclical relationships |
| **Zero Dependencies** | Core algebra has no external dependencies; pure Rust |
| **Dual Interface** | Rust crate + Python bindings via PyO3 |

---

## 2. Problem Statement

### Current Limitations in Knowledge Graph Embeddings

| Problem | Impact |
|---------|--------|
| **Low interpretability** | Vector embeddings are opaque; "r_42" conveys no meaning |
| **Symmetric-only similarity** | Dot products cannot model asymmetric relationships |
| **No cyclical support** | Vector spaces cannot represent A > B > C > A patterns |
| **No algebraic composition** | Cannot compose relationships algebraically |
| **Disconnected from semantics** | Mathematical representations lack semantic grounding |

### Target Users

| User | Need |
|------|------|
| **AI Agent Developers** | Interpretable relationship reasoning in agent memory systems |
| **Knowledge Graph Engineers** | Richer edge representations with algebraic properties |
| **NLP Researchers** | Novel semantic representation based on geometric algebra |
| **Chinese Philosophy Researchers** | Formal mathematical model of Bagua/Wuxing systems |

---

## 3. Product Overview

### 3.1 Core Concept

The system operates on three levels:

```
┌─────────────────────────────────────────────────────────┐
│  Semantic Layer (Bagua Taxonomy)                        │
│  8 interpretable relationship categories                │
├─────────────────────────────────────────────────────────┤
│  Algebraic Layer (Geometric Algebra)                    │
│  Cl(3) multivectors, rotors, geometric product          │
├─────────────────────────────────────────────────────────┤
│  Storage Layer (8 × f64 per multivector)                │
│  Same cost as 8-dim vector embeddings                   │
└─────────────────────────────────────────────────────────┘
```

### 3.2 The Cl(3) ↔ Bagua Isomorphism

| Grade | Basis Blades | Count | Bagua Trigram | Semantic Meaning |
|-------|-------------|:-----:|---------------|------------------|
| 0 | `1` (scalar) | 1 | ☷ Kūn (Earth) | Ground state, receptive, undifferentiated |
| 1 | `e₁, e₂, e₃` (vectors) | 3 | ☳ Zhèn, ☵ Kǎn, ☶ Gèn | Directed action, flow, boundedness |
| 2 | `e₁₂, e₂₃, e₃₁` (bivectors) | 3 | ☲ Lí, ☴ Xùn, ☱ Duì | Transformation planes, rotation, reflection |
| 3 | `e₁₂₃` (trivector) | 1 | ☰ Qián (Heaven) | All-encompassing, creative principle |

**Total: 8 basis blades ⟷ 8 trigrams**

### 3.3 Relationship Role Taxonomy

| Role | Bagua (Internal) | When to Use |
|------|------------------|-------------|
| `generative` | ☰ Qián — creative | New pattern introduced; innovation |
| `receptive` | ☷ Kūn — receptive | Convention adopted; dependency accepted |
| `causal` | ☳ Zhèn — initiating | Causal relationship: A triggers B |
| `transmissive` | ☵ Kǎn — flowing | Data flow; pipe/stream patterns |
| `constraining` | ☶ Gèn — bounding | Constraint; boundary; "don't cross this" |
| `influential` | ☴ Xùn — penetrating | Gradual influence; convention spreading |
| `clarifying` | ☲ Lí — illuminating | Clarification; dependency revelation |
| `balancing` | ☱ Duì — reflecting | Symmetric relationship; mutual dependency |

---

## 4. Product Capabilities

### 4.1 Core Algebra Operations

| Operation | Description | Semantic Use |
|-----------|-------------|--------------|
| Geometric Product | `ab = a·b + a∧b` | Complete relational signature |
| Inner Product | `⟨a, b⟩` | Degree of alignment/similarity |
| Wedge Product | `a∧b` | Degree of orthogonality/difference |
| Rotor Application | `a' = R a R̃` | Semantic transformation (analogy) |
| Grade Projection | `⟨A⟩ₖ` | Extract specific relationship dimension |
| Inverse | `A⁻¹` | Reverse relationship direction |
| Dualization | `A * e₁₂₃` | Complement/flip (yin⟷yang) |

### 4.2 Semantic Operations

| Operation | Description |
|-----------|-------------|
| `semantic_similarity()` | Scalar part of geometric product → [-1, 1] |
| `semantic_difference()` | Bivector magnitude → [0, 1] |
| `semantic_relation()` | Returns a `RelationType` label (generative, causal, constraining, etc.) |
| `analogy(a, b, c)` | Compute `(a⁻¹b) * c` — analogical reasoning |
| `compose(r1, r2)` | Rotor multiplication — compose relationships |

### 4.3 Role & Taxonomy Integration

| Feature | Description |
|---------|-------------|
| `dominant_role()` | Find which role best describes a multivector |
| `relationship_to(other)` | Classify relationship between two concepts as a `RelationType` |
| `wuxing_phase()` | Map underlying trigram to Five Elements phase (advanced) |
| Bagua hexagram construction | Stack two trigrams → 64 compound states (advanced) |

---

## 5. Competitive Position

| System | Approach | Interpretability | Asymmetric | Cyclical |
|--------|----------|:---:|:---:|:---:|
| Vector embeddings (word2vec, BERT) | Dot product similarity | Low | No | No |
| Complex embeddings (ComplEx, RotatE) | Complex-valued vectors | Low | Partial | Partial |
| Geometric Algebra (GeomE, 2020) | Multivector + geometric product | Low | Yes | Yes |
| **ga-semantics** | **Multivector + 8 semantic role labels** | **High** | **Yes** | **Yes** |

### Unique Contribution

GeomE proved GA works for knowledge graphs but provides no interpretability. **ga-semantics** adds 8 intuitive semantic role labels (generative, causal, influential, etc.) on top of the algebraic structure, making the system usable not just as a black-box embedder but as a tool for understanding *what kind* of relationship exists between concepts. The role labels are grounded in the I-Ching Bagua system as a canonical mathematical naming convention (see advanced docs), but the public interface requires no knowledge of Bagua to use.

---

## 6. Integration Path

### Phase 1: Standalone Library

- Published on crates.io as `ga-semantics`
- Python bindings via PyO3 on PyPI
- Independent versioning and releases

### Phase 2: ACC Integration

```sql
-- Add relation type label to knowledge relations
ALTER TABLE knowledge_relations ADD COLUMN relation_type TEXT;

-- Store multivector representation
ALTER TABLE knowledge_relations ADD COLUMN relation_multivector BLOB;
```

| ACC Component | Integration |
|--------------|-------------|
| Knowledge Compounder | Classify relations into semantic role categories |
| Knowledge Relations | Store `relation_type` + `relation_multivector` |
| Contradiction Detection | Use bivector magnitude as contradiction signal |
| KG Queries | Compose multi-hop relations via rotor multiplication |
| KG Visualization | Color-code edges by role category |

---

## 7. Success Criteria

| Criterion | Metric |
|-----------|--------|
| Mathematical correctness | All algebraic operations pass property-based tests |
| Benchmarks published | Head-to-head comparison with GeomE and standard embeddings |
| ACC integration demo | knowledge_relations edges tagged with relation roles |
| Crate published | On crates.io with ≥80% documentation coverage |
| Community communication | Blog post or preprint explaining the algebra and its role taxonomy |

---

## 8. Open Questions

| Question | Context |
|----------|---------|
| Can the 8 role labels be **learned automatically**? | Instead of the fixed mapping, could we learn which blade maps to which role from data? |
| Does **higher-dimensional Cl(n)** provide more expressive power? | Cl(4) has 16 basis blades but loses the elegant 8-fold mapping. Worth exploring if 8 roles prove insufficient. |
| Can the **Wuxing generating/controlling cycles be validated empirically**? | Do KG relationships in real datasets follow the five-phase cycle patterns? |
| How does this compare to **hyperbolic embeddings** for hierarchy? | Hyperbolic space captures tree-like structures well; GA captures transformational relationships. Complementary or competitive? |

---

## 9. Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Benchmarks show no improvement over GeomE | High | High | Semantic role labels alone add interpretability — still valuable |
| Users ignore because labels seem arbitrary | Medium | Medium | Labels are grounded in a principled algebra, not ad-hoc; publish the mathematical justification |
| Academic community dismisses novelty | Medium | Low | Lead with Cl(3) math; the role taxonomy is a naming layer, not the core contribution |
| Practical value for agent reasoning unproven | High | Medium | Low cost of trying (~5 weeks); no ACC dependency |

---

## 10. References

- Xu et al. (2020). "Knowledge Graph Embeddings in Geometric Algebras." COLING 2020.
- Pustejovsky, J. (2026). "Toward a Functional Geometric Algebra for Natural Language Semantics." arXiv:2604.25902.
- Coecke, Sadrzadeh, Clark (2010). "Mathematical Foundations for a Compositional Distributional Model of Meaning."
- Hestenes, D. (1966-2003). Foundational works on geometric algebra.
- Wilhelm, R. & Baynes, C.F. (1967). "The I Ching or Book of Changes." — canonical Bagua taxonomy used as internal naming convention
