# GA-Bagua LLM Pipeline Pattern

**How GA-Bagua fits into an LLM agent's workflow — the correct usage pattern.**

---

## The Pattern

```
┌─────────────────────────────────────────────────────────────┐
│                     LLM Agent (Reasoning)                    │
│                                                             │
│  "Which concepts constrain throughput in this codebase?"    │
│                                                             │
│  1. GA-Bagua retrieves top-10 candidates (algebra, 500ns)   │
│  2. LLM reads ONLY those 10 descriptions (~200 tok)         │
│  3. LLM verifies, filters, reasons                          │
│  4. LLM presents answer                                     │
│                                                             │
│  WITHOUT GA-Bagua: LLM reads ALL 500 descriptions (~20K tok)│
│  WITH GA-Bagua:    LLM reads 10 descriptions (~200 tok)     │
│  TOKEN SAVINGS:    100x per query                           │
└─────────────────────────────────────────────────────────────┘
```

## Token Economics

| Phase | Without GA-Bagua | With GA-Bagua |
|-------|:----------------:|:-------------:|
| One-time encoding | 0 | N concepts × 200 tok |
| Per query (simple) | All N × 40 tok | Top-10 × 20 tok |
| Per query (complex) | All N × 80 tok | Top-10 × 50 tok |

**Break-even:** After ~N/5 queries (encoding cost amortized). At 500 concepts: ~100 queries.

**Savings curve:**
```
Queries  │ Without │ With   │ Savings
─────────┼─────────┼────────┼────────
1        │ 20K     │ 120K   │ -100K (below break-even)
10       │ 200K    │ 102K   │ 98K
50       │ 1,000K  │ 110K   │ 890K (9x)
200      │ 4,000K  │ 140K   │ 3,860K (29x)
```

## Use Case: Codebase Exploration

```
Agent task: "Analyze how modules in this codebase relate to each other."

Step 1: Agent scans codebase, identifies 200 key modules/symbols
Step 2: GA-Bagua encodes each → 64 bytes × 200 = 12.8 KB
Step 3: Agent asks 200 exploration queries:
  - "Find all constraining modules"
  - "What generates the most dependencies?"
  - "Which modules balance each other?"
  - "Show me the control chain from Auth to Database"

Step 4: Agent presents findings

Without GA-Bagua: 200 × 500 tok = 100K tokens, $1.00, 600s
With GA-Bagua:    200 × 200 (encode) + 200 × 50 (verify) = 50K tokens, $0.50, 100s
Savings: 2x tokens, 6x latency
```

## Use Case: Document Analysis

```
Agent task: "Analyze the key concepts in this 50-page legal contract."

Step 1: LLM reads contract, identifies 80 key concepts
Step 2: GA-Bagua encodes each → 64 bytes × 80 = 5 KB
Step 3: Agent queries relationships:
  - "Which clauses constrain the other party?"
  - "What obligations does Section 3 generate?"
  - "Show me balancing provisions"

Without GA-Bagua: Re-read 50 pages for each query → 25K tok × N queries
With GA-Bagua:    Encode once (80 × 200 = 16K tok), query algebraically (0 tok)
Savings: After 1 query, GA-Bagua is already cheaper (16K vs 25K)
```

## What GA-Bagua Does Well In This Pattern

| Task | Quality | Cost |
|------|:-------:|:----:|
| Encode concept → 64 bytes | Reliable (99.8% stable) | 200 tok one-time |
| Find same-role concepts | 54% P@1 (same domain) | 0 tok, 500ns |
| Find related concepts | ~7% P@5 | 0 tok, 500ns |
| Classify pair relation | 24% test accuracy | 0 tok, 500ns |
| Multi-hop rotor chain | 100-hop, zero drift | 0 tok, ~200us |
| Store graph | 64 bytes/concept | JSON file, <1ms |

## What GA-Bagua Does NOT Do In This Pattern

- **Does not replace LLM reasoning.** The LLM always does final verification and explanation.
- **Does not classify relations accurately.** At 24% test accuracy, relation labels are directional hints, not answers.
- **Does not retrieve specific related concepts.** At 7% P@5, the specific related concept is rarely found. Use same-role retrieval instead.
- **Does not handle ambiguous concepts.** If a concept could reasonably be multiple roles, the encoding may be wrong.

## Integration Architecture

```
┌──────────────────────────────────────────────────────┐
│  LLM Agent (Claude, GPT-4, etc.)                     │
│  ┌────────────────────────────────────────────┐     │
│  │  Bagua Encoder Skill (SKILL.md)             │     │
│  │  text → 8 coefficients                     │     │
│  └──────────────┬─────────────────────────────┘     │
└─────────────────┼───────────────────────────────────┘
                  │ MCP protocol
┌─────────────────▼───────────────────────────────────┐
│  GA-Bagua MCP Server                                 │
│                                                      │
│  Tools:                                              │
│    llm_encode → store_llm_concept                    │
│    store_query_similar  (same-role retrieval)        │
│    classify_relation    (pair classification)        │
│    semantic_similarity  (pair similarity)             │
│    analogy              (A:B::C:?)                   │
│    compose_relations    (multi-hop rotor chain)      │
│    detect_contradiction (is A contradictory to B?)   │
│    store_export         (dump concept graph)         │
│                                                      │
│  ┌────────────────────────────────────────────┐     │
│  │  ga-semantics-core                          │     │
│  │  Cl(3) GA | Bagua | WuXing | Rotors        │     │
│  └────────────────────────────────────────────┘     │
└──────────────────────────────────────────────────────┘
```

## Recommendations

1. **Use GA-Bagua for same-role discovery.** "Find all constraining concepts" is the tool's best-supported query. 54% P@1 in multi-domain settings, higher within a single domain.

2. **Use GA-Bagua for ranked candidate generation.** Even at 54% P@1, the top-10 candidates from a same-role search contain ~5-6 relevant hits. The LLM verifies which ones are truly relevant.

3. **Do NOT use GA-Bagua for relation classification as a final answer.** At 24% test accuracy, the relation labels are directional hints. Use them as conversation starters, not conclusions.

4. **Do NOT use GA-Bagua for specific concept retrieval.** "Find the concept that relates to X" gives 7% P@5. Use keyword search or embedding similarity for this.

5. **Use GA-Bagua for multi-hop composition.** 100-hop rotor chains with zero drift is a unique capability. No LLM or vector DB can do this.

6. **Prefer domain-filtered queries.** WuXingIndex with domain tags improves P@1 by excluding cross-domain false positives.
