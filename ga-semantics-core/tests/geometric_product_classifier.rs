// Test: classify relations using A*B dominant blade instead of A's trigram.
// Hypothesis: the geometric product IS the relationship — its dominant blade
// should match the expected relation type.

use ga_semantics_core::prelude::*;
use ga_semantics_core::relation_type::RelationType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JsonConcept { index: usize, name: String, description: String, domain: String, coefficients: Vec<f64> }
#[derive(Debug, Deserialize)]
struct JsonRelation { index: usize, idx_a: usize, idx_b: usize, label: String, confidence: String, cross_domain: bool }
#[derive(Debug, Deserialize)]
struct BenchmarkDataset { concepts: Vec<JsonConcept>, relations: Vec<JsonRelation> }

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

/// Classify using ONLY A*B's dominant blade — no WuXing cycle, no trigram rules.
fn classify_by_product(a: &Multivector, b: &Multivector) -> (RelationType, f64) {
    let product = a.geo_product(b);
    let dom = product.dominant_role(); // dominant blade of A*B = relation type
    let conf = product.coefficient(dom.blade().index()).abs() / product.norm().max(f64::EPSILON);
    (dom, conf.clamp(0.0, 1.0))
}

#[test]
fn geometric_product_classifier_accuracy() {
    let ds = load();
    let encoded: Vec<Multivector> = ds.concepts.iter().map(|c| {
        let coeffs: [f64; 8] = c.coefficients.as_slice().try_into().unwrap();
        llm_encode(&coeffs)
    }).collect();

    println!("\n{:=^70}", " A*B GEOMETRIC PRODUCT CLASSIFIER ");
    println!("  Hypothesis: A*B dominant blade = relation type");
    println!("  (No WuXing cycle, no trigram rules — pure geometry)");
    println!("{:=^70}", "");

    // ── Per-pair analysis ──
    println!("\n  {:<30} -> {:<30} | {:<12} | {:<12} | {:<16} | {}", "A", "B", "Expected", "A*B says", "Product blade", "Result");
    println!("  {:-<30}---{:-<30}---{:-<12}---{:-<12}---{:-<16}---{}", "", "", "", "", "", "------");

    let mut correct = 0usize;
    let mut blade_matches_expected = 0usize; // product blade == expected blade

    for r in &ds.relations {
        let expected = label_to_type(&r.label);
        let a = &encoded[r.idx_a];
        let b = &encoded[r.idx_b];
        let product = a.geo_product(b);
        let prod_role = product.dominant_role();
        let prod_blade = prod_role.blade();

        let matched = prod_role == expected;
        if matched { correct += 1; }

        // Does the product's dominant blade match the expected label's blade?
        let blade_match = prod_blade == expected.blade();
        if blade_match { blade_matches_expected += 1; }

        println!("  {:<30} -> {:<30} | {:<12} | {:<12} | {:<16} | {}",
            &ds.concepts[r.idx_a].name, &ds.concepts[r.idx_b].name,
            r.label, prod_role.role_name(),
            format!("{:?}", prod_blade),
            if matched { "OK" } else if blade_match { "BLADE" } else { "FAIL" },
        );
    }

    let acc = correct as f64 / ds.relations.len() as f64 * 100.0;
    let blade_acc = blade_matches_expected as f64 / ds.relations.len() as f64 * 100.0;

    println!("\n  ── RESULTS ──");
    println!("  Exact label match:     {}/{} = {:.1}%", correct, ds.relations.len(), acc);
    println!("  Blade (trigram) match: {}/{} = {:.1}%", blade_matches_expected, ds.relations.len(), blade_acc);

    // Per-label breakdown
    println!("\n  ── PER-LABEL ──");
    let labels = ["generative","receptive","causal","transmissive","constraining","influential","clarifying","balancing"];
    for lbl in &labels {
        let lt = label_to_type(lbl);
        let total = ds.relations.iter().filter(|r| label_to_type(&r.label) == lt).count();
        let ok = ds.relations.iter().filter(|r| {
            label_to_type(&r.label) == lt && {
                let p = encoded[r.idx_a].geo_product(&encoded[r.idx_b]);
                p.dominant_role() == lt
            }
        }).count();
        println!("  {:<15} | {:.1}% ({}/{})", lbl, ok as f64/total.max(1) as f64*100.0, ok, total);
    }

    // ── COMPARISON ──
    println!("\n  ── COMPARISON TO OTHER CLASSIFIERS ──");
    let (oc, ot, _) = eval_all(&ds, &encoded, false);
    let (mc, mt, _) = eval_all(&ds, &encoded, true);
    println!("  Original (from_pair):       {}/{} = {:.1}%", oc, ot, oc as f64/ot as f64*100.0);
    println!("  Multi-hyp (from_pair_multi):{}/{} = {:.1}%", mc, mt, mc as f64/mt as f64*100.0);
    println!("  A*B dominant blade:         {}/{} = {:.1}%", correct, ds.relations.len(), acc);
    println!("  A*B blade match:            {}/{} = {:.1}%", blade_matches_expected, ds.relations.len(), blade_acc);

    // The geometric product hypothesis was REJECTED: 5.7% (worse than random 12.5%).
    // The geometric product encodes algebraic structure, not semantic relation type.
    assert!(blade_acc < 12.5,
        "A*B geometric product classifier should be < random (12.5%), got {:.1}%", blade_acc);
}

fn eval_all(ds: &BenchmarkDataset, enc: &[Multivector], multi: bool) -> (usize, usize, f64) {
    let c = ds.relations.iter().filter(|r| {
        let (p, _) = if multi { RelationType::from_pair_multi(&enc[r.idx_a], &enc[r.idx_b]) }
                     else { RelationType::from_pair(&enc[r.idx_a], &enc[r.idx_b]) };
        p == label_to_type(&r.label)
    }).count();
    (c, ds.relations.len(), c as f64 / ds.relations.len() as f64)
}
