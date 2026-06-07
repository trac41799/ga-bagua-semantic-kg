# ga-semantics — Functionalities

**Organized by Capability Area**

---

## 1. Algebraic Core

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| Multivector construction | `[f64; 8]` or `Blade + coefficient` | `Multivector` | Create a multivector with given coefficients |
| Geometric product | `(Multivector, Multivector)` | `Multivector` | `ab = a·b + a∧b` — full relational signature |
| Inner product | `(Multivector, Multivector)` | `f64` | Grade-0 component — degree of alignment |
| Wedge product | `(Multivector, Multivector)` | `Multivector` | Grade-2+ components — degree of difference |
| Grade projection | `(Multivector, k: usize)` | `Multivector` | Extract grade-k component only |
| Reverse | `Multivector` | `Multivector` | Reverse blade order |
| Norm | `Multivector` | `f64` | Magnitude of multivector |
| Inverse | `Multivector` | `Option<Multivector>` | Multiplicative inverse (fails for zero-norm) |
| Dualize | `Multivector` | `Multivector` | Multiply by pseudoscalar `e123` |
| Rotor construction | `theta, bivector plane` | `Rotor` | Create rotation operator |
| Rotor application | `(Rotor, Multivector)` | `Multivector` | Sandwich product `R A R̃` |
| Rotor composition | `(Rotor, Rotor)` | `Rotor` | Multiply two rotors |

---

## 2. RelationType & Role Taxonomy

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `RelationType` enum | — | 8 variants | generative, receptive, causal, transmissive, constraining, influential, clarifying, balancing |
| `RelationType::bagua()` | `RelationType` | `Trigram` | Map role to canonical Bagua trigram (advanced) |
| `Trigrams` enum (internal) | — | 8 variants | Kun, Gen, Kan, Xun, Zhen, Li, Dui, Qian |
| Trigram ↔ Blade mapping | `Trigram` | `Blade` (and reverse) | Bidirectional mapping (advanced) |
| Trigram metadata (advanced) | `Trigram` | name, translation, binary, description | Chinese, English, line encoding |
| Hexagram construction (advanced) | `(Trigram, Trigram)` | `Hexagram` | Upper + lower trigram |
| Hexagram numbering (advanced) | `Hexagram` | `u8` (1–64) | Traditional I-Ching hexagram number |
| Hexagram name (advanced) | `Hexagram` | `&str` | Traditional hexagram name |
| Dominant role | `Multivector` | `RelationType` | Find role with max coefficient |
| Relationship classification | `(Multivector, Multivector)` | `RelationType` | Classify relationship between two concepts |
| Role weights | `Multivector` | `[f64; 8]` | Coefficients per role (in blade order) |

---

## 3. WuXing (Five Elements)

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `WuXing` enum | — | 5 variants | Wood, Fire, Earth, Metal, Water |
| Phase → Trigram mapping | `WuXing` | `&[Trigram]` | Which trigrams belong to each phase |
| Trigram → Phase mapping | `Trigram` | `WuXing` | Which phase a trigram belongs to |
| Generating cycle | `WuXing` | `WuXing` | Wood→Fire→Earth→Metal→Water→Wood |
| Controlling cycle | `WuXing` | `WuXing` | Wood→Earth→Water→Fire→Metal→Wood |

---

## 4. Semantic Operations

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| Semantic similarity | `(Multivector, Multivector)` | `f64` | Scalar part of geometric product, normalized to [-1, 1] |
| Semantic difference | `(Multivector, Multivector)` | `f64` | Bivector magnitude, normalized to [0, 1] |
| Relation strength | `(Multivector, Multivector)` | `f64` | Magnitude of full geometric product |
| Contradiction detection | `(Multivector, Multivector, threshold)` | `bool` | True if bivector magnitude exceeds threshold |
| Analogy | `(a, b, c)` → Multivector | `Multivector` | `(a⁻¹b) * c` — rotor from A→B applied to C |
| Analogy confidence | `(a, b, c, expected)` | `f64` | Similarity between result and expected |
| Relation composition | `(Rotor, Rotor)` | `Rotor` | Apply r1 then r2 = r2 * r1 |
| Relation chain | `&[Rotor]` | `Rotor` | Fold composition across a chain |
| Relation inverse | `Rotor` | `Rotor` | Reverse the relationship |
| Relation type | `(Multivector, Multivector)` | `RelationType` | Classify as generative, causal, constraining, etc. |
| Context apply | `(Context, Multivector)` | `Multivector` | Transform entity to new context |
| Context compose | `(Context, Context)` | `Context` | Chain context transformations |
| Context identity | — | `Context` | No-op context transformation |

---

## 5. Serialization

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| Serialize (JSON) | `Multivector` | `String` | JSON representation (serde feature) |
| Deserialize (JSON) | `String` | `Multivector` | Construct from JSON (serde feature) |
| Serialize (Bincode) | `Multivector` | `Vec<u8>` | Compact binary representation (serde feature) |
| Serialize Blade | `Blade` | `String`/`Vec<u8>` | Same for Blade enum (serde feature) |

---

## 6. Python Bindings

| Function | Python API | Description |
|----------|-----------|-------------|
| Constructor | `Multivector([f64; 8])` | Create from list of 8 floats |
| NumPy interop | `Multivector.from_numpy(arr)`, `.to_numpy()` | Convert to/from numpy arrays |
| All operations | `a.semantic_similarity(b)` etc. | Same API as Rust |

---

## 7. Benchmarking

| Benchmark | What It Measures | Metric |
|-----------|-----------------|--------|
| Geometric product latency | Per-operation cost | ns/op |
| Rotor application latency | Sandwich product cost | ns/op |
| Batch throughput | Operations per second with N multivectors | ops/sec |
| Relation classification | Trigram classification accuracy | Accuracy % |
| Analogical reasoning | Top-1 and top-k analogy accuracy | Accuracy % |
| KG link prediction | MRR, Hits@1/3/10 | MRR, % |

---

## 8. MCP Server Tools

| Tool | Input | Output | Transport |
|------|-------|--------|-----------|
| `create_multivector` | `{coefficients: [f64; 8]}` or `{pairs: [{blade, value}]}` | `Multivector` | stdio, SSE |
| `semantic_similarity` | `{a: Multivector, b: Multivector}` | `{score: f64}` | stdio, SSE |
| `semantic_difference` | `{a: Multivector, b: Multivector}` | `{score: f64}` | stdio, SSE |
| `analogy` | `{a: Multivector, b: Multivector, c: Multivector}` | `{result: Multivector, confidence: f64}` | stdio, SSE |
| `classify_relation` | `{a: Multivector, b: Multivector}` | `{relation_type: "causal", confidence: f64}` | stdio, SSE |
| `compose_relations` | `{r1: Rotor, r2: Rotor}` | `{result: Rotor}` | stdio, SSE |
| `detect_contradiction` | `{a, b, threshold?: f64}` | `{is_contradiction: bool, magnitude: f64}` | stdio, SSE |
| `relation_type_info` | `{role: "causal" \| "generative" \| ...}` | `{definition, bagua_trigram, wuxing, example}` | stdio, SSE |
| `wuxing_query` | `{phase, cycle?: "generating"\|"controlling"}` | `{phase, next, prev, roles}` | stdio, SSE |
| `context_apply` | `{context: Rotor, entity: Multivector}` | `{result: Multivector}` | stdio, SSE |
| `batch_process` | `{operations: BatchOp[]}` | `{results: any[]}` | stdio, SSE |

### Transport Options

| Transport | Flag | Use Case |
|-----------|------|----------|
| stdio | `ga-semantics-mcp` (default) | Claude Desktop, local agents |
| SSE | `ga-semantics-mcp --transport sse --port 3100` | Remote/containerized agents |

---

## 9. CLI Commands

### 9.1 Command Reference

| Command | Alias | Description |
|---------|-------|-------------|
| `ga-semantics mv` | — | Create, inspect, manipulate multivectors |
| `ga-semantics sim` | `similarity` | Semantic similarity between two concepts |
| `ga-semantics diff` | `difference` | Semantic difference |
| `ga-semantics analogy` | `ana` | Solve `a : b :: c : ?` |
| `ga-semantics classify` | `class` | Classify relation as role label (causal, generative, etc.) |
| `ga-semantics compose` | `comp` | Chain multiple rotors |
| `ga-semantics contradict` | `conflict` | Check contradiction |
| `ga-semantics relation-type` | `rt` | Look up relation type metadata (role → Bagua, WuXing) |
| `ga-semantics wuxing` | `wu` | Query five-phase cycles |
| `ga-semantics context` | `ctx` | Apply context rotors |
| `ga-semantics batch` | — | Execute batch operations from file |
| `ga-semantics eval` | — | Run benchmarks/evaluations |

### 9.2 Shared Flags

| Flag | Description | Applies To |
|------|-------------|------------|
| `--json` | Input as JSON string, output as JSON | All data commands |
| `--file <path>` | Read input from JSON file | All data commands |
| `--stdin` | Read input from stdin | All data commands |
| `--csv` | Output as CSV | sim, diff, eval |
| `--pretty` | Force human-readable output (default) | All commands |
| `--color <auto\|always\|never>` | ANSI color control | All commands |

### 9.3 Examples

```bash
# Human-readable similarity
ga-semantics sim 1 0 0 0 0 0 0 0  0 1 0 0 0 0 0 0

# JSON pipe to jq
ga-semantics sim --json '[1,0,0,0,0,0,0,0]' '[0,1,0,0,0,0,0,0]' | jq .score

# Stdin pipe
echo '{"a":[1,0,0,0,0,0,0,0],"b":[0,1,0,0,0,0,0,0]}' | ga-semantics sim --stdin

# Batch evaluation
ga-semantics eval analogy_benchmark.json --csv > results.csv

# Relation type lookup (returns role definition + optional bagua/wuxing metadata)
ga-semantics relation-type causal

# File-based analogy
ga-semantics analogy --file king.json queen.json man.json

# Classify relation between two concepts
ga-semantics classify --json '[1,0,0,0,0,0,0,0]' '[0,1,0,0,0,0,0,0]'
