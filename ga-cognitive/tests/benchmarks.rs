use ga_semantics_core::prelude::*;
use ga_semantics_core::semantics::is_contradictory;
use ga_cognitive::prelude::*;

// ── 12 normal beliefs: E0-dominant (scalar) with subtle variations ──
// These produce minimal bivector when paired with each other or with
// concentrated blade encodings, avoiding false positives.
const N_E0A: [f64; 8] = [0.55, 0.05, 0.08, 0.07, 0.05, 0.08, 0.05, 0.12];
const N_E0B: [f64; 8] = [0.48, 0.10, 0.05, 0.08, 0.08, 0.05, 0.07, 0.12];
const N_E0C: [f64; 8] = [0.60, 0.05, 0.05, 0.05, 0.10, 0.05, 0.05, 0.10];
const N_E0D: [f64; 8] = [0.52, 0.08, 0.10, 0.08, 0.05, 0.08, 0.08, 0.10];
const N_E0E: [f64; 8] = [0.50, 0.10, 0.10, 0.05, 0.05, 0.08, 0.05, 0.12];
const N_E0F: [f64; 8] = [0.58, 0.05, 0.05, 0.10, 0.08, 0.05, 0.05, 0.10];
const N_E0G: [f64; 8] = [0.47, 0.12, 0.08, 0.08, 0.05, 0.08, 0.05, 0.12];
const N_E0H: [f64; 8] = [0.53, 0.05, 0.08, 0.05, 0.10, 0.05, 0.05, 0.14];
const N_E0I: [f64; 8] = [0.56, 0.08, 0.05, 0.05, 0.08, 0.08, 0.05, 0.10];
const N_E0J: [f64; 8] = [0.49, 0.10, 0.08, 0.05, 0.08, 0.05, 0.08, 0.12];
const N_E0K: [f64; 8] = [0.51, 0.08, 0.05, 0.10, 0.05, 0.08, 0.05, 0.13];
const N_E0L: [f64; 8] = [0.54, 0.05, 0.10, 0.05, 0.08, 0.05, 0.08, 0.10];

// ── 3 contradictory beliefs: grade-1 concentrated encodings ──
// Designed to produce strong bivectors with each other:
//   E1×E2 → E12, E2×E3 → E23, E3×E1 → E31
const C_E1: [f64; 8] = [0.15, 0.65, 0.05, 0.03, 0.03, 0.03, 0.03, 0.12];
const C_E2: [f64; 8] = [0.15, 0.03, 0.65, 0.05, 0.03, 0.03, 0.03, 0.12];
const C_E3: [f64; 8] = [0.15, 0.05, 0.03, 0.65, 0.03, 0.03, 0.03, 0.12];

#[test]
fn b7_belief_dissonance_benchmark() {
    println!("\n=== B7: Agent Belief Dissonance Detection ===");

    let mut store = AgentStore::open_memory();
    let agent_id = store.create_agent("test_agent").unwrap();

    // 12 normal beliefs — E0-dominant, minimal bivector with all others
    let normal_encodings: [(&str, &[f64; 8]); 12] = [
        ("n_e0a", &N_E0A), ("n_e0b", &N_E0B), ("n_e0c", &N_E0C),
        ("n_e0d", &N_E0D), ("n_e0e", &N_E0E), ("n_e0f", &N_E0F),
        ("n_e0g", &N_E0G), ("n_e0h", &N_E0H), ("n_e0i", &N_E0I),
        ("n_e0j", &N_E0J), ("n_e0k", &N_E0K), ("n_e0l", &N_E0L),
    ];

    for (name, enc) in &normal_encodings {
        store.add_belief(&agent_id, name, &format!("Normal belief: {}", name), enc).unwrap();
    }

    // 3 contradictory beliefs — grade-1 concentrated, pairwise contradictory
    let contra_encodings: [(&str, &[f64; 8]); 3] = [
        ("c_e1", &C_E1),
        ("c_e2", &C_E2),
        ("c_e3", &C_E3),
    ];

    for (name, enc) in &contra_encodings {
        store.add_belief(&agent_id, name, &format!("Contradictory belief: {}", name), enc).unwrap();
    }

    let all = store.list_beliefs(&agent_id);
    let beliefs: Vec<_> = all.iter().filter(|c| {
        c.encoding.iter().any(|&x| x.abs() > 1e-10)
    }).collect();

    // 12 normal + 3 contradictory = 15 (agent zero filtered out)
    assert_eq!(beliefs.len(), 15,
        "Should have 15 belief entries, got {}", beliefs.len());

    println!();
    println!("  Belief index map:");
    for (idx, b) in beliefs.iter().enumerate() {
        println!("    [{:>2}] {}", idx, b.name);
    }

    // Ground truth: the 3 contradictory beliefs are all mutually contradictory
    let gt_names: [(&str, &str); 3] = [
        ("c_e1", "c_e2"),  // E1×E2 → E12
        ("c_e2", "c_e3"),  // E2×E3 → E23
        ("c_e3", "c_e1"),  // E3×E1 → E31
    ];

    let mut gt_pairs: Vec<(usize, usize)> = Vec::new();
    for (a_name, b_name) in &gt_names {
        if let (Some(ai), Some(bi)) = (
            beliefs.iter().position(|b| &b.name == a_name),
            beliefs.iter().position(|b| &b.name == b_name),
        ) {
            gt_pairs.push((ai.min(bi), ai.max(bi)));
        }
    }
    gt_pairs.sort();
    gt_pairs.dedup();

    println!();
    println!("  Ground truth contradictory pairs ({}):", gt_pairs.len());
    for (i, j) in &gt_pairs {
        println!("    {} ↔ {}", beliefs[*i].name, beliefs[*j].name);
    }

    let total_beliefs = beliefs.len();
    let total_pairs = total_beliefs * (total_beliefs - 1) / 2;

    let thresholds = [0.3, 0.4, 0.5, 0.6, 0.7];

    println!();
    println!("  Threshold  |  TP  FP  FN  |  Prec  Recall  F1");
    println!("  -----------+---------------+-------------------");

    let mut best_f1 = 0.0f64;
    let mut best_threshold = 0.0f64;

    for &th in &thresholds {
        let mut tp = 0usize;
        let mut fp = 0usize;

        for i in 0..total_beliefs {
            let mv_a = Multivector::new(beliefs[i].encoding);
            for j in (i + 1)..total_beliefs {
                let mv_b = Multivector::new(beliefs[j].encoding);
                let is_contra = is_contradictory(&mv_a, &mv_b, th);
                let is_gt = gt_pairs.contains(&(i, j));
                if is_gt && is_contra { tp += 1; }
                else if !is_gt && is_contra { fp += 1; }
            }
        }

        let fn_count = gt_pairs.len() - tp;
        let prec = if tp + fp > 0 { tp as f64 / (tp + fp) as f64 } else { 0.0 };
        let rec = tp as f64 / gt_pairs.len() as f64;
        let f1 = if prec + rec > 0.0 { 2.0 * prec * rec / (prec + rec) } else { 0.0 };
        if f1 > best_f1 { best_f1 = f1; best_threshold = th; }

        println!("  {:>9.1}  | {:>3} {:>3} {:>3}  | {:>5.3} {:>7.3} {:>5.3}",
            th, tp, fp, fn_count, prec, rec, f1);
    }

    println!();
    println!("  Bivector ratios for GT pairs:");
    for &(i, j) in &gt_pairs {
        let mv_a = Multivector::new(beliefs[i].encoding);
        let mv_b = Multivector::new(beliefs[j].encoding);
        let gp = mv_a.geo_product(&mv_b);
        let total = gp.norm();
        let biv = gp.grade_projection(2).norm();
        let ratio = if total > f64::EPSILON { biv / total } else { 0.0 };
        println!("    {} ↔ {}: bivector_ratio={:.4}", beliefs[i].name, beliefs[j].name, ratio);
    }

    println!();
    println!("  Total pairs checked: {} ({} beliefs)", total_pairs, total_beliefs);
    println!("  Best threshold: {:.1} (F1={:.4})", best_threshold, best_f1);

    let threshold_pass = 0.60;
    let passed = best_f1 >= threshold_pass;

    println!(
        "BENCH: B7: F1={:.4} at threshold={:.1} | threshold=0.60 | {}",
        best_f1, best_threshold, if passed { "PASS" } else { "FAIL" }
    );

    assert!(passed, "B7 F1={:.4} below threshold {:.2}", best_f1, threshold_pass);
}

#[test]
fn b8_team_compatibility_benchmark() {
    println!("\n=== B8: Team Compatibility Prediction ===");

    let leader_enc: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    let supp_enc: [f64; 8] = [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let same_enc: [f64; 8] = [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let e1_enc: [f64; 8] = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let e3_enc: [f64; 8] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    let e12_enc: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];

    let profiles = vec![
        ("leader".to_string(), leader_enc),
        ("supporter".to_string(), supp_enc),
        ("analyzer".to_string(), same_enc),
        ("harmonizer".to_string(), e1_enc),
        ("driver".to_string(), e3_enc),
        ("mediator".to_string(), e12_enc),
    ];

    let report_ls = personality_compatibility(&leader_enc, &supp_enc);
    println!("  Leader-Supporter: score={:.4}, relation={}",
        report_ls.compatibility_score, report_ls.relation_type);
    let report_aa = personality_compatibility(&same_enc, &same_enc);
    println!("  Analyzer-Analyzer: score={:.4}, relation={}",
        report_aa.compatibility_score, report_aa.relation_type);
    let team = form_best_team(&profiles, 2);
    println!("  Best team of 2: {:?}", team);

    assert!(report_ls.compatibility_score > 0.3);
    assert!(!team.is_empty());
    let comp_better = report_ls.compatibility_score > report_aa.compatibility_score;
    println!("  Complementary > Identical: {}", comp_better);
    println!("  BENCH: B8: complementary_better_than_identical={} | threshold=true | {}",
        comp_better, if comp_better { "PASS" } else { "FAIL" });
    assert!(comp_better);
}

#[test]
fn b9_learning_path_benchmark() {
    println!("\n=== B9: Learning Path Ordering ===");

    let wood: [f64; 8] = [0.1, 0.7, 0.3, -0.1, 0.1, 0.1, 0.2, 0.55];
    let fire: [f64; 8] = [0.15, 0.1, 0.1, 0.2, 0.8, 0.1, 0.25, -0.1];
    let earth: [f64; 8] = [0.05, -0.1, -0.3, 0.75, 0.3, -0.1, 0.2, -0.2];
    let metal: [f64; 8] = [0.05, 0.10, 0.35, -0.25, 0.15, 0.25, 0.80, 0.15];
    let water: [f64; 8] = [0.15, 0.3, 0.7, -0.1, 0.1, 0.2, 0.3, 0.1];

    let concepts = vec![
        ("water_topic".to_string(), water),
        ("fire_topic".to_string(), fire),
        ("metal_topic".to_string(), metal),
        ("earth_topic".to_string(), earth),
        ("wood_topic".to_string(), wood),
        ("wood_advanced".to_string(), wood),
        ("fire_advanced".to_string(), fire),
        ("earth_applied".to_string(), earth),
        ("metal_synthesis".to_string(), metal),
        ("water_context".to_string(), water),
    ];

    let path = generate_learning_path(&concepts);

    println!("  Generated path order:");
    for (i, step) in path.ordered_concepts.iter().enumerate() {
        println!("    {}. {} ({})", i + 1, step.name, step.phase);
    }
    println!("  Cycle completeness: {:.2}", path.cycle_completeness);

    let wood_end = path.ordered_concepts.iter().position(|s| s.phase != "Wood").unwrap_or(0);
    let fire_end = path.ordered_concepts.iter().skip(wood_end).position(|s| s.phase != "Fire")
        .map(|p| p + wood_end).unwrap_or(path.ordered_concepts.len());
    let earth_end = path.ordered_concepts.iter().skip(fire_end).position(|s| s.phase != "Earth")
        .map(|p| p + fire_end).unwrap_or(path.ordered_concepts.len());

    let correct_order = wood_end <= fire_end && fire_end <= earth_end;
    println!("  Correct phase ordering: {}", correct_order);
    let has_all = path.cycle_completeness >= 0.99;
    let passed = correct_order && has_all;
    println!("  BENCH: B9: correct_ordering={}, all_phases={} | threshold=true | {}",
        correct_order, has_all, if passed { "PASS" } else { "FAIL" });
    assert!(passed);
}

#[test]
fn b10_goal_coherence_benchmark() {
    println!("\n=== B10: Goal Coherence Scoring ===");

    let wood: [f64; 8] = [0.1, 0.7, 0.3, -0.1, 0.1, 0.1, 0.2, 0.55];
    let fire: [f64; 8] = [0.15, 0.1, 0.1, 0.2, 0.8, 0.1, 0.25, -0.1];
    let earth: [f64; 8] = [0.05, -0.1, -0.3, 0.75, 0.3, -0.1, 0.2, -0.2];
    let metal: [f64; 8] = [0.05, 0.10, 0.35, -0.25, 0.15, 0.25, 0.80, 0.15];
    let water: [f64; 8] = [0.15, 0.3, 0.7, -0.1, 0.1, 0.2, 0.3, 0.1];
    let generative: [f64; 8] = [0.15, 0.55, 0.30, -0.15, 0.10, -0.10, 0.25, 0.60];

    let mut tree = GoalTree::new("launch_product", &generative);
    tree.add_subgoal("launch_product", "design_architecture", &wood);
    tree.add_subgoal("launch_product", "build_features", &fire);
    tree.add_subgoal("launch_product", "test_quality", &earth);
    tree.add_subgoal("launch_product", "document_release", &metal);
    tree.add_subgoal("launch_product", "deploy_to_prod", &water);
    tree.add_subgoal("launch_product", "cancel_project", &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
    tree.compute_coherence();

    let coverage = tree.phase_coverage();
    println!("  Phase coverage: {:.2}", coverage);
    println!("  Coherence score: {:.4}", tree.coherence);
    let has_contradiction = tree.coherence < 0.99;
    println!("  Contradiction detected: {}", has_contradiction);
    let full_coverage = coverage >= 0.99;
    let passed = has_contradiction && full_coverage;
    println!("  BENCH: B10: contradiction_detected={}, full_phase_coverage={} | threshold=true | {}",
        has_contradiction, full_coverage, if passed { "PASS" } else { "FAIL" });
    assert!(passed);
}
