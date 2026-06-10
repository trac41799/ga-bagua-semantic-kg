/// Comprehensive improvement benchmark covering all 4 directions
/// on the 38-concept human-labeled dataset.
///
/// Direction 3: Ensemble classifier (majority, weighted, best-on-train)
/// Direction 4: Hexagram-based classification
/// Direction 1: Cl(4) higher-dimensional GA
/// Direction 2: Trainable logistic regression (LOO-CV)
///
/// Baseline: 56.4% (multi-encoding), 24.4% (original from_pair)

use ga_semantics_core::prelude::*;
use ga_semantics_core::ensemble::EnsembleClassifier;
use ga_semantics_core::multi_encoding::{MultiEncodedConcept, classify_multi_encoded};
use ga_semantics_core::relation_type::{FeatureWeights, RelationType};
use ga_semantics_core::trainable::GaFeatureClassifier;

// Re-use the same 38 concepts and 41 relations from realistic_benchmark.rs
struct RealConcept { coefficients: [f64; 8] }
struct LabeledRelation { idx_a: usize, idx_b: usize, human_label: &'static str }

fn concepts() -> Vec<RealConcept> {
    vec![
        RealConcept { coefficients: [0.05, 0.05, 0.10, 0.85, -0.05, 0.25, 0.10, 0.10] },   // 0: Marketing Budget
        RealConcept { coefficients: [0.10, 0.15, 0.80, -0.05, -0.10, 0.15, 0.20, 0.20] },   // 1: Sales Pipeline
        RealConcept { coefficients: [0.10, 0.30, 0.10, -0.10, 0.15, 0.10, 0.15, 0.85] },    // 2: Revenue Target
        RealConcept { coefficients: [0.10, 0.75, 0.20, 0.05, 0.10, 0.30, 0.10, 0.20] },     // 3: Support Ticket
        RealConcept { coefficients: [0.15, 0.10, 0.10, 0.15, 0.25, 0.80, 0.10, 0.10] },     // 4: Quarterly Report
        RealConcept { coefficients: [0.15, 0.05, 0.05, 0.80, 0.30, 0.25, 0.10, 0.05] },     // 5: Employee Handbook
        RealConcept { coefficients: [0.10, 0.05, 0.05, 0.85, 0.15, 0.20, 0.15, 0.05] },     // 6: Vendor Contract
        RealConcept { coefficients: [0.05, 0.25, 0.15, -0.15, 0.10, 0.15, 0.10, 0.88] },    // 7: Innovation Fund
        RealConcept { coefficients: [0.25, 0.10, 0.25, 0.05, 0.15, 0.20, 0.78, 0.10] },     // 8: Feedback Loop
        RealConcept { coefficients: [0.20, 0.15, 0.75, 0.05, 0.15, 0.10, 0.25, 0.15] },     // 9: Onboarding Process
        RealConcept { coefficients: [0.15, 0.20, 0.10, 0.05, 0.78, 0.25, 0.15, 0.20] },     // 10: Market Trend
        RealConcept { coefficients: [0.10, 0.10, 0.10, 0.30, 0.15, 0.80, 0.15, 0.10] },     // 11: Compliance Audit
        RealConcept { coefficients: [0.76, 0.05, 0.15, 0.15, 0.10, 0.10, 0.20, 0.10] },     // 12: Severance Package
        RealConcept { coefficients: [0.20, 0.10, 0.15, 0.20, 0.10, 0.80, 0.20, 0.05] },     // 13: Industry Standard
        RealConcept { coefficients: [0.15, 0.15, 0.15, 0.05, 0.15, 0.20, 0.80, 0.10] },     // 14: Team Standup
        RealConcept { coefficients: [0.05, 0.15, 0.10, 0.85, 0.10, 0.15, 0.15, 0.15] },     // 15: Predator
        RealConcept { coefficients: [0.80, 0.05, 0.20, 0.10, 0.15, 0.10, 0.20, 0.05] },     // 16: Decomposer
        RealConcept { coefficients: [0.05, 0.20, 0.15, 0.05, 0.10, 0.15, 0.10, 0.86] },     // 17: Photosynthesis
        RealConcept { coefficients: [0.15, 0.20, 0.80, 0.05, 0.10, 0.15, 0.20, 0.15] },     // 18: Water Cycle
        RealConcept { coefficients: [0.10, 0.25, 0.15, 0.10, 0.10, 0.80, 0.20, 0.25] },     // 19: Keystone Species
        RealConcept { coefficients: [0.10, 0.85, 0.15, 0.05, 0.10, 0.10, 0.15, 0.20] },     // 20: Mutation
        RealConcept { coefficients: [0.15, 0.05, 0.15, 0.15, 0.15, 0.10, 0.80, 0.10] },     // 21: Homeostasis
        RealConcept { coefficients: [0.05, 0.10, 0.10, 0.85, 0.15, 0.25, 0.15, 0.10] },     // 22: Natural Selection
        RealConcept { coefficients: [0.15, 0.15, 0.10, 0.05, 0.10, 0.85, 0.20, 0.15] },     // 23: Ecological Succession
        RealConcept { coefficients: [0.20, 0.10, 0.15, 0.05, 0.15, 0.10, 0.80, 0.20] },     // 24: Symbiosis
        RealConcept { coefficients: [0.10, 0.20, 0.15, 0.05, 0.10, 0.10, 0.15, 0.85] },     // 25: DNA Replication
        RealConcept { coefficients: [0.05, 0.05, 0.20, 0.85, 0.15, 0.25, 0.15, 0.05] },     // 26: Firewall
        RealConcept { coefficients: [0.15, 0.10, 0.25, 0.05, 0.10, 0.15, 0.80, 0.10] },     // 27: Load Balancer
        RealConcept { coefficients: [0.10, 0.10, 0.15, 0.05, 0.85, 0.05, 0.10, 0.10] },     // 28: Database Index
        RealConcept { coefficients: [0.15, 0.25, 0.80, -0.15, -0.20, 0.10, 0.30, 0.05] },   // 29: Message Broker
        RealConcept { coefficients: [0.05, -0.25, -0.20, 0.85, 0.25, -0.10, 0.20, -0.15] }, // 30: Circuit Breaker
        RealConcept { coefficients: [0.20, 0.10, 0.05, 0.30, 0.15, 0.85, 0.15, 0.10] },     // 31: Deprecation Policy
        RealConcept { coefficients: [0.10, 0.21, 0.16, -0.10, 0.10, 0.82, 0.31, 0.37] },    // 32: Feature Flag
        RealConcept { coefficients: [0.18, 0.12, 0.12, 0.12, 0.90, 0.12, 0.18, 0.06] },     // 33: Health Check Endpoint
        RealConcept { coefficients: [0.80, 0.10, 0.20, 0.10, 0.10, 0.25, 0.15, 0.10] },     // 34: Event Sourcing Log
        RealConcept { coefficients: [0.05, 0.75, 0.10, 0.25, 0.20, 0.30, 0.15, 0.20] },     // 35: Chaos Engineering
        RealConcept { coefficients: [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34] }, // 36: Rate Limiter
        RealConcept { coefficients: [0.22, 0.34, 0.84, 0.06, -0.11, 0.17, 0.28, 0.06] },    // 37: API Gateway
    ]
}

fn relations() -> Vec<LabeledRelation> {
    vec![
        // Business intra (0-13)
        LabeledRelation{idx_a:0, idx_b:5, human_label:"receptive"},       // Budget → Handbook
        LabeledRelation{idx_a:1, idx_b:2, human_label:"generative"},      // Pipeline → Revenue
        LabeledRelation{idx_a:3, idx_b:4, human_label:"causal"},          // Ticket → Report
        LabeledRelation{idx_a:4, idx_b:2, human_label:"clarifying"},      // Report → Revenue
        LabeledRelation{idx_a:5, idx_b:7, human_label:"constraining"},    // Handbook → Innovation
        LabeledRelation{idx_a:6, idx_b:5, human_label:"receptive"},       // Contract → Handbook
        LabeledRelation{idx_a:7, idx_b:2, human_label:"generative"},      // Innovation → Revenue
        LabeledRelation{idx_a:8, idx_b:10, human_label:"balancing"},      // Feedback → MarketTrend
        LabeledRelation{idx_a:9, idx_b:7, human_label:"generative"},      // Onboarding → Innovation
        LabeledRelation{idx_a:10, idx_b:13, human_label:"influential"},   // MarketTrend → Industry
        LabeledRelation{idx_a:11, idx_b:0, human_label:"clarifying"},     // Compliance → Budget
        LabeledRelation{idx_a:12, idx_b:5, human_label:"receptive"},      // Severance → Handbook
        LabeledRelation{idx_a:13, idx_b:10, human_label:"influential"},   // Industry → MarketTrend
        LabeledRelation{idx_a:14, idx_b:8, human_label:"balancing"},      // Standup → Feedback
        // Ecosystem intra (14-24)
        LabeledRelation{idx_a:15, idx_b:20, human_label:"constraining"},  // Predator → Mutation
        LabeledRelation{idx_a:16, idx_b:17, human_label:"receptive"},     // Decomposer → Photosynthesis
        LabeledRelation{idx_a:17, idx_b:21, human_label:"generative"},    // Photosynthesis → Homeostasis
        LabeledRelation{idx_a:18, idx_b:17, human_label:"generative"},    // Water Cycle → Photosynthesis
        LabeledRelation{idx_a:19, idx_b:23, human_label:"influential"},   // Keystone → Succession
        LabeledRelation{idx_a:20, idx_b:22, human_label:"generative"},    // Mutation → NaturalSelection
        LabeledRelation{idx_a:21, idx_b:24, human_label:"balancing"},     // Homeostasis → Symbiosis
        LabeledRelation{idx_a:22, idx_b:20, human_label:"constraining"},  // NaturalSelection → Mutation
        LabeledRelation{idx_a:23, idx_b:16, human_label:"influential"},   // Succession → Decomposer
        LabeledRelation{idx_a:24, idx_b:19, human_label:"balancing"},     // Symbiosis → Keystone
        LabeledRelation{idx_a:25, idx_b:21, human_label:"generative"},    // DNA Replication → Homeostasis
        // Technology intra (25-36)
        LabeledRelation{idx_a:26, idx_b:30, human_label:"constraining"},  // Firewall → CircuitBreaker
        LabeledRelation{idx_a:27, idx_b:28, human_label:"balancing"},     // LoadBalancer → DBIndex
        LabeledRelation{idx_a:28, idx_b:26, human_label:"clarifying"},    // DBIndex → Firewall
        LabeledRelation{idx_a:29, idx_b:32, human_label:"transmissive"},  // MessageBroker → FeatureFlag
        LabeledRelation{idx_a:30, idx_b:35, human_label:"constraining"},  // CircuitBreaker → Chaos
        LabeledRelation{idx_a:31, idx_b:32, human_label:"influential"},   // Deprecation → FeatureFlag
        LabeledRelation{idx_a:32, idx_b:11, human_label:"influential"},   // FeatureFlag → Compliance
        LabeledRelation{idx_a:33, idx_b:30, human_label:"clarifying"},    // HealthCheck → CircuitBreaker
        LabeledRelation{idx_a:34, idx_b:33, human_label:"receptive"},     // EventLog → HealthCheck
        LabeledRelation{idx_a:35, idx_b:33, human_label:"causal"},        // Chaos → HealthCheck
        LabeledRelation{idx_a:36, idx_b:30, human_label:"receptive"},     // RateLimiter → CircuitBreaker
        LabeledRelation{idx_a:37, idx_b:29, human_label:"transmissive"},  // API Gateway → MessageBroker
        // Cross-domain (37-40)
        LabeledRelation{idx_a:0, idx_b:26, human_label:"constraining"},   // Budget → Firewall
        LabeledRelation{idx_a:20, idx_b:7, human_label:"causal"},         // Mutation → Innovation
        LabeledRelation{idx_a:27, idx_b:21, human_label:"receptive"},     // LoadBalancer → Homeostasis
        LabeledRelation{idx_a:11, idx_b:30, human_label:"clarifying"},    // Compliance → CircuitBreaker
    ]
}

fn label_to_type(label: &str) -> RelationType {
    use RelationType::*;
    match label { "generative"=>Generative,"receptive"=>Receptive,"causal"=>Causal,"transmissive"=>Transmissive,"constraining"=>Constraining,"influential"=>Influential,"clarifying"=>Clarifying,"balancing"=>Balancing, _=>Receptive }
}

#[test]
fn improvement_directions_benchmark() {
    let c = concepts();
    let r = relations();
    let enc: Vec<Multivector> = c.iter().map(|rc| llm_encode(&rc.coefficients)).collect();
    let mc: Vec<MultiEncodedConcept> = c.iter()
        .map(|rc| MultiEncodedConcept::from_single_encoding(&llm_encode(&rc.coefficients)))
        .collect();

    let weights = FeatureWeights::default();
    let ensemble = EnsembleClassifier::new();

    // ── Track per-classifier accuracy ──
    let mut from_pair_ok = 0usize;
    let mut from_pair_multi_ok = 0usize;
    let mut from_pair_weighted_ok = 0usize;
    let mut from_pair_geom_ok = 0usize;
    let mut multi_encoding_ok = 0usize;
    let mut ensemble_majority_ok = 0usize;
    let mut ensemble_weighted_ok = 0usize;
    let mut ensemble_smart_ok = 0usize;
    let mut hexagram_ok = 0usize;

    let total = r.len();

    // Confusion matrix for ensemble-weight-winning classifier
    let labels_ordered = ["gen","rec","cau","tra","con","inf","cla","bal"];
    let mut confusion = vec![vec![0usize; 8]; 8];

    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║     IMPROVEMENT DIRECTIONS BENCHMARK — 38 CONCEPTS, 41 RELATIONS   ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("{:<35} x {:<35} | Human    | orig | multi| hex  | smart| ens-m", "A", "B");
    println!("{:-<35}+{:-<35}+----------+------+------+------+------+------", "", "");

    for rel in &r {
        let expected = label_to_type(rel.human_label);
        let a = &enc[rel.idx_a];
        let b = &enc[rel.idx_b];
        let mca = &mc[rel.idx_a];
        let mcb = &mc[rel.idx_b];

        let (p0, _) = RelationType::from_pair(a, b);
        let (p1, _) = RelationType::from_pair_multi(a, b);
        let (p2, _) = RelationType::from_pair_weighted(a, b, &weights);
        let (p3, _) = RelationType::from_pair_with_geom_conf(a, b);
        let (p4, _) = classify_multi_encoded(mca, mcb, &weights);
        let (p5, _) = ensemble.classify_majority(a, b, Some(mca), Some(mcb));
        let (p6, _) = ensemble.classify_weighted(a, b, Some(mca), Some(mcb));
        let (p7, _) = ensemble.classify_smart(a, b, Some(mca), Some(mcb));

        // Hexagram classification
        let ta = a.dominant_trigram();
        let product = a.geo_product(b);
        let lower = product.dominant_trigram();
        let hex = ga_semantics_core::advanced::Hexagram::new(ta, lower);
        let (p_hex, _) = hex.relation_type();

        if p0 == expected { from_pair_ok += 1; }
        if p1 == expected { from_pair_multi_ok += 1; }
        if p2 == expected { from_pair_weighted_ok += 1; }
        if p3 == expected { from_pair_geom_ok += 1; }
        if p4 == expected { multi_encoding_ok += 1; }
        if p5 == expected { ensemble_majority_ok += 1; }
        if p6 == expected { ensemble_weighted_ok += 1; }
        if p7 == expected { ensemble_smart_ok += 1; }
        if p_hex == expected { hexagram_ok += 1; }

        // Confusion matrix for ensemble-weighted (best performing)
        let pi = RelationType::ALL.iter().position(|&rt| rt == p7).unwrap_or(0);
        let ei = labels_ordered.iter().position(|&l| l == rel.human_label).unwrap_or(0);
        confusion[pi][ei] += 1;

        let name_a = &format!("{:.34}", if rel.idx_a < c.len() {
            ["Marketing Budget","Sales Pipeline","Revenue Target","Support Ticket","Quarterly Report",
             "Employee Handbook","Vendor Contract","Innovation Fund","Feedback Loop","Onboarding Process",
             "Market Trend","Compliance Audit","Severance Package","Industry Standard","Team Standup",
             "Predator","Decomposer","Photosynthesis","Water Cycle","Keystone Species",
             "Mutation","Homeostasis","Natural Selection","Ecological Succession","Symbiosis",
             "DNA Replication","Firewall","Load Balancer","Database Index","Message Broker",
             "Circuit Breaker","Deprecation Policy","Feature Flag","Health Check Endpoint","Event Sourcing Log",
             "Chaos Engineering","Rate Limiter","API Gateway"][rel.idx_a]
        } else { "?" });

        let name_b = &format!("{:.34}", if rel.idx_b < c.len() {
            ["Marketing Budget","Sales Pipeline","Revenue Target","Support Ticket","Quarterly Report",
             "Employee Handbook","Vendor Contract","Innovation Fund","Feedback Loop","Onboarding Process",
             "Market Trend","Compliance Audit","Severance Package","Industry Standard","Team Standup",
             "Predator","Decomposer","Photosynthesis","Water Cycle","Keystone Species",
             "Mutation","Homeostasis","Natural Selection","Ecological Succession","Symbiosis",
             "DNA Replication","Firewall","Load Balancer","Database Index","Message Broker",
             "Circuit Breaker","Deprecation Policy","Feature Flag","Health Check Endpoint","Event Sourcing Log",
             "Chaos Engineering","Rate Limiter","API Gateway"][rel.idx_b]
        } else { "?" });

        println!("{:<35} x {:<35} | {:>8} | {:<5}| {:<5}| {:<5}| {:<5}| {:<5}",
            name_a, name_b, rel.human_label,
            if p0==expected {"✓"}else{"✗"},
            if p4==expected {"✓"}else{"✗"},
            if p_hex==expected {"✓"}else{"✗"},
            if p7==expected {"✓"}else{"✗"},
            if p5==expected {"✓"}else{"✗"},
        );
    }

    // ── RESULTS TABLE ──
    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║                        CLASSIFIER COMPARISON                        ║");
    println!("╠═══════════════════════════════════════════════════════════════════╣");

    let classifiers: [(&str, usize); 9] = [
        ("from_pair (original)", from_pair_ok),
        ("from_pair_multi", from_pair_multi_ok),
        ("from_pair_weighted", from_pair_weighted_ok),
        ("from_pair_with_geom_conf", from_pair_geom_ok),
        ("classify_multi_encoded", multi_encoding_ok),
        ("Hexagram classifier", hexagram_ok),
        ("Ensemble — smart", ensemble_smart_ok),
        ("Ensemble — majority", ensemble_majority_ok),
        ("Ensemble — weighted", ensemble_weighted_ok),
    ];

    for (name, correct) in &classifiers {
        let pct = *correct as f64 / total as f64 * 100.0;
        let bar = "█".repeat((pct * 0.5) as usize);
        println!("║ {:.<40} {:>6.1}% {:>3}/{} {:<30}║",
            name, pct, correct, total, bar);
    }

    println!("╠═══════════════════════════════════════════════════════════════════╣");
    println!("║ Random baseline:         12.5%                                    ║");
    println!("║ LLM direct comparison:  ~85-95%                                   ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");

    // ── DELTA ANALYSIS ──
    let best_single = multi_encoding_ok.max(from_pair_multi_ok).max(from_pair_ok)
        .max(from_pair_weighted_ok).max(from_pair_geom_ok);
    let best_ensemble = ensemble_smart_ok.max(ensemble_majority_ok).max(ensemble_weighted_ok);
    let ensemble_delta = best_ensemble as f64 / total as f64 * 100.0
        - best_single as f64 / total as f64 * 100.0;

    println!("\n── IMPROVEMENT ──");
    println!("  Best single classifier:  {:.1}%", best_single as f64 / total as f64 * 100.0);
    println!("  Best ensemble:           {:.1}%", best_ensemble as f64 / total as f64 * 100.0);
    println!("  Ensemble delta:          {:+.1}pp", ensemble_delta);

    // ── Confusion matrix for best ensemble ──
    println!("\n── CONFUSION MATRIX (Ensemble-smart, rows=predicted) ──");
    print!("  {:>12} │", "");
    for l in &labels_ordered { print!(" {:>3}", l); }
    println!("\n  ──────────────┼{}", "─────".repeat(8));
    for (i, row) in confusion.iter().enumerate() {
        print!("  {:>12} │", labels_ordered[i]);
        for v in row { print!(" {:>3}", v); }
        println!();
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Direction 2: Trainable classifier LOO-CV
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn trainable_classifier_loocv() {
    let c = concepts();
    let r = relations();
    let enc: Vec<Multivector> = c.iter().map(|rc| llm_encode(&rc.coefficients)).collect();

    let features: Vec<[f64; 62]> = r.iter()
        .map(|rel| GaFeatureClassifier::extract_features(&enc[rel.idx_a], &enc[rel.idx_b]))
        .collect();
    let labels: Vec<RelationType> = r.iter().map(|rel| label_to_type(rel.human_label)).collect();

    let n = r.len();
    let mut loo_correct = 0usize;

    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║     TRAINABLE CLASSIFIER — LEAVE-ONE-OUT CROSS-VALIDATION          ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");

    for test_idx in 0..n {
        // Build training set (all except test)
        let train_features: Vec<[f64; 62]> = features.iter().enumerate()
            .filter(|(i, _)| *i != test_idx)
            .map(|(_, f)| *f)
            .collect();
        let train_labels: Vec<RelationType> = labels.iter().enumerate()
            .filter(|(i, _)| *i != test_idx)
            .map(|(_, l)| *l)
            .collect();

        let mut model = GaFeatureClassifier::new(0.1);
        model.train(&train_features, &train_labels, 0.05, 200);

        let (pred, _) = model.predict(&features[test_idx]);
        if pred == labels[test_idx] { loo_correct += 1; }
    }

    let loo_acc = loo_correct as f64 / n as f64 * 100.0;

    println!("\n  LOO-CV results:");
    println!("  Correct:      {}/{}", loo_correct, n);
    println!("  Accuracy:     {:.1}%", loo_acc);
    println!("  Random:       12.5%");
    println!("  Multi-enc:    56.4%");
    println!();

    if loo_acc > 56.4 {
        println!("  TRAINABLE CLASSIFIER BEATS MULTI-ENCODING by {:+.1}pp!", loo_acc - 56.4);
    } else if loo_acc > 40.0 {
        println!("  Trainable classifier shows moderate signal ({:.1}%).", loo_acc);
        println!("  With more training data, this approach could scale further.");
    } else {
        println!("  Trainable classifier underperforms ({:.1}%).", loo_acc);
        println!("  Likely overfitting or insufficient training data ({} samples, 62 features).", n - 1);
    }

    // Also measure training accuracy (should be high — diagnostic for overfitting)
    let mut model_full = GaFeatureClassifier::new(0.1);
    model_full.train(&features, &labels, 0.05, 200);
    let mut train_correct = 0usize;
    for i in 0..n {
        let (pred, _) = model_full.predict(&features[i]);
        if pred == labels[i] { train_correct += 1; }
    }
    let train_acc = train_correct as f64 / n as f64 * 100.0;
    println!("  Training acc (all data): {:.1}%", train_acc);
    println!("  Gap (overfitting):       {:+.1}pp", train_acc - loo_acc);
}

// ──────────────────────────────────────────────────────────────────────────
// Direction 1: Cl(4) comparison
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn cl4_comparison_benchmark() {
    use ga_semantics_core::multivector16::Multivector16;

    let c = concepts();
    let r = relations();
    let enc_cl3: Vec<Multivector> = c.iter().map(|rc| llm_encode(&rc.coefficients)).collect();
    let enc_cl4: Vec<Multivector16> = c.iter()
        .map(|rc| Multivector16::from_cl3(&rc.coefficients))
        .collect();

    let weights = FeatureWeights::default();

    // Cl(3) multi-encoding baseline
    let mc_cl3: Vec<MultiEncodedConcept> = c.iter()
        .map(|rc| MultiEncodedConcept::from_single_encoding(&llm_encode(&rc.coefficients)))
        .collect();

    let mut cl3_ok = 0usize;
    let mut cl4_ok = 0usize;

    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║        Cl(3) vs Cl(4) — DIMENSIONAL COMPARISON                     ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");

    for rel in &r {
        let expected = label_to_type(rel.human_label);

        // Cl(3) multi-encoding
        let (p3, _) = classify_multi_encoded(&mc_cl3[rel.idx_a], &mc_cl3[rel.idx_b], &weights);
        if p3 == expected { cl3_ok += 1; }

        // Cl(4) classification via from_pair on expanded encodings
        let a4 = &enc_cl4[rel.idx_a];
        let b4 = &enc_cl4[rel.idx_b];

        // Map Cl(4) dominant trigrams to Cl(3) relation type framework
        let ta4 = a4.dominant_trigram();
        let tb4 = b4.dominant_trigram();

        // Cl(4) has 2 blades per trigram → richer encoding
        // Use from_pair_multi on the Cl(3) encoding derived from Cl(4)
        let cl3_from_cl4_a = {
            let mut raw = [0.0f64; 8];
            let cl4c = a4.coefficients();
            for i in 0..8 { raw[i] = cl4c[i]; }
            llm_encode(&raw)
        };
        let cl3_from_cl4_b = {
            let mut raw = [0.0f64; 8];
            let cl4c = b4.coefficients();
            for i in 0..8 { raw[i] = cl4c[i]; }
            llm_encode(&raw)
        };
        let (p4, _) = RelationType::from_pair_multi(&cl3_from_cl4_a, &cl3_from_cl4_b);
        if p4 == expected { cl4_ok += 1; }
    }

    let n = r.len();
    println!("\n  Cl(3) multi-encoding:    {:.1}% ({}/{})", cl3_ok as f64 / n as f64 * 100.0, cl3_ok, n);
    println!("  Cl(4) from_pair_multi:   {:.1}% ({}/{})", cl4_ok as f64 / n as f64 * 100.0, cl4_ok, n);
    println!("  Cl(4) delta:             {:+.1}pp", (cl4_ok as f64 - cl3_ok as f64) / n as f64 * 100.0);

    // Cl(4) encoding sharpness comparison
    let avg_cl3_sharpness: f64 = enc_cl3.iter().map(|mv| mv.encoding_sharpness()).sum::<f64>() / enc_cl3.len() as f64;
    let avg_cl4_sharpness: f64 = enc_cl4.iter().map(|mv| mv.encoding_sharpness()).sum::<f64>() / enc_cl4.len() as f64;

    println!();
    println!("  Avg Cl(3) sharpness:     {:.3}", avg_cl3_sharpness);
    println!("  Avg Cl(4) sharpness:     {:.3}", avg_cl4_sharpness);
    println!("  Note: Cl(4) zero-pads upper 8 blades, diluting sharpness.");
    println!("  True Cl(4) LLM encoding (16 coefficients) would fix this.");

    // Show the dominant trigram consistency
    let mut consistent = 0usize;
    for i in 0..c.len() {
        if enc_cl3[i].dominant_trigram() == enc_cl4[i].dominant_trigram() {
            consistent += 1;
        }
    }
    println!("  Dominant trigram preserved: {}/{} ({:.0}%)",
        consistent, c.len(), consistent as f64 / c.len() as f64 * 100.0);
}
