// Re-encode benchmark dataset with SKILL.md v2 coefficients, then compare.
// Reads data/benchmark_dataset.json, applies v2 encodings, writes output,
// and runs v1 vs v2 side-by-side comparison.

use ga_semantics_core::prelude::*;
use ga_semantics_core::RelationType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Dataset types ──

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DatasetMeta {
    description: String, num_concepts: usize, num_relations: usize,
    #[serde(default)]
    semantic_roles: Vec<String>, #[serde(default)]
    domain_counts: HashMap<String, usize>, #[serde(default)]
    generated_at: String, #[serde(default)]
    domains: Vec<String>, #[serde(default)]
    encoding_protocol: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct JsonConcept {
    index: usize, name: String, description: String,
    domain: String, coefficients: Vec<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct JsonRelation {
    index: usize, idx_a: usize, idx_b: usize,
    label: String, confidence: String, cross_domain: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct JsonSplit {
    strategy: String, test_concept_count: usize, train_relation_count: usize,
    train_relation_indices: Vec<usize>, test_relation_indices: Vec<usize>,
    test_concept_indices: Vec<usize>, train_concept_indices: Vec<usize>,
    train_concept_count: usize, test_relation_count: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct BenchmarkDataset {
    split: JsonSplit, concepts: Vec<JsonConcept>,
    relations: Vec<JsonRelation>, meta: DatasetMeta,
}

fn load_dataset(path: &str) -> BenchmarkDataset {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Could not read {path}: {e}"));
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse {path}: {e}"))
}

fn load_encoded(ds: &BenchmarkDataset) -> Vec<Multivector> {
    ds.concepts.iter().map(|c| {
        let coeffs: [f64; 8] = c.coefficients.as_slice().try_into().unwrap();
        llm_encode(&coeffs)
    }).collect()
}

fn label_to_type(label: &str) -> RelationType {
    match label {
        "generative" => RelationType::Generative,
        "receptive" => RelationType::Receptive,
        "causal" => RelationType::Causal,
        "transmissive" => RelationType::Transmissive,
        "constraining" => RelationType::Constraining,
        "influential" => RelationType::Influential,
        "clarifying" => RelationType::Clarifying,
        "balancing" => RelationType::Balancing,
        _ => panic!("Unknown label: {label}"),
    }
}

fn eval(encoded: &[Multivector], ds: &BenchmarkDataset, label: &str, use_multi: bool)
    -> (usize, usize, f64)
{
    let mut correct = 0usize;
    for r in &ds.relations {
        let expected = label_to_type(&r.label);
        let (pred, _) = if use_multi {
            RelationType::from_pair_multi(&encoded[r.idx_a], &encoded[r.idx_b])
        } else {
            RelationType::from_pair(&encoded[r.idx_a], &encoded[r.idx_b])
        };
        if pred == expected { correct += 1; }
    }
    let total = ds.relations.len();
    (correct, total, correct as f64 / total as f64 * 100.0)
}

// ── Test ──

#[test]
fn v2_reencoding_comparison() {
    let v1_path = {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest.join("..").join("data").join("benchmark_dataset.json")
    };
    let v2_path = {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest.join("..").join("data").join("benchmark_dataset_v2.json")
    };

    // Load or create v2 dataset
    let ds_v2 = if v2_path.exists() {
        load_dataset(&v2_path.to_string_lossy())
    } else {
        create_v2_dataset(&v1_path.to_string_lossy(), &v2_path.to_string_lossy())
    };

    let ds_v1 = load_dataset(&v1_path.to_string_lossy());
    let enc_v1 = load_encoded(&ds_v1);
    let enc_v2 = load_encoded(&ds_v2);

    println!("\n{:=^80}", " SKILL.md v1 vs v2 ENCODING COMPARISON ");
    println!("  v1 protocol: intrinsic properties (what IS this?)");
    println!("  v2 protocol: relational dynamics (what does it DO?)");
    println!("{:=^80}\n", "");

    // ── ACCURACY ──
    println!("── CLASSIFICATION ACCURACY ──");
    println!("  {:<12} | {:<15} | {:<15} | {:<15}", "Classifier", "v1 SKILL.md", "v2 SKILL.md", "Delta");
    println!("  {:-<12}-+-{:-<15}-+-{:-<15}-+-{:-<15}", "", "", "", "");

    let (_, _, v1o) = eval(&enc_v1, &ds_v2, "original", false);
    let (_, _, v2o) = eval(&enc_v2, &ds_v2, "original", false);
    let (_, _, v1m) = eval(&enc_v1, &ds_v2, "multi", true);
    let (_, _, v2m) = eval(&enc_v2, &ds_v2, "multi", true);

    println!("  {:<12} | {:>13.1}%  | {:>13.1}%  | {:>+13.1}%", "Original", v1o, v2o, v2o - v1o);
    println!("  {:<12} | {:>13.1}%  | {:>13.1}%  | {:>+13.1}%", "Multi-hyp", v1m, v2m, v2m - v1m);

    // ── PER-LABEL ──
    println!("\n── PER-LABEL (Multi-hypothesis) ──");
    println!("  {:<15} | {:<12} | {:<12} | {:<12}", "Label", "v1", "v2", "Delta");
    println!("  {:-<15}-+-{:-<12}-+-{:-<12}-+-{:-<12}", "", "", "", "");

    let labels = ["generative", "receptive", "causal", "transmissive",
        "constraining", "influential", "clarifying", "balancing"];

    for lbl in &labels {
        let lt = label_to_type(lbl);
        let total = ds_v2.relations.iter().filter(|r| label_to_type(&r.label) == lt).count();

        let v1c = ds_v2.relations.iter().filter(|r| {
            label_to_type(&r.label) == lt
                && {
                    let (p, _) = RelationType::from_pair_multi(&enc_v1[r.idx_a], &enc_v1[r.idx_b]);
                    p == lt
                }
        }).count();
        let v2c = ds_v2.relations.iter().filter(|r| {
            label_to_type(&r.label) == lt
                && {
                    let (p, _) = RelationType::from_pair_multi(&enc_v2[r.idx_a], &enc_v2[r.idx_b]);
                    p == lt
                }
        }).count();

        let v1p = v1c as f64 / total.max(1) as f64 * 100.0;
        let v2p = v2c as f64 / total.max(1) as f64 * 100.0;
        println!("  {:<15} | {:>6.1}% ({}/{}) | {:>6.1}% ({}/{}) | {:>+7.1}%",
            lbl, v1p, v1c, total, v2p, v2c, total, v2p - v1p);
    }

    // ── DOMINANT ROLE SHIFTS ──
    println!("\n── DOMINANT ROLE SHIFTS (v1→v2) ──");
    let mut shifts = 0usize;
    for (i, (c1, c2)) in ds_v1.concepts.iter().zip(ds_v2.concepts.iter()).enumerate() {
        let mv1 = llm_encode(&(c1.coefficients.as_slice().try_into().unwrap()));
        let mv2 = llm_encode(&(c2.coefficients.as_slice().try_into().unwrap()));
        let r1 = mv1.dominant_role().role_name();
        let r2 = mv2.dominant_role().role_name();
        if r1 != r2 {
            shifts += 1;
            println!("  {} ({}) -> {} ({}) | {}",
                r1, mv1.dominant_role().wuxing_phase().name(),
                r2, mv2.dominant_role().wuxing_phase().name(),
                c1.name);
        }
    }
    println!("  Total role shifts: {}/{}", shifts, ds_v1.concepts.len());

    // ── ENCODING SHARPNESS ──
    println!("\n── ENCODING SHARPNESS ──");
    let s1: Vec<f64> = enc_v1.iter().map(|mv| mv.encoding_sharpness()).collect();
    let s2: Vec<f64> = enc_v2.iter().map(|mv| mv.encoding_sharpness()).collect();
    let mean1 = s1.iter().sum::<f64>() / s1.len() as f64;
    let mean2 = s2.iter().sum::<f64>() / s2.len() as f64;
    println!("  v1: mean={:.3}  v2: mean={:.3}  delta={:+.3}", mean1, mean2, mean2 - mean1);

    // ── SUMMARY ──
    println!("\n{:=^80}", " SUMMARY ");
    println!("  {:<30} | {:<12} | {:<12} | {:<12}", "Metric", "v1", "v2", "Delta");
    println!("  {:-<30}-+-{:-<12}-+-{:-<12}-+-{:-<12}", "", "", "", "");
    println!("  {:<30} | {:>10.1}%  | {:>10.1}%  | {:>+10.1}%", "Original classifier", v1o, v2o, v2o - v1o);
    println!("  {:<30} | {:>10.1}%  | {:>10.1}%  | {:>+10.1}%", "Multi-hyp classifier", v1m, v2m, v2m - v1m);
    println!("  {:<30} | {:>10.3}  | {:>10.3}  | {:>+10.3}", "Mean sharpness", mean1, mean2, mean2 - mean1);
    println!("  {:<30} | {:>10} | {:>10} | {:>+10}", "Role shifts", "-", shifts, "-");
    println!("{:=^80}\n", "");

    // HONEST ASSERTIONS
    // v2 should improve or at minimum not regress significantly
    assert!(v2m >= v1m - 3.0,
        "v2 multi-hyp accuracy ({:.1}%) should not significantly regress from v1 ({:.1}%)",
        v2m, v1m);
}

fn create_v2_dataset(v1_path: &str, v2_path: &str) -> BenchmarkDataset {
    let mut ds = load_dataset(v1_path);

    // SKILL.md v2 re-encodings (raw coefficients, before normalization)
    let v2_raw: [[f64; 8]; 50] = [
        // 0: Rate Limiter
        [0.05, -0.15, -0.55, 0.85, 0.3, -0.2, 0.2, -0.35],
        // 1: Message Queue
        [0.15, 0.3, 0.85, -0.15, -0.2, 0.15, 0.3, 0.05],
        // 2: Database Index
        [0.1, 0.25, 0.75, 0.2, 0.35, 0.05, 0.0, 0.2],
        // 3: API Gateway
        [0.2, 0.15, 0.8, 0.3, 0.1, 0.0, 0.15, 0.05],
        // 4: Load Balancer
        [0.15, -0.05, 0.35, -0.05, 0.15, 0.1, 0.9, 0.05],
        // 5: Circuit Breaker
        [0.05, -0.2, -0.6, 0.85, 0.35, -0.1, 0.2, -0.3],
        // 6: Monitoring Dashboard
        [0.2, 0.3, 0.15, -0.05, 0.85, 0.2, 0.3, 0.05],
        // 7: Feature Flag
        [0.1, 0.25, 0.2, 0.1, 0.15, 0.8, 0.25, 0.35],
        // 8: Logging System
        [0.3, 0.15, 0.1, 0.0, 0.85, 0.15, 0.05, 0.25],
        // 9: Configuration Store
        [0.7, 0.1, 0.3, 0.2, 0.35, 0.15, 0.05, 0.05],
        // 10: Authentication Provider
        [0.2, 0.15, 0.1, 0.85, 0.3, 0.15, 0.0, 0.3],
        // 11: Cache Layer
        [0.15, 0.2, 0.7, 0.1, 0.05, 0.2, 0.05, 0.25],
        // 12: Event Stream Processor
        [0.15, 0.25, 0.8, -0.05, 0.2, 0.25, 0.05, 0.3],
        // 13: Database Transaction
        [0.15, 0.2, 0.1, 0.85, 0.15, 0.2, 0.35, 0.25],
        // 14: Deprecation Policy
        [0.1, 0.2, 0.15, 0.35, 0.25, 0.8, 0.0, 0.15],
        // 15: Health Check Endpoint
        [0.1, -0.05, 0.15, 0.0, 0.85, 0.1, 0.2, 0.05],
        // 16: Schema Registry
        [0.15, 0.1, 0.25, 0.85, 0.2, 0.25, 0.35, 0.1],
        // 17: Marketing Budget
        [0.15, 0.1, 0.35, 0.85, 0.2, 0.3, 0.05, 0.2],
        // 18: Sales Pipeline
        [0.25, 0.3, 0.8, -0.05, 0.3, 0.2, 0.15, 0.25],
        // 19: Customer Support Ticket
        [0.25, 0.8, 0.2, 0.1, 0.3, 0.15, 0.0, 0.2],
        // 20: Quarterly Report
        [0.15, 0.25, 0.2, 0.0, 0.85, 0.3, 0.05, 0.05],
        // 21: Revenue Target
        [0.05, 0.8, 0.2, 0.3, 0.25, 0.35, -0.05, 0.15],
        // 22: Employee Handbook
        [0.2, 0.05, 0.25, 0.85, 0.3, 0.35, 0.2, 0.05],
        // 23: Vendor Contract
        [0.15, 0.2, 0.05, 0.85, 0.3, 0.25, 0.35, 0.2],
        // 24: Innovation Fund
        [0.1, 0.3, 0.35, 0.25, 0.1, 0.2, 0.0, 0.8],
        // 25: Customer Feedback Loop
        [0.3, 0.2, 0.25, 0.0, 0.35, 0.3, 0.8, 0.2],
        // 26: Onboarding Program
        [0.25, 0.2, 0.85, 0.15, 0.3, 0.25, 0.05, 0.15],
        // 27: Market Trend Analysis
        [0.15, 0.15, 0.35, 0.0, 0.35, 0.8, 0.05, 0.2],
        // 28: Compliance Audit
        [0.1, 0.15, 0.2, 0.15, 0.85, 0.25, 0.35, 0.1],
        // 29: Severance Agreement
        [0.85, 0.15, 0.25, 0.35, 0.2, 0.05, 0.3, 0.15],
        // 30: Industry Regulation
        [0.05, 0.15, 0.2, 0.4, 0.1, 0.85, 0.3, 0.05],
        // 31: Team Standup Meeting
        [0.1, 0.15, 0.25, 0.1, 0.3, 0.2, 0.85, 0.05],
        // 32: Supply Chain
        [0.2, 0.15, 0.85, 0.15, 0.15, 0.25, 0.05, 0.2],
        // 33: Hiring Freeze
        [0.1, -0.1, -0.3, 0.85, 0.2, 0.25, 0.0, -0.35],
        // 34: Predator
        [0.0, 0.15, 0.3, 0.85, 0.05, 0.35, 0.4, 0.1],
        // 35: Decomposer
        [0.25, 0.15, 0.4, 0.0, 0.05, 0.35, 0.3, 0.8],
        // 36: Photosynthesis
        [0.25, 0.15, 0.4, 0.0, 0.05, 0.3, 0.05, 0.85],
        // 37: Water Cycle
        [0.2, 0.15, 0.85, 0.0, 0.05, 0.3, 0.35, 0.2],
        // 38: Keystone Species
        [0.0, 0.2, 0.15, 0.25, 0.05, 0.85, 0.35, 0.3],
        // 39: Mutation
        [0.05, 0.85, 0.0, 0.05, 0.15, 0.25, 0.0, 0.35],
        // 40: Homeostasis
        [0.15, 0.15, 0.2, 0.3, 0.05, 0.25, 0.85, 0.05],
        // 41: Natural Selection
        [0.05, 0.15, 0.0, 0.85, 0.25, 0.4, 0.1, 0.2],
        // 42: Ecological Succession
        [0.0, 0.15, 0.15, 0.0, 0.2, 0.85, 0.05, 0.25],
        // 43: Symbiosis
        [0.2, 0.15, 0.35, 0.0, 0.05, 0.3, 0.85, 0.25],
        // 44: DNA Replication
        [0.25, 0.15, 0.3, 0.0, 0.1, 0.05, 0.3, 0.85],
        // 45: Immune Response
        [0.15, 0.85, 0.35, 0.3, 0.1, 0.25, 0.2, 0.15],
        // 46: Enzyme Catalyst
        [0.2, 0.85, 0.35, 0.0, 0.05, 0.15, 0.0, 0.15],
        // 47: Hormone Signal
        [0.05, 0.3, 0.85, 0.0, 0.1, 0.25, 0.3, 0.05],
        // 48: Cell Membrane
        [0.15, 0.1, 0.35, 0.85, 0.0, 0.15, 0.3, 0.2],
        // 49: Neural Plasticity
        [0.2, 0.2, 0.15, 0.0, 0.05, 0.85, 0.2, 0.3],
    ];

    // Apply v2 coefficients
    for (i, raw) in v2_raw.iter().enumerate() {
        let mv = llm_encode(raw); // normalizes
        ds.concepts[i].coefficients = mv.coefficients().to_vec();
    }

    ds.meta.encoding_protocol = "SKILL.md v2: relational encoding (what does it DO?)".into();
    ds.meta.description = "GA-Bagua v2 semantic knowledge graph benchmark dataset (SKILL.md v2 encodings)".into();

    // Write v2 dataset
    let json = serde_json::to_string_pretty(&ds).unwrap();
    let parent = std::path::Path::new(v2_path).parent().unwrap();
    std::fs::create_dir_all(parent).ok();
    std::fs::write(v2_path, &json).unwrap();
    println!("Wrote v2 dataset to {v2_path}");

    ds
}
