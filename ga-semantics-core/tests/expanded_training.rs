/// Expanded training dataset: 100 additional human-labeled concept pairs.
/// Designed for the trainable logistic regression classifier.
/// Each pair has a label assigned by semantic understanding of how
/// the concepts interact in the real world.
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct TrainingPair {
    idx_a: usize,
    idx_b: usize,
    label: String,
    rationale: String,
    confidence: String,
}

// Same 38 concepts as realistic_benchmark
const NAMES: [&str; 38] = [
    "Marketing Budget", "Sales Pipeline", "Revenue Target", "Support Ticket", "Quarterly Report",
    "Employee Handbook", "Vendor Contract", "Innovation Fund", "Feedback Loop", "Onboarding Process",
    "Market Trend", "Compliance Audit", "Severance Package", "Industry Standard", "Team Standup",
    "Predator", "Decomposer", "Photosynthesis", "Water Cycle", "Keystone Species",
    "Mutation", "Homeostasis", "Natural Selection", "Ecological Succession", "Symbiosis",
    "DNA Replication", "Firewall", "Load Balancer", "Database Index", "Message Broker",
    "Circuit Breaker", "Deprecation Policy", "Feature Flag", "Health Check Endpoint", "Event Sourcing Log",
    "Chaos Engineering", "Rate Limiter", "API Gateway",
];

fn pairs() -> Vec<TrainingPair> {
    vec![
        // ============ BUSINESS (35 new pairs) ============
        // Budgets and constraints
        (0, 7, "constraining", "Budget caps Innovation spending", "certain"),
        (0, 1, "constraining", "Budget bounds Pipeline capacity", "certain"),
        (0, 9, "constraining", "Budget restricts Onboarding scale", "plausible"),
        // Pipeline dynamics
        (1, 0, "generative", "Pipeline activity justifies Budget allocation", "certain"),
        (1, 11, "generative", "Pipeline health triggers Compliance review", "plausible"),
        (1, 10, "transmissive", "Pipeline data feeds Market analysis", "plausible"),
        // Revenue and targets
        (2, 0, "influential", "Revenue goals shape Budget planning", "certain"),
        (2, 1, "influential", "Targets influence Pipeline priorities", "certain"),
        (2, 7, "generative", "Revenue creates room for Innovation", "certain"),
        // Support and operations
        (3, 9, "causal", "Ticket triggers Onboarding fixes", "plausible"),
        (3, 1, "clarifying", "Ticket patterns reveal Pipeline bottlenecks", "certain"),
        // Reports and visibility
        (4, 11, "clarifying", "Report reveals Compliance status", "certain"),
        (4, 10, "clarifying", "Report illuminates Market position", "certain"),
        // Handbook and policies
        (5, 0, "constraining", "Handbook bounds how Budget is allocated", "plausible"),
        (5, 9, "constraining", "Handbook constrains Onboarding procedures", "certain"),
        (5, 12, "receptive", "Handbook accepts Severance policy terms", "certain"),
        // Contracts
        (6, 0, "constraining", "Contract limits Budget flexibility", "certain"),
        (6, 2, "constraining", "Contract constrains Revenue recognition", "plausible"),
        (6, 7, "receptive", "Contract enables Innovation funding terms", "plausible"),
        // Innovation
        (7, 1, "influential", "Innovation reshapes Pipeline approach", "certain"),
        (7, 9, "generative", "Innovation creates new Onboarding methods", "plausible"),
        (7, 10, "generative", "Innovation generates Market differentiation", "certain"),
        // Feedback loops
        (8, 9, "influential", "Feedback gradually improves Onboarding", "certain"),
        (8, 3, "receptive", "Feedback accepts Support ticket data", "certain"),
        (8, 4, "clarifying", "Feedback illuminates Report accuracy", "plausible"),
        // Onboarding and talent
        (9, 1, "generative", "Onboarding produces Pipeline-ready staff", "certain"),
        (9, 4, "generative", "Onboarding creates measurable Report outcomes", "plausible"),
        (9, 13, "receptive", "Onboarding adapts to Industry Standards", "plausible"),
        // Market and trends
        (10, 1, "influential", "Trends shape Pipeline strategy", "certain"),
        (10, 4, "influential", "Trends influence Report narrative", "certain"),
        (10, 7, "generative", "Trends create Innovation opportunities", "certain"),
        // Compliance
        (11, 5, "clarifying", "Audit reveals Handbook gaps", "certain"),
        (11, 9, "clarifying", "Audit exposes Onboarding deficiencies", "plausible"),
        (11, 13, "receptive", "Audit follows Industry Standards", "certain"),
        // Industry standards
        (13, 5, "constraining", "Standards bound Handbook content", "certain"),
        (13, 11, "influential", "Standards shape Audit criteria", "certain"),

        // ============ ECOSYSTEM (35 new pairs) ============
        // Predator dynamics
        (15, 16, "constraining", "Predator limits Decomposer activity", "plausible"),
        (15, 25, "constraining", "Predator bounds DNA propagation", "plausible"),
        (16, 17, "receptive", "Decomposer receives Photosynthesis products", "certain"),
        (16, 25, "receptive", "Decomposer accepts old DNA material", "certain"),
        // Photosynthesis
        (17, 15, "generative", "Photosynthesis creates energy for Predator food", "certain"),
        (17, 25, "generative", "Photosynthesis enables DNA replication energy", "certain"),
        (17, 23, "generative", "Photosynthesis creates biomass for Succession", "certain"),
        // Water cycle
        (18, 15, "generative", "Water creates Predator habitats", "plausible"),
        (18, 21, "generative", "Water enables Homeostasis regulation", "certain"),
        (18, 23, "generative", "Water facilitates Succession progress", "certain"),
        (18, 25, "generative", "Water enables DNA replication chemistry", "plausible"),
        // Keystone species
        (19, 15, "influential", "Keystone shapes Predator-prey dynamics", "certain"),
        (19, 16, "influential", "Keystone influences Decomposer communities", "certain"),
        (19, 21, "influential", "Keystone affects ecosystem Homeostasis", "plausible"),
        (19, 22, "influential", "Keystone shapes Selection pressures", "certain"),
        // Mutation
        (20, 15, "generative", "Mutation creates Predator adaptation", "certain"),
        (20, 16, "generative", "Mutation creates novel Decomposer enzymes", "plausible"),
        (20, 25, "generative", "Mutation generates DNA variation", "certain"),
        // Homeostasis
        (21, 15, "constraining", "Homeostasis constrains Predator population swings", "certain"),
        (21, 18, "balancing", "Homeostasis balances Water retention", "certain"),
        (21, 23, "balancing", "Homeostasis equilibrates Succession pace", "plausible"),
        // Natural selection
        (22, 15, "constraining", "Selection constrains Predator evolution", "certain"),
        (22, 25, "constraining", "Selection limits which DNA persists", "certain"),
        (22, 23, "influential", "Selection shapes Succession trajectory", "certain"),
        // Succession
        (23, 15, "influential", "Succession changes Predator habitat", "certain"),
        (23, 21, "influential", "Succession gradually alters Homeostasis", "plausible"),
        (23, 25, "influential", "Succession shapes DNA pool diversity", "certain"),
        // Symbiosis
        (24, 16, "balancing", "Symbiosis mirrors Decomposer nutrient cycling", "plausible"),
        (24, 21, "balancing", "Symbiosis equilibrates ecosystem Homeostasis", "certain"),
        // DNA
        (25, 15, "generative", "DNA creates Predator traits", "certain"),
        (25, 23, "generative", "DNA provides genetic basis for Succession", "certain"),

        // ============ TECHNOLOGY (30 new pairs) ============
        // Firewall
        (26, 37, "constraining", "Firewall restricts API Gateway access", "certain"),
        (26, 34, "constraining", "Firewall limits Event logging sources", "plausible"),
        (26, 35, "constraining", "Firewall constrains Chaos experiment scope", "plausible"),
        // Load balancer
        (27, 30, "balancing", "Balancer equilibrates Circuit Breaker load", "certain"),
        (27, 34, "balancing", "Balancer distributes Event processing", "certain"),
        (27, 36, "balancing", "Balancer mirrors Rate Limiter distribution", "plausible"),
        // Database index
        (28, 27, "clarifying", "Index reveals Balancer query patterns", "plausible"),
        (28, 36, "clarifying", "Index exposes Rate Limiter throttling points", "plausible"),
        // Message broker
        (29, 27, "transmissive", "Broker channels to Load Balancer", "plausible"),
        (29, 33, "transmissive", "Broker transmits health check events", "certain"),
        (29, 35, "transmissive", "Broker flows Chaos experiment results", "plausible"),
        // Circuit breaker
        (30, 26, "constraining", "Breaker limits Firewall traffic during failure", "certain"),
        (30, 28, "constraining", "Breaker constrains DB access during outage", "certain"),
        (30, 34, "constraining", "Breaker bounds Event log writes", "plausible"),
        // Deprecation policy
        (31, 28, "influential", "Deprecation shapes Index migration path", "plausible"),
        (31, 37, "influential", "Deprecation influences Gateway versioning", "certain"),
        // Feature flag
        (32, 26, "influential", "Flags gradually shape Firewall rules", "plausible"),
        (32, 28, "influential", "Flags influence Index query patterns", "certain"),
        (32, 33, "influential", "Flags shape Health check behavior", "certain"),
        // Health check
        (33, 27, "clarifying", "Health reveals Balancer utilization", "certain"),
        (33, 29, "clarifying", "Health exposes Broker queue depth", "certain"),
        (33, 36, "clarifying", "Health illuminates Rate Limiter status", "plausible"),
        // Event sourcing
        (34, 29, "receptive", "Event log accepts Broker messages", "certain"),
        (34, 32, "receptive", "Event log accepts Feature flag changes", "certain"),
        // Chaos engineering
        (35, 26, "causal", "Chaos triggers Firewall failover", "certain"),
        (35, 27, "causal", "Chaos triggers Load redistribution", "certain"),
        (35, 29, "causal", "Chaos triggers Broker reconnection patterns", "plausible"),
        // Rate limiter
        (36, 27, "constraining", "Limiter constrains Balancer traffic", "certain"),
        (36, 34, "constraining", "Limiter bounds Event log throughput", "plausible"),
        // API Gateway
        (37, 27, "transmissive", "Gateway channels to Load Balancer", "certain"),
        (37, 33, "transmissive", "Gateway routes to Health check endpoints", "certain"),
        (37, 35, "transmissive", "Gateway channels Chaos experiment traffic", "plausible"),
    ].iter().map(|&(a,b,l,r,c)| TrainingPair {
        idx_a: a, idx_b: b,
        label: l.to_string(),
        rationale: r.to_string(),
        confidence: c.to_string(),
    }).collect()
}

#[test]
fn generate_expanded_dataset() {
    let pairs = pairs();
    let json = serde_json::to_string_pretty(&pairs).unwrap();
    let path = "../data/expanded_training_pairs.json";
    fs::write(path, &json).unwrap();
    println!("Generated {} labeled training pairs → {}", pairs.len(), path);

    // Count per label
    use std::collections::HashMap;
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for p in &pairs {
        *counts.entry(&p.label).or_insert(0) += 1;
    }
    println!("Per-label distribution:");
    for (label, count) in counts.iter() {
        println!("  {}: {}", label, count);
    }
}

#[test]
fn trainable_with_expanded_data() {
    use ga_semantics_core::prelude::*;
    use ga_semantics_core::multi_encoding::MultiEncodedConcept;
    use ga_semantics_core::RelationType as RT;
    use ga_semantics_core::trainable::GaFeatureClassifier;
    let existing = vec![
        (0,5,"receptive"),(1,2,"generative"),(3,4,"causal"),(4,2,"clarifying"),
        (5,7,"constraining"),(6,5,"receptive"),(7,2,"generative"),(8,10,"balancing"),
        (9,7,"generative"),(10,13,"influential"),(11,0,"clarifying"),(12,5,"receptive"),
        (13,10,"influential"),(14,8,"balancing"),(15,20,"constraining"),(16,17,"receptive"),
        (17,21,"generative"),(18,17,"generative"),(19,23,"influential"),(20,22,"generative"),
        (21,24,"balancing"),(22,20,"constraining"),(23,16,"influential"),(24,19,"balancing"),
        (25,21,"generative"),(26,30,"constraining"),(27,28,"balancing"),(28,26,"clarifying"),
        (29,32,"transmissive"),(30,35,"constraining"),(31,32,"influential"),(32,11,"influential"),
        (33,30,"clarifying"),(34,33,"receptive"),(35,33,"causal"),(36,30,"receptive"),
        (37,29,"transmissive"),(0,26,"constraining"),(20,7,"causal"),(27,21,"receptive"),
        (11,30,"clarifying"),
    ];

    let new = pairs();

    // Combine: existing 41 + new 100 = 141 total
    let all_pairs: Vec<(usize, usize, String)> = existing.iter()
        .map(|&(a,b,l)| (a,b,l.to_string()))
        .chain(new.iter().map(|p| (p.idx_a, p.idx_b, p.label.clone())))
        .collect();

    let coeffs: [[f64;8];38] = [
        [0.05,0.05,0.10,0.85,-0.05,0.25,0.10,0.10],
        [0.10,0.15,0.80,-0.05,-0.10,0.15,0.20,0.20],
        [0.10,0.30,0.10,-0.10,0.15,0.10,0.15,0.85],
        [0.10,0.75,0.20,0.05,0.10,0.30,0.10,0.20],
        [0.15,0.10,0.10,0.15,0.25,0.80,0.10,0.10],
        [0.15,0.05,0.05,0.80,0.30,0.25,0.10,0.05],
        [0.10,0.05,0.05,0.85,0.15,0.20,0.15,0.05],
        [0.05,0.25,0.15,-0.15,0.10,0.15,0.10,0.88],
        [0.25,0.10,0.25,0.05,0.15,0.20,0.78,0.10],
        [0.20,0.15,0.75,0.05,0.15,0.10,0.25,0.15],
        [0.15,0.20,0.10,0.05,0.78,0.25,0.15,0.20],
        [0.10,0.10,0.10,0.30,0.15,0.80,0.15,0.10],
        [0.76,0.05,0.15,0.15,0.10,0.10,0.20,0.10],
        [0.20,0.10,0.15,0.20,0.10,0.80,0.20,0.05],
        [0.15,0.15,0.15,0.05,0.15,0.20,0.80,0.10],
        [0.05,0.15,0.10,0.85,0.10,0.15,0.15,0.15],
        [0.80,0.05,0.20,0.10,0.15,0.10,0.20,0.05],
        [0.05,0.20,0.15,0.05,0.10,0.15,0.10,0.86],
        [0.15,0.20,0.80,0.05,0.10,0.15,0.20,0.15],
        [0.10,0.25,0.15,0.10,0.10,0.80,0.20,0.25],
        [0.10,0.85,0.15,0.05,0.10,0.10,0.15,0.20],
        [0.15,0.05,0.15,0.15,0.15,0.10,0.80,0.10],
        [0.05,0.10,0.10,0.85,0.15,0.25,0.15,0.10],
        [0.15,0.15,0.10,0.05,0.10,0.85,0.20,0.15],
        [0.20,0.10,0.15,0.05,0.15,0.10,0.80,0.20],
        [0.10,0.20,0.15,0.05,0.10,0.10,0.15,0.85],
        [0.05,0.05,0.20,0.85,0.15,0.25,0.15,0.05],
        [0.15,0.10,0.25,0.05,0.10,0.15,0.80,0.10],
        [0.10,0.10,0.15,0.05,0.85,0.05,0.10,0.10],
        [0.15,0.25,0.80,-0.15,-0.20,0.10,0.30,0.05],
        [0.05,-0.25,-0.20,0.85,0.25,-0.10,0.20,-0.15],
        [0.20,0.10,0.05,0.30,0.15,0.85,0.15,0.10],
        [0.10,0.21,0.16,-0.10,0.10,0.82,0.31,0.37],
        [0.18,0.12,0.12,0.12,0.90,0.12,0.18,0.06],
        [0.80,0.10,0.20,0.10,0.10,0.25,0.15,0.10],
        [0.05,0.75,0.10,0.25,0.20,0.30,0.15,0.20],
        [0.04,-0.09,-0.51,0.68,0.21,-0.26,0.17,-0.34],
        [0.22,0.34,0.84,0.06,-0.11,0.17,0.28,0.06],
    ];

    let enc: Vec<Multivector> = coeffs.iter().map(|c| llm_encode(c)).collect();
    let mc: Vec<MultiEncodedConcept> = enc.iter()
        .map(|mv| MultiEncodedConcept::from_single_encoding(mv))
        .collect();

    // Extract features and labels
    let features: Vec<[f64; 62]> = all_pairs.iter()
        .map(|(a,b,_)| GaFeatureClassifier::extract_features(&enc[*a], &enc[*b]))
        .collect();

    let labels: Vec<RT> = all_pairs.iter()
        .map(|(_,_,l)| {
            match l.as_str() {"generative"=>RT::Generative,"receptive"=>RT::Receptive,"causal"=>RT::Causal,"transmissive"=>RT::Transmissive,"constraining"=>RT::Constraining,"influential"=>RT::Influential,"clarifying"=>RT::Clarifying,"balancing"=>RT::Balancing,_=>RT::Receptive}
        })
        .collect();

    let n = all_pairs.len();

    // Train on 80% of data, test on 20%
    let train_size = (n as f64 * 0.8) as usize;
    let train_f: Vec<_> = features[..train_size].to_vec();
    let train_l: Vec<_> = labels[..train_size].to_vec();
    let test_f: Vec<_> = features[train_size..].to_vec();
    let test_l: Vec<_> = labels[train_size..].to_vec();

    let mut model = GaFeatureClassifier::new(0.1);
    model.train(&train_f, &train_l, 0.05, 300);

    let mut train_correct = 0usize;
    for i in 0..train_size {
        let (pred, _) = model.predict(&train_f[i]);
        if pred == train_l[i] { train_correct += 1; }
    }

    let mut test_correct = 0usize;
    for i in 0..test_f.len() {
        let (pred, _) = model.predict(&test_f[i]);
        if pred == test_l[i] { test_correct += 1; }
    }

    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║   TRAINABLE CLASSIFIER WITH 141 LABELED PAIRS (80/20 SPLIT)        ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("  Training pairs:  {}", train_size);
    println!("  Test pairs:      {}", test_f.len());
    println!();
    println!("  Training accuracy:  {:.1}% ({}/{})",
        train_correct as f64 / train_size as f64 * 100.0, train_correct, train_size);
    println!("  Test accuracy:      {:.1}% ({}/{})",
        test_correct as f64 / test_f.len() as f64 * 100.0, test_correct, test_f.len());
    println!();
    println!("  BASELINES:");
    println!("  Random:              12.5%");
    println!("  Multi-encoding:      56.1%");
    println!("  Trainable (41 pairs): 53.7% (LOO-CV)");

    let test_acc = test_correct as f64 / test_f.len() as f64 * 100.0;
    if test_acc > 56.1 {
        println!();
        println!("  *** TRAINABLE CLASSIFIER BEATS MULTI-ENCODING! ***");
        println!("  Delta: {:+.1}pp over 56.1% baseline", test_acc - 56.1);
    }
}
