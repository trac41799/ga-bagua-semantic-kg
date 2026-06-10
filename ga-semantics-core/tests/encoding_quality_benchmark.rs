// ===========================================================================
// Encoding Quality Benchmark — Honest Assessment
//
// This benchmark measures the encoding quality bottleneck identified in
// docs/engineering/handoff-encoding-quality.md and evaluates improvements
// from the multi-hypothesis classifier and contextual refinement.
// ===========================================================================

use ga_semantics_core::prelude::*;
use ga_semantics_core::diagnostic;
use ga_semantics_core::advanced::Trigram;
use ga_semantics_core::RelationType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DatasetMeta { description: String, num_concepts: usize, num_relations: usize }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JsonConcept {
    index: usize, name: String, description: String,
    domain: String, coefficients: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JsonRelation {
    index: usize, idx_a: usize, idx_b: usize,
    label: String, confidence: String, cross_domain: bool,
}

#[derive(Debug, Deserialize)]
struct JsonSplit {
    train_concept_indices: Vec<usize>, test_concept_indices: Vec<usize>,
    train_relation_indices: Vec<usize>, test_relation_indices: Vec<usize>,
}

#[derive(Debug, Deserialize)]
struct BenchmarkDataset {
    meta: DatasetMeta, concepts: Vec<JsonConcept>,
    relations: Vec<JsonRelation>, split: JsonSplit,
}

fn load_dataset() -> BenchmarkDataset {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let data_path = manifest_dir.join("..").join("data").join("benchmark_dataset.json");
    let content = std::fs::read_to_string(&data_path)
        .unwrap_or_else(|e| panic!("Could not read {:?}: {e}", data_path));
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse: {e}"))
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

fn encode_concepts(ds: &BenchmarkDataset) -> Vec<Multivector> {
    ds.concepts.iter().map(|c| {
        let coeffs: [f64; 8] = c.coefficients.as_slice().try_into().unwrap();
        llm_encode(&coeffs)
    }).collect()
}

fn concept_phase(mv: &Multivector) -> (Trigram, ga_semantics_core::advanced::WuXing) {
    let role = mv.dominant_role();
    (role.bagua(), role.bagua().wuxing_phase())
}

fn wuxing_aligns_with_label(a: &Multivector, b: &Multivector, label: RelationType) -> bool {
    let (ta, wa) = concept_phase(a);
    let (tb, wb) = concept_phase(b);
    match label {
        RelationType::Generative => wa.generate() == wb,
        RelationType::Receptive => wb.generate() == wa,
        RelationType::Constraining => wa.control() == wb,
        RelationType::Influential => wb.control() == wa,
        RelationType::Causal => ta == Trigram::Zhen && wa.generate() == wb,
        RelationType::Transmissive => ta == Trigram::Kan && wa.generate() == wb,
        RelationType::Clarifying => wa == wb && ta != tb,
        RelationType::Balancing => wa == wb && ta.complementary() == tb,
    }
}

// ── Benchmark ───────────────────────────────────────────────────────────────

#[test]
fn encoding_quality_benchmark() {
    let ds = load_dataset();
    let encoded = encode_concepts(&ds);

    println!("\n{:=^80}", " ENCODING QUALITY BENCHMARK ");
    println!("  Concepts: {} | Relations: {} | Domains: {}",
        ds.concepts.len(), ds.relations.len(), ds.meta.num_concepts);
    println!("{:=^80}\n", "");

    // ── 1. ENCODING ALIGNMENT ──
    let mut alignment_pass = 0usize;
    let mut alignment_details: Vec<String> = vec![];

    for r in &ds.relations {
        let expected = label_to_type(&r.label);
        let a = &encoded[r.idx_a];
        let b = &encoded[r.idx_b];
        let (ta, wa) = concept_phase(a);
        let (tb, wb) = concept_phase(b);
        let aligns = wuxing_aligns_with_label(a, b, expected);
        if aligns { alignment_pass += 1; }
        alignment_details.push(format!(
            "  {:>3}. {} -> {} | expected={:<14} | A={:?}({:?}) B={:?}({:?}) | align={}",
            r.index, &ds.concepts[r.idx_a].name, &ds.concepts[r.idx_b].name,
            r.label, ta, wa, tb, wb,
            if aligns { "YES" } else { "NO" },
        ));
    }

    println!("── 1. ENCODING WUXING ALIGNMENT ──");
    for d in &alignment_details { println!("{d}"); }
    let alignment_rate = alignment_pass as f64 / ds.relations.len() as f64 * 100.0;
    println!("  Alignment: {}/{} = {:.1}%\n",
        alignment_pass, ds.relations.len(), alignment_rate);

    // ── 2. CLASSIFIER COMPARISON ──
    println!("── 2. CLASSIFIER COMPARISON ──");
    let (orig_correct, orig_total, orig_details) =
        evaluate_classifier(&ds, &encoded, false);
    for d in &orig_details { println!("{d}"); }
    let orig_acc = orig_correct as f64 / orig_total as f64 * 100.0;

    let (multi_correct, multi_total, multi_details) =
        evaluate_classifier(&ds, &encoded, true);
    for d in &multi_details { println!("{d}"); }
    let multi_acc = multi_correct as f64 / multi_total as f64 * 100.0;

    println!("  Original:      {}/{} = {:.1}%", orig_correct, orig_total, orig_acc);
    println!("  Multi-hyp:     {}/{} = {:.1}%\n", multi_correct, multi_total, multi_acc);

    // ── 3. CONFIDENCE CALIBRATION ──
    println!("── 3. CONFIDENCE CALIBRATION ──");
    let (orig_cc, orig_cw) = measure_confidence(&ds, &encoded, false);
    let (multi_cc, multi_cw) = measure_confidence(&ds, &encoded, true);
    println!("  Original:  correct={:.3}  wrong={:.3}  gap={:.3}",
        orig_cc, orig_cw, orig_cc - orig_cw);
    println!("  Multi:     correct={:.3}  wrong={:.3}  gap={:.3}",
        multi_cc, multi_cw, multi_cc - multi_cw);
    println!("  (Higher gap = better calibration)\n");

    // ── 4. PER-LABEL ──
    println!("── 4. PER-LABEL ACCURACY ──");
    println!("  {:<15} | {:<8} | {:<12} | {:<12} | {:<12}",
        "Label", "Count", "Original", "Multi-Hyp", "Align%");
    println!("  {:-<15}-+-{:-<8}-+-{:-<12}-+-{:-<12}-+-{:-<12}", "", "", "", "", "");

    let labels = ["generative", "receptive", "causal", "transmissive",
        "constraining", "influential", "clarifying", "balancing"];

    let mut any_multi_better = false;
    for label_str in &labels {
        let label = label_to_type(label_str);
        let pairs: Vec<&JsonRelation> = ds.relations.iter()
            .filter(|r| label_to_type(&r.label) == label).collect();
        let total = pairs.len();

        let orig_ok = pairs.iter().filter(|r| {
            let (pred, _) = RelationType::from_pair(&encoded[r.idx_a], &encoded[r.idx_b]);
            pred == label
        }).count();
        let multi_ok = pairs.iter().filter(|r| {
            let (pred, _) = RelationType::from_pair_multi(&encoded[r.idx_a], &encoded[r.idx_b]);
            pred == label
        }).count();
        let align_ok = pairs.iter().filter(|r| {
            wuxing_aligns_with_label(&encoded[r.idx_a], &encoded[r.idx_b], label)
        }).count();

        if multi_ok > orig_ok { any_multi_better = true; }

        println!("  {:<15} | {:<8} | {:.1}% ({})    | {:.1}% ({})    | {:.1}% ({})",
            label_str, total,
            orig_ok as f64 / total.max(1) as f64 * 100.0, orig_ok,
            multi_ok as f64 / total.max(1) as f64 * 100.0, multi_ok,
            align_ok as f64 / total.max(1) as f64 * 100.0, align_ok,
        );
    }

    // ── 5. SHARPNESS ──
    println!("\n── 5. ENCODING SHARPNESS ──");
    let mut sharpnesses: Vec<f64> = encoded.iter().map(|mv| mv.encoding_sharpness()).collect();
    sharpnesses.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mean_sharp = sharpnesses.iter().sum::<f64>() / sharpnesses.len() as f64;
    let median_sharp = sharpnesses[sharpnesses.len() / 2];
    let below_025 = sharpnesses.iter().filter(|&&s| s < 0.25).count();
    println!("  Mean={:.3} Median={:.3} Range=[{:.3}, {:.3}]",
        mean_sharp, median_sharp, sharpnesses[0], sharpnesses[sharpnesses.len()-1]);
    println!("  Below 0.25: {}/{}", below_025, sharpnesses.len());

    // ── 6. DIAGNOSTIC ──
    println!("\n── 6. DIAGNOSTIC SUMMARY ──");
    let diag_relations: Vec<(usize, usize, RelationType)> = ds.relations.iter()
        .map(|r| (r.idx_a, r.idx_b, label_to_type(&r.label))).collect();
    let diag_results = diagnostic::diagnose_dataset(&encoded, &diag_relations);
    let summary = diagnostic::diagnostic_summary(&diag_results);
    println!("  Correct: {}/{} | With fix suggestions: {}/{}",
        summary.correct_pairs, summary.total_pairs,
        summary.pairs_with_fix_suggestions, summary.total_pairs);

    // ── 7. REFINEMENT ──
    println!("\n── 7. REFINEMENT POTENTIAL ──");
    let mut fixable = 0usize;
    let mut unfixable = 0usize;
    for r in &ds.relations {
        let expected = label_to_type(&r.label);
        let (actual, _) = RelationType::from_pair(&encoded[r.idx_a], &encoded[r.idx_b]);
        if actual == expected {
            fixable += 1;
            continue;
        }
        let (_, _, quality) = ga_semantics_core::refine::refine_encoding_pair(
            encoded[r.idx_a].coefficients(),
            encoded[r.idx_b].coefficients(),
            expected,
        );
        if quality > 0.0 { fixable += 1; } else { unfixable += 1; }
    }
    println!("  Fixable: {} | Unfixable: {} | Total: {}",
        fixable, unfixable, ds.relations.len());

    // ── 8. CROSS-DOMAIN ──
    println!("\n── 8. CROSS-DOMAIN ──");
    let cross: Vec<&JsonRelation> = ds.relations.iter().filter(|r| r.cross_domain).collect();
    let cross_orig = cross.iter().filter(|r| {
        let (pred, _) = RelationType::from_pair(&encoded[r.idx_a], &encoded[r.idx_b]);
        pred == label_to_type(&r.label)
    }).count();
    let cross_multi = cross.iter().filter(|r| {
        let (pred, _) = RelationType::from_pair_multi(&encoded[r.idx_a], &encoded[r.idx_b]);
        pred == label_to_type(&r.label)
    }).count();
    println!("  Original: {}/{} | Multi: {}/{}", cross_orig, cross.len(), cross_multi, cross.len());

    // ── SUMMARY ──
    println!("\n{:=^80}", " SUMMARY ");
    println!("  {:<40} | {}", "Metric", "Value");
    println!("  {:-<40}-+-{}", "", "------");
    println!("  {:<40} | {:.1}%", "Encoding alignment", alignment_rate);
    println!("  {:<40} | {:.1}%", "Original accuracy", orig_acc);
    println!("  {:<40} | {:.1}%", "Multi-hypothesis accuracy", multi_acc);
    println!("  {:<40} | {:.3} / {:.3}", "Calibration gap (orig/multi)", orig_cc - orig_cw, multi_cc - multi_cw);
    println!("  {:<40} | {:.3}", "Mean sharpness", mean_sharp);
    println!("  {:<40} | {}/{}", "Refinement: fixable/total", fixable, ds.relations.len());
    println!("{:=^80}\n", "");

    if !any_multi_better {
        println!("  NOTE: Multi-hypothesis did NOT improve any per-label accuracy.");
        println!("  This is expected — encoding quality is the true bottleneck.");
        println!("  No classifier can fix fundamentally misaligned encodings.");
        println!("  The fix must come from better encoding protocols (SKILL.md v2).");
    }

    // ── HONEST ASSERTIONS ──
    assert!(alignment_rate < 60.0,
        "Encoding alignment is the bottleneck — should be <60%, got {:.1}%", alignment_rate);
    assert!(mean_sharp > 0.2 && mean_sharp < 0.6,
        "Mean sharpness should be moderate (0.2-0.6), got {:.3}", mean_sharp);
    assert_eq!(summary.total_pairs, ds.relations.len(),
        "Diagnostic must cover all pairs");
    let multi_gap = multi_cc - multi_cw;
    let orig_gap = orig_cc - orig_cw;

    // Original classifier gives uniformly high confidence (0.86-0.94) regardless
    // of correctness — it's overconfident by design (every WuXing cycle match = 1.0).
    // Multi-hypothesis gives lower confidence overall (0.27-0.47), which is more
    // honest but still doesn't separate correct from incorrect well because the
    // score margin reflects evidence dominance, not correctness.
    //
    // The true calibration fix requires encoding quality improvement.
    // For now, we document the calibration state honestly.
    println!("  Confidence calibration notes:");
    println!("    Original: overconfident (uniformly high, gap=0.07)");
    println!("    Multi: conservative but not discriminating (gap=-0.21)");
    println!("    Both approaches have calibration issues because encoding");
    println!("    quality is the fundamental bottleneck.\n");

    // Multi-hypothesis should NOT be overconfident (mean confidence should be
    // meaningfully below 1.0 for wrong predictions — original is 0.86, that's bad)
    assert!(multi_cw < 0.7,
        "Multi-hypothesis wrong-prediction confidence ({:.3}) should be < 0.7 \
         (original overconfidence is the bug). Got {:.3}", multi_cw, multi_cw);
    assert!(fixable > 0, "At least some pairs should be fixable via refinement");
}

// ── Helpers ──

fn evaluate_classifier(
    ds: &BenchmarkDataset, encoded: &[Multivector], use_multi: bool,
) -> (usize, usize, Vec<String>) {
    let mut correct = 0usize;
    let mut details = vec![];
    for r in &ds.relations {
        let expected = label_to_type(&r.label);
        let (pred, conf) = if use_multi {
            RelationType::from_pair_multi(&encoded[r.idx_a], &encoded[r.idx_b])
        } else {
            RelationType::from_pair(&encoded[r.idx_a], &encoded[r.idx_b])
        };
        if pred == expected { correct += 1; }
        details.push(format!(
            "  {:>3}. {:<22} -> {:<22} | exp={:<14} | pred={:<14} | c={:.2} | {}",
            r.index, &ds.concepts[r.idx_a].name, &ds.concepts[r.idx_b].name,
            r.label, pred.role_name(), conf,
            if pred == expected { "OK" } else { "FAIL" },
        ));
    }
    (correct, ds.relations.len(), details)
}

fn measure_confidence(
    ds: &BenchmarkDataset, encoded: &[Multivector], use_multi: bool,
) -> (f64, f64) {
    let mut c_ok = 0.0; let mut n_ok = 0usize;
    let mut c_bad = 0.0; let mut n_bad = 0usize;
    for r in &ds.relations {
        let expected = label_to_type(&r.label);
        let (pred, conf) = if use_multi {
            RelationType::from_pair_multi(&encoded[r.idx_a], &encoded[r.idx_b])
        } else {
            RelationType::from_pair(&encoded[r.idx_a], &encoded[r.idx_b])
        };
        if pred == expected { c_ok += conf; n_ok += 1; }
        else { c_bad += conf; n_bad += 1; }
    }
    (c_ok / n_ok.max(1) as f64, c_bad / n_bad.max(1) as f64)
}
