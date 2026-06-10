// Weight optimization benchmark: learn feature weights from training split,
// evaluate on test split. Honest — if weights don't help, report honestly.

use ga_semantics_core::prelude::*;
use ga_semantics_core::relation_type::{FeatureWeights, RelationType};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JsonConcept { name: String, coefficients: Vec<f64> }
#[derive(Debug, Deserialize)]
struct JsonRelation { idx_a: usize, idx_b: usize, label: String }
#[derive(Debug, Deserialize)]
struct JsonSplit {
    train_relation_indices: Vec<usize>, test_relation_indices: Vec<usize>,
}
#[derive(Debug, Deserialize)]
struct BenchmarkDataset { concepts: Vec<JsonConcept>, relations: Vec<JsonRelation>, split: JsonSplit }

fn load() -> BenchmarkDataset {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("data").join("benchmark_dataset.json");
    let content = std::fs::read_to_string(&path).unwrap();
    serde_json::from_str(&content).unwrap()
}

fn label_to_type(label: &str) -> RelationType {
    use RelationType::*;
    match label { "generative"=>Generative,"receptive"=>Receptive,"causal"=>Causal,"transmissive"=>Transmissive,"constraining"=>Constraining,"influential"=>Influential,"clarifying"=>Clarifying,"balancing"=>Balancing, _=>panic!("{label}") }
}

#[test]
fn weighted_classifier_benchmark() {
    let ds = load();
    let encoded: Vec<Multivector> = ds.concepts.iter().map(|c| {
        let coeffs: [f64; 8] = c.coefficients.as_slice().try_into().unwrap();
        llm_encode(&coeffs)
    }).collect();

    // Prepare train/test pairs
    let train_pairs: Vec<(&Multivector, &Multivector, RelationType)> = ds.split.train_relation_indices.iter()
        .map(|&i| {
            let r = &ds.relations[i];
            (&encoded[r.idx_a], &encoded[r.idx_b], label_to_type(&r.label))
        }).collect();
    let test_pairs: Vec<(&Multivector, &Multivector, RelationType)> = ds.split.test_relation_indices.iter()
        .map(|&i| {
            let r = &ds.relations[i];
            (&encoded[r.idx_a], &encoded[r.idx_b], label_to_type(&r.label))
        }).collect();

    println!("\n{:=^70}", " WEIGHTED CLASSIFIER BENCHMARK ");
    println!("  Train pairs: {}  |  Test pairs: {}", train_pairs.len(), test_pairs.len());
    println!("{:=^70}\n", "");

    // ── 1. Optimize weights on training set ──
    println!("── 1. OPTIMIZING WEIGHTS (grid search on train set) ──");
    let optimal = RelationType::optimize_weights(&train_pairs);
    println!("  Optimal weights: f1={:.1}, f2={:.1}, f3={:.1}, f4={:.1}", optimal.f1, optimal.f2, optimal.f3, optimal.f4);
    println!("  Default weights: f1={:.1}, f2={:.1}, f3={:.1}, f4={:.1}", 0.5, 0.1, 0.2, 0.2);

    // ── 2. Evaluate on training set ──
    println!("\n── 2. TRAINING SET ACCURACY ──");
    let train_orig = eval(&train_pairs, false, &FeatureWeights::default());
    let train_def = eval(&train_pairs, true, &FeatureWeights::default());
    let train_opt = eval(&train_pairs, true, &optimal);
    println!("  Original (from_pair):      {:.1}%", train_orig * 100.0);
    println!("  Weighted (default):        {:.1}%", train_def * 100.0);
    println!("  Weighted (optimized):      {:.1}%", train_opt * 100.0);

    // ── 3. Evaluate on test set ──
    println!("\n── 3. TEST SET ACCURACY ──");
    let test_orig = eval(&test_pairs, false, &FeatureWeights::default());
    let test_def = eval(&test_pairs, true, &FeatureWeights::default());
    let test_opt = eval(&test_pairs, true, &optimal);
    println!("  Original (from_pair):      {:.1}%", test_orig * 100.0);
    println!("  Weighted (default):        {:.1}%", test_def * 100.0);
    println!("  Weighted (optimized):      {:.1}%", test_opt * 100.0);

    let gap = (train_opt - test_opt) * 100.0;
    println!("  Generalization gap:        {:+.1}pp", gap);
    if gap.abs() > 20.0 {
        println!("  WARNING: Large generalization gap — weights may be overfitting.");
    }

    // ── 4. Per-label test F1 ──
    println!("\n── 4. PER-LABEL F1 (test set, optimized weights) ──");
    let labels = ["generative","receptive","causal","transmissive","constraining","influential","clarifying","balancing"];
    println!("  {:<15} | {:<8} | {:<8} | {:<8} | {:<8}", "Label", "Count", "Prec", "Recall", "F1");
    println!("  {:-<15}-+-{:-<8}-+-{:-<8}-+-{:-<8}-+-{:-<8}", "", "", "", "", "");

    for lbl in &labels {
        let lt = label_to_type(lbl);
        let (prec, recall, f1) = per_label_metrics(&test_pairs, lt, &optimal);
        let count = test_pairs.iter().filter(|(_, _, e)| *e == lt).count();
        println!("  {:<15} | {:<8} | {:.3}  | {:.3}  | {:.3}", lbl, count, prec, recall, f1);
    }

    // ── 5. Comparison table ──
    println!("\n── 5. FULL COMPARISON (all 53 pairs) ──");
    let all_pairs: Vec<(&Multivector, &Multivector, RelationType)> = ds.relations.iter()
        .map(|r| (&encoded[r.idx_a], &encoded[r.idx_b], label_to_type(&r.label))).collect();
    let a_orig = eval(&all_pairs, false, &FeatureWeights::default());
    let a_multi = eval(&all_pairs, true, &FeatureWeights::default());
    let a_opt = eval(&all_pairs, true, &optimal);
    println!("  Original (from_pair):      {:.1}%", a_orig * 100.0);
    println!("  Multi-hyp (default):       {:.1}%", a_multi * 100.0);
    println!("  Weighted (optimized):      {:.1}%", a_opt * 100.0);

    // ── Honest assertions ──
    assert!(test_opt >= test_orig - 0.05,
        "Weighted classifier ({:.1}%) should not significantly regress from original ({:.1}%)",
        test_opt * 100.0, test_orig * 100.0);
}

fn eval(pairs: &[(&Multivector, &Multivector, RelationType)], weighted: bool, w: &FeatureWeights) -> f64 {
    let correct = pairs.iter().filter(|(a, b, expected)| {
        let (pred, _) = if weighted {
            RelationType::from_pair_weighted(a, b, w)
        } else {
            RelationType::from_pair(a, b)
        };
        pred == *expected
    }).count();
    correct as f64 / pairs.len().max(1) as f64
}

fn per_label_metrics(pairs: &[(&Multivector, &Multivector, RelationType)], label: RelationType, w: &FeatureWeights)
    -> (f64, f64, f64)
{
    let mut tp = 0usize; let mut fp = 0usize; let mut fn_ = 0usize;
    for (a, b, expected) in pairs {
        let (pred, _) = RelationType::from_pair_weighted(a, b, w);
        if *expected == label && pred == label { tp += 1; }
        else if *expected == label && pred != label { fn_ += 1; }
        else if *expected != label && pred == label { fp += 1; }
    }
    let prec = if tp + fp > 0 { tp as f64 / (tp + fp) as f64 } else { 0.0 };
    let recall = if tp + fn_ > 0 { tp as f64 / (tp + fn_) as f64 } else { 0.0 };
    let f1 = if prec + recall > 0.0 { 2.0 * prec * recall / (prec + recall) } else { 0.0 };
    (prec, recall, f1)
}
