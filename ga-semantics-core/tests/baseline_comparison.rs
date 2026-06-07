use ga_semantics_core::prelude::*;
use ga_semantics_core::RelationType;

#[derive(Debug)]
struct ConceptPair {
    name_a: &'static str,
    name_b: &'static str,
    expected_relation: &'static str,
    coeffs_a: [f64; 8],
    coeffs_b: [f64; 8],
}

fn benchmark_pairs() -> Vec<ConceptPair> {
    vec![
        ConceptPair {
            name_a: "Rate Limiter", name_b: "Circuit Breaker",
            expected_relation: "receptive",
            coeffs_a: [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34],
            coeffs_b: [0.05, -0.30, -0.20, 0.75, 0.25, -0.10, 0.25, -0.20],
        },
        ConceptPair {
            name_a: "Message Queue", name_b: "Cache Layer",
            expected_relation: "receptive",
            coeffs_a: [0.15, 0.25, 0.81, -0.20, -0.25, 0.10, 0.36, 0.05],
            coeffs_b: [0.30, 0.10, 0.60, -0.25, -0.30, 0.15, 0.35, 0.10],
        },
        ConceptPair {
            name_a: "Background Job Scheduler", name_b: "Event Stream Processor",
            expected_relation: "generative",
            coeffs_a: [0.15, 0.55, 0.30, -0.15, 0.10, -0.10, 0.25, 0.60],
            coeffs_b: [0.10, 0.30, 0.60, -0.10, 0.15, 0.30, 0.15, 0.45],
        },
        ConceptPair {
            name_a: "Black Box Module", name_b: "Innovation Lab",
            expected_relation: "constraining",
            coeffs_a: [0.05, -0.10, -0.30, -0.05, -0.70, -0.65, -0.15, 0.05],
            coeffs_b: [-0.30, 0.25, 0.15, -0.55, 0.15, 0.30, 0.10, 0.85],
        },
        ConceptPair {
            name_a: "Compliance Validator", name_b: "Rate Limiter",
            expected_relation: "receptive",
            coeffs_a: [0.20, 0.05, -0.15, 0.80, 0.35, 0.10, -0.05, -0.05],
            coeffs_b: [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34],
        },
        ConceptPair {
            name_a: "Peer-to-Peer Network", name_b: "API Gateway",
            expected_relation: "generative",
            coeffs_a: [0.05, 0.10, 0.35, -0.25, 0.15, 0.25, 0.80, 0.15],
            coeffs_b: [0.20, 0.30, 0.65, 0.05, -0.10, 0.15, 0.25, 0.05],
        },
        ConceptPair {
            name_a: "Data Warehouse", name_b: "Event Stream Processor",
            expected_relation: "constraining",
            coeffs_a: [0.50, 0.05, 0.15, 0.20, 0.45, 0.10, -0.05, -0.15],
            coeffs_b: [0.10, 0.30, 0.60, -0.10, 0.15, 0.30, 0.15, 0.45],
        },
        ConceptPair {
            name_a: "Innovation Lab", name_b: "Background Job Scheduler",
            expected_relation: "receptive",
            coeffs_a: [-0.30, 0.25, 0.15, -0.55, 0.15, 0.30, 0.10, 0.85],
            coeffs_b: [0.15, 0.55, 0.30, -0.15, 0.10, -0.10, 0.25, 0.60],
        },
        ConceptPair {
            name_a: "Feature Flag", name_b: "Notification Service",
            expected_relation: "receptive",
            coeffs_a: [0.10, 0.20, 0.20, -0.10, 0.10, 0.78, 0.35, 0.40],
            coeffs_b: [0.10, 0.40, 0.45, -0.05, 0.30, 0.50, 0.10, 0.10],
        },
        ConceptPair {
            name_a: "Load Balancer", name_b: "Circuit Breaker",
            expected_relation: "receptive",
            coeffs_a: [0.30, -0.10, 0.45, -0.05, 0.10, 0.15, 0.80, 0.10],
            coeffs_b: [0.05, -0.30, -0.20, 0.75, 0.25, -0.10, 0.25, -0.20],
        },
        // Additional pairs for statistical significance
        ConceptPair {
            name_a: "Logging System", name_b: "Monitoring Dashboard",
            expected_relation: "receptive",
            coeffs_a: [0.15, 0.05, 0.10, 0.30, 0.85, 0.05, 0.25, -0.15],
            coeffs_b: [0.20, 0.25, 0.05, 0.15, 0.75, 0.10, 0.30, -0.05],
        },
        ConceptPair {
            name_a: "Database Transaction", name_b: "Auth System",
            expected_relation: "receptive",
            coeffs_a: [0.28, 0.05, 0.14, 0.79, 0.32, 0.18, 0.37, 0.09],
            coeffs_b: [0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20],
        },
        ConceptPair {
            name_a: "Config Store", name_b: "Data Warehouse",
            expected_relation: "receptive",
            coeffs_a: [0.65, 0.05, 0.10, 0.15, 0.30, 0.15, 0.10, 0.00],
            coeffs_b: [0.50, 0.05, 0.15, 0.20, 0.45, 0.10, -0.05, -0.15],
        },
        ConceptPair {
            name_a: "P2P Network", name_b: "Load Balancer",
            expected_relation: "receptive",
            coeffs_a: [0.05, 0.10, 0.35, -0.25, 0.15, 0.25, 0.80, 0.15],
            coeffs_b: [0.30, -0.10, 0.45, -0.05, 0.10, 0.15, 0.80, 0.10],
        },
        ConceptPair {
            name_a: "Auth System", name_b: "Rate Limiter",
            expected_relation: "receptive",
            coeffs_a: [0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20],
            coeffs_b: [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34],
        },
    ]
}

fn cosine_similarity(a: &[f64; 8], b: &[f64; 8]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let nb: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if na < f64::EPSILON || nb < f64::EPSILON { return 0.0; }
    dot / (na * nb)
}

fn euclidean_distance(a: &[f64; 8], b: &[f64; 8]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
}

fn random_relation_label(seed: u64) -> &'static str {
    let labels = ["generative", "receptive", "causal", "transmissive", "constraining", "influential", "clarifying", "balancing"];
    labels[(seed % 8) as usize]
}

fn baseline_comparison_report() {
    let pairs = benchmark_pairs();
    let all_relations: Vec<&str> = pairs.iter().map(|p| p.expected_relation).collect();
    let random_baseline = 1.0 / 8.0; // 8 classes → 12.5% random chance
    let majority_baseline = {
        let mut counts = std::collections::HashMap::new();
        for r in &all_relations { *counts.entry(*r).or_insert(0) += 1; }
        *counts.values().max().unwrap_or(&0) as f64 / pairs.len() as f64
    };

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║           BASELINE COMPARISON BENCHMARK                          ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  GA-Bagua vs Random vs Cosine vs Euclidean on relation class    ║");
    println!("║  {:>2} concept pairs across software architecture domain          ║", pairs.len());
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // 1. GA-Bagua classification
    let mut bagua_correct = 0usize;
    let mut bagua_conf = 0.0f64;
    for p in &pairs {
        let a = llm_encode(&p.coeffs_a);
        let b = llm_encode(&p.coeffs_b);
        let (rel, conf) = RelationType::from_pair(&a, &b);
        bagua_conf += conf;
        if rel.role_name() == p.expected_relation { bagua_correct += 1; }
    }
    let bagua_acc = bagua_correct as f64 / pairs.len() as f64;
    let bagua_avg_conf = bagua_conf / pairs.len() as f64;

    // 2. Cosine similarity threshold classifier
    // For "receptive" pairs (same role, high cosine), classify as receptive if cos > 0.8
    let mut cos_correct = 0usize;
    for p in &pairs {
        let cos = cosine_similarity(&p.coeffs_a, &p.coeffs_b);
        let guess = if cos > 0.7 { "receptive" } else { "generative" };
        if guess == p.expected_relation { cos_correct += 1; }
    }
    let cos_acc = cos_correct as f64 / pairs.len() as f64;

    // 3. Euclidean distance threshold
    let mut euc_correct = 0usize;
    for p in &pairs {
        let dist = euclidean_distance(&p.coeffs_a, &p.coeffs_b);
        let guess = if dist < 0.6 { "receptive" } else { "constraining" };
        if guess == p.expected_relation { euc_correct += 1; }
    }
    let euc_acc = euc_correct as f64 / pairs.len() as f64;

    // 4. Retrieval quality comparison
    // Given a query concept, can each method retrieve same-role peers?
    let all_concepts: Vec<([f64; 8], &str)> = pairs.iter()
        .flat_map(|p| vec![(p.coeffs_a, p.expected_relation), (p.coeffs_b, p.expected_relation)])
        .collect();

    let mut bagua_prec = 0.0f64;
    let mut cos_prec = 0.0f64;
    let mut euc_prec = 0.0f64;
    let mut query_count = 0usize;

    for (qi, (qcoeffs, qrole)) in all_concepts.iter().enumerate() {
        let qmv = llm_encode(qcoeffs);
        let peers: Vec<_> = all_concepts.iter().enumerate()
            .filter(|(i, (_, r))| *i != qi && *r == *qrole)
            .collect();
        if peers.len() < 2 { continue; }
        query_count += 1;
        let k = 3usize.min(peers.len());

        // GA-Bagua dominant_similarity retrieval
        {
            let mut scored: Vec<(usize, f64)> = all_concepts.iter().enumerate()
                .filter(|(i, _)| *i != qi)
                .map(|(i, (c, _))| (i, dominant_similarity(&qmv, &llm_encode(c))))
                .collect();
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let hits = scored.iter().take(k).filter(|(i, _)| all_concepts[*i].1 == *qrole).count();
            bagua_prec += hits as f64 / k as f64;
        }

        // Cosine similarity retrieval
        {
            let mut scored: Vec<(usize, f64)> = all_concepts.iter().enumerate()
                .filter(|(i, _)| *i != qi)
                .map(|(i, (c, _))| (i, cosine_similarity(qcoeffs, c)))
                .collect();
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let hits = scored.iter().take(k).filter(|(i, _)| all_concepts[*i].1 == *qrole).count();
            cos_prec += hits as f64 / k as f64;
        }

        // Euclidean retrieval
        {
            let mut scored: Vec<(usize, f64)> = all_concepts.iter().enumerate()
                .filter(|(i, _)| *i != qi)
                .map(|(i, (c, _))| (i, -euclidean_distance(qcoeffs, c)))
                .collect();
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let hits = scored.iter().take(k).filter(|(i, _)| all_concepts[*i].1 == *qrole).count();
            euc_prec += hits as f64 / k as f64;
        }
    }

    let bagua_prec = bagua_prec / query_count as f64;
    let cos_prec = cos_prec / query_count as f64;
    let euc_prec = euc_prec / query_count as f64;

    // Print results
    println!("  ── RELATION CLASSIFICATION ACCURACY ──");
    println!("  Method                   │ Accuracy │ Mean Confidence");
    println!("  ─────────────────────────┼──────────┼────────────────");
    println!("  GA-Bagua (WuXing cycle)  │ {:>6.1}%  │ {:.3}", bagua_acc * 100.0, bagua_avg_conf);
    println!("  Cosine (threshold 0.7)   │ {:>6.1}%  │ N/A (distance metric)", cos_acc * 100.0);
    println!("  Euclidean (threshold 0.6)│ {:>6.1}%  │ N/A (distance metric)", euc_acc * 100.0);
    println!("  Random (uniform 8-class) │ {:>7.1}%  │ N/A", random_baseline * 100.0);
    println!("  Majority class           │ {:>7.1}%  │ N/A", majority_baseline * 100.0);

    println!("\n  ── RETRIEVAL PRECISION@3 ──");
    println!("  Method                │ Precision@3 │ Same-role peers found?");
    println!("  ──────────────────────┼─────────────┼────────────────────────");
    println!("  GA-Bagua              │ {:>9.1}%  │ dominant_similarity()", bagua_prec * 100.0);
    println!("  Cosine similarity     │ {:>9.1}%  │ Standard dot product", cos_prec * 100.0);
    println!("  Euclidean distance    │ {:>9.1}%  │ Inverse L2 distance", euc_prec * 100.0);

    // 5. Pairwise breakdown
    println!("\n  ── PAIRWISE CLASSIFICATION ──");
    println!("  {:<25} x {:<25} | Expected   | GA-Bagua  | Cosine   | Euc", "Concept A", "Concept B");
    println!("  {:-<25}-+-{:-<25}-+------------+------------+----------+------", "", "");
    for p in &pairs {
        let a = llm_encode(&p.coeffs_a);
        let b = llm_encode(&p.coeffs_b);
        let (bagua_rel, _) = RelationType::from_pair(&a, &b);

        let cos = cosine_similarity(&p.coeffs_a, &p.coeffs_b);
        let cos_rel = if cos > 0.7 { "receptive" } else { "generative" };

        let dist = euclidean_distance(&p.coeffs_a, &p.coeffs_b);
        let euc_rel = if dist < 0.6 { "receptive" } else { "constraining" };

        let b_ok = bagua_rel.role_name() == p.expected_relation;
        let c_ok = cos_rel == p.expected_relation;
        let e_ok = euc_rel == p.expected_relation;

        println!("  {:>25} ⊗ {:>25} │ {:>8} │ {:>8} {} │ {:>6} {} │ {:>4} {}",
            format!("{}", p.name_a), format!("{}", p.name_b),
            p.expected_relation,
            bagua_rel.role_name(), if b_ok { "✓" } else { "✗" },
            cos_rel, if c_ok { "✓" } else { "✗" },
            euc_rel, if e_ok { "✓" } else { "✗" },
        );
    }

    // 6. Win/Loss summary
    println!("\n  ── SUMMARY ──");
    let total = pairs.len();
    println!("  GA-Bagua:     {:>3}/{} correct ({:.0}%)  — WuXing cycle + hexagram fallback",
        bagua_correct, total, bagua_acc * 100.0);
    println!("  Cosine:       {:>3}/{} correct ({:.0}%)  — Simple threshold guesses", cos_correct, total, cos_acc * 100.0);
    println!("  Euclidean:    {:>3}/{} correct ({:.0}%)  — Simple threshold guesses", euc_correct, total, euc_acc * 100.0);
    println!("  Random:       ~{:>2}/{} expected ({:.0}%)", (total as f64 * random_baseline) as usize, total, random_baseline * 100.0);

    println!("\n  GA-Bagua provides interpretable relation LABELS (not distances).");
    println!("  Cosine/Euclidean only measure similarity; they can't classify.");
    println!("  WuXing cycle gives deterministic, zero-training relation semantics.");
}

#[test]
fn baseline_comparison() {
    baseline_comparison_report();
}
