use ga_semantics_core::prelude::*;
use ga_semantics_core::RelationType;

#[derive(Debug)]
struct ConceptFixture {
    name: &'static str,
    description: &'static str,
    coefficients: [f64; 8],
    expected_dominant_role: &'static str,
}

#[derive(Debug)]
struct RelationFixture {
    pair: (usize, usize),
    expected_relation: &'static str,
    expectation_strength: &'static str,
}

#[derive(Debug)]
struct AnalogyFixture {
    a: usize, b: usize, c: usize,
    expected_d_relation: &'static str,
}

fn concept_fixtures() -> Vec<ConceptFixture> {
    vec![
        ConceptFixture { name: "Rate Limiter", description: "Restricts request frequency per time window",
            coefficients: [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34], expected_dominant_role: "constraining" },
        ConceptFixture { name: "Message Queue", description: "Transmits events asynchronously with delivery guarantees",
            coefficients: [0.15, 0.25, 0.81, -0.20, -0.25, 0.10, 0.36, 0.05], expected_dominant_role: "transmissive" },
        ConceptFixture { name: "Database Transaction", description: "Atomic, consistent, isolated, durable writes",
            coefficients: [0.28, 0.05, 0.14, 0.79, 0.32, 0.18, 0.37, 0.09], expected_dominant_role: "constraining" },
        ConceptFixture { name: "Auth System", description: "Verifies identity before granting resource access",
            coefficients: [0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20], expected_dominant_role: "constraining" },
        ConceptFixture { name: "Cache Layer", description: "Stores frequently accessed data in memory for fast retrieval",
            coefficients: [0.30, 0.10, 0.60, -0.25, -0.30, 0.15, 0.35, 0.10], expected_dominant_role: "transmissive" },
        ConceptFixture { name: "Logging System", description: "Records every operation for audit trails and debugging",
            coefficients: [0.15, 0.05, 0.10, 0.30, 0.85, 0.05, 0.25, -0.15], expected_dominant_role: "clarifying" },
        ConceptFixture { name: "Feature Flag", description: "Enables gradual rollout of new functionality to user subsets",
            coefficients: [0.10, 0.20, 0.20, -0.10, 0.10, 0.78, 0.35, 0.40], expected_dominant_role: "influential" },
        ConceptFixture { name: "Load Balancer", description: "Distributes traffic evenly across multiple server instances",
            coefficients: [0.30, -0.10, 0.45, -0.05, 0.10, 0.15, 0.80, 0.10], expected_dominant_role: "balancing" },
        ConceptFixture { name: "Background Job Scheduler", description: "Dispatches workloads to workers at specified intervals",
            coefficients: [0.15, 0.55, 0.30, -0.15, 0.10, -0.10, 0.25, 0.60], expected_dominant_role: "generative" },
        ConceptFixture { name: "API Gateway", description: "Routes incoming client requests to appropriate backend services",
            coefficients: [0.20, 0.30, 0.65, 0.05, -0.10, 0.15, 0.25, 0.05], expected_dominant_role: "transmissive" },
        ConceptFixture { name: "Monitoring Dashboard", description: "Observes system health metrics and alerts on anomalies",
            coefficients: [0.20, 0.25, 0.05, 0.15, 0.75, 0.10, 0.30, -0.05], expected_dominant_role: "clarifying" },
        ConceptFixture { name: "Circuit Breaker", description: "Prevents cascading failures by stopping calls to failing services",
            coefficients: [0.05, -0.30, -0.20, 0.75, 0.25, -0.10, 0.25, -0.20], expected_dominant_role: "constraining" },
        ConceptFixture { name: "Configuration Store", description: "Central repository for application configuration values",
            coefficients: [0.65, 0.05, 0.10, 0.15, 0.30, 0.15, 0.10, 0.00], expected_dominant_role: "receptive" },
        ConceptFixture { name: "Event Stream Processor", description: "Transforms and enriches data in real-time event pipelines",
            coefficients: [0.10, 0.30, 0.60, -0.10, 0.15, 0.30, 0.15, 0.45], expected_dominant_role: "transmissive" },
        ConceptFixture { name: "Black Box Module", description: "Opaque component that hides its internal implementation details",
            coefficients: [0.05, -0.10, -0.30, -0.05, -0.70, -0.65, -0.15, 0.05], expected_dominant_role: "clarifying" },
        ConceptFixture { name: "Innovation Lab", description: "Creates and prototypes novel technologies without constraints",
            coefficients: [-0.30, 0.25, 0.15, -0.55, 0.15, 0.30, 0.10, 0.85], expected_dominant_role: "generative" },
        ConceptFixture { name: "Compliance Validator", description: "Checks every action against regulatory rules before execution",
            coefficients: [0.20, 0.05, -0.15, 0.80, 0.35, 0.10, -0.05, -0.05], expected_dominant_role: "constraining" },
        ConceptFixture { name: "Peer-to-Peer Network", description: "Decentralized system where nodes communicate directly as equals",
            coefficients: [0.05, 0.10, 0.35, -0.25, 0.15, 0.25, 0.80, 0.15], expected_dominant_role: "balancing" },
        ConceptFixture { name: "Notification Service", description: "Pushes alerts and updates to subscribed users in real-time",
            coefficients: [0.10, 0.40, 0.45, -0.05, 0.30, 0.50, 0.10, 0.10], expected_dominant_role: "influential" },
        ConceptFixture { name: "Data Warehouse", description: "Accumulates and structures historical data for analytical queries",
            coefficients: [0.50, 0.05, 0.15, 0.20, 0.45, 0.10, -0.05, -0.15], expected_dominant_role: "receptive" },
    ]
}

fn relation_fixtures() -> Vec<RelationFixture> {
    vec![
        RelationFixture { pair: (0, 2), expected_relation: "receptive", expectation_strength: "strong" },
        RelationFixture { pair: (1, 9), expected_relation: "receptive", expectation_strength: "strong" },
        RelationFixture { pair: (3, 0), expected_relation: "receptive", expectation_strength: "strong" },
        RelationFixture { pair: (5, 10), expected_relation: "receptive", expectation_strength: "strong" },
        RelationFixture { pair: (6, 18), expected_relation: "receptive", expectation_strength: "strong" },
        RelationFixture { pair: (7, 11), expected_relation: "receptive", expectation_strength: "moderate" },
        RelationFixture { pair: (8, 13), expected_relation: "generative", expectation_strength: "strong" },
        RelationFixture { pair: (14, 15), expected_relation: "constraining", expectation_strength: "strong" },
        RelationFixture { pair: (4, 12), expected_relation: "influential", expectation_strength: "moderate" },
        RelationFixture { pair: (16, 0), expected_relation: "receptive", expectation_strength: "strong" },
        RelationFixture { pair: (17, 9), expected_relation: "generative", expectation_strength: "strong" },
        RelationFixture { pair: (19, 13), expected_relation: "constraining", expectation_strength: "strong" },
        RelationFixture { pair: (15, 8), expected_relation: "receptive", expectation_strength: "strong" },
        RelationFixture { pair: (1, 4), expected_relation: "receptive", expectation_strength: "strong" },
        RelationFixture { pair: (10, 5), expected_relation: "receptive", expectation_strength: "strong" },
    ]
}

fn analogy_fixtures() -> Vec<AnalogyFixture> {
    vec![
        AnalogyFixture { a: 8, b: 13, c: 6, expected_d_relation: "clarifying" },
        AnalogyFixture { a: 0, b: 1, c: 15, expected_d_relation: "influential" },
        AnalogyFixture { a: 6, b: 5, c: 12, expected_d_relation: "generative" },
        AnalogyFixture { a: 5, b: 8, c: 9, expected_d_relation: "clarifying" },
        AnalogyFixture { a: 12, b: 8, c: 13, expected_d_relation: "influential" },
    ]
}

fn load_concepts() -> Vec<Multivector> {
    concept_fixtures().iter().map(|f| llm_encode(&f.coefficients)).collect()
}

struct BenchmarkResult {
    name: String,
    metric: String,
    value: f64,
    interpretation: String,
}

fn run_benchmarks() -> (Vec<BenchmarkResult>, Vec<String>) {
    let fixtures = concept_fixtures();
    let concepts = load_concepts();
    let relations = relation_fixtures();
    let analogies = analogy_fixtures();
    let mut results = vec![];
    let mut notes = vec![];

    // ── BENCHMARK 1: Dominant Role Accuracy ──
    let mut correct_roles = 0usize;
    for (i, f) in fixtures.iter().enumerate() {
        let mv = &concepts[i];
        let dom = mv.dominant_role();
        if dom.role_name() == f.expected_dominant_role {
            correct_roles += 1;
        } else {
            notes.push(format!("  {} expected '{}' got '{}' (w={:.2})",
                f.name, f.expected_dominant_role, dom.role_name(),
                mv.coefficients()[dom.blade().index()]));
        }
    }
    let role_acc = correct_roles as f64 / fixtures.len() as f64;
    results.push(BenchmarkResult {
        name: "Dominant Role Accuracy".into(),
        metric: format!("{}/{}", correct_roles, fixtures.len()),
        value: role_acc,
        interpretation: if role_acc > 0.9 { "Excellent — encoding protocol is consistent".into() }
            else if role_acc > 0.7 { "Good — most concepts map correctly".into() }
            else if role_acc > 0.5 { "Marginal — encoding needs refinement".into() }
            else { "Poor — encoding protocol is broken".into() },
    });

    // ── BENCHMARK 2: Relation Classification Accuracy ──
    let mut correct_relations = 0usize;
    let mut strong_correct = 0usize;
    let mut strong_total = 0usize;
    for r in &relations {
        let (a_mv, b_mv) = (&concepts[r.pair.0], &concepts[r.pair.1]);
        let (actual, confidence) = RelationType::from_pair(a_mv, b_mv);
        let matched = actual.role_name() == r.expected_relation;
        if matched { correct_relations += 1; }
        if r.expectation_strength == "strong" {
            strong_total += 1;
            if matched { strong_correct += 1; }
        }
        notes.push(format!("  {} ⊗ {}: expected '{}' got '{}' (conf={:.2}, {})",
            fixtures[r.pair.0].name, fixtures[r.pair.1].name,
            r.expected_relation, actual.role_name(), confidence,
            if matched { "PASS" } else { "FAIL" }));
    }
    let rel_acc = correct_relations as f64 / relations.len() as f64;
    let strong_acc = if strong_total > 0 { strong_correct as f64 / strong_total as f64 } else { 0.0 };
    results.push(BenchmarkResult {
        name: "Relation Classification (all)".into(),
        metric: format!("{}/{}", correct_relations, relations.len()),
        value: rel_acc,
        interpretation: if rel_acc > 0.8 { "Strong — relation classification is reliable".into() }
            else if rel_acc > 0.6 { "Moderate — useful with verification".into() }
            else if rel_acc > 0.4 { "Weak — better than random, not production-ready".into() }
            else { "Random or worse — classification is unreliable".into() },
    });
    results.push(BenchmarkResult {
        name: "Relation Classification (strong expectations only)".into(),
        metric: format!("{}/{}", strong_correct, strong_total),
        value: strong_acc,
        interpretation: format!("{} strong-expectation pairs", strong_total),
    });

    // ── BENCHMARK 3: Category Discrimination ──
    let categories: Vec<(&str, Vec<usize>)> = vec![
        ("constraining", (0..fixtures.len()).filter(|&i| fixtures[i].expected_dominant_role == "constraining").collect()),
        ("transmissive", (0..fixtures.len()).filter(|&i| fixtures[i].expected_dominant_role == "transmissive").collect()),
        ("clarifying", (0..fixtures.len()).filter(|&i| fixtures[i].expected_dominant_role == "clarifying").collect()),
        ("generative", (0..fixtures.len()).filter(|&i| fixtures[i].expected_dominant_role == "generative").collect()),
    ];
    let mut intra_cat_sims = vec![];
    let mut inter_cat_sims = vec![];
    for (cat_name, indices) in &categories {
        for i in 0..indices.len() {
            for j in (i+1)..indices.len() {
                intra_cat_sims.push(semantic_similarity(&concepts[indices[i]], &concepts[indices[j]]));
            }
        }
    }
    for (c1_name, idx1) in &categories {
        for (c2_name, idx2) in &categories {
            if c1_name >= c2_name { continue; }
            for &i in idx1 { for &j in idx2 {
                inter_cat_sims.push(semantic_similarity(&concepts[i], &concepts[j]));
            }}
        }
    }
    let intra_mean = intra_cat_sims.iter().sum::<f64>() / intra_cat_sims.len() as f64;
    let inter_mean = inter_cat_sims.iter().sum::<f64>() / inter_cat_sims.len() as f64;
    let discrimination = intra_mean - inter_mean;
    results.push(BenchmarkResult {
        name: "Category Discrimination (intra - inter)".into(),
        metric: format!("{:.3}", discrimination),
        value: discrimination,
        interpretation: if discrimination > 0.2 { "Strong — similar concepts cluster together".into() }
            else if discrimination > 0.1 { "Moderate — some clustering visible".into() }
            else if discrimination > 0.0 { "Weak — slight clustering".into() }
            else { "Negative — similar concepts are NOT closer than different ones".into() },
    });

    // ── BENCHMARK 4: Retrieval Precision@K + MRR ──
    let all_roles: Vec<&str> = fixtures.iter().map(|f| f.expected_dominant_role).collect();
    let role_counts: std::collections::HashMap<&str, usize> = {
        let mut m = std::collections::HashMap::new();
        for r in &all_roles { *m.entry(*r).or_insert(0) += 1; }
        m
    };
    let mut prec_sum = 0.0f64;
    let mut mrr_sum = 0.0f64;
    let mut query_count = 0usize;
    let mut all_intra: Vec<f64> = vec![];
    let mut all_inter: Vec<f64> = vec![];
    let mut diagnosis = vec![];

    for q_idx in 0..fixtures.len() {
        let query_mv = &concepts[q_idx];
        let query_role = fixtures[q_idx].expected_dominant_role;
        let peer_count = *role_counts.get(query_role).unwrap_or(&0) - 1;
        if peer_count == 0 { continue; }
        let k_eff = 3usize.min(peer_count);

        let mut scored: Vec<(usize, f64)> = concepts.iter().enumerate()
            .filter(|(i, _)| *i != q_idx)
            .map(|(i, mv)| (i, dominant_similarity(query_mv, mv)))
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let top_k: Vec<usize> = scored.iter().take(k_eff).map(|(i, _)| *i).collect();
        let hits = top_k.iter().filter(|&&i| fixtures[i].expected_dominant_role == query_role).count();
        prec_sum += hits as f64 / k_eff as f64;

        let first_hit_rank = scored.iter().position(|(i, _)| fixtures[*i].expected_dominant_role == query_role);
        if let Some(rank) = first_hit_rank {
            mrr_sum += 1.0 / (rank as f64 + 1.0);
        }

        for (i, s) in &scored {
            if fixtures[*i].expected_dominant_role == query_role { all_intra.push(*s); }
            else { all_inter.push(*s); }
        }
        diagnosis.push(format!("  {} ({}) top3: {:?}", fixtures[q_idx].name, query_role,
            top_k.iter().map(|&i| format!("{}({})", fixtures[i].name, fixtures[i].expected_dominant_role)).collect::<Vec<_>>()));
        query_count += 1;
    }

    let precision = prec_sum / query_count as f64;
    let mrr = mrr_sum / query_count as f64;
    let intra_ret_mean = all_intra.iter().sum::<f64>() / all_intra.len().max(1) as f64;
    let inter_ret_mean = all_inter.iter().sum::<f64>() / all_inter.len().max(1) as f64;
    let ret_discrimination = intra_ret_mean - inter_ret_mean;

    results.push(BenchmarkResult {
        name: "Retrieval Precision@K".into(),
        metric: format!("{:.1}% ({} queries)", precision * 100.0, query_count),
        value: precision,
        interpretation: if precision > 0.7 { "Strong — retrieval finds category peers".into() }
            else if precision > 0.5 { "Moderate — better than random".into() }
            else { "Weak — retrieval struggles".into() },
    });
    results.push(BenchmarkResult {
        name: "Retrieval MRR (first peer rank)".into(),
        metric: format!("{:.3}", mrr),
        value: mrr,
        interpretation: if mrr > 0.5 { "First peer usually #1-2".into() }
            else if mrr > 0.3 { "First peer typically in top 3".into() }
            else { "First peer not in top results".into() },
    });
    results.push(BenchmarkResult {
        name: "Retrieval Discrimination".into(),
        metric: format!("{:.3}", ret_discrimination),
        value: ret_discrimination,
        interpretation: if ret_discrimination > 0.1 { "dominant_similarity separates peers from non-peers".into() }
            else if ret_discrimination > 0.0 { "Weak separation".into() }
            else { "No separation — similarity metric is random for retrieval".into() },
    });
    for d in &diagnosis { notes.push(d.clone()); }

    // ── BENCHMARK 5: Analogy Accuracy ──
    let mut correct_analogies = 0usize;
    for a in &analogies {
        let (a_mv, b_mv, c_mv) = (&concepts[a.a], &concepts[a.b], &concepts[a.c]);
        if let Some(result) = analogy(a_mv, b_mv, c_mv) {
            let dom = result.dominant_role();
            if dom.role_name() == a.expected_d_relation { correct_analogies += 1; }
            notes.push(format!("  {}:{}::{}:? -> expected '{}' got '{}'",
                fixtures[a.a].name, fixtures[a.b].name, fixtures[a.c].name,
                a.expected_d_relation, dom.role_name()));
        }
    }
    let analogy_acc = correct_analogies as f64 / analogies.len() as f64;
    results.push(BenchmarkResult {
        name: "Analogy Accuracy".into(),
        metric: format!("{}/{}", correct_analogies, analogies.len()),
        value: analogy_acc,
        interpretation: if analogy_acc > 0.6 { "Strong — analogical reasoning works".into() }
            else if analogy_acc > 0.3 { "Moderate — better than random, useful directionally".into() }
            else { "Weak — analogy operation is not semantically reliable".into() },
    });

    // ── BENCHMARK 6: Combined Semantic Score ──
    let combined = (role_acc + rel_acc + strong_acc + discrimination.max(0.0).min(1.0) + precision + mrr + analogy_acc) / 7.0;
    results.push(BenchmarkResult {
        name: "COMBINED SEMANTIC SCORE".into(),
        metric: format!("{:.1}%", combined * 100.0),
        value: combined,
        interpretation: if combined > 0.8 { "EXCELLENT — ready for production KG use".into() }
            else if combined > 0.65 { "GOOD — useful with LLM oversight".into() }
            else if combined > 0.5 { "ADEQUATE — works for coarse-grained classification".into() }
            else { "NEEDS WORK — encoding protocol requires iteration".into() },
    });

    (results, notes)
}

#[test]
fn semantic_accuracy_benchmark() {
    let (results, notes) = run_benchmarks();

    println!("\n{:=^70}", " GA-BAGUA SEMANTIC ACCURACY BENCHMARK ");
    println!("  Fixtures: {} concepts, {} relations, {} analogies, {} categories",
        concept_fixtures().len(), relation_fixtures().len(), analogy_fixtures().len(), 4);
    println!("{:-<70}", "");
    for r in &results {
        println!("  {:<42} {:>12}  {:>10}", r.name, r.metric, r.interpretation);
    }
    println!("{:-<70}", "");
    println!("\n  Detailed results:");
    for n in &notes { println!("{}", n); }

    assert!(results.iter().all(|r| !r.metric.is_empty()), "All metrics present");
}
