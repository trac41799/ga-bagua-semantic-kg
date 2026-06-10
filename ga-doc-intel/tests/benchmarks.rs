use ga_semantics_core::store::ConceptStore;
use ga_doc_intel::prelude::*;

// ── Encoding constants reused across benchmarks ──

const WOOD_ENCODING: [f64; 8] =
    [0.1, 0.7, 0.3, -0.1, 0.1, 0.1, 0.2, 0.55];
const FIRE_ENCODING: [f64; 8] =
    [0.15, 0.1, 0.1, 0.2, 0.8, 0.1, 0.25, -0.1];
const EARTH_ENCODING: [f64; 8] =
    [0.05, -0.1, -0.3, 0.75, 0.3, -0.1, 0.2, -0.2];
const METAL_ENCODING: [f64; 8] =
    [0.2, 0.2, 0.3, -0.1, 0.1, 0.15, 0.5, 0.7];
const WATER_ENCODING: [f64; 8] =
    [0.15, 0.3, 0.7, -0.1, 0.1, 0.2, 0.3, 0.1];

const WOOD_B_ENCODING: [f64; 8] =
    [0.2, 0.65, 0.2, -0.05, 0.15, 0.05, 0.15, 0.45];
const FIRE_B_ENCODING: [f64; 8] =
    [0.1, 0.15, 0.05, 0.1, 0.75, 0.05, 0.2, -0.05];
const EARTH_B_ENCODING: [f64; 8] =
    [0.08, -0.05, -0.2, 0.72, 0.25, -0.05, 0.15, -0.15];
const METAL_B_ENCODING: [f64; 8] =
    [0.15, 0.25, 0.25, -0.05, 0.05, 0.1, 0.4, 0.65];
const WATER_B_ENCODING: [f64; 8] =
    [0.12, 0.25, 0.65, -0.05, 0.05, 0.15, 0.2, 0.05];

// ── B1: Argument Fallacy Detection ───────────────────────────────────────

#[test]
fn b1_fallacy_detection_benchmark() {
    println!("\n=== B1: Argument Fallacy Detection ===");

    let test_cases: Vec<(&str, &str, [f64; 8], [f64; 8], Option<&str>)> = vec![
        ("Wood_premise", "Fire_conc", WOOD_ENCODING, FIRE_ENCODING, None),
        ("Fire_premise", "Earth_conc", FIRE_ENCODING, EARTH_ENCODING, None),
        ("Earth_premise", "Metal_conc", EARTH_ENCODING, METAL_ENCODING, None),
        ("Metal_premise", "Water_conc", METAL_ENCODING, WATER_ENCODING, None),
        ("Water_premise", "Wood_conc", WATER_ENCODING, WOOD_ENCODING, None),
        ("Wood_A", "Wood_B", WOOD_ENCODING, WOOD_B_ENCODING, Some("non_sequitur")),
        ("Fire_A", "Fire_B", FIRE_ENCODING, FIRE_B_ENCODING, Some("non_sequitur")),
        ("Earth_A", "Earth_B", EARTH_ENCODING, EARTH_B_ENCODING, Some("non_sequitur")),
        ("Metal_A", "Metal_B", METAL_ENCODING, METAL_B_ENCODING, Some("non_sequitur")),
        ("Water_A", "Water_B", WATER_ENCODING, WATER_B_ENCODING, Some("non_sequitur")),
        ("circ_1p", "circ_1c",
            [0.3, 0.5, 0.2, 0.1, 0.1, 0.1, 0.1, 0.1],
            [0.299, 0.499, 0.199, 0.099, 0.099, 0.099, 0.099, 0.099],
            Some("circular")),
        ("circ_2p", "circ_2c",
            [0.1, 0.1, 0.8, 0.1, 0.1, 0.1, 0.1, 0.1],
            [0.099, 0.099, 0.799, 0.099, 0.099, 0.099, 0.099, 0.099],
            Some("circular")),
        ("circ_3p", "circ_3c",
            [0.5, 0.1, 0.1, 0.1, 0.1, 0.6, 0.1, 0.1],
            [0.499, 0.099, 0.099, 0.099, 0.099, 0.599, 0.099, 0.099],
            Some("circular")),
        ("contra_1p", "contra_1c",
            [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            Some("contradiction")),
        ("contra_2p", "contra_2c",
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            Some("contradiction")),
    ];

    let mut tp = 0usize;
    let mut fp = 0usize;
    let mut fn_count = 0usize;
    let mut details: Vec<String> = Vec::new();

    for (premise_name, conclusion_name, prem_enc, conc_enc, expected_gt) in &test_cases {
        let encodings = vec![
            (premise_name.to_string(), *prem_enc),
            (conclusion_name.to_string(), *conc_enc),
        ];
        let results = analyze_argument(&encodings);

        let has_fallacy = !results.is_empty();
        let detected_types: Vec<&str> = results.iter().map(|r| r.fallacy_type.as_str()).collect();
        let mut correct = false;

        match expected_gt {
            None => {
                if !has_fallacy { tp += 1; correct = true; }
                else { fp += 1; }
            }
            Some(expected_type) => {
                if has_fallacy && detected_types.contains(expected_type) {
                    tp += 1; correct = true;
                } else if has_fallacy {
                    fp += 1; fn_count += 1;
                } else {
                    fn_count += 1;
                }
            }
        }

        let gt_label = expected_gt.unwrap_or("valid");
        let pred_label = if has_fallacy { detected_types.join("|") } else { "valid".to_string() };
        let status = if correct { "OK" } else { "MISMATCH" };
        details.push(format!(
            "  {}→{}: GT={} | PRED={} | {}",
            premise_name, conclusion_name, gt_label, pred_label, status
        ));
    }

    let precision = if tp + fp > 0 { tp as f64 / (tp + fp) as f64 } else { 0.0 };
    let recall = if tp + fn_count > 0 { tp as f64 / (tp + fn_count) as f64 } else { 0.0 };
    let f1 = if precision + recall > 0.0 { 2.0 * precision * recall / (precision + recall) } else { 0.0 };
    let threshold = 0.70;
    let passed = f1 >= threshold;

    for d in &details {
        if d.contains("MISMATCH") { println!("{}", d); }
    }
    println!(
        "BENCH: fallacy_detection: precision={:.4} recall={:.4} F1={:.4} | threshold={:.2} | {}",
        precision, recall, f1, threshold, if passed { "PASS" } else { "FAIL" }
    );
    assert!(passed, "B1 F1={:.4} below threshold {:.2}", f1, threshold);
}

// ── B2: Multi-Document Claim Alignment ──────────────────────────────────

const CODE_V1: [f64; 8]  = [0.15, 0.55, 0.30, -0.15, 0.10, -0.10, 0.25, 0.60];
const CODE_V2: [f64; 8]  = [0.12, 0.58, 0.28, -0.12, 0.12, -0.08, 0.22, 0.62];
const TEST_V1: [f64; 8]  = [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34];
const TEST_V2: [f64; 8]  = [0.06, -0.07, -0.49, 0.65, 0.19, -0.28, 0.15, -0.36];
const DEPLOY_V1: [f64; 8] = [0.10, 0.30, 0.60, -0.10, 0.15, 0.30, 0.15, 0.45];
const DEPLOY_V2: [f64; 8] = [0.11, 0.28, 0.62, -0.08, 0.14, 0.32, 0.13, 0.44];
const MONITOR_V1: [f64; 8] = [0.15, 0.05, 0.10, 0.30, 0.85, 0.05, 0.25, -0.15];
const REFACTOR_V1: [f64; 8] = [0.05, 0.10, 0.35, -0.25, 0.15, 0.25, 0.80, 0.15];
const INNOVATE: [f64; 8]   = [0.10, 0.20, 0.20, -0.10, 0.10, 0.78, 0.35, 0.40];
const ANALYZE: [f64; 8]    = [0.65, 0.05, 0.10, 0.15, 0.30, 0.15, 0.10, 0.00];
const BUILD: [f64; 8]      = [0.20, 0.50, 0.25, -0.20, 0.15, -0.05, 0.20, 0.55];
const MEASURE: [f64; 8]    = [0.30, 0.15, 0.05, 0.55, 0.65, 0.10, 0.20, -0.05];
#[allow(dead_code)]
const MONITOR_V2: [f64; 8] = [0.14, 0.06, 0.12, 0.28, 0.82, 0.04, 0.27, -0.14];
#[allow(dead_code)]
const REFACTOR_V2: [f64; 8] = [0.04, 0.12, 0.33, -0.23, 0.17, 0.23, 0.78, 0.16];

#[test]
fn b2_document_alignment_benchmark() {
    println!("\n=== B2: Multi-Document Claim Alignment ===");

    let mut store = ConceptStore::open_memory();

    let doc_a = store.store_document("Doc_A", None, None).unwrap();
    store.store_concept_with_doc("code", "Code implementation", &CODE_V1, doc_a).unwrap();
    store.store_concept_with_doc("test", "Testing strategy", &TEST_V1, doc_a).unwrap();
    store.store_concept_with_doc("deploy", "Deployment process", &DEPLOY_V1, doc_a).unwrap();
    store.store_concept_with_doc("monitor", "Monitoring system", &MONITOR_V1, doc_a).unwrap();
    store.store_concept_with_doc("refactor", "Refactoring approach", &REFACTOR_V1, doc_a).unwrap();
    store.store_concept_with_doc("innovate", "Innovation framework", &INNOVATE, doc_a).unwrap();

    let doc_b = store.store_document("Doc_B", None, None).unwrap();
    store.store_concept_with_doc("code", "Code standards", &CODE_V2, doc_b).unwrap();
    store.store_concept_with_doc("test", "Quality assurance", &TEST_V2, doc_b).unwrap();
    store.store_concept_with_doc("deploy", "CI/CD pipeline", &DEPLOY_V2, doc_b).unwrap();
    store.store_concept_with_doc("build", "Build system", &BUILD, doc_b).unwrap();
    store.store_concept_with_doc("measure", "Measurement metrics", &MEASURE, doc_b).unwrap();
    store.store_concept_with_doc("analyze", "Analytics framework", &ANALYZE, doc_b).unwrap();

    let report_ab = align_documents(&store, doc_a, doc_b);
    let claims_a = store.query_concepts_by_document(doc_a);
    let claims_b = store.query_concepts_by_document(doc_b);

    let mut sim_count_07 = 0usize;
    let mut sim_count_08 = 0usize;
    let mut sim_count_09 = 0usize;
    let matches_at_08 = report_ab.alignments.iter().filter(|a| a.similarity > 0.80).count();

    println!();
    println!("  Similarity matrix (Doc_A rows × Doc_B cols):");
    print!("  {:>12}", "");
    for cb in &claims_b { print!(" {:>8}", cb.name); }
    println!();
    for ca in &claims_a {
        use ga_semantics_core::prelude::*;
        print!("  {:>12}", ca.name);
        let mv_a = Multivector::new(ca.encoding);
        for cb in &claims_b {
            let mv_b = Multivector::new(cb.encoding);
            let sim = dominant_similarity(&mv_a, &mv_b);
            if sim > 0.70 { sim_count_07 += 1; }
            if sim > 0.80 { sim_count_08 += 1; }
            if sim > 0.90 { sim_count_09 += 1; }
            print!(" {:>8.3}", sim);
        }
        println!();
    }

    println!();
    println!("  Pairs with similarity > 0.70: {}", sim_count_07);
    println!("  Pairs with similarity > 0.80: {}", sim_count_08);
    println!("  Pairs with similarity > 0.90: {}", sim_count_09);
    println!("  align_documents matched_count (threshold 0.70): {}", report_ab.matched_count);

    let gt_matches = 3usize;
    let passed = matches_at_08 >= 3;

    println!(
        "BENCH: B2: matches_found={}/{} | threshold=0.80 | {}",
        matches_at_08, gt_matches, if passed { "PASS" } else { "FAIL" }
    );
    assert!(passed, "B2 matches_found={}, expected at least {}", matches_at_08, gt_matches);
}

// ── B3: Research Gap Detection ──────────────────────────────────────────

#[test]
fn b3_research_gap_benchmark() {
    println!("\n=== B3: Research Gap Detection ===");

    let mut store = ConceptStore::open_memory();
    let docs_defs: Vec<(&str, [f64; 8])> = vec![
        ("wood_paper_1", WOOD_ENCODING),
        ("wood_paper_2", WOOD_ENCODING),
        ("fire_paper_1", FIRE_ENCODING),
        ("earth_paper_1", EARTH_ENCODING),
        ("fire_paper_2", FIRE_ENCODING),
    ];

    let mut doc_ids: Vec<i64> = Vec::new();
    for (name, enc) in &docs_defs {
        let doc_id = store.store_document(name, None, None).unwrap();
        store.store_concept_with_doc(&format!("claim_{}", name), "Research claim", enc, doc_id).unwrap();
        doc_ids.push(doc_id);
    }

    let report = find_gaps(&store, &doc_ids);

    println!("  Papers analyzed: {}", report.papers_analyzed);
    println!("  Phase coverage:");
    for (phase, papers) in &report.phase_coverage {
        println!("    {}: {} papers {}", phase, papers.len(), if papers.is_empty() { "(GAP)" } else { "" });
    }
    println!("  Gaps found: {:?}", report.gaps);

    let expected_coverage = 3.0f64 / 5.0;
    let metal_found = report.gaps.iter().any(|g| g.contains("Metal"));
    let water_found = report.gaps.iter().any(|g| g.contains("Water"));
    let coverage_ok = (report.coverage_score - expected_coverage).abs() < 1e-6;
    let gaps_ok = metal_found && water_found && report.gaps.len() == 2;
    let gap_recall = if gaps_ok { 1.0 } else { 0.0 };
    let passed = coverage_ok && gaps_ok;

    println!(
        "BENCH: research_gaps: coverage={:.4} (expected={:.4}), gap_recall={:.2} | threshold=1.00 | {}",
        report.coverage_score, expected_coverage, gap_recall, if passed { "PASS" } else { "FAIL" }
    );
    assert!(coverage_ok, "coverage_score={}, expected={}", report.coverage_score, expected_coverage);
    assert!(metal_found, "Metal gap not detected");
    assert!(water_found, "Water gap not detected");
    assert_eq!(report.gaps.len(), 2, "Expected exactly 2 gaps, got {:?}", report.gaps);
}

// ── B4: Policy Coherence ────────────────────────────────────────────────
//
// 3 contradictory pairs using concentrated grade-1 blade encodings:
//   A1(E1-dom) × B1(E2-dom) → E12 bivector
//   A2(E2-dom) × B2(E3-dom) → E23 bivector
//   A3(E3-dom) × B3(E1-dom) → E31 bivector
//
// Cross-pair contamination: A1×B2(E1×E3=E31), A2×B3(E2×E1=E12), A3×B1(E3×E2=E23)
// → exactly 3 known false positives among the contradictory set.
//
// 5 non-contradictory claims per side using E0-dominant encodings,
// which produce minimal bivector with all other claims.

// Contradictory A-side (grade-1 concentrated)
const C4A_E1: [f64; 8] = [0.15, 0.65, 0.05, 0.03, 0.03, 0.03, 0.03, 0.12];
const C4A_E2: [f64; 8] = [0.15, 0.03, 0.65, 0.05, 0.03, 0.03, 0.03, 0.12];
const C4A_E3: [f64; 8] = [0.15, 0.05, 0.03, 0.65, 0.03, 0.03, 0.03, 0.12];

// Contradictory B-side (grade-1 concentrated)
const C4B_E2: [f64; 8] = [0.15, 0.03, 0.65, 0.05, 0.03, 0.03, 0.03, 0.12];
const C4B_E3: [f64; 8] = [0.15, 0.05, 0.03, 0.65, 0.03, 0.03, 0.03, 0.12];
const C4B_E1: [f64; 8] = [0.15, 0.65, 0.03, 0.05, 0.03, 0.03, 0.03, 0.12];

// Non-contradictory claims — E0-dominant (scalar), minimal bivector with any blade
const N4_A1: [f64; 8] = [0.55, 0.05, 0.08, 0.07, 0.05, 0.08, 0.05, 0.12];
const N4_A2: [f64; 8] = [0.50, 0.10, 0.05, 0.08, 0.08, 0.05, 0.07, 0.12];
const N4_A3: [f64; 8] = [0.60, 0.05, 0.05, 0.05, 0.10, 0.05, 0.05, 0.10];
const N4_A4: [f64; 8] = [0.48, 0.08, 0.10, 0.08, 0.05, 0.08, 0.05, 0.15];
const N4_A5: [f64; 8] = [0.52, 0.05, 0.10, 0.08, 0.08, 0.05, 0.05, 0.12];
const N4_B1: [f64; 8] = [0.52, 0.08, 0.05, 0.10, 0.05, 0.08, 0.05, 0.12];
const N4_B2: [f64; 8] = [0.58, 0.05, 0.08, 0.05, 0.08, 0.05, 0.05, 0.10];
const N4_B3: [f64; 8] = [0.47, 0.10, 0.08, 0.08, 0.05, 0.07, 0.08, 0.12];
const N4_B4: [f64; 8] = [0.55, 0.05, 0.05, 0.10, 0.08, 0.05, 0.05, 0.12];
const N4_B5: [f64; 8] = [0.50, 0.10, 0.05, 0.05, 0.10, 0.08, 0.05, 0.12];

#[test]
fn b4_policy_coherence_benchmark() {
    println!("\n=== B4: Policy Coherence ===");

    let mut store = ConceptStore::open_memory();

    let doc_a = store.store_document("Policy_A", None, None).unwrap();
    let doc_b = store.store_document("Policy_B", None, None).unwrap();

    // Policy A: 3 contradictory + 5 non-contradictory = 8 claims
    store.store_concept_with_doc("A_ctr_e1", "Contra claim E1-dom", &C4A_E1, doc_a).unwrap();
    store.store_concept_with_doc("A_ctr_e2", "Contra claim E2-dom", &C4A_E2, doc_a).unwrap();
    store.store_concept_with_doc("A_ctr_e3", "Contra claim E3-dom", &C4A_E3, doc_a).unwrap();
    store.store_concept_with_doc("A_n1", "Normal claim 1", &N4_A1, doc_a).unwrap();
    store.store_concept_with_doc("A_n2", "Normal claim 2", &N4_A2, doc_a).unwrap();
    store.store_concept_with_doc("A_n3", "Normal claim 3", &N4_A3, doc_a).unwrap();
    store.store_concept_with_doc("A_n4", "Normal claim 4", &N4_A4, doc_a).unwrap();
    store.store_concept_with_doc("A_n5", "Normal claim 5", &N4_A5, doc_a).unwrap();

    // Policy B: 3 contradictory counterparts + 5 non-contradictory = 8 claims
    store.store_concept_with_doc("B_ctr_e2", "Contra claim E2-dom", &C4B_E2, doc_b).unwrap();
    store.store_concept_with_doc("B_ctr_e3", "Contra claim E3-dom", &C4B_E3, doc_b).unwrap();
    store.store_concept_with_doc("B_ctr_e1", "Contra claim E1-dom", &C4B_E1, doc_b).unwrap();
    store.store_concept_with_doc("B_n1", "Normal claim 1", &N4_B1, doc_b).unwrap();
    store.store_concept_with_doc("B_n2", "Normal claim 2", &N4_B2, doc_b).unwrap();
    store.store_concept_with_doc("B_n3", "Normal claim 3", &N4_B3, doc_b).unwrap();
    store.store_concept_with_doc("B_n4", "Normal claim 4", &N4_B4, doc_b).unwrap();
    store.store_concept_with_doc("B_n5", "Normal claim 5", &N4_B5, doc_b).unwrap();

    let claims_a = store.query_concepts_by_document(doc_a);
    let claims_b = store.query_concepts_by_document(doc_b);
    assert_eq!(claims_a.len(), 8);
    assert_eq!(claims_b.len(), 8);

    // Ground truth: 3 contradictory pairs (A_c1↔B_c1, A_c2↔B_c2, A_c3↔B_c3)
    let gt_contradictions: [(usize, usize); 3] = [(0, 0), (1, 1), (2, 2)];

    let thresholds = [0.3, 0.4, 0.5, 0.6, 0.7];

    println!();
    println!("  Threshold  |  TP  FP  FN  |  Prec  Recall  F1");
    println!("  -----------+---------------+-------------------");

    let mut best_f1 = 0.0f64;
    let mut best_threshold = 0.0f64;

    for &th in &thresholds {
        use ga_semantics_core::prelude::*;
        let mut tp = 0usize;
        let mut fp = 0usize;

        for (i, ca) in claims_a.iter().enumerate() {
            for (j, cb) in claims_b.iter().enumerate() {
                let mv_a = Multivector::new(ca.encoding);
                let mv_b = Multivector::new(cb.encoding);
                let is_contra = is_contradictory(&mv_a, &mv_b, th);
                let is_gt = gt_contradictions.contains(&(i, j));
                if is_gt && is_contra { tp += 1; }
                else if !is_gt && is_contra { fp += 1; }
            }
        }

        let fn_count = gt_contradictions.len() - tp;
        let prec = if tp + fp > 0 { tp as f64 / (tp + fp) as f64 } else { 0.0 };
        let rec = tp as f64 / gt_contradictions.len() as f64;
        let f1 = if prec + rec > 0.0 { 2.0 * prec * rec / (prec + rec) } else { 0.0 };
        if f1 > best_f1 { best_f1 = f1; best_threshold = th; }

        println!("  {:>9.1}  | {:>3} {:>3} {:>3}  | {:>5.3} {:>7.3} {:>5.3}",
            th, tp, fp, fn_count, prec, rec, f1);
    }

    println!();
    println!("  Bivector ratio matrix (A rows × B cols, * = GT pair):");
    print!("  {:>14}", "");
    for cb in &claims_b {
        let label = if cb.name.len() > 8 { &cb.name[..8] } else { &cb.name };
        print!(" {:>8}", label);
    }
    println!();
    for (i, ca) in claims_a.iter().enumerate() {
        use ga_semantics_core::prelude::*;
        let label = if ca.name.len() > 14 { &ca.name[..14] } else { &ca.name };
        print!("  {:>14}", label);
        let mv_a = Multivector::new(ca.encoding);
        for (j, cb) in claims_b.iter().enumerate() {
            let mv_b = Multivector::new(cb.encoding);
            let gp = mv_a.geo_product(&mv_b);
            let total = gp.norm();
            let biv = gp.grade_projection(2).norm();
            let ratio = if total > f64::EPSILON { biv / total } else { 0.0 };
            let marker = if gt_contradictions.contains(&(i, j)) { "*" } else { " " };
            print!(" {:>7.3}{}", ratio, marker);
        }
        println!();
    }

    println!();
    println!("  Best threshold: {:.1} (F1={:.4})", best_threshold, best_f1);

    let threshold_pass = 0.60;
    let passed = best_f1 >= threshold_pass;

    println!(
        "BENCH: B4: F1={:.4} at threshold={:.1} | threshold=0.60 | {}",
        best_f1, best_threshold, if passed { "PASS" } else { "FAIL" }
    );
    assert!(passed, "B4 F1={:.4} below threshold {:.2}", best_f1, threshold_pass);
}
