# PLAN: Application Expansion — 11 COMBINE Ideas

**Status:** Draft  
**Date:** 2026-06-08  
**Context:** Brainstorm of 35 potential applications for GA-Bagua pairing. 11 ideas classified as COMBINE (build within this project), 2 as DETACH (standalone repos). This document covers the 11 COMBINE ideas only.

---

## Overview

The 11 COMBINE ideas cluster into **three cohesive subsystems** that share the same core pipeline — encode concepts via 8-coefficient multivectors, classify/compare via WuXing cycles and geometric product, store in ConceptStore — but each adds domain-specific logic on top.

```
                     ┌─────────────────────────────┐
                     │    ga-semantics-core         │
                     │  (minimal additions needed)  │
                     └──────────┬──────────────────┘
              ┌─────────────────┼──────────────────┐
              ▼                 ▼                  ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│   System A      │  │   System B      │  │   System C      │
│ Doc Intelligence│  │   Cognitive     │  │   Ideation      │
│ (6 ideas)       │  │   Systems       │  │   (1 idea)      │
│                 │  │   (4 ideas)     │  │                 │
│ ga-doc-intel    │  │ ga-cognitive    │  │ in-core         │
│ crate           │  │ crate           │  │ (bagua.rs +)    │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

---

### Interface Principle: MCP and CLI are One

Following the existing project pattern, **MCP tools and CLI subcommands expose the same set of operations**. They are not two separate APIs — they are two interfaces to the same library functions.

| Interface | Purpose | How |
|-----------|---------|-----|
| **MCP tools** (JSON-RPC) | LLM programmatic use | Each operation is a named tool with typed JSON arguments |
| **CLI subcommands** | Human interactive use | Operations grouped under nested subcommands with flags; `ga-semantics doc align pair` = same logic as `doc_align_pair` MCP tool |

Every "New code" section below lists the shared operations once. The implementation adds them to both `mcp/main.rs` and `cli/main.rs` in parallel — same function calls, different argument parsing.

---

## System A: Document Intelligence Framework

**Crate:** `ga-doc-intel` (new workspace member)  
**Ideas:** #1, #2, #4, #6, #8, #29

### Shared Architecture

```
ga-doc-intel/
  Cargo.toml
  src/
    lib.rs                 — Public API, prelude
    document.rs            — Document container (claims, metadata, language tag)
    alignment.rs           — Cross-document alignment scoring
    synthesis.rs           — WuXing cycle completeness for research synthesis
    coherence.rs           — Intra-document contradiction detection
    fallacy.rs             — Argument structure, premise-conclusion DAG, fallacy patterns
    contract.rs            — Smart contract intent vs bytecode semantic comparison
```

### Shared Infrastructure

| Component | What It Adds |
|-----------|-------------|
| `Document` struct | Groups concepts into a named document with metadata (title, source, language, timestamp) |
| `DocumentStore` | Like ConceptStore but tracks which concepts belong to which documents; cross-document link queries |
| `AlignmentReport` | Pairs of aligned/conflicting/supporting claims across documents, scored |
| `SynthesisScore` | Completeness of coverage across WuXing phases for a given topic |

### Core Changes Needed

| Module | Change | Reason |
|--------|--------|--------|
| `store.rs` | Add `document_id: Option<i64>` to `StoredConcept`; add document table to `GraphData` | Backward-compatible via `Option`; existing stores continue to work |
| `lib.rs` | No changes | New crate consumes core as-is |

---

### Idea #1: Argument Mapping & Fallacy Detection

**What it does:** Decompose an argument into premises and conclusions, encode each as a multivector, then classify the inferential relationships.

| Premise→Conclusion Relationship | WuXing Interpretation | RelationType |
|----------------------------------|-----------------------|--------------|
| Premise generates conclusion phase | Valid deductive support | generative |
| Premise controls conclusion phase | Rebuttal / undermining | constraining |
| Same phase, complementary trigrams | Consistent reinforcement | balancing |
| High bivector magnitude | Logical contradiction | (contradiction detected) |

**Fallacy patterns (WuXing-based heuristics):**

| Fallacy | Detection Rule |
|---------|---------------|
| Circular reasoning | premise.multivector == conclusion.multivector (similarity > 0.99) |
| False dichotomy | Only 2 identified premises for a conclusion with low bivector energy |
| Straw man | Dominant trigram of conclusion differs from dominant trigram of premises, but similarity to a different (unstated) encoding is high |
| Non sequitur | Premise phase and conclusion phase have no generating/controlling edge |

**New code:**
- `fallacy.rs` — `ArgumentGraph` (premise/conclusion nodes + edges), `FallacyDetector` trait, per-fallacy rule implementations
- MCP tools: `doc_add_argument`, `doc_analyze_argument`, `doc_fallacy_check`
- CLI: `ga-semantics doc arg add/analyze/check`

**Depends on:** Document container (#2 infra), existing `classify_relation`, `detect_contradiction`

---

### Idea #2: Multi-Document Semantic Alignment

**What it does:** Ingest multiple documents, encode each claim as a concept, then find semantically equivalent, supporting, or contradictory claims across documents.

**Alignment types:**

| Type | Detection |
|------|-----------|
| Identical claim | `semantic_similarity(a, b) > 0.95` |
| Supporting claim | `classify_relation(a, b) == generative` |
| Conflicting claim | `is_contradictory(a, b)` or `classify_relation(a, b) == constraining` |
| Complementary claim | `classify_relation(a, b) == balancing` |
| Unrelated | `semantic_similarity(a, b) < 0.3` AND no WuXing edge |

**New code:**
- `alignment.rs` — `align_documents(a, b) → Vec<ClaimAlignment>`, `AlignmentReport`
- MCP tools: `doc_align_pair`, `doc_align_multi`, `doc_conflict_matrix`
- CLI: `ga-semantics doc align pair/multi`

**Use cases:** Contract review, policy harmonization, regulatory compliance checking, legal discovery.

**Depends on:** Document container, `DocumentStore`

---

### Idea #4: Research Paper Synthesis

**What it does:** Given multiple research papers (encoded as documents), detect which findings complement each other (generating cycle), which contradict (controlling cycle), and which are orthogonal. Produce a synthesis score indicating how fully a research question is covered.

**WuXing Synthesis Model:**
- A complete research synthesis should cover all 5 WuXing phases:
  - **Wood** (initiation) — papers proposing new ideas/hypotheses
  - **Fire** (clarification) — papers providing evidence/data
  - **Earth** (grounding) — papers establishing context/baselines/limitations
  - **Metal** (synthesis) — papers reconciling/balancing other findings
  - **Water** (transmission) — papers connecting to related fields
- `synthesis_completeness()` scores 0.0—1.0 based on phase coverage
- Missing phases → identified as research gaps

**New code:**
- `synthesis.rs` — `SynthesisEngine`, `synthesis_completeness()`, `identify_gaps()`
- MCP tools: `doc_synthesize`, `doc_find_gaps`
- CLI: `ga-semantics doc synthesize`

**Depends on:** Document container, alignment, WuXing cycle data

---

### Idea #6: Policy Coherence Engine

**What it does:** Encode organizational policies as documents of claims. Detect internal contradictions within a policy, contradictions between policies, and redundant/overlapping policies.

**Coherence scoring:**

| Metric | Formula |
|--------|---------|
| Intra-policy coherence | `1.0 - mean(contradiction_magnitude across all claim pairs within policy)` |
| Inter-policy coherence | `1.0 - mean(contradiction_magnitude across claim pairs between policies A and B)` |
| Redundancy ratio | `count(sim > 0.9 cross-policy pairs) / total cross-policy pairs` |
| Coverage gap | `count(concepts_not_referenced_by_any_policy)` |

**New code:**
- `coherence.rs` — `PolicyCoherenceReport`, per-policy and cross-policy scorers
- MCP tools: `doc_coherence_intra`, `doc_coherence_inter`, `doc_redundancy_check`
- CLI: `ga-semantics doc coherence`

**Depends on:** Document container, alignment, contradiction detection

---

### Idea #8: Cross-Lingual Concept Alignment

**What it does:** Encode the same concept description in different languages. Measure how close the resulting multivectors are. If they converge, the encoding is language-invariant; divergence suggests translation distortion.

**Key insight:** The encoding questions (what does this concept DO?) are semantic-functional, not lexical. If the LLM understands both languages equally, the 8 coefficients should be similar regardless of input language.

| Scenario | Expected Result |
|----------|----------------|
| Direct translation | `semantic_similarity(concept_en, concept_fr) > 0.85` |
| Culturally divergent concept | `semantic_similarity(concept_en, concept_jp) in [0.5, 0.85]` |
| Untranslatable concept | `semantic_similarity < 0.5` + divergent dominant trigrams |

**New code:**
- Add `language: String` to `Document` struct
- `alignment.rs` — `cross_lingual_align(doc_a, doc_b) → CrossLingualReport` with per-concept alignment scores
- MCP tools: `doc_xlang_align`, `doc_xlang_gap`
- CLI: `ga-semantics doc xlang align`

**Depends on:** Document container (#2 infra), existing `llm_encode()` + `semantic_similarity()`

---

### Idea #29: Smart Contract Semantic Auditor

**What it does:** Given a smart contract, encode:
1. The natural-language intent (what the contract is supposed to do)
2. The actual bytecode logic (what the code actually does, described by an LLM reading the source)

Compare the two encodings. Geometric distance quantifies semantic drift between intent and implementation.

**Audit report:**

| Condition | Interpretation |
|-----------|---------------|
| `similarity > 0.9` | Implementation matches intent |
| `similarity in [0.6, 0.9]` | Minor semantic drift — review specific diverging roles |
| `similarity < 0.6` | Significant drift — possible vulnerability or misimplementation |
| `is_contradictory(intent, impl)` | Critical: implementation contradicts intent |

**New code:**
- `contract.rs` — `audit_contract(intent_encoding, implementation_encoding) → ContractAuditReport`
- MCP tools: `doc_contract_audit`, `doc_contract_compare`
- CLI: `ga-semantics doc contract audit`

**Depends on:** `llm_encode()`, `semantic_similarity()`, `is_contradictory()`, `multivector_to_roles()` (for explaining which roles diverged)

---

### System A: New Operations (surfaced identically on MCP + CLI)

Each operation below exists as both an MCP tool (JSON-RPC) and CLI subcommand (grouped under `ga-semantics doc`). Same function calls; only argument handling differs.

| # | Operation | MCP tool name | CLI invocation | Description |
|---|-----------|--------------|----------------|-------------|
| 1 | Add document | `doc_add_document` | `ga-semantics doc add` | Create a document container with name, source, language |
| 2 | Add claim | `doc_add_claim` | `ga-semantics doc add --claim` | Encode a claim and attach to a document |
| 3 | List claims | `doc_list_claims` | `ga-semantics doc list` | List all claims in a document with encoding summaries |
| 4 | Align pair | `doc_align_pair` | `ga-semantics doc align pair` | Align two documents — matched/conflicting claim pairs |
| 5 | Align multi | `doc_align_multi` | `ga-semantics doc align multi` | Align N documents pairwise, returning full matrix |
| 6 | Conflict matrix | `doc_conflict_matrix` | `ga-semantics doc align --conflicts` | NxN contradiction scores between all document pairs |
| 7 | Synthesize | `doc_synthesize` | `ga-semantics doc synthesize` | Synthesis report with gap analysis |
| 8 | Coherence report | `doc_coherence_report` | `ga-semantics doc coherence` | Intra- and inter-policy coherence scores |
| 9 | Cross-lingual align | `doc_xlang_align` | `ga-semantics doc xlang` | Cross-lingual concept alignment report |
| 10 | Contract audit | `doc_contract_audit` | `ga-semantics doc contract` | Compare intent vs implementation encodings |

---

## System B: Cognitive Systems Framework

**Crate:** `ga-cognitive` (new workspace member)  
**Ideas:** #3, #5, #7, #10

### Shared Architecture

```
ga-cognitive/
  Cargo.toml
  src/
    lib.rs                — Public API, prelude
    agent.rs              — AgentStore: multi-agent belief tracking
    belief.rs             — Belief revision via rotor, belief state timeline
    compatibility.rs      — Agent-agent compatibility, team composition
    learning.rs           — Learning path generation via WuXing cycles
    goal.rs               — Goal decomposition, hierarchy, coherence check
```

### Shared Infrastructure

| Component | What It Adds |
|-----------|-------------|
| `AgentStore` | Multiplexed ConceptStore: each concept belongs to an agent. Cross-agent queries. |
| `BeliefState` | A multivector + timestamp + revision count. Tracks belief evolution. |
| `CompatibilityMatrix` | NxN compatibility scores between all agents in a system. |
| `GoalTree` | DAG of goal → subgoals, with WuXing ordering. |

### Core Changes Needed

| Module | Change | Reason |
|--------|--------|--------|
| `store.rs` | Add `agent_id: Option<String>` to `StoredConcept`; add agent metadata table | Parallel to document_id; backward-compatible |
| `semantics.rs` | Add `belief_revise(old_mv, new_mv) → Rotor` | Computes the rotor that transforms old belief to new belief |
| `lib.rs` | No changes | |

---

### Idea #3: Agent Belief State Tracking

**What it does:** Each AI agent (or human, via proxy) maintains a belief store as a set of encoded multivectors. Beliefs can be added, revised, and compared across agents.

**Belief operations:**

| Operation | Implementation |
|-----------|---------------|
| Add belief | `llm_encode(belief_text)` → store under agent_id |
| Revise belief | Compute `Rotor` between old and new encoding; track in revision log |
| Detect dissonance | `is_contradictory()` across all belief pairs for one agent |
| Compare agents | `dominant_similarity()` matrix between agents A and B |
| Believe consensus | Majority-encode: find the multivector most similar to all others |

**New code:**
- `agent.rs` — `AgentStore` (wraps ConceptStore with agent multiplexing), `create_agent()`, `add_belief()`, `list_beliefs()`
- `belief.rs` — `BeliefTimeline`, `revise_belief()`, `dissonance_score()`
- MCP tools: `cog_create_agent`, `cog_add_belief`, `cog_revise_belief`, `cog_detect_dissonance`, `cog_compare_agents`, `cog_agent_consensus`
- CLI: `ga-semantics agent create/add/revise/dissonance/compare`

**Depends on:** AgentStore, `llm_encode()`, `is_contradictory()`, `dominant_similarity()`

---

### Idea #7: Personality / Team Compatibility

**What it does:** Encode personality profiles (MBTI, Big 5, Enneagram, or custom) as multivectors, then compute compatibility between individuals and form optimized teams using WuXing cycles.

**Personality encoding:** An LLM reads a personality profile description and answers the 8 diagnostic questions (same `SKILL.md` protocol, but applied to personality traits instead of concept definitions).

**Compatibility model:**

| Relationship | WuXing Edge | Interpretation |
|-------------|------------|----------------|
| A generates B | A.generation → B | A enables/motivates B; good leader-follower pair |
| B generates A | B.generation → A | B enables/motivates A; opposite hierarchy |
| A controls B | A.control → B | A grounds/constrains B; useful for check-and-balance |
| Same phase, complementary trigrams | balancing | Equal partnership, mutual reflection |
| Same trigram | receptive | Too similar — risk of redundancy/blandness |
| Contradictory (high bivector) | — | Fundamental incompatibility — avoid pairing |

**Team formation algorithm:**
1. Encode all candidates
2. For target team size N, find the set that maximizes WuXing phase diversity and rotor-based compatibility
3. Recommend team with highest `mean(dominant_similarity)` and lowest `max(contradiction_magnitude)`

**New code:**
- `compatibility.rs` — `personality_encode()`, `compatibility_score()`, `team_form()`, `team_score()`
- MCP tools: `cog_encode_personality`, `cog_person_compatibility`, `cog_team_form`, `cog_team_score`
- CLI: `ga-semantics agent personality/team`

**Depends on:** AgentStore (#3 infra), `llm_encode()`, `classify_relation()`, `is_contradictory()`

---

### Idea #5: Learning Path Generation

**What it does:** Given a set of knowledge concepts, generate an optimal learning sequence using WuXing cycle ordering.

**WuXing learning model:**
- **Wood → Fire**: Learn fundamentals (Wood) before applying them (Fire)
- **Fire → Earth**: Apply knowledge (Fire) then consolidate/ground (Earth)
- **Earth → Metal**: Consolidate (Earth) then synthesize across topics (Metal)
- **Metal → Water**: Synthesize (Metal) then connect to broader context (Water)
- **Water → Wood**: Understand context (Water) then tackle new fundamentals (Wood)

**Algorithm:**
1. Encode all knowledge concepts
2. Classify each into a WuXing phase via `dominant_role().wuxing_phase()`
3. Sort within each phase by difficulty (inverse of norm sharpness)
4. Interleave phases to follow generating cycle
5. Detect prerequisite violations: if A controls B, A must come before B

**New code:**
- `learning.rs` — `LearningPath`, `generate_path(concepts, target_phase)`, `rank_paths()`
- MCP tools: `cog_learning_path`, `cog_learning_prerequisites`
- CLI: `ga-semantics agent learn path/prereq`

**Depends on:** AgentStore (for storing knowledge concepts), WuXing cycle data, `classify_relation()`

---

### Idea #10: Goal Decomposition & Coherence

**What it does:** Decompose a high-level goal into subgoals, order them by WuXing cycle, and verify the decomposition is coherent (no contradictory subgoals).

**Goal decomposition algorithm:**
1. Encode the top-level goal
2. LLM proposes subgoals → encode each
3. Classify each subgoal's WuXing phase
4. Order subgoals to follow generating cycle
5. Check for contradictions between subgoals
6. Verify the rotor chain from first to last subgoal covers the full cycle

**Coherence check:**
- All 5 WuXing phases should be covered (or gaps are flagged)
- No contradictory subgoal pairs
- The geometric product of the full chain should have low bivector energy

**New code:**
- `goal.rs` — `Goal`, `GoalTree`, `decompose()`, `order_by_cycle()`, `coherence_score()`
- MCP tools: `cog_goal_decompose`, `cog_goal_order`, `cog_goal_coherence`, `cog_goal_gaps`
- CLI: `ga-semantics agent goal decompose/order/coherence`

**Depends on:** AgentStore, `classify_relation()`, `is_contradictory()`, WuXing cycle data

---

### System B: New Operations (surfaced identically on MCP + CLI)

Each operation below exists as both an MCP tool (JSON-RPC) and CLI subcommand (grouped under `ga-semantics agent`). Same function calls; only argument handling differs.

| # | Operation | MCP tool name | CLI invocation | Description |
|---|-----------|--------------|----------------|-------------|
| 1 | Create agent | `cog_create_agent` | `ga-semantics agent create` | Initialize belief/goal store for an agent |
| 2 | Add belief | `cog_add_belief` | `ga-semantics agent belief add` | Encode and store a belief |
| 3 | Revise belief | `cog_revise_belief` | `ga-semantics agent belief revise` | Revise a belief, tracking the rotor transform |
| 4 | List beliefs | `cog_list_beliefs` | `ga-semantics agent belief list` | List all beliefs for an agent |
| 5 | Belief timeline | `cog_belief_timeline` | `ga-semantics agent belief timeline` | Show belief revision history |
| 6 | Detect dissonance | `cog_detect_dissonance` | `ga-semantics agent dissonance` | Find internal contradictions in beliefs |
| 7 | Compare agents | `cog_compare_agents` | `ga-semantics agent compare` | Similarity/difference matrix between two agents |
| 8 | Agent consensus | `cog_agent_consensus` | `ga-semantics agent consensus` | Find consensus encoding across multiple agents |
| 9 | Encode personality | `cog_encode_personality` | `ga-semantics agent personality encode` | Encode a personality profile |
| 10 | Person compatibility | `cog_person_compatibility` | `ga-semantics agent personality cmp` | Compatibility score between two people |
| 11 | Team form | `cog_team_form` | `ga-semantics agent team form` | Form optimal team from candidate pool |
| 12 | Team score | `cog_team_score` | `ga-semantics agent team score` | Score an existing team |
| 13 | Learning path | `cog_learning_path` | `ga-semantics agent learn path` | Generate optimal learning sequence |
| 14 | Goal decompose | `cog_goal_decompose` | `ga-semantics agent goal decompose` | Decompose goal into ordered subgoals |
| 15 | Goal coherence | `cog_goal_coherence` | `ga-semantics agent goal coherence` | Check subgoal contradiction and cycle coverage |

---

## System C: Creative Ideation Engine

**Location:** In-core (no new crate). Extends `bagua.rs` + new MCP/CLI tools.  
**Ideas:** #9

### Idea #9: I Ching Creative Ideation Engine

**What it does:** Use the 64 hexagrams as structured creative prompts. Given a problem description, the LLM encodes it as a seed multivector. The engine then steps through hexagram space via rotor transformations, producing novel perspectives.

**Core algorithm:**
1. **Seed:** Encode the problem as a multivector via `llm_encode()`
2. **Step:** For a target hexagram H (upper trigram U, lower trigram L):
   - Compute the rotor from the seed's dominant trigram to H
   - Apply the rotor to the seed → produces a "shifted perspective" multivector
   - Decode the shifted multivector via `multivector_to_roles()` → human-readable perspective
3. **Explore:** Loop over all 64 hexagrams (or a filtered subset), ranking by how much the perspective shifts from the seed (controlled by rotor angle)
4. **Blend:** If the user provides a second concept, compute geometric product `A * B`, derive its hexagram, and interpret as the "emergent property" of combining A and B

**Hexagram-to-prompt mapping:** Each hexagram has a traditional interpretation already stored in `bagua.rs` (`HEXAGRAM_TABLE`). These serve as creative prompt templates. Example:
- Hexagram 1 (Qian/Qian, The Creative): "What would happen if you took complete ownership and initiated without waiting?"
- Hexagram 2 (Kun/Kun, The Receptive): "What if you instead yielded, listened, and let others lead?"
- Hexagram 11 (Kun/Qian, Peace): "How can the receptive ground the generative — bringing heaven down to earth?"

**New code:**
- `bagua.rs` — `Hexagram::interpretation()` (already exists via `HEXAGRAM_TABLE`), `hexagram_step(seed_mv, target_hexagram) → Multivector` (new), `hexagram_explore(seed_mv, top_n) → Vec<(Hexagram, Multivector, interpretation)>` (new)
- MCP tools: `ideate_seed`, `ideate_step`, `ideate_explore`, `ideate_blend`
- CLI: `ga-semantics ideate seed/step/explore/blend`

**Depends on:** `bagua.rs` (existing), `Rotor` (existing), `llm_encode()` (existing)

### System C: New Operations (surfaced identically on MCP + CLI)

| # | Operation | MCP tool name | CLI invocation | Description |
|---|-----------|--------------|----------------|-------------|
| 1 | Seed | `ideate_seed` | `ga-semantics ideate seed` | Encode a problem as a seed multivector |
| 2 | Step | `ideate_step` | `ga-semantics ideate step` | Step to a target hexagram, return shifted perspective |
| 3 | Explore | `ideate_explore` | `ga-semantics ideate explore` | Step through top-N hexagrams, ranked perspectives |
| 4 | Blend | `ideate_blend` | `ga-semantics ideate blend` | Blend two concepts via geometric product, interpret hexagram |

---

## Implementation Phases

### Phase 1: System C — Creative Ideation (Week 1)

**Rationale:** Quickest to implement. Only extends existing `bagua.rs` with 3 new functions. No new crate needed. All dependencies already exist. Good demo value.

**Tasks:**
- [ ] `bagua.rs`: Add `hexagram_step()`, `hexagram_explore()` (uses existing `Rotor`, `HEXAGRAM_TABLE`)
- [ ] `mcp/main.rs`: Add 4 tools (ideate_seed, step, explore, blend)
- [ ] `cli/main.rs`: Add `Ideate` subcommand with seed/step/explore/blend sub-subcommands
- [ ] Tests: verify rotor correctness, hexagram coverage
- [ ] Update `docs/product/functionalities.md`

**Estimated effort:** 2-3 days

### Phase 2: System A — Document Intelligence (Weeks 2-4)

**Rationale:** Highest-value subsystem. Document analysis is directly adjacent to the existing knowledge graph use case. Six ideas share the Document container abstraction.

**Tasks:**
- [ ] `ga-semantics-core/store.rs`: Add `document_id` to `StoredConcept`, add document table
- [ ] Create `ga-doc-intel` crate in workspace
- [ ] `document.rs`: Document, DocumentStore structs
- [ ] `alignment.rs`: Pair and multi-document alignment
- [ ] `coherence.rs`: Intra/inter-policy coherence
- [ ] `synthesis.rs`: Research synthesis engine
- [ ] `fallacy.rs`: Argument graph + fallacy detection rules
- [ ] `contract.rs`: Contract intent vs implementation comparison
- [ ] `mcp/main.rs`: Add 10 doc_* tools
- [ ] `cli/main.rs`: Add `Doc` subcommand
- [ ] Tests for each module

**Estimated effort:** 2-3 weeks

### Phase 3: System B — Cognitive Systems (Weeks 4-6)

**Rationale:** Most architecturally distinct. Introduces agent multiplexing, which requires careful store design. Builds on patterns established in System A.

**Tasks:**
- [ ] `ga-semantics-core/store.rs`: Add `agent_id` to `StoredConcept`
- [ ] `ga-semantics-core/semantics.rs`: Add `belief_revise()`
- [ ] Create `ga-cognitive` crate in workspace
- [ ] `agent.rs`: AgentStore (multi-agent concept storage)
- [ ] `belief.rs`: Belief timeline, revision tracking
- [ ] `compatibility.rs`: Personality encoding, team formation
- [ ] `learning.rs`: Learning path generation
- [ ] `goal.rs`: Goal decomposition, ordering, coherence
- [ ] `mcp/main.rs`: Add 15 cog_* tools
- [ ] `cli/main.rs`: Expand `Agent` subcommand group
- [ ] Tests for each module

**Estimated effort:** 2-3 weeks

---

## Dependency Graph

```
Phase 1 (Ideation: bagua.rs extensions)
  │
  ├─► Phase 2 (Doc Intel: depends on store.rs document_id addition)
  │       │
  │       └─► Phase 3 (Cognitive: depends on store.rs agent_id addition)
  │
  └─► (No dependency — Ideation is independent)
```

Within System A:
```
document.rs ──► alignment.rs ──► synthesis.rs
     │               │
     │               └──► coherence.rs
     │
     └──► fallacy.rs
     │
     └──► contract.rs
```

Within System B:
```
agent.rs ──► belief.rs
    │
    ├──► compatibility.rs
    ├──► learning.rs
    └──► goal.rs
```

---

## Workspace Structure After Expansion

```
ga-bagua-semantic-kg/
├── Cargo.toml                         # members: [core, cli, mcp, doc-intel, cognitive]
├── ga-semantics-core/                 # + agent_id to store, + belief_revise(), + hexagram_step/explore
├── ga-semantics-cli/                  # + 3 ideate subcommands, + 6 doc subcommands, + 11 agent subcommands
├── ga-semantics-mcp/                  # + 29 new tools across 3 tool groups
├── ga-doc-intel/                      # NEW: System A crate
│   ├── Cargo.toml                     # depends on ga-semantics-core
│   └── src/
│       ├── lib.rs
│       ├── document.rs
│       ├── alignment.rs
│       ├── synthesis.rs
│       ├── coherence.rs
│       ├── fallacy.rs
│       └── contract.rs
├── ga-cognitive/                      # NEW: System B crate
│   ├── Cargo.toml                     # depends on ga-semantics-core
│   └── src/
│       ├── lib.rs
│       ├── agent.rs
│       ├── belief.rs
│       ├── compatibility.rs
│       ├── learning.rs
│       └── goal.rs
├── data/
├── docs/
│   ├── product/
│   │   ├── epics.md                   # + Epics 5, 6, 7
│   │   └── functionalities.md         # + Doc Intel, Cognitive, Ideation sections
│   └── engineering/
│       └── development/
│           └── 2026-06-08-app-expansion/
│               └── PLAN.md            # This file
└── npm/
```

---

## Risk Assessment

| Risk | Mitigation |
|------|-----------|
| AgentStore and DocumentStore may diverge in API | Share a `TaggedConceptStore<T>` generic base; parameterize on tag type |
| llm_encode() quality varies across domain-specific encodings | Each subsystem provides its own `SKILL.md` encoding profile variant |
| MCP server grows too large (29 new tools) | Use feature flags: `doc-intel`, `cognitive`, `ideate` — each gate their tool group |
| ConceptStore breaking changes | `document_id` and `agent_id` are `Option<i64>` — backward-compatible; existing JSON stores load without issue |
| Hexagram exploration may produce bland results | Seed quality is LLM-dependent; provide encoding refinement feedback loop |

---

## Summary

| System | Ideas | Crate | Operations (MCP + CLI) | Core Changes |
|--------|-------|-------|------------------------|--------------|
| C — Ideation | #9 | in-core (bagua.rs) | 4 | `hexagram_step()`, `hexagram_explore()` |
| A — Doc Intel | #1,#2,#4,#6,#8,#29 | ga-doc-intel | 10 | `document_id` in store |
| B — Cognitive | #3,#5,#7,#10 | ga-cognitive | 15 | `agent_id` in store, `belief_revise()` |
| **Total** | **11** | 2 new crates | **29** | minimal |

**All 11 ideas reuse the same core pipeline:** `llm_encode()` → `Multivector` (64 bytes) → `RelationType::from_pair()` / `semantic_similarity()` / `is_contradictory()` / `analogy()`. No new math. No new GA operations. Just domain logic on top.

**MCP and CLI are the same operations, two interfaces.** Every operation is both an MCP JSON-RPC tool and a CLI subcommand. The CLI groups related operations under nested subcommands with flags (e.g., `ga-semantics doc add --claim` = `doc_add_claim`), following the existing project pattern.
