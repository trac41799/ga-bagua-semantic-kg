# ga-semantics — Epics & User Stories

---

## Epic 1: Core Algebra Engine

**Goal:** Implement a mathematically correct Cl(3) Clifford algebra engine in Rust.

### User Stories

| ID | Story | Priority | Acceptance Criteria |
|----|-------|----------|---------------------|
| US-1.1 | As a developer, I want to construct multivectors from 8 f64 coefficients so that I can represent semantic concepts | P0 | `Multivector::new([f64; 8])` compiles and stores 8 coefficients |
| US-1.2 | As a developer, I want to compute the geometric product of two multivectors so that I can compute complete relational signatures | P0 | `a.geo_product(&b)` returns correct result matching Cl(3) Cayley table |
| US-1.3 | As a developer, I want to project a multivector to a specific grade so that I can extract relationship dimensions | P0 | `a.grade(k)` returns the grade-k component |
| US-1.4 | As a developer, I want to compute inner and wedge products so that I can measure alignment and orthogonality | P0 | Inner product returns scalar; wedge returns bivector |
| US-1.5 | As a developer, I want to compute the inverse of a multivector so that I can reverse relationships | P0 | `a * a.inverse() ≈ 1` for non-degenerate multivectors |
| US-1.6 | As a developer, I want to construct and apply rotors so that I can perform semantic transformations | P0 | Rotor sandwich product `R * a * R̃` produces correct rotation |
| US-1.7 | As a developer, I want to dualize multivectors via pseudoscalar multiplication so that I can flip yin↔yang | P1 | `a.dualize() * a.dualize() ≈ -a` (Cl(3) pseudoscalar squares to -1) |
| US-1.8 | As a developer, I want to serialize/deserialize multivectors via serde so that I can persist them | P1 | JSON and Bincode roundtrip works with `serde` feature enabled |

### Definition of Done

- [ ] All Cl(3) basis blade multiplication table entries verified
- [ ] Inverse property holds for all non-degenerate multivectors
- [ ] Rotor property `R * R̃ = 1` verified
- [ ] Grade consistency verified across all grade combinations
- [ ] Property-based tests pass with ≥1000 random inputs

---

## Epic 2: RelationType & Role Taxonomy

**Goal:** Implement the 8 semantic role labels as the public interface, with Bagua as the canonical internal naming convention for basis blades.

### User Stories

| ID | Story | Priority | Acceptance Criteria |
|----|-------|----------|---------------------|
| US-2.1 | As a developer, I want a `RelationType` enum with 8 semantic role variants so that I can label KG edges immediately without decoding | P0 | Enum compiles with generative, receptive, causal, transmissive, constraining, influential, clarifying, balancing variants |
| US-2.2 | As a developer, I want each `RelationType` mapped to its canonical Bagua trigram so that I can access the philosophical grounding when needed | P0 | `RelationType::Causal.bagua() == Trigram::Zhen` |
| US-2.3 | As a developer, I want to query which role a multivector most aligns with so that I can classify concepts | P0 | `multivector.dominant_role()` returns `RelationType` |
| US-2.4 | As a developer, I want to classify the relationship between two multivectors as a `RelationType` so that I can label KG edges | P0 | `a.relationship_to(&b)` returns `RelationType` |
| US-2.5 | As an advanced user, I want access to the underlying `Trigram` enum and Bagua metadata so that I can explore the mathematical isomorphism | P1 | `Trigram` enum with blade mapping, Chinese names, hexagram construction all available behind an `advanced` module |
| US-2.6 | As an advanced user, I want to construct hexagrams from two trigrams so that I can model compound relationships | P1 | `Hexagram::new(Trigram::Qian, Trigram::Kun)` creates valid hexagram |
| US-2.7 | As a developer, I want to map trigrams to WuXing phases so that I can model transformation cycles | P1 | `Trigram::Zhen.wuxing_phase() == WuXing::Wood` |
| US-2.8 | As a developer, I want to query generating and controlling cycles so that I can model sequential transformations | P1 | `WuXing::Wood.generate() == WuXing::Fire` |

### Definition of Done

- [ ] `RelationType` enum with 8 role variants, each with `.bagua()` conversion method
- [ ] `dominant_role()` returns `RelationType` (not `Trigram`)
- [ ] `relationship_to()` returns `RelationType` (not `Hexagram`)
- [ ] `Trigram` and `Hexagram` accessible via `ga_semantics::advanced` module
- [ ] WuXing generating and controlling cycles verified
- [ ] All Bagua metadata (Chinese names, translations, binary encoding) accessible via advanced module

---

## Epic 3: Semantic Operations API

**Goal:** Provide high-level semantic operations built on the algebraic foundation.

### User Stories

| ID | Story | Priority | Acceptance Criteria |
|----|-------|----------|---------------------|
| US-3.1 | As a developer, I want to compute semantic similarity between concepts so that I can measure alignment | P0 | `a.semantic_similarity(&b)` returns value in [-1, 1]; `sim(a,a) ≈ 1.0` and `sim(a,b) == sim(b,a)` |
| US-3.2 | As a developer, I want to compute semantic difference so that I can measure orthogonality | P0 | `a.semantic_difference(&b)` returns value in [0, 1] |
| US-3.3 | As a developer, I want to classify the semantic relation type between concepts so that I can label edges | P0 | `a.semantic_relation(&b)` returns `RelationType` (e.g., `RelationType::Causal`) |
| US-3.4 | As a developer, I want to compute analogies ("A is to B as C is to ?") so that I can perform analogical reasoning | P0 | `analogy(&a, &b, &c)` returns `(a⁻¹b) * c`; correctness verified on ≥10 curated test cases |
| US-3.5 | As a developer, I want to compose two relations into a compound relation so that I can chain dependencies | P0 | `compose(&r1, &r2)` returns combined rotor |
| US-3.6 | As a developer, I want to detect contradictions via bivector magnitude so that I can flag conflicts | P1 | Contradictory pairs produce high bivector magnitude |
| US-3.7 | As a developer, I want to apply context transformations via rotors so that I can switch semantic frames | P1 | `context.apply(&entity)` transforms entity to new context |
| US-3.8 | As a developer, I want to batch-process multiple multivectors so that I can operate on entire KGs efficiently | P2 | Batch operations parallelize across CPU cores |

### Definition of Done

- [ ] Similarity metric correlates with human judgment on sample pairs
- [ ] Analogy computation produces correct results on standard test cases
- [ ] Relation composition is associative (mathematical property verified)
- [ ] Contradiction detection identifies known contradictory pairs
- [ ] Batch operations achieve ≥4x speedup on 4+ cores

---

## Epic 4: Python Bindings

**Goal:** Expose the library to Python ML/AI ecosystems via PyO3.

### User Stories

| ID | Story | Priority | Acceptance Criteria |
|----|-------|----------|---------------------|
| US-4.1 | As a Python developer, I want to create multivectors from Python so that I can use the library without Rust | P1 | `Multivector([1.0, 0.0, ...])` works from Python |
| US-4.2 | As a Python developer, I want to call all semantic operations from Python so that I can integrate with ML pipelines | P1 | `a.semantic_similarity(b)` works from Python |
| US-4.3 | As a Python developer, I want to convert between numpy arrays and multivectors so that I can interop with existing tools | P1 | `Multivector.from_numpy(arr)` and `mv.to_numpy()` work |
| US-4.4 | As a Python developer, I want type hints and docstrings so that I can use the library with IDE support | P2 | All Python functions have type hints and docstrings |

### Definition of Done

- [ ] Package installs via `pip install ga-semantics`
- [ ] All core operations accessible from Python
- [ ] NumPy interop works for array conversion
- [ ] Documentation builds and hosts on ReadTheDocs

---

## Epic 5: Documentation & Publishing

**Goal:** Publish a well-documented crate to crates.io and PyPI.

### User Stories

| ID | Story | Priority | Acceptance Criteria |
|----|-------|----------|---------------------|
| US-5.1 | As a user, I want rustdoc API documentation so that I can understand the library | P0 | `cargo doc` generates ≥80% coverage |
| US-5.2 | As a user, I want example programs so that I can learn by example | P0 | At least 3 examples (analogy, relation compose, cycle demo) |
| US-5.3 | As a user, I want a README with quickstart so that I can get started in <5 minutes | P0 | README contains install, basic usage, and link to full docs |
| US-5.4 | As a researcher, I want a mathematical background document so that I can understand the theory | P1 | docs/math.md covers Cl(3), Bagua mapping, and proofs |
| US-5.5 | As a user, I want the crate on crates.io so that I can add it as a dependency | P0 | `cargo add ga-semantics` works |
| US-5.6 | As a Python user, I want the package on PyPI so that I can install with pip | P1 | `pip install ga-semantics` works |

### Definition of Done

- [ ] Crate passes `cargo publish --dry-run`
- [ ] All public items have rustdoc documentation
- [ ] README contains working quickstart example
- [ ] At least 3 runnable examples in `examples/`
- [ ] Benchmarks documented and reproducible

---

## Epic 6: Benchmarking & Validation

**Goal:** Validate mathematical correctness and semantic value against established baselines.

### User Stories

| ID | Story | Priority | Acceptance Criteria |
|----|-------|----------|---------------------|
| US-6.1 | As a researcher, I want algebra correctness tests so that I can trust the implementation | P0 | All Cayley table entries verified; property tests pass |
| US-6.2 | As a researcher, I want relation classification benchmarks so that I can compare against baselines | P0 | Benchmark runs on standard KG dataset; accuracy reported |
| US-6.3 | As a researcher, I want analogical reasoning benchmarks so that I can validate analogy computation | P1 | Google word analogy test adaptation produces results |
| US-6.4 | As a researcher, I want KG link prediction benchmarks so that I can compare with GeomE | P1 | MRR, Hits@1/3/10 computed on standard benchmark |
| US-6.5 | As a researcher, I want performance benchmarks so that I can verify O(1) operation complexity | P0 | Criterion benchmarks report latency for all core operations |

### Definition of Done

- [ ] All algebra property tests pass
- [ ] Relation classification benchmark runs and reports accuracy
- [ ] Analogy benchmark runs on at least 100 test cases
- [ ] Link prediction benchmark produces MRR and Hits@k metrics
- [ ] Performance benchmarks confirm O(1) for core operations

---

## Epic 7: MCP Server

**Goal:** Expose all ga-semantics operations as MCP tools for AI agent consumption.

### MCP Tool Definitions

| Tool | Input | Output | Description |
|------|-------|--------|-------------|
| `create_multivector` | `coefficients: [f64; 8]` or `{blade, value}[]` | `Multivector` | Construct a multivector |
| `semantic_similarity` | `a: Multivector, b: Multivector` | `{score: f64}` | Similarity in [-1, 1] |
| `semantic_difference` | `a: Multivector, b: Multivector` | `{score: f64}` | Difference in [0, 1] |
| `analogy` | `a: Multivector, b: Multivector, c: Multivector` | `{result: Multivector, confidence: f64}` | `a : b :: c : ?` |
| `classify_relation` | `a: Multivector, b: Multivector` | `{relation_type: "causal", confidence: f64}` | Role-based relation classification |
| `compose_relations` | `r1: Rotor, r2: Rotor` | `{result: Rotor}` | Compose two relations |
| `detect_contradiction` | `a: Multivector, b: Multivector, threshold?: f64` | `{is_contradiction: bool, magnitude: f64}` | Bivector-based conflict check |
| `relation_type_info` | `role: string` | `{definition, bagua_trigram, wuxing, example}` | Lookup role metadata |
| `wuxing_query` | `phase: str, cycle?: "generating" \| "controlling"` | `{phase, next, prev, roles[]}` | Query five-phase cycles |
| `context_apply` | `context: Rotor, entity: Multivector` | `{result: Multivector}` | Transform entity via context |
| `batch_process` | `operations: BatchOp[]` | `{results: any[]}` | Run multiple operations |

### User Stories

| ID | Story | Priority | Acceptance Criteria |
|----|-------|----------|---------------------|
| US-7.1 | As an AI agent developer, I want all operations available as MCP tools so that my agent can reason about semantic relationships | P0 | Server registers ≥10 tools; each responds correctly |
| US-7.2 | As an AI agent developer, I want structured JSON inputs/outputs so that I can parse results without text scraping | P0 | Every tool response is valid JSON with typed fields |
| US-7.3 | As an AI agent developer, I want the MCP tools to return semantic role labels (e.g. "causal", "generative") not Bagua trigram names so that I can use the output directly without a decoding step | P0 | `classify_relation` returns `{relation_type: "causal"}`, not `{trigram: "Zhen"}` |
| US-7.4 | As an AI agent developer, I want the server to connect via stdio so that Claude Desktop can discover it automatically | P0 | `claude_desktop_config.json` snippet works |
| US-7.5 | As an AI agent developer, I want error messages in tool responses so that I can recover gracefully | P1 | Invalid inputs return `{error, message, code}` |
| US-7.6 | As a deployer, I want SSE transport support so that the server can run remotely | P2 | `--transport sse --port 3100` launches HTTP server |
| US-7.7 | As a developer, I want a `ga-semantics-mcp --help` to show available transports and tools | P1 | Help output lists all tools with descriptions |

### Definition of Done

- [ ] Server registers all 11 MCP tools with correct schemas
- [ ] Stdio transport works with `claude_desktop_config.json`
- [ ] SSE transport works with `curl` testing
- [ ] Every tool has at least one integration test (mock client → tool → assert response)
- [ ] Server exits cleanly on SIGINT/SIGTERM
- [ ] `--help` output lists all tools and transport options
- [ ] CI publishes `ga-semantics-mcp` binary to GitHub Releases

---

## Epic 8: CLI Application

**Goal:** Provide a human-friendly CLI for interactive use, shell scripting, and batch processing.

### Commands

| Command | Description |
|---------|-------------|
| `ga-semantics mv` | Create, inspect, and manipulate multivectors |
| `ga-semantics sim` | Compute semantic similarity between two concepts |
| `ga-semantics diff` | Compute semantic difference |
| `ga-semantics analogy` | Solve analogies |
| `ga-semantics classify` | Classify relation between concepts (returns role label) |
| `ga-semantics compose` | Chain multiple relations |
| `ga-semantics contradict` | Check for contradiction |
| `ga-semantics relation-type` | Look up relation type metadata (role → Bagua, WuXing) |
| `ga-semantics wuxing` | Query five-phase cycles |
| `ga-semantics context` | Apply context transformations |
| `ga-semantics batch` | Execute batch operations from JSON file |
| `ga-semantics eval` | Run benchmark evaluations |

### Input/Output Modes

| Mode | Flag | Description |
|------|------|-------------|
| Inline args | (default) | `ga-semantics sim 1 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0` |
| JSON string | `--json` | `ga-semantics sim --json '[1,0,...]' '[0,1,...]'` |
| JSON file | `--file` | `ga-semantics sim --file a.json b.json` |
| stdin pipe | `--stdin` | `echo '{"a":...,"b":...}' \| ga-semantics sim --stdin` |

### Output Formats

| Format | Flag | Use Case |
|--------|------|----------|
| Human-readable | (default) | Colored tables, formatted floats, trigram symbols |
| JSON | `--json` | Pipe to other tools (`jq`, scripts) |
| CSV | `--csv` | Log analysis, spreadsheets |

### User Stories

| ID | Story | Priority | Acceptance Criteria |
|----|-------|----------|---------------------|
| US-8.1 | As a KG engineer, I want to classify a relationship and get a role label like "causal" rather than a trigram name so that I can use the result without consulting a reference table | P0 | `ga-semantics classify --json '[...]' '[...]'` prints `Relation: causal (conf: 0.87)` |
| US-8.2 | As a KG engineer, I want to compute similarity between two concepts from the shell so that I can quickly test embeddings | P0 | `ga-semantics sim --json '[1,0...]' '[0,1...]'` prints `0.0` |
| US-8.3 | As a researcher, I want to run batch evaluations from a file so that I can reproduce benchmarks | P0 | `ga-semantics eval bench.json` prints pass/fail per test case |
| US-8.4 | As a DevOps engineer, I want JSON output so that I can pipe results into monitoring tools | P0 | `--json` flag produces parseable JSON on stdout |
| US-8.5 | As a developer, I want to pipe data via stdin so that I can chain with other CLI tools | P1 | `echo '...' \| ga-semantics classify --stdin` works |
| US-8.6 | As a user, I want colored/readable output by default so that I don't need extra flags for interactive use | P1 | Default output shows ANSI colors, aligned columns |
| US-8.7 | As a user, I want `--help` for every subcommand so that I can discover options | P0 | `ga-semantics sim --help` shows all flags and examples |
| US-8.8 | As a developer, I want to look up what a role label means via the CLI so that I can explore the taxonomy interactively | P1 | `ga-semantics relation-type causal` shows definition, bagua mapping, wuxing phase |

### Definition of Done

- [ ] All 12 subcommands implemented and tested
- [ ] `--json`, `--file`, `--stdin` work for all data-taking subcommands
- [ ] Default output is human-readable with colors and alignment
- [ ] `--json` output is valid JSON for every command
- [ ] Batch eval runs benchmark JSON and reports summary
- [ ] `ga-semantics --help` and `ga-semantics <subcommand> --help` are complete
- [ ] CI publishes `ga-semantics` binary to GitHub Releases
