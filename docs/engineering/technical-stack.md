# ga-semantics — Technical Stack Proposal

---

## 1. Core Technology Stack

### 1.1 Primary Language: Rust

| Property | Value |
|----------|-------|
| **Version** | Rust 2021 edition (1.78+) |
| **Rationale** | Zero-cost abstractions, no GC, strong type system, excellent for mathematical libraries |
| **MSRV** | 1.78.0 (minimum supported Rust version) |

**Why Rust:**
- Performance-critical algebraic operations benefit from zero-cost abstractions
- Strong type system catches mathematical errors at compile time
- No runtime dependencies (GC, VM) — ideal for a foundational library
- Ecosystem support for PyO3 bindings, criterion benchmarks, serde serialization

### 1.2 Core Dependencies

| Crate | Version | Purpose | Required |
|-------|---------|---------|----------|
| `thiserror` | 2.x | Error types for degenerate cases | Yes |
| `serde` | 1.x | Serialization framework | Optional (feature-gated) |
| `pyo3` | 0.22+ | Python bindings | Optional (feature-gated) |
| `ndarray` | 0.16+ | Batch array operations | Optional (feature-gated) |

**Nearly zero required dependencies** — core algebra depends only on `thiserror` for typed error handling.

### 1.3 MCP Server Dependencies

| Crate | Version | Purpose | Required |
|-------|---------|---------|----------|
| `mcp-rs` | 0.6+ | MCP protocol server implementation | Yes |
| `tokio` | 1.x | Async runtime for SSE transport | Yes |
| `serde_json` | 1.x | JSON tool I/O serialization | Yes |

### 1.4 CLI Dependencies

| Crate | Version | Purpose | Required |
|-------|---------|---------|----------|
| `clap` | 4.x | Argument parsing with derive macros | Yes |
| `serde_json` | 1.x | JSON I/O for `--json`/`--file`/`--stdin` | Yes |
| `colored` | 2.x | ANSI-colored terminal output | Yes |
| `csv` | 1.x | CSV output mode | Yes |

### 1.5 Development Dependencies

---

## 2. Workspace Structure

The project uses a Cargo workspace with three crates:

```
ga-semantics/
├── Cargo.toml              # Workspace root
├── ga-semantics-core/      # Core library crate
├── ga-semantics-mcp/       # MCP server binary crate
└── ga-semantics-cli/       # CLI binary crate
```

### 2.1 Workspace Cargo.toml

```toml
[workspace]
members = [
    "ga-semantics-core",
    "ga-semantics-mcp",
    "ga-semantics-cli",
]
resolver = "2"
```

### 2.2 Core Library — Cargo.toml

```toml
[package]
name = "ga-semantics-core"
version = "0.1.0"
edition = "2021"
rust-version = "1.78.0"
license = "MIT OR Apache-2.0"
description = "Geometric Algebra semantic layer with Bagua taxonomy for knowledge graphs"
repository = "https://github.com/<org>/ga-semantics-core"
documentation = "https://docs.rs/ga-semantics-core"
readme = "README.md"
keywords = ["geometric-algebra", "clifford-algebra", "knowledge-graph", "bagua", "semantics"]
categories = ["science", "data-structures", "algorithms"]

[features]
default = []
serde = ["dep:serde"]
python = ["dep:pyo3"]
batch = ["dep:ndarray"]

[dependencies]
thiserror = "2"
serde = { version = "1", features = ["derive"], optional = true }
pyo3 = { version = "0.22", features = ["extension-module"], optional = true }
ndarray = { version = "0.16", optional = true }

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
proptest = "1"
rand = "0.8"
rand_distr = "0.4"

[[bench]]
name = "product_bench"
harness = false

[[bench]]
name = "rotor_bench"
harness = false
```

### 2.3 MCP Server — Cargo.toml

```toml
[package]
name = "ga-semantics-mcp"
version = "0.1.0"
edition = "2021"
description = "MCP server for ga-semantics — AI agent tool interface"

[[bin]]
name = "ga-semantics-mcp"
path = "src/main.rs"

[dependencies]
ga-semantics-core = { path = "../ga-semantics-core", features = ["serde"] }
mcp-rs = "0.6"
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

### 2.4 CLI — Cargo.toml

```toml
[package]
name = "ga-semantics-cli"
version = "0.1.0"
edition = "2021"
description = "CLI for ga-semantics — interactive and batch semantic operations"

[[bin]]
name = "ga-semantics"
path = "src/main.rs"

[dependencies]
ga-semantics-core = { path = "../ga-semantics-core", features = ["serde"] }
clap = { version = "4", features = ["derive"] }
serde_json = "1"
colored = "2"
csv = "1"
```

### 2.5 Feature Flags (Core Library)

| Feature | Default | Description |
|---------|---------|-------------|
| `serde` | No | Enable Serialize/Deserialize for Multivector, Trigrams, Hexagram |
| `python` | No | Enable PyO3 Python bindings |
| `batch` | No | Enable ndarray-based batch operations |

---

## 3. Architecture

### 3.1 Workspace Module Structure

```
ga-semantics/
│
├── Cargo.toml                       # Workspace root
│
├── ga-semantics-core/               # Core library crate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                   # Public API, re-exports, prelude
│   │   ├── multivector.rs           # Cl(3) Multivector type + operations
│   │   ├── blades.rs                # Basis blade constants, grade helpers
│   │   ├── rotor.rs                 # Rotor construction, application, composition
│   │   ├── relation_type.rs         # RelationType enum, role labels, bagua conversion
│   ├── bagua.rs                 # Trigram enum, hexagram, mapping to blades (advanced module)
│   │   ├── wuxing.rs               # Five-phase transformation cycles
│   │   ├── semantics.rs             # High-level semantic operations
│   │   ├── error.rs                 # Error types (degenerate inverses, etc.)
│   │   ├── serde.rs                 # Serialization (serde feature)
│   │   └── python.rs                # PyO3 bindings (python feature)
│   ├── benches/
│   │   ├── product_bench.rs         # Geometric product performance
│   │   └── rotor_bench.rs           # Rotor composition benchmarks
│   ├── tests/
│   │   ├── algebra_tests.rs         # Mathematical correctness tests
│   │   ├── bagua_tests.rs           # Bagua mapping validation
│   │   └── analogy_tests.rs         # Analogical reasoning tests
│   ├── examples/
│   │   ├── analogy.rs               # "king - man + woman = queen" in GA
│   │   ├── relation_compose.rs      # Composing KG relations as rotors
│   │   └── cycle_demo.rs            # Wuxing cycles as rotor sequences
│   └── docs/
│       ├── math.md                  # Mathematical background
│       └── bagua_reference.md       # Bagua taxonomy reference
│
├── ga-semantics-mcp/                # MCP server binary crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                  # Server entry point, transport setup
│       └── tools.rs                 # Tool definitions and handler functions
│
└── ga-semantics-cli/                # CLI binary crate
    ├── Cargo.toml
    └── src/
        ├── main.rs                  # CLI entry point, clap setup
        ├── commands/                # One module per subcommand
        │   ├── mod.rs
        │   ├── multivector.rs
        │   ├── similarity.rs
        │   ├── analogy.rs
        │   ├── classify.rs
        │   ├── compose.rs
        │   ├── trigram.rs
        │   ├── wuxing.rs
        │   ├── context.rs
        │   ├── batch.rs
        │   └── eval.rs
        └── output.rs                # Human-readable, JSON, CSV formatters
```

### 3.2 Core Data Structures

```rust
/// Primary public API — returns semantic role labels
impl Multivector {
    /// Degree of alignment with another concept [-1, 1]
    pub fn semantic_similarity(&self, other: &Self) -> f64;

    /// Degree of difference/orthogonality [0, 1]
    pub fn semantic_difference(&self, other: &Self) -> f64;

    /// Classify relationship type as a semantic role label
    /// Returns: RelationType (e.g. Causal, Generative, Constraining)
    pub fn semantic_relation(&self, other: &Self) -> RelationType;

    /// Full relational signature (geometric product)
    pub fn relational_signature(&self, other: &Self) -> Multivector;

    /// Check if two concepts are contradictory
    pub fn is_contradictory(&self, other: &Self, threshold: f64) -> bool;
}
```

```rust
/// A multivector in Cl(3) — 8 coefficients for the 8 basis blades
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Multivector {
    /// Coefficients: [scalar, e1, e2, e3, e12, e23, e31, e123]
    coefficients: [f64; 8],
}

/// Basis blades of Cl(3)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Blade {
    Scalar,     // Grade 0
    E1,         // Grade 1 — Thunder (Zhèn)
    E2,         // Grade 1 — Water (Kǎn)
    E3,         // Grade 1 — Mountain (Gèn)
    E12,        // Grade 2 — Fire (Lí)
    E23,        // Grade 2 — Wind (Xùn)
    E31,        // Grade 2 — Lake (Duì)
    E123,       // Grade 3 — Heaven (Qián)
}

/// The eight semantic role labels — primary public interface for relationship classification.
/// Maps to canonical Bagua trigrams internally.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RelationType {
    Generative,     // Introduces, creates — maps to ☰ Qian
    Receptive,      // Accepts, follows — maps to ☷ Kun
    Causal,         // Triggers, starts — maps to ☳ Zhen
    Transmissive,   // Channels, flows — maps to ☵ Kan
    Constraining,   // Limits, bounds — maps to ☶ Gen
    Influential,    // Pervades, affects — maps to ☴ Xun
    Clarifying,     // Reveals, clarifies — maps to ☲ Li
    Balancing,      // Mirrors, equilibrates — maps to ☱ Dui
}

impl RelationType {
    /// Convert to the canonical Bagua trigram (advanced)
    pub fn bagua(&self) -> Trigram;
}

/// The eight trigrams of the Bagua — internal canonical naming convention.
/// Accessed via `ga_semantics::advanced` for users who want the philosophical grounding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trigram {
    Kun,    // ☷ Earth — receptive
    Gen,    // ☶ Mountain — bounding
    Kan,    // ☵ Water — flowing
    Xun,    // ☴ Wind — penetrating
    Zhen,   // ☳ Thunder — initiating
    Li,     // ☲ Fire — illuminating
    Dui,    // ☱ Lake — reflecting
    Qian,   // ☰ Heaven — creative
}

/// A hexagram composed of upper and lower trigrams
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hexagram {
    upper: Trigram,
    lower: Trigram,
}

/// A rotor in Cl(3) — performs rotations via sandwich product
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotor {
    multivector: Multivector,  // Even-grade element with |R| = 1
}

/// Five Elements / Five Phases
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WuXing {
    Wood,
    Fire,
    Earth,
    Metal,
    Water,
}
```

### 3.3 MCP Server Architecture

```
┌──────────────────────────────────────────────────────┐
│  AI Agent (Claude Desktop, custom agent)              │
│  MCP Client                                           │
└───────────────┬──────────────────────────────────────┘
                │ JSON-RPC over stdio / SSE
┌───────────────▼──────────────────────────────────────┐
│  ga-semantics-mcp (MCP Server)                        │
│                                                       │
│  ┌─────────────┐    ┌──────────────────────────────┐ │
│  │ main.rs      │───▶│ tools.rs                    │ │
│  │ transport     │    │                              │ │
│  │  · stdio      │    │  create_multivector()        │ │
│  │  · SSE        │    │  semantic_similarity()       │ │
│  └─────────────┘    │  analogy()                    │ │
│                      │  classify_relation()          │ │
│                      │  compose_relations()          │ │
│                      │  ... (11 tools total)         │ │
│                      └──────────┬───────────────────┘ │
└──────────────────────────────────┼───────────────────┘
                                   │
┌──────────────────────────────────▼───────────────────┐
│  ga-semantics-core (Library)                          │
│  Multivector, Rotor, Bagua, WuXing, Semantic ops      │
└──────────────────────────────────────────────────────┘
```

Each tool handler:
1. Deserializes JSON params into typed Rust structs
2. Calls the corresponding `ga-semantics-core` function
3. Serializes the result back to JSON
4. Returns structured error on failure

### 3.4 CLI Architecture

```
┌──────────────────────────────────────────────────────┐
│  User / Shell Script / Pipeline                       │
│  stdin, args, files                                   │
└───────────────┬──────────────────────────────────────┘
                │
┌───────────────▼──────────────────────────────────────┐
│  ga-semantics (CLI)                                   │
│                                                       │
│  ┌─────────────┐    ┌──────────────────────────────┐ │
│  │ main.rs      │───▶│ commands/                   │ │
│  │ clap setup    │    │  multivector.rs             │ │
│  │ subcommand     │    │  similarity.rs             │ │
│  │ dispatch       │    │  analogy.rs               │ │
│  └─────────────┘    │  classify.rs                 │ │
│                      │  ... (12 commands)           │ │
│                      └──────────┬───────────────────┘ │
│                      ┌──────────▼───────────────────┐ │
│                      │ output.rs                    │ │
│                      │  format_human()              │ │
│                      │  format_json()               │ │
│                      │  format_csv()                │ │
│                      └──────────────────────────────┘ │
└──────────────────────────────────┼───────────────────┘
                                   │
┌──────────────────────────────────▼───────────────────┐
│  ga-semantics-core (Library)                          │
└──────────────────────────────────────────────────────┘
```

The CLI supports three input modes:
- **Inline args**: `ga-semantics sim 1 0 0 0 0 0 0 0  0 1 0 0 0 0 0 0`
- **JSON file**: `ga-semantics analogy --file king.json queen.json man.json`
- **Stdin pipe**: `echo '{"a":...,"b":...}' | ga-semantics sim --stdin`

And three output modes:
- **Human-readable**: Colored tables with trigram symbols (default)
- **JSON**: `--json` flag for machine parsing
- **CSV**: `--csv` flag for spreadsheet import

### 3.5 Performance Characteristics

| Operation | Complexity | Instruction Count | Notes |
|-----------|-----------|-------------------|-------|
| Geometric product | O(1) | 64 mul, 56 add | Constant for Cl(3) |
| Inner product | O(1) | 8 mul, 7 add | Grade-0 projection of geo product |
| Wedge product | O(1) | 8 mul, 7 add | Grade-2+ projection |
| Inverse | O(1) | ~200 ops | Reverse + norm + division |
| Rotor application | O(1) | 3 × geo product | Sandwich product |
| Dominant trigram | O(1) | 7 comparisons | Find max among 8 coefficients |
| Storage per multivector | 64 bytes | — | 8 × f64 |

---

## 4. Testing Strategy

### 4.1 Unit Tests

| Test Category | Coverage Target | Method |
|--------------|----------------|--------|
| Algebra correctness | 100% of operations | Cayley table verification |
| Inverse property | All non-degenerate multivectors | Property-based testing (proptest) |
| Rotor unitarity | All constructed rotors | `R * R̃ == 1` verification |
| Grade consistency | All grade combinations | Mathematical proof verification |
| Bagua mapping | All 8 trigrams ↔ blades | Bidirectional mapping tests |

### 4.2 Property-Based Tests (proptest)

```rust
proptest! {
    #[test]
    fn inverse_property(a in multivector_strategy()) {
        let inv = a.inverse().unwrap();
        let product = a geo_product(&inv);
        prop_assert!((product.scalar() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn rotor_unitarity(theta in 0.0f64..std::f64::consts::TAU,
                       plane in bivector_strategy()) {
        let r = Rotor::new(theta, plane);
        let product = r.multivector().geo_product(&r.multivector().reverse());
        prop_assert!((product.scalar() - 1.0).abs() < 1e-10);
    }
}
```

### 4.3 Integration Tests

| Test | Description |
|------|-------------|
| `analogy_tests.rs` | Verify "A:B :: C:D" analogy computation |
| `bagua_tests.rs` | Verify 8 trigrams ↔ 8 blades bidirectional mapping |
| `cycle_tests.rs` | Verify WuXing generating and controlling cycles |

### 4.4 Benchmarks

| Benchmark | Metrics | Target |
|-----------|---------|--------|
| `product_bench` | ns/op | <50ns for geometric product |
| `rotor_bench` | ns/op | <150ns for rotor application |
| Batch operations | ops/sec | >10M multivectors/sec |

---

## 5. Documentation Standards

### 5.1 rustdoc Coverage

| Item Type | Minimum Coverage |
|-----------|-----------------|
| Public structs | 100% |
| Public enums | 100% |
| Public functions | 100% |
| Public traits | 100% |
| Private items | Best effort |

### 5.2 Documentation Structure

```
docs/
├── math.md              # Mathematical foundations (Cl(3), Bagua mapping)
├── bagua_reference.md   # Complete Bagua taxonomy reference
├── api.md               # API overview and patterns
└── examples.md          # Walkthrough of all examples
```

### 5.3 README Requirements

- [ ] One-line description
- [ ] Install instructions (`cargo add ga-semantics`)
- [ ] Quickstart code example
- [ ] Feature flags documentation
- [ ] Link to full documentation
- [ ] Link to mathematical background
- [ ] License information

---

## 6. Publishing & Distribution

### 6.1 crates.io

| Field | Value |
|-------|-------|
| Package name | `ga-semantics` |
| License | MIT OR Apache-2.0 |
| Repository | GitHub link |
| Documentation | docs.rs/ga-semantics |
| Keywords | geometric-algebra, clifford-algebra, knowledge-graph, bagua, semantics |
| Categories | science, data-structures, algorithms |

### 6.2 PyPI (if Python bindings)

| Field | Value |
|-------|-------|
| Package name | `ga-semantics` |
| Python versions | 3.9+ |
| Platform | linux, macos, windows (via maturin wheels) |

### 6.3 CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo clippy --all-features -- -D warnings
      - run: cargo fmt --check

  publish:
    needs: [test]
    runs-on: ubuntu-latest
    if: startsWith(github.ref, 'refs/tags/v')
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      # Publish core library
      - run: cargo publish -p ga-semantics-core --token ${{ secrets.CRATES_IO_TOKEN }}

  release-binaries:
    needs: [test]
    strategy:
      matrix:
        target: [x86_64-unknown-linux-gnu, x86_64-pc-windows-msvc, aarch64-apple-darwin]
    runs-on: ubuntu-latest
    if: startsWith(github.ref, 'refs/tags/v')
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release -p ga-semantics-mcp -p ga-semantics-cli
      # Upload both binaries to GitHub Release
      - uses: softprops/action-gh-release@v1
        with:
          files: |
            target/release/ga-semantics-mcp*
            target/release/ga-semantics*
```

---

## 7. Security Considerations

| Concern | Mitigation |
|---------|-----------|
| Floating-point precision | Use `f64`; document precision limits; provide `approx_eq` utilities |
| Degenerate inverses | Return `Option<Multivector>` for inverse; document conditions |
| Integer overflow in blade indexing | Use `Blade` enum (not raw indices) |
| Supply chain | Minimal dependencies; pin versions in CI |
| Unsafe code | No unsafe code in core library |
| MCP input validation | Validate all tool params before dispatching to core; reject malformed JSON |
| CLI no arbitrary execution | CLI is read-only computation; no file write or network capabilities |
| Stdio injection | MCP server validates message framing; does not eval untrusted input |
| Binary supply chain | Publish CLI/MCP binaries via GitHub Releases with SHA256 checksums |

---

## 8. Performance Optimization Roadmap

### 8.1 Phase 1: Baseline (v0.1)

- Simple coefficient arrays
- No SIMD optimization
- Focus on correctness

### 8.2 Phase 2: Optimized (v0.2)

- SIMD-optimized geometric product (SSE2/AVX2)
- Compile-time blade multiplication table via `const fn`
- Inlined operations via `#[inline]`

### 8.3 Phase 3: Batch (v0.3)

- Rayon-based parallel batch operations
- Arena allocation for batch multivectors
- GPU compute shaders (optional, via `wgpu`)

### 8.4 Phase 4: MCP & CLI (v0.4)

- MCP server with stdio and SSE transports
- 11 MCP tools covering all core operations
- CLI with 12 subcommands, 3 input modes, 3 output formats
- Batch eval command for reproducible benchmarks
- GitHub Releases CI for prebuilt binaries
