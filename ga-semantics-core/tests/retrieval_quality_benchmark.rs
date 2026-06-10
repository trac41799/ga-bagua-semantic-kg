// ─────────────────────────────────────────────────────────────────────
// RETRIEVAL QUALITY & BASELINE COMPARISON BENCHMARK
// ─────────────────────────────────────────────────────────────────────

use ga_semantics_core::prelude::*;

#[derive(Debug, Clone)]
struct RConcept {
    name: &'static str,
    domain: &'static str,
    coefficients: [f64; 8],
}

fn retrieval_concepts() -> Vec<RConcept> {
    vec![
        RConcept { name: "Rate Limiter", domain: "software", coefficients: [0.05, -0.10, -0.50, 0.70, 0.20, -0.15, 0.15, -0.25] },
        RConcept { name: "Message Queue", domain: "software", coefficients: [0.10, 0.20, 0.82, -0.15, -0.20, 0.10, 0.30, 0.05] },
        RConcept { name: "Database Index", domain: "software", coefficients: [0.10, 0.10, 0.30, 0.10, 0.15, 0.80, 0.10, 0.10] },
        RConcept { name: "Load Balancer", domain: "software", coefficients: [0.20, -0.05, 0.35, 0.05, 0.10, 0.15, 0.78, 0.10] },
        RConcept { name: "Circuit Breaker", domain: "software", coefficients: [0.05, -0.25, -0.15, 0.78, 0.20, -0.10, 0.25, -0.15] },
        RConcept { name: "Feature Flag", domain: "software", coefficients: [0.10, 0.20, 0.15, -0.05, 0.78, 0.10, 0.30, 0.25] },
        RConcept { name: "Monitoring Dashboard", domain: "software", coefficients: [0.15, 0.20, 0.10, 0.10, 0.75, 0.12, 0.25, -0.05] },
        RConcept { name: "API Gateway", domain: "software", coefficients: [0.20, 0.30, 0.68, 0.08, -0.10, 0.15, 0.20, 0.05] },
        RConcept { name: "Auth Service", domain: "software", coefficients: [0.15, 0.10, -0.08, 0.72, 0.30, 0.20, 0.25, 0.15] },
        RConcept { name: "Event Sourcing Log", domain: "software", coefficients: [0.78, 0.10, 0.25, 0.05, 0.10, 0.12, 0.15, 0.05] },
        RConcept { name: "Cache Layer", domain: "software", coefficients: [0.25, 0.12, 0.65, -0.20, -0.25, 0.18, 0.30, 0.08] },
        RConcept { name: "Database Transaction", domain: "software", coefficients: [0.20, 0.05, 0.10, 0.80, 0.25, 0.18, 0.25, 0.08] },
        RConcept { name: "Background Job Scheduler", domain: "software", coefficients: [0.15, 0.50, 0.35, -0.10, 0.10, -0.08, 0.20, 0.58] },
        RConcept { name: "Marketing Budget", domain: "business", coefficients: [0.05, 0.05, 0.10, 0.85, -0.05, 0.25, 0.10, 0.10] },
        RConcept { name: "Sales Pipeline", domain: "business", coefficients: [0.12, 0.18, 0.75, -0.05, -0.10, 0.18, 0.22, 0.25] },
        RConcept { name: "Revenue Target", domain: "business", coefficients: [0.08, 0.28, 0.12, -0.12, 0.18, 0.10, 0.15, 0.82] },
        RConcept { name: "Quarterly Report", domain: "business", coefficients: [0.12, 0.08, 0.10, 0.12, 0.22, 0.82, 0.10, 0.08] },
        RConcept { name: "Employee Handbook", domain: "business", coefficients: [0.18, 0.05, 0.05, 0.78, 0.32, 0.22, 0.08, 0.05] },
        RConcept { name: "Innovation Fund", domain: "business", coefficients: [0.05, 0.30, 0.15, -0.18, 0.15, 0.12, 0.12, 0.82] },
        RConcept { name: "Customer Feedback Loop", domain: "business", coefficients: [0.22, 0.12, 0.28, 0.05, 0.18, 0.20, 0.75, 0.12] },
        RConcept { name: "Market Trend Analysis", domain: "business", coefficients: [0.15, 0.18, 0.12, 0.05, 0.78, 0.25, 0.15, 0.18] },
        RConcept { name: "Compliance Audit", domain: "business", coefficients: [0.10, 0.10, 0.08, 0.32, 0.15, 0.80, 0.12, 0.08] },
        RConcept { name: "Team Standup Meeting", domain: "business", coefficients: [0.15, 0.15, 0.18, 0.05, 0.15, 0.22, 0.78, 0.10] },
        RConcept { name: "Customer Support Ticket", domain: "business", coefficients: [0.10, 0.72, 0.22, 0.08, 0.12, 0.32, 0.12, 0.18] },
        RConcept { name: "Supply Chain", domain: "business", coefficients: [0.12, 0.22, 0.78, 0.08, 0.18, 0.10, 0.18, 0.18] },
        RConcept { name: "NPS Survey", domain: "business", coefficients: [0.28, 0.10, 0.15, 0.10, 0.22, 0.75, 0.18, 0.08] },
        RConcept { name: "Predator", domain: "biology", coefficients: [0.05, 0.18, 0.12, 0.80, 0.10, 0.15, 0.15, 0.15] },
        RConcept { name: "Decomposer", domain: "biology", coefficients: [0.78, 0.05, 0.22, 0.10, 0.15, 0.10, 0.22, 0.05] },
        RConcept { name: "Photosynthesis", domain: "biology", coefficients: [0.05, 0.28, 0.18, 0.05, 0.12, 0.18, 0.12, 0.82] },
        RConcept { name: "Keystone Species", domain: "biology", coefficients: [0.10, 0.28, 0.15, 0.12, 0.75, 0.10, 0.22, 0.25] },
        RConcept { name: "Mutation", domain: "biology", coefficients: [0.10, 0.78, 0.15, 0.05, 0.18, 0.10, 0.18, 0.28] },
        RConcept { name: "Homeostasis", domain: "biology", coefficients: [0.15, 0.05, 0.15, 0.18, 0.12, 0.10, 0.82, 0.10] },
        RConcept { name: "Natural Selection", domain: "biology", coefficients: [0.05, 0.12, 0.10, 0.82, 0.18, 0.25, 0.15, 0.10] },
        RConcept { name: "Ecological Succession", domain: "biology", coefficients: [0.15, 0.22, 0.12, 0.05, 0.78, 0.15, 0.18, 0.15] },
        RConcept { name: "Symbiosis", domain: "biology", coefficients: [0.18, 0.10, 0.15, 0.05, 0.18, 0.10, 0.80, 0.22] },
        RConcept { name: "DNA Replication", domain: "biology", coefficients: [0.10, 0.22, 0.15, 0.08, 0.10, 0.10, 0.15, 0.82] },
        RConcept { name: "Immune Response", domain: "biology", coefficients: [0.12, 0.30, 0.15, 0.60, 0.10, 0.18, 0.30, 0.28] },
        RConcept { name: "Hormone Signaling", domain: "biology", coefficients: [0.18, 0.35, 0.25, 0.05, 0.72, 0.12, 0.22, 0.25] },
        RConcept { name: "Friction", domain: "physics", coefficients: [0.05, 0.10, 0.10, 0.85, 0.10, 0.15, 0.18, 0.05] },
        RConcept { name: "Electric Current", domain: "physics", coefficients: [0.12, 0.25, 0.80, 0.05, 0.10, 0.10, 0.15, 0.18] },
        RConcept { name: "Nuclear Fusion", domain: "physics", coefficients: [0.05, 0.35, 0.15, 0.05, 0.10, 0.15, 0.12, 0.82] },
        RConcept { name: "Heat Sink", domain: "physics", coefficients: [0.22, 0.10, 0.70, 0.12, 0.10, 0.15, 0.25, 0.10] },
        RConcept { name: "Thermal Expansion", domain: "physics", coefficients: [0.18, 0.20, 0.18, 0.10, 0.75, 0.10, 0.22, 0.15] },
        RConcept { name: "Gravity", domain: "physics", coefficients: [0.08, 0.15, 0.10, 0.82, 0.18, 0.10, 0.15, 0.18] },
        RConcept { name: "Catalyst", domain: "physics", coefficients: [0.10, 0.72, 0.18, 0.05, 0.18, 0.15, 0.18, 0.28] },
        RConcept { name: "Entropy", domain: "physics", coefficients: [0.10, 0.18, 0.55, 0.05, 0.25, 0.25, 0.22, 0.32] },
        RConcept { name: "Resonance", domain: "physics", coefficients: [0.18, 0.25, 0.18, 0.05, 0.18, 0.12, 0.75, 0.20] },
        RConcept { name: "Superconductor", domain: "physics", coefficients: [0.15, 0.10, 0.78, 0.08, 0.12, 0.10, 0.20, 0.15] },
        RConcept { name: "Star Formation", domain: "physics", coefficients: [0.08, 0.32, 0.15, 0.05, 0.12, 0.12, 0.15, 0.85] },
        RConcept { name: "Feedback Control System", domain: "physics", coefficients: [0.18, 0.15, 0.22, 0.12, 0.18, 0.15, 0.78, 0.15] },
    ]
}

fn build_role_sets(concepts: &[RConcept], encoded: &[Multivector]) -> Vec<(String, Vec<usize>)> {
    let role_names = ["generative", "receptive", "causal", "transmissive",
        "constraining", "influential", "clarifying", "balancing"];
    let mut sets = Vec::new();
    for role_name in &role_names {
        let relevant: Vec<usize> = concepts.iter().enumerate()
            .filter(|(_, c)| encoded[concepts.iter().position(|x| x.name == c.name).unwrap_or(0)]
                .dominant_role().role_name() == *role_name)
            .map(|(i, _)| i)
            .collect();
        if relevant.len() >= 2 {
            sets.push((role_name.to_string(), relevant));
        }
    }
    sets
}

fn recall_at_k(scored: &[(usize, f64)], ground_truth: &std::collections::HashSet<usize>, k: usize, n: usize) -> f64 {
    if n == 0 { return 1.0; }
    let hits = scored.iter().take(k).filter(|(i, _)| ground_truth.contains(i)).count();
    (hits as f64 / n as f64).min(1.0)
}

fn measure(concepts: &[RConcept], encoded: &[Multivector], role_sets: &[(String, Vec<usize>)], sim_fn: fn(&Multivector, &Multivector) -> f64) -> (f64, f64, f64, f64) {
    let mut r1 = 0.0; let mut r3 = 0.0; let mut r5 = 0.0; let mut r10 = 0.0; let mut qc = 0;
    for (_, relevant) in role_sets {
        if relevant.len() < 2 { continue; }
        let qi = relevant[0];
        let qm = &encoded[qi];
        let qd = concepts[qi].domain;
        let peers: std::collections::HashSet<usize> = relevant.iter()
            .filter(|&&i| i != qi && concepts[i].domain == qd).cloned().collect();
        if peers.len() < 2 { continue; }
        qc += 1;
        let mut scored: Vec<(usize, f64)> = encoded.iter().enumerate()
            .filter(|(i, _)| *i != qi)
            .map(|(i, mv)| (i, sim_fn(qm, mv))).collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let n = peers.len();
        r1 += recall_at_k(&scored, &peers, 1, n);
        r3 += recall_at_k(&scored, &peers, 3, n);
        r5 += recall_at_k(&scored, &peers, 5, n);
        r10 += recall_at_k(&scored, &peers, 10, n);
    }
    if qc == 0 { return (0.0, 0.0, 0.0, 0.0); }
    (r1 / qc as f64, r3 / qc as f64, r5 / qc as f64, r10 / qc as f64)
}

fn keyword_jaccard(a_name: &str, b_name: &str) -> f64 {
    let sa: std::collections::HashSet<&str> = a_name.split_whitespace().collect();
    let sb: std::collections::HashSet<&str> = b_name.split_whitespace().collect();
    if sa.is_empty() && sb.is_empty() { return 0.0; }
    let inter = sa.intersection(&sb).count();
    let union = sa.union(&sb).count();
    if union == 0 { return 0.0; }
    inter as f64 / union as f64
}

fn run_retrieval_benchmark() {
    let concepts = retrieval_concepts();
    let encoded: Vec<Multivector> = concepts.iter().map(|c| llm_encode(&c.coefficients)).collect();
    let role_sets = build_role_sets(&concepts, &encoded);

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║        RETRIEVAL QUALITY — BASELINE COMPARISON                   ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  {} concepts, 4 domains, {} role-based queries                ║", concepts.len(), role_sets.len());
    println!("║  Same-role + same-domain retrieval                              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("  Method                    │ R@1  │ R@3  │ R@5  │ R@10");
    println!("  ──────────────────────────┼──────┼──────┼──────┼──────");

    let (ds1, ds3, ds5, ds10) = measure(&concepts, &encoded, &role_sets, dominant_similarity);
    println!("  dominant_similarity       │ {:.0}% │ {:.0}% │ {:.0}% │ {:.0}%", ds1*100.0, ds3*100.0, ds5*100.0, ds10*100.0);

    let (fp1, fp3, fp5, fp10) = measure(&concepts, &encoded, &role_sets, fingerprint_similarity);
    let delta = fp1 - ds1;
    let delta_str = if delta > 0.005 { format!("+{:.0}pp", delta*100.0) } else if delta < -0.005 { format!("{:.0}pp", delta*100.0) } else { "no change".into() };
    println!("  fingerprint_similarity     │ {:.0}% │ {:.0}% │ {:.0}% │ {:.0}%  {}",
        fp1*100.0, fp3*100.0, fp5*100.0, fp10*100.0, delta_str);

    // Keyword baseline: Jaccard on concept names
    let mut kw_r1 = 0.0; let mut kw_r3 = 0.0; let mut kw_r5 = 0.0; let mut kw_r10 = 0.0; let mut kw_qc = 0;
    for (_, relevant) in &role_sets {
        if relevant.len() < 2 { continue; }
        let qi = relevant[0];
        let qd = concepts[qi].domain;
        let peers: std::collections::HashSet<usize> = relevant.iter()
            .filter(|&&i| i != qi && concepts[i].domain == qd).cloned().collect();
        if peers.len() < 2 { continue; }
        kw_qc += 1;
        let mut scored: Vec<(usize, f64)> = concepts.iter().enumerate()
            .filter(|(i, _)| *i != qi)
            .map(|(i, c)| (i, keyword_jaccard(concepts[qi].name, c.name))).collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let n = peers.len();
        kw_r1 += recall_at_k(&scored, &peers, 1, n);
        kw_r3 += recall_at_k(&scored, &peers, 3, n);
        kw_r5 += recall_at_k(&scored, &peers, 5, n);
        kw_r10 += recall_at_k(&scored, &peers, 10, n);
    }
    if kw_qc > 0 {
        println!("  keyword (Jaccard names)    │ {:.0}% │ {:.0}% │ {:.0}% │ {:.0}%",
            kw_r1/kw_qc as f64*100.0, kw_r3/kw_qc as f64*100.0, kw_r5/kw_qc as f64*100.0, kw_r10/kw_qc as f64*100.0);
    }

    // Random baseline
    let mut rnd_r1 = 0.0; let mut rnd_r3 = 0.0; let mut rnd_r5 = 0.0; let mut rnd_r10 = 0.0; let mut rnd_qc = 0;
    let mut seed: u64 = 0xCAFE;
    for (_, relevant) in &role_sets {
        if relevant.len() < 2 { continue; }
        let qi = relevant[0];
        let qd = concepts[qi].domain;
        let peers: std::collections::HashSet<usize> = relevant.iter()
            .filter(|&&i| i != qi && concepts[i].domain == qd).cloned().collect();
        if peers.len() < 2 { continue; }
        rnd_qc += 1;
        let mut scored: Vec<(usize, f64)> = encoded.iter().enumerate()
            .filter(|(i, _)| *i != qi)
            .map(|(i, _)| {
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                (i, (seed as f64) / (u64::MAX as f64))
            }).collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let n = peers.len();
        rnd_r1 += recall_at_k(&scored, &peers, 1, n);
        rnd_r3 += recall_at_k(&scored, &peers, 3, n);
        rnd_r5 += recall_at_k(&scored, &peers, 5, n);
        rnd_r10 += recall_at_k(&scored, &peers, 10, n);
    }
    if rnd_qc > 0 {
        println!("  random                    │ {:.0}% │ {:.0}% │ {:.0}% │ {:.0}%",
            rnd_r1/rnd_qc as f64*100.0, rnd_r3/rnd_qc as f64*100.0, rnd_r5/rnd_qc as f64*100.0, rnd_r10/rnd_qc as f64*100.0);
    }

    println!("\n  ── ANALYSIS ──");
    println!("  dominant_similarity R@1: {:.0}%", ds1 * 100.0);
    println!("  fingerprint_similarity R@1: {:.0}% ({})", fp1 * 100.0,
        if delta > 0.005 { format!("improves by {:.0}pp", delta * 100.0) }
        else if delta < -0.005 { format!("reduces by {:.0}pp", (-delta) * 100.0) }
        else { "no significant change".into() });
    println!("  keyword baseline R@1: {:.0}%", if kw_qc > 0 { kw_r1/kw_qc as f64*100.0 } else { 0.0 });
    println!("  random baseline R@1: {:.0}%", if rnd_qc > 0 { rnd_r1/rnd_qc as f64*100.0 } else { 0.0 });
    println!();
    if delta > 0.005 {
        println!("  fingerprint_similarity improves same-role within-domain R@1.");
        println!("  Secondary coefficient patterns provide useful ranking signal.");
    } else if delta < -0.005 {
        println!("  fingerprint_similarity degrades R@1 — secondary coefficients are noise.");
        println!("  Encoding distinctiveness (encoding-quality workstream) is prerequisite.");
    } else {
        println!("  fingerprint_similarity matches dominant_similarity — secondary patterns");
        println!("  don't yet have enough signal to break within-role ties.");
    }
}

#[test]
fn retrieval_quality_benchmark() {
    run_retrieval_benchmark();
}
