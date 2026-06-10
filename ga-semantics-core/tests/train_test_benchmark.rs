// Train/test benchmark with multi-encoding pipeline.
// Compares original from_pair() vs classify_multi_encoded() side-by-side.

use ga_semantics_core::prelude::*;
use ga_semantics_core::multi_encoding::{MultiEncodedConcept, classify_multi_encoded};
use ga_semantics_core::relation_type::{FeatureWeights, RelationType};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DatasetMeta { description: String }
#[derive(Debug, Deserialize)]
struct JsonConcept { index: usize, name: String, domain: String, coefficients: Vec<f64> }
#[derive(Debug, Deserialize)]
struct JsonRelation { index: usize, idx_a: usize, idx_b: usize, label: String, confidence: String, cross_domain: bool }
#[derive(Debug, Deserialize)]
struct JsonSplit { train_relation_indices: Vec<usize>, test_relation_indices: Vec<usize>, train_concept_indices: Vec<usize>, test_concept_indices: Vec<usize> }
#[derive(Debug, Deserialize)]
struct BenchmarkDataset { meta: DatasetMeta, concepts: Vec<JsonConcept>, relations: Vec<JsonRelation>, split: JsonSplit }

fn load() -> BenchmarkDataset {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("data").join("benchmark_dataset.json");
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap()
}

fn label_to_type(label: &str) -> RelationType {
    use RelationType::*;
    match label { "generative"=>Generative,"receptive"=>Receptive,"causal"=>Causal,"transmissive"=>Transmissive,"constraining"=>Constraining,"influential"=>Influential,"clarifying"=>Clarifying,"balancing"=>Balancing, _=>panic!("{label}") }
}

#[test]
fn train_test_benchmark() {
    let ds = load();
    let enc: Vec<Multivector> = ds.concepts.iter().map(|c| {
        let coeffs: [f64; 8] = c.coefficients.as_slice().try_into().unwrap();
        llm_encode(&coeffs)
    }).collect();
    let mc: Vec<MultiEncodedConcept> = ds.concepts.iter().map(|c| {
        let coeffs: [f64; 8] = c.coefficients.as_slice().try_into().unwrap();
        MultiEncodedConcept::from_single_encoding(&llm_encode(&coeffs))
    }).collect();

    println!("\n{:=^80}", " TRAIN-TEST BENCHMARK (multi-encoding) ");
    println!("  {} concepts, {} relations | {} train / {} test",
        ds.concepts.len(), ds.relations.len(), ds.split.train_relation_indices.len(), ds.split.test_relation_indices.len());
    println!("{:=^80}\n", "");

    let weights = FeatureWeights::default();

    let ev = |indices: &[usize], use_multi: bool| -> (usize, usize, f64) {
        let mut ok = 0;
        for &i in indices {
            let r = &ds.relations[i];
            let expected = label_to_type(&r.label);
            let (pred, _) = if use_multi {
                classify_multi_encoded(&mc[r.idx_a], &mc[r.idx_b], &weights)
            } else {
                RelationType::from_pair(&enc[r.idx_a], &enc[r.idx_b])
            };
            if pred == expected { ok += 1; }
        }
        (ok, indices.len(), ok as f64 / indices.len().max(1) as f64 * 100.)
    };

    let (otc, ott, ota) = ev(&ds.split.train_relation_indices, false);
    let (osc, ost, osa) = ev(&ds.split.test_relation_indices, false);
    let (tc, tt, ta) = ev(&ds.split.train_relation_indices, true);
    let (sc, st, sa) = ev(&ds.split.test_relation_indices, true);

    println!("── CLASSIFICATION ──");
    println!("  {:<25} | {:<15} | {:<15} | {:<15}", "Split", "Original", "Multi-enc", "Delta");
    println!("  {:-<25}-+-{:-<15}-+-{:-<15}-+-{:-<15}", "", "", "", "");
    println!("  {:<25} | {:.1}% ({}/{})     | {:.1}% ({}/{})     | {:+.1}pp", "Train", ota, otc, ott, ta, tc, tt, ta - ota);
    println!("  {:<25} | {:.1}% ({}/{})     | {:.1}% ({}/{})     | {:+.1}pp", "Test", osa, osc, ost, sa, sc, st, sa - osa);

    let all_indices: Vec<usize> = (0..ds.relations.len()).collect();
    let (ac, at, aa) = ev(&all_indices, true);
    println!("\n  Multi-encoding all pairs: {}/{} = {:.1}%", ac, at, aa);

    // Per-label test
    println!("\n── PER-LABEL TEST (multi-encoding) ──");
    let labels = ["generative","receptive","causal","transmissive","constraining","influential","clarifying","balancing"];
    for lbl in &labels {
        let lt = label_to_type(lbl);
        let count = ds.split.test_relation_indices.iter().filter(|&&i| label_to_type(&ds.relations[i].label) == lt).count();
        let ok = ds.split.test_relation_indices.iter().filter(|&&i| {
            let r = &ds.relations[i];
            label_to_type(&r.label) == lt && {
                let (p,_) = classify_multi_encoded(&mc[r.idx_a], &mc[r.idx_b], &weights); p == lt
            }
        }).count();
        println!("  {:<15} | {:.1}% ({}/{})", lbl, ok as f64/count.max(1) as f64*100., ok, count);
    }

    // Cross-domain
    let cross: Vec<usize> = ds.relations.iter().enumerate().filter(|(_,r)| r.cross_domain).map(|(i,_)| i).collect();
    let (cc, ct_, ca) = ev(&cross, true);
    println!("\n── CROSS-DOMAIN (multi-encoding) ──");
    println!("  {}/{} = {:.1}%", cc, ct_, ca);

    // Retrieval still uses single encoding (dominant_similarity)
    println!("\n── RETRIEVAL (v1 encoding, unchanged) ──");
    let mut mrr = 0.0f64; let mut p5 = 0.0f64;
    for &i in &ds.split.test_relation_indices {
        let r = &ds.relations[i];
        let query = &enc[r.idx_a]; let target = r.idx_b;
        let mut scored: Vec<(usize, f64)> = enc.iter().enumerate().filter(|(j,_)| *j != r.idx_a)
            .map(|(j,mv)| (j, dominant_similarity(query, mv))).collect();
        scored.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
        let top5: Vec<usize> = scored.iter().take(5).map(|(j,_)| *j).collect();
        if top5.contains(&target) { p5 += 1.0/5.0; }
        if let Some(rk) = scored.iter().position(|(j,_)| *j == target) { mrr += 1.0/(rk as f64+1.0); }
    }
    let n = ds.split.test_relation_indices.len();
    println!("  P@5: {:.1}% | MRR: {:.3}", p5/n as f64*100., mrr/n as f64);

    assert!(aa > 50.0, "Multi-encoding should exceed 50% accuracy, got {:.1}%", aa);
}
