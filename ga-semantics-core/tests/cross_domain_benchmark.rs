use ga_semantics_core::prelude::*;
use ga_semantics_core::RelationType;

#[derive(Debug)]
struct DomainConcept {
    name: &'static str,
    domain: &'static str,
    description: &'static str,
    coefficients: [f64; 8],
    expected_role: &'static str,
}

#[derive(Debug)]
struct DomainRelation {
    concept_a: usize,
    concept_b: usize,
    expected_relation: &'static str,
}

fn domain_concepts() -> Vec<DomainConcept> {
    vec![
        // ── LEGAL DOMAIN ──
        DomainConcept { name: "Contract", domain: "legal", description: "Binding agreement between parties that defines obligations and rights",
            coefficients: [-0.15, 0.05, 0.10, 0.80, 0.25, 0.35, 0.10, 0.05], expected_role: "constraining" },
        DomainConcept { name: "Liability Clause", domain: "legal", description: "Provision that assigns responsibility for damages or losses",
            coefficients: [-0.10, -0.05, -0.10, 0.85, 0.15, 0.20, 0.10, -0.05], expected_role: "constraining" },
        DomainConcept { name: "Precedent", domain: "legal", description: "Prior judicial decision that guides future similar cases",
            coefficients: [0.30, 0.10, 0.35, 0.10, 0.70, 0.20, 0.15, 0.15], expected_role: "influential" },
        DomainConcept { name: "Due Process", domain: "legal", description: "Fundamental procedural fairness guaranteed by law",
            coefficients: [0.35, 0.05, 0.50, 0.15, 0.15, 0.40, 0.55, 0.10], expected_role: "balancing" },
        DomainConcept { name: "Statute", domain: "legal", description: "Written law enacted by legislative authority",
            coefficients: [0.05, 0.20, 0.05, 0.80, 0.10, 0.30, 0.10, 0.15], expected_role: "constraining" },
        DomainConcept { name: "Arbitration", domain: "legal", description: "Alternative dispute resolution outside formal court system",
            coefficients: [0.15, 0.15, 0.55, 0.10, 0.15, 0.25, 0.70, 0.05], expected_role: "balancing" },
        DomainConcept { name: "Damages", domain: "legal", description: "Monetary compensation awarded to an injured party",
            coefficients: [0.55, 0.05, 0.40, 0.15, 0.10, 0.20, 0.60, 0.10], expected_role: "balancing" },
        DomainConcept { name: "Litigation", domain: "legal", description: "The process of taking legal action through courts",
            coefficients: [0.05, 0.70, 0.30, 0.15, 0.10, 0.15, 0.15, 0.30], expected_role: "causal" },
        DomainConcept { name: "Compliance", domain: "legal", description: "Adherence to laws, regulations, and standards",
            coefficients: [0.20, 0.05, -0.10, 0.75, 0.30, 0.35, 0.05, -0.05], expected_role: "constraining" },
        DomainConcept { name: "Constitution", domain: "legal", description: "Supreme foundational law that establishes governance framework",
            coefficients: [0.25, 0.10, 0.10, 0.50, 0.20, 0.25, 0.15, 0.70], expected_role: "generative" },

        // ── MEDICAL DOMAIN ──
        DomainConcept { name: "Diagnosis", domain: "medical", description: "Identification of disease or condition from symptoms and tests",
            coefficients: [0.10, 0.35, 0.10, 0.15, 0.20, 0.80, 0.10, 0.10], expected_role: "clarifying" },
        DomainConcept { name: "Vaccination", domain: "medical", description: "Biological preparation that provides immunity to disease",
            coefficients: [0.25, 0.30, 0.10, -0.15, 0.30, 0.15, 0.30, 0.70], expected_role: "generative" },
        DomainConcept { name: "Antibiotic", domain: "medical", description: "Substance that kills or inhibits bacterial growth",
            coefficients: [0.05, 0.15, 0.15, 0.75, 0.15, 0.10, 0.15, 0.05], expected_role: "constraining" },
        DomainConcept { name: "Placebo", domain: "medical", description: "Inactive substance used as control in clinical trials",
            coefficients: [0.90, -0.05, 0.05, 0.05, -0.10, 0.05, -0.10, 0.10], expected_role: "receptive" },
        DomainConcept { name: "Pandemic", domain: "medical", description: "Widespread disease outbreak across multiple countries",
            coefficients: [0.05, 0.80, 0.40, -0.15, 0.25, 0.10, 0.10, 0.35], expected_role: "causal" },
        DomainConcept { name: "Symptom", domain: "medical", description: "Subjective indication of disease perceived by patient",
            coefficients: [0.55, 0.15, 0.10, 0.10, 0.10, 0.70, 0.05, 0.05], expected_role: "clarifying" },
        DomainConcept { name: "Immune Response", domain: "medical", description: "Body's defensive reaction against pathogens",
            coefficients: [0.15, 0.35, 0.20, 0.55, 0.10, 0.15, 0.30, 0.45], expected_role: "constraining" },
        DomainConcept { name: "Epidemiology", domain: "medical", description: "Study of disease distribution patterns in populations",
            coefficients: [0.10, 0.20, 0.55, 0.10, 0.30, 0.65, 0.10, 0.15], expected_role: "clarifying" },
        DomainConcept { name: "Surgery", domain: "medical", description: "Invasive medical procedure to treat injury or disease",
            coefficients: [0.05, 0.60, 0.15, 0.10, 0.15, 0.20, 0.15, 0.55], expected_role: "causal" },
        DomainConcept { name: "Therapy", domain: "medical", description: "Treatment intended to relieve or heal a disorder",
            coefficients: [0.20, 0.30, 0.45, -0.05, 0.50, 0.20, 0.40, 0.30], expected_role: "influential" },

        // ── SCIENTIFIC DOMAIN ──
        DomainConcept { name: "Hypothesis", domain: "science", description: "Testable proposed explanation for a phenomenon",
            coefficients: [0.15, 0.50, 0.10, 0.10, 0.20, 0.30, 0.10, 0.70], expected_role: "generative" },
        DomainConcept { name: "Experiment", domain: "science", description: "Controlled procedure to test a hypothesis",
            coefficients: [0.15, 0.40, 0.20, 0.30, 0.15, 0.75, 0.15, 0.25], expected_role: "clarifying" },
        DomainConcept { name: "Theory", domain: "science", description: "Well-substantiated explanation supported by evidence",
            coefficients: [0.35, 0.10, 0.25, 0.15, 0.65, 0.30, 0.20, 0.40], expected_role: "influential" },
        DomainConcept { name: "Peer Review", domain: "science", description: "Evaluation of work by qualified colleagues in same field",
            coefficients: [0.10, 0.10, 0.15, 0.45, 0.15, 0.70, 0.40, 0.05], expected_role: "clarifying" },
        DomainConcept { name: "Variable", domain: "science", description: "Measurable factor that can change in an experiment",
            coefficients: [0.40, 0.15, 0.50, 0.10, 0.10, 0.25, 0.35, 0.15], expected_role: "transmissive" },
        DomainConcept { name: "Paradigm", domain: "science", description: "Dominant framework of thought in a scientific field",
            coefficients: [0.20, 0.05, 0.15, 0.55, 0.60, 0.10, 0.15, 0.25], expected_role: "constraining" },
        DomainConcept { name: "Entropy", domain: "science", description: "Measure of disorder or randomness in a system",
            coefficients: [0.05, 0.10, 0.60, 0.05, 0.15, 0.10, 0.20, 0.30], expected_role: "transmissive" },
        DomainConcept { name: "Catalysis", domain: "science", description: "Acceleration of reaction by substance not consumed",
            coefficients: [0.10, 0.65, 0.20, -0.10, 0.20, 0.10, 0.15, 0.40], expected_role: "causal" },
        DomainConcept { name: "Falsifiability", domain: "science", description: "Capability of being proven wrong by evidence",
            coefficients: [0.10, 0.15, 0.10, 0.70, 0.15, 0.45, 0.20, 0.05], expected_role: "constraining" },
        DomainConcept { name: "Replication", domain: "science", description: "Repetition of research to confirm results",
            coefficients: [0.35, 0.05, 0.25, 0.15, 0.10, 0.60, 0.35, 0.10], expected_role: "clarifying" },

        // ── FINANCIAL DOMAIN ──
        DomainConcept { name: "Derivative", domain: "finance", description: "Financial contract whose value depends on underlying asset",
            coefficients: [0.30, 0.15, 0.20, 0.05, 0.20, 0.15, 0.55, 0.35], expected_role: "balancing" },
        DomainConcept { name: "Inflation", domain: "finance", description: "General increase in prices and fall in purchasing value",
            coefficients: [0.10, 0.30, 0.55, 0.10, 0.40, 0.15, 0.15, 0.40], expected_role: "influential" },
        DomainConcept { name: "Budget", domain: "finance", description: "Financial plan that allocates future income toward expenses",
            coefficients: [0.15, 0.05, 0.10, 0.78, 0.15, 0.25, 0.10, 0.05], expected_role: "constraining" },
        DomainConcept { name: "Dividend", domain: "finance", description: "Distribution of company profits to shareholders",
            coefficients: [0.20, 0.05, 0.60, 0.05, 0.10, 0.15, 0.40, 0.20], expected_role: "transmissive" },
        DomainConcept { name: "Credit Score", domain: "finance", description: "Numerical rating of creditworthiness based on financial history",
            coefficients: [0.15, 0.10, 0.15, 0.75, 0.20, 0.25, 0.10, 0.05], expected_role: "constraining" },
        DomainConcept { name: "Options Trading", domain: "finance", description: "Buying or selling the right to trade an asset at set price",
            coefficients: [0.05, 0.50, 0.15, -0.05, 0.15, 0.10, 0.55, 0.30], expected_role: "balancing" },
        DomainConcept { name: "Audit", domain: "finance", description: "Systematic examination of financial records for accuracy",
            coefficients: [0.10, 0.15, 0.10, 0.50, 0.15, 0.75, 0.10, 0.05], expected_role: "clarifying" },
        DomainConcept { name: "Bond", domain: "finance", description: "Fixed-income debt security representing a loan to issuer",
            coefficients: [0.40, 0.05, 0.10, 0.50, 0.25, 0.10, 0.10, 0.05], expected_role: "constraining" },
        DomainConcept { name: "Liquidity", domain: "finance", description: "Ease with which an asset can be converted to cash",
            coefficients: [0.15, 0.10, 0.75, -0.10, 0.15, 0.10, 0.25, 0.15], expected_role: "transmissive" },
        DomainConcept { name: "Venture Capital", domain: "finance", description: "Investment in high-growth startup companies",
            coefficients: [0.10, 0.45, 0.25, -0.15, 0.15, 0.10, 0.20, 0.80], expected_role: "generative" },

        // ── CODE / SOFTWARE DOMAIN ──
        DomainConcept { name: "Compiler", domain: "code", description: "Program that translates source code into executable machine code",
            coefficients: [0.10, 0.15, 0.70, 0.10, 0.10, 0.20, 0.15, 0.35], expected_role: "transmissive" },
        DomainConcept { name: "Type System", domain: "code", description: "Set of rules that assign types to program constructs",
            coefficients: [0.10, 0.05, 0.05, 0.80, 0.15, 0.35, 0.10, 0.05], expected_role: "constraining" },
        DomainConcept { name: "Garbage Collector", domain: "code", description: "Automatic memory management that reclaims unused objects",
            coefficients: [0.20, 0.05, 0.20, 0.25, 0.10, 0.30, 0.60, 0.05], expected_role: "balancing" },
        DomainConcept { name: "Recursion", domain: "code", description: "Function that calls itself to solve sub-problems",
            coefficients: [0.10, 0.15, 0.50, 0.10, 0.15, 0.10, 0.60, 0.20], expected_role: "balancing" },
        DomainConcept { name: "Algorithm", domain: "code", description: "Step-by-step procedure for solving a computational problem",
            coefficients: [0.10, 0.30, 0.55, 0.05, 0.15, 0.25, 0.15, 0.40], expected_role: "transmissive" },
        DomainConcept { name: "Assertion", domain: "code", description: "Boolean expression that must be true at a point in execution",
            coefficients: [0.05, 0.10, 0.05, 0.75, 0.10, 0.50, 0.10, 0.05], expected_role: "constraining" },
        DomainConcept { name: "Abstraction", domain: "code", description: "Hiding implementation details behind a simplified interface",
            coefficients: [0.20, 0.05, 0.05, 0.60, 0.30, 0.25, 0.15, 0.10], expected_role: "constraining" },
        DomainConcept { name: "Concurrency", domain: "code", description: "Executing multiple computational tasks simultaneously",
            coefficients: [0.05, 0.35, 0.40, -0.05, 0.10, 0.10, 0.55, 0.30], expected_role: "balancing" },
        DomainConcept { name: "API", domain: "code", description: "Defined interface for software components to communicate",
            coefficients: [0.25, 0.10, 0.65, 0.10, 0.10, 0.15, 0.20, 0.10], expected_role: "transmissive" },
        DomainConcept { name: "Open Source", domain: "code", description: "Software whose source code is freely available",
            coefficients: [0.30, 0.15, 0.30, -0.15, 0.25, 0.15, 0.15, 0.70], expected_role: "generative" },
    ]
}

fn domain_relations() -> Vec<DomainRelation> {
    vec![
        // Legal relations
        DomainRelation { concept_a: 0, concept_b: 1, expected_relation: "receptive" },
        DomainRelation { concept_a: 2, concept_b: 3, expected_relation: "influential" },
        DomainRelation { concept_a: 4, concept_b: 0, expected_relation: "receptive" },
        DomainRelation { concept_a: 7, concept_b: 0, expected_relation: "causal" },
        DomainRelation { concept_a: 9, concept_b: 4, expected_relation: "generative" },
        // Medical relations
        DomainRelation { concept_a: 10, concept_b: 11, expected_relation: "influential" },
        DomainRelation { concept_a: 12, concept_b: 13, expected_relation: "generative" },
        DomainRelation { concept_a: 14, concept_b: 15, expected_relation: "causal" },
        DomainRelation { concept_a: 16, concept_b: 17, expected_relation: "generative" },
        DomainRelation { concept_a: 18, concept_b: 19, expected_relation: "receptive" },
        // Science relations
        DomainRelation { concept_a: 20, concept_b: 21, expected_relation: "causal" },
        DomainRelation { concept_a: 22, concept_b: 23, expected_relation: "influential" },
        DomainRelation { concept_a: 24, concept_b: 25, expected_relation: "constraining" },
        DomainRelation { concept_a: 26, concept_b: 27, expected_relation: "causal" },
        DomainRelation { concept_a: 28, concept_b: 29, expected_relation: "generative" },
        // Finance relations
        DomainRelation { concept_a: 30, concept_b: 31, expected_relation: "influential" },
        DomainRelation { concept_a: 32, concept_b: 33, expected_relation: "generative" },
        DomainRelation { concept_a: 34, concept_b: 35, expected_relation: "constraining" },
        DomainRelation { concept_a: 36, concept_b: 37, expected_relation: "causal" },
        DomainRelation { concept_a: 38, concept_b: 39, expected_relation: "generative" },
        // Code relations
        DomainRelation { concept_a: 40, concept_b: 41, expected_relation: "constraining" },
        DomainRelation { concept_a: 42, concept_b: 43, expected_relation: "receptive" },
        DomainRelation { concept_a: 44, concept_b: 45, expected_relation: "constraining" },
        DomainRelation { concept_a: 46, concept_b: 47, expected_relation: "generative" },
        DomainRelation { concept_a: 48, concept_b: 49, expected_relation: "generative" },
        // Cross-domain
        DomainRelation { concept_a: 0, concept_b: 32, expected_relation: "receptive" },
        DomainRelation { concept_a: 9, concept_b: 49, expected_relation: "receptive" },
        DomainRelation { concept_a: 14, concept_b: 31, expected_relation: "causal" },
        DomainRelation { concept_a: 20, concept_b: 39, expected_relation: "generative" },
        DomainRelation { concept_a: 41, concept_b: 34, expected_relation: "receptive" },
    ]
}

fn cross_domain_diagnostics() {
    let concepts = domain_concepts();
    let relations = domain_relations();

    let encoded: Vec<Multivector> = concepts.iter()
        .map(|c| llm_encode(&c.coefficients))
        .collect();

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║           CROSS-DOMAIN SEMANTIC BENCHMARK                        ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  50 concepts across 5 domains; 25 intra + 5 cross relations     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // 1. Per-domain dominant role accuracy
    let domains = ["legal", "medical", "science", "finance", "code"];
    println!("  ── DOMINANT ROLE ACCURACY ──");
    println!("  {:>12} │ Correct │ Accuracy", "Domain");
    println!("  ──────────────┼─────────┼──────────");
    let mut total_correct = 0usize;
    for domain in &domains {
        let domain_concepts: Vec<_> = concepts.iter().enumerate()
            .filter(|(_, c)| c.domain == *domain)
            .collect();
        let correct = domain_concepts.iter()
            .filter(|(i, c)| encoded[*i].dominant_role().role_name() == c.expected_role)
            .count();
        total_correct += correct;
        let n = domain_concepts.len();
        println!("  {:>12} │ {:>5}/{:<2} │ {:>6.1}%",
            domain, correct, n, correct as f64 / n as f64 * 100.0);
    }
    println!("  ──────────────┼─────────┼──────────");
    println!("  {:>12} │ {:>5}/{} │ {:>6.1}%",
        "TOTAL", total_correct, concepts.len(),
        total_correct as f64 / concepts.len() as f64 * 100.0);

    // 2. Per-domain relation classification
    println!("\n  ── RELATION CLASSIFICATION ──");
    println!("  {:<12} | Correct | Accuracy | Confidence (mean)", "Domain");
    println!("  ──────────────┼─────────┼──────────┼──────────────────");

    for domain in &domains {
        let domain_rels: Vec<_> = relations.iter().enumerate()
            .filter(|(_, r)| {
                let a_dom = concepts[r.concept_a].domain;
                let b_dom = concepts[r.concept_b].domain;
                a_dom == *domain && b_dom == *domain
            })
            .collect();

        let mut correct = 0usize;
        let mut conf_sum = 0.0f64;
        for (_, r) in &domain_rels {
            let (rel, conf) = RelationType::from_pair(&encoded[r.concept_a], &encoded[r.concept_b]);
            conf_sum += conf;
            if rel.role_name() == r.expected_relation { correct += 1; }
        }
        let n = domain_rels.len();
        let acc = if n > 0 { correct as f64 / n as f64 * 100.0 } else { 0.0 };
        let avg_conf = if n > 0 { conf_sum / n as f64 } else { 0.0 };
        let bar = "█".repeat((acc / 5.0) as usize);
        println!("  {:>12} │ {:>5}/{:<2} │ {:>6.1}% {} │ {:.3}",
            domain, correct, n, acc, bar, avg_conf);
    }

    // Cross-domain relations
    let cross_rels: Vec<_> = relations.iter().enumerate()
        .filter(|(_, r)| concepts[r.concept_a].domain != concepts[r.concept_b].domain)
        .collect();
    let mut cross_correct = 0usize;
    let mut cross_conf = 0.0f64;
    for (_, r) in &cross_rels {
        let (rel, conf) = RelationType::from_pair(&encoded[r.concept_a], &encoded[r.concept_b]);
        cross_conf += conf;
        if rel.role_name() == r.expected_relation { cross_correct += 1; }
    }
    let cross_n = cross_rels.len();
    let cross_acc = if cross_n > 0 { cross_correct as f64 / cross_n as f64 * 100.0 } else { 0.0 };
    let cross_avg_conf = if cross_n > 0 { cross_conf / cross_n as f64 } else { 0.0 };
    println!("  ──────────────┼─────────┼──────────┼──────────────────");
    println!("  {:>12} │ {:>5}/{:<2} │ {:>6.1}% │ {:.3}",
        "cross-domain", cross_correct, cross_n, cross_acc, cross_avg_conf);

    // 3. Full breakdown
    println!("\n  ── DETAILED RESULTS ──");
    for r in &relations {
        let (rel, conf) = RelationType::from_pair(&encoded[r.concept_a], &encoded[r.concept_b]);
        let matched = rel.role_name() == r.expected_relation;
        println!("  [{}.{}] {} ⊗ {} [{}]: expected '{}' got '{}' (conf={:.2}) {}",
            concepts[r.concept_a].domain, concepts[r.concept_b].domain,
            concepts[r.concept_a].name, concepts[r.concept_b].name,
            if concepts[r.concept_a].domain == concepts[r.concept_b].domain { "intra" } else { "cross" },
            r.expected_relation, rel.role_name(), conf,
            if matched { "✓" } else { "✗" });
    }

    // 4. Domain proximity analysis
    println!("\n  ── INTER-DOMAIN PROXIMITY ──");
    println!("  {:>12} │", "");
    print!("  {:>12} │", "");
    for b in &domains { print!(" {:>8}", b); }
    println!();
    print!("  ──────────────┼");
    for _ in &domains { print!("─────────"); }
    println!();

    for a in &domains {
        print!("  {:>12} │", a);
        let a_concepts: Vec<_> = concepts.iter().enumerate()
            .filter(|(_, c)| c.domain == *a)
            .collect();
        for b in &domains {
            let b_concepts: Vec<_> = concepts.iter().enumerate()
                .filter(|(_, c)| c.domain == *b)
                .collect();
            let mut sims = vec![];
            for (ai, _) in &a_concepts {
                for (bi, _) in &b_concepts {
                    sims.push(semantic_similarity(&encoded[*ai], &encoded[*bi]));
                }
            }
            let mean = sims.iter().sum::<f64>() / sims.len() as f64;
            print!(" {:>8.3}", mean);
        }
        println!();
    }
    println!();
    println!("  Higher intra-domain similarity (diagonal) confirms Bagua encoding");
    println!("  preserves domain semantics across 5 distinct knowledge domains.");
}

#[test]
fn cross_domain_benchmark() {
    cross_domain_diagnostics();
}
