// LLM Feedback Loop Benchmark: measure improvement from WuXing-aligned re-encoding
// Phase 2: Apply corrective prompts, create v3 dataset, measure delta

use ga_semantics_core::prelude::*;
use ga_semantics_core::relation_type::{FeatureWeights, RelationType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DatasetMeta {
    description: String, num_concepts: usize, num_relations: usize,
    #[serde(default)] semantic_roles: Vec<String>, #[serde(default)] domain_counts: HashMap<String,usize>,
    #[serde(default)] generated_at: String, #[serde(default)] domains: Vec<String>,
    #[serde(default)] encoding_protocol: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct JsonConcept { index: usize, name: String, description: String, domain: String, coefficients: Vec<f64> }
#[derive(Debug, Deserialize, Serialize, Clone)]
struct JsonRelation { index: usize, idx_a: usize, idx_b: usize, label: String, confidence: String, cross_domain: bool }
#[derive(Debug, Deserialize, Serialize, Clone)]
struct JsonSplit { strategy: String, test_concept_count: usize, train_relation_count: usize,
    train_relation_indices: Vec<usize>, test_relation_indices: Vec<usize>,
    test_concept_indices: Vec<usize>, train_concept_indices: Vec<usize>,
    train_concept_count: usize, test_relation_count: usize }
#[derive(Debug, Deserialize, Serialize, Clone)]
struct BenchmarkDataset { split: JsonSplit, concepts: Vec<JsonConcept>, relations: Vec<JsonRelation>, meta: DatasetMeta }

fn label_to_type(label: &str) -> RelationType {
    use RelationType::*;
    match label { "generative"=>Generative,"receptive"=>Receptive,"causal"=>Causal,"transmissive"=>Transmissive,"constraining"=>Constraining,"influential"=>Influential,"clarifying"=>Clarifying,"balancing"=>Balancing, _=>panic!("{label}") }
}

#[test]
fn feedback_loop_benchmark() {
    // Load v1
    let v1_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("data").join("benchmark_dataset.json");
    let content = std::fs::read_to_string(&v1_path).unwrap();
    let mut ds: BenchmarkDataset = serde_json::from_str(&content).unwrap();

    // LLM feedback loop re-encodings (35 concepts re-encoded, 15 unchanged)
    let v3_raw: [(usize, [f64; 8]); 50] = [
        (0, [0.05, 0.10, 0.10, 0.20, 0.10, 0.10, 0.80, -0.10]),
        (1, [0.10, 0.80, 0.20, -0.10, 0.05, 0.15, 0.15, 0.15]),
        (2, [0.10, 0.10, 0.80, 0.10, 0.10, 0.15, 0.05, 0.10]),
        (3, [0.15, 0.10, 0.20, 0.15, 0.10, 0.80, 0.15, 0.15]),
        (4, [0.15, 0.05, 0.80, 0.15, 0.10, 0.10, 0.20, 0.10]),
        (5, [0.0543, -0.2714, -0.2171, 0.8468, 0.2714, -0.1086, 0.2171, -0.1629]),
        (6, [0.2195, 0.2744, 0.0549, 0.1098, 0.8781, 0.1098, 0.2744, -0.0549]),
        (7, [0.10, 0.10, 0.10, 0.80, 0.10, 0.15, 0.05, 0.10]),
        (8, [0.1606, 0.0535, 0.1071, 0.2141, 0.9100, 0.0535, 0.2676, -0.1071]),
        (9, [0.20, 0.05, 0.15, 0.10, 0.80, 0.10, 0.10, 0.00]),
        (10, [0.2826, 0.1696, -0.1130, 0.7348, 0.4522, 0.0565, 0.2826, 0.2261]),
        (11, [0.15, 0.05, 0.20, 0.10, 0.10, 0.10, 0.80, 0.10]),
        (12, [0.10, 0.20, 0.15, 0.05, 0.80, 0.10, 0.10, 0.15]),
        (13, [0.10, 0.15, 0.80, 0.15, 0.10, 0.10, 0.15, 0.10]),
        (14, [0.2267, 0.1133, 0.0567, 0.2834, 0.1700, 0.8841, 0.1700, 0.1133]),
        (15, [0.1759, 0.1172, 0.1172, 0.1172, 0.9379, 0.1172, 0.1759, 0.0586]),
        (16, [0.1711, 0.0570, 0.1711, 0.8898, 0.3422, 0.1141, 0.1141, 0.0570]),
        (17, [0.10, 0.10, 0.10, 0.20, 0.80, 0.15, 0.10, 0.10]),
        (18, [0.15, 0.10, 0.20, 0.80, 0.10, 0.10, 0.10, 0.10]),
        (19, [0.1179, 0.8839, 0.2357, 0.0589, 0.1179, 0.2946, 0.1179, 0.1768]),
        (20, [0.15, 0.10, 0.15, 0.05, 0.20, 0.10, 0.10, 0.80]),
        (21, [0.1086, 0.3259, 0.1086, -0.1086, 0.1629, 0.1086, 0.1629, 0.8908]),
        (22, [0.15, 0.05, 0.10, 0.20, 0.15, 0.80, 0.10, 0.10]),
        (23, [0.1155, 0.1155, 0.0577, 0.9238, 0.1732, 0.2309, 0.1732, 0.0577]),
        (24, [0.80, 0.15, 0.15, 0.10, 0.10, 0.10, 0.05, 0.15]),
        (25, [0.20, 0.10, 0.15, 0.05, 0.80, 0.10, 0.10, 0.10]),
        (26, [0.20, 0.80, 0.15, 0.05, 0.10, 0.15, 0.05, 0.10]),
        (27, [0.15, 0.10, 0.15, 0.80, 0.20, 0.15, 0.05, 0.10]),
        (28, [0.1116, 0.1116, 0.1116, 0.3349, 0.8930, 0.1116, 0.1674, 0.1116]),
        (29, [0.20, 0.05, 0.10, 0.15, 0.10, 0.05, 0.80, 0.10]),
        (30, [0.15, 0.10, 0.80, 0.20, 0.10, 0.20, 0.10, 0.10]),
        (31, [0.1696, 0.1696, 0.1696, 0.0565, 0.1696, 0.2261, 0.9044, 0.1130]),
        (32, [0.15, 0.10, 0.20, 0.10, 0.10, 0.80, 0.10, 0.10]),
        (33, [0.0542, -0.3249, -0.1625, 0.8123, 0.2166, -0.1083, 0.2708, -0.2708]),
        (34, [0.10, 0.10, 0.10, 0.20, 0.10, 0.10, 0.10, 0.80]),
        (35, [0.20, 0.10, 0.80, 0.10, 0.05, 0.15, 0.10, 0.10]),
        (36, [0.80, 0.10, 0.15, 0.05, 0.05, 0.15, 0.10, 0.15]),
        (37, [0.15, 0.80, 0.20, 0.10, 0.05, 0.15, 0.10, 0.10]),
        (38, [0.10, 0.15, 0.10, 0.80, 0.10, 0.20, 0.10, 0.10]),
        (39, [0.10, 0.15, 0.10, -0.05, 0.80, 0.15, 0.05, 0.20]),
        (40, [0.1729, 0.0576, 0.1729, 0.1729, 0.1729, 0.1153, 0.9222, 0.1153]),
        (41, [0.10, 0.10, 0.10, 0.20, 0.15, 0.15, 0.80, 0.10]),
        (42, [0.15, 0.10, 0.80, 0.10, 0.10, 0.20, 0.10, 0.10]),
        (43, [0.15, 0.10, 0.15, 0.10, 0.10, 0.80, 0.20, 0.10]),
        (44, [0.80, 0.15, 0.15, 0.10, 0.05, 0.10, 0.10, 0.10]),
        (45, [0.10, 0.20, 0.10, 0.10, 0.80, 0.10, 0.15, 0.10]),
        (46, [0.15, 0.10, 0.10, 0.80, 0.05, 0.10, 0.05, 0.10]),
        (47, [0.10, 0.80, 0.20, 0.10, 0.05, 0.15, 0.05, 0.10]),
        (48, [0.10, 0.05, 0.15, 0.20, 0.80, 0.10, 0.10, 0.10]),
        (49, [0.1143, 0.2285, 0.1143, 0.0571, 0.1143, 0.8913, 0.1714, 0.2857]),
    ];

    // Apply v3 to dataset
    for (idx, raw) in &v3_raw {
        let mv = llm_encode(raw);
        ds.concepts[*idx].coefficients = mv.coefficients().to_vec();
    }
    ds.meta.description = "GA-Bagua v3: WuXing-aligned encodings (LLM feedback loop)".into();
    ds.meta.encoding_protocol = "SKILL.md v2 feedback loop: WuXing-aligned encoding".into();

    let v3_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("data").join("benchmark_dataset_v3.json");
    std::fs::write(&v3_path, serde_json::to_string_pretty(&ds).unwrap()).unwrap();

    // Encode all concepts
    let enc: Vec<Multivector> = ds.concepts.iter().map(|c| {
        let coeffs: [f64; 8] = c.coefficients.as_slice().try_into().unwrap();
        llm_encode(&coeffs)
    }).collect();

    // ── MEASURE ──
    println!("\n{:=^70}", " LLM FEEDBACK LOOP RESULTS (v3) ");
    println!("  Protocol: corrective_prompt() → LLM re-encode → measure");
    println!("  Re-encoded: 35/50 concepts in WuXing-aligned phases");
    println!("{:=^70}", "");

    // Alignment
    let align = measure_align(&ds, &enc);
    println!("\n── ENCODING ALIGNMENT ──");
    println!("  {}/{} = {:.1}% (v1 was 15.1%)", align.0, align.1, align.2 * 100.0);

    // Accuracy comparison
    println!("\n── ACCURACY COMPARISON ──");
    let pairs: Vec<(&Multivector, &Multivector, RelationType)> = ds.relations.iter()
        .map(|r| (&enc[r.idx_a], &enc[r.idx_b], label_to_type(&r.label))).collect();
    let train_pairs: Vec<(&Multivector, &Multivector, RelationType)> = ds.split.train_relation_indices.iter()
        .map(|&i| (&enc[ds.relations[i].idx_a], &enc[ds.relations[i].idx_b], label_to_type(&ds.relations[i].label))).collect();
    let test_pairs: Vec<(&Multivector, &Multivector, RelationType)> = ds.split.test_relation_indices.iter()
        .map(|&i| (&enc[ds.relations[i].idx_a], &enc[ds.relations[i].idx_b], label_to_type(&ds.relations[i].label))).collect();

    let orig_all = eval(&pairs, false, &FeatureWeights::default());
    let multi_all = eval(&pairs, true, &FeatureWeights::default());

    // Optimize weights on train, evaluate on test
    let optimal = RelationType::optimize_weights(&train_pairs);
    let train_opt = eval(&train_pairs, true, &optimal);
    let test_opt = eval(&test_pairs, true, &optimal);
    let all_opt = eval(&pairs, true, &optimal);

    println!("  {:<30} | {:<12} | {:<12}", "Classifier", "v3 Acc", "v1 Acc (ref)");
    println!("  {:-<30}-+-{:-<12}-+-{:-<12}", "", "", "");
    println!("  {:<30} | {:.1}%        | 20.8%", "Original (from_pair)", orig_all * 100.0);
    println!("  {:<30} | {:.1}%        | 39.6%", "Multi-hyp (default)", multi_all * 100.0);
    println!("  {:<30} | {:.1}%         | 92.9%", "Weighted train (opt)", train_opt * 100.0);
    println!("  {:<30} | {:.1}%         | 80.0%", "Weighted test (opt)", test_opt * 100.0);
    println!("  {:<30} | {:.1}%         | 86.8%", "Weighted all (opt)", all_opt * 100.0);

    println!("\n  Optimal weights: f1={:.1}, f2={:.1}, f3={:.1}, f4={:.1}",
        optimal.f1, optimal.f2, optimal.f3, optimal.f4);

    // Per-label
    println!("\n── PER-LABEL TEST ACCURACY (weighted, optimized) ──");
    let labels = ["generative","receptive","causal","transmissive","constraining","influential","clarifying","balancing"];
    for lbl in &labels {
        let lt = label_to_type(lbl);
        let total = test_pairs.iter().filter(|(_,_,e)| *e == lt).count();
        let ok = test_pairs.iter().filter(|(a,b,e)| *e == lt && {
            let (p,_) = RelationType::from_pair_weighted(a,b,&optimal); p == lt
        }).count();
        println!("  {:<15} | {:.1}% ({}/{})", lbl, ok as f64/total.max(1) as f64*100.0, ok, total);
    }

    // Cross-domain
    let cross: Vec<usize> = ds.relations.iter().enumerate()
        .filter(|(_,r)| r.cross_domain).map(|(i,_)| i).collect();
    let cross_ok = cross.iter().filter(|&&i| {
        let r = &ds.relations[i];
        let (p,_) = RelationType::from_pair_weighted(&enc[r.idx_a], &enc[r.idx_b], &optimal);
        p == label_to_type(&r.label)
    }).count();
    println!("\n── CROSS-DOMAIN ──");
    println!("  {}/{} = {:.1}%", cross_ok, cross.len(), cross_ok as f64/cross.len().max(1) as f64*100.0);

    // ── KEY QUESTION: Does f1 (WuXing cycle) now have signal? ──
    println!("\n── WUXING CYCLE SIGNAL ──");
    let f1_active = optimal.f1 > 0.0;
    let f2_active = optimal.f2 > 0.0;
    println!("  f1 (WuXing exact) = {:.1} — {}", optimal.f1, if f1_active { "SIGNAL DETECTED" } else { "still zero" });
    println!("  f2 (WuXing partial) = {:.1} — {}", optimal.f2, if f2_active { "SIGNAL DETECTED" } else { "still zero" });

    // ── HONEST ASSERTIONS ──
    assert!(align.2 > 0.15, "v3 alignment ({:.1}%) should exceed v1 (15.1%)", align.2 * 100.0);
    // If f1 is still 0, the feedback loop didn't help WuXing alignment enough
    if !f1_active && !f2_active {
        println!("\n  WARNING: WuXing cycle still has no signal (f1=f2=0).");
        println!("  The feedback loop shifted phases but the resulting encodings");
        println!("  may not be sharp enough for the cycle to dominate.");
    }
}

fn measure_align(ds: &BenchmarkDataset, enc: &[Multivector]) -> (usize, usize, f64) {
    let ok = ds.relations.iter().filter(|r| {
        let expected = label_to_type(&r.label);
        let a = &enc[r.idx_a]; let b = &enc[r.idx_b];
        let ta = a.dominant_role().bagua(); let tb = b.dominant_role().bagua();
        let wa = ta.wuxing_phase(); let wb = tb.wuxing_phase();
        use ga_semantics_core::advanced::Trigram;
        match expected {
            RelationType::Generative => wa.generate() == wb,
            RelationType::Receptive => wb.generate() == wa,
            RelationType::Constraining => wa.control() == wb,
            RelationType::Influential => wb.control() == wa,
            RelationType::Causal => ta == Trigram::Zhen && wa.generate() == wb,
            RelationType::Transmissive => ta == Trigram::Kan && wa.generate() == wb,
            RelationType::Clarifying => wa == wb && ta != tb,
            RelationType::Balancing => wa == wb && ta.complementary() == tb,
        }
    }).count();
    (ok, ds.relations.len(), ok as f64 / ds.relations.len() as f64)
}

fn eval(pairs: &[(&Multivector, &Multivector, RelationType)], weighted: bool, w: &FeatureWeights) -> f64 {
    let ok = pairs.iter().filter(|(a, b, e)| {
        let (p, _) = if weighted { RelationType::from_pair_weighted(a, b, w) }
                     else { RelationType::from_pair(a, b) };
        p == *e
    }).count();
    ok as f64 / pairs.len().max(1) as f64
}
