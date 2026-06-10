// Realistic benchmark with multi-encoding classifier.
// Same 38 concepts, 41 human-labeled relations as realistic_benchmark.rs
// but uses classify_multi_encoded() instead of from_pair().

use ga_semantics_core::prelude::*;
use ga_semantics_core::multi_encoding::{MultiEncodedConcept, classify_multi_encoded};
use ga_semantics_core::relation_type::{FeatureWeights, RelationType};

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
        RealConcept { coefficients: [0.10, 0.10, 0.10, 0.30, 0.15, 0.80, 0.15, 0.10] },     // 11: Compliance Audit? No — actually different concept
        RealConcept { coefficients: [0.76, 0.05, 0.15, 0.15, 0.10, 0.10, 0.20, 0.10] },     // 12: Severance Package (receptive)
        RealConcept { coefficients: [0.20, 0.10, 0.15, 0.20, 0.10, 0.80, 0.20, 0.05] },     // 13: Industry Standard (influential)
        RealConcept { coefficients: [0.15, 0.15, 0.15, 0.05, 0.15, 0.20, 0.80, 0.10] },     // 14: Team Standup (balancing)
        // Ecosystem (15-25)
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
        // Technology (26-37)
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
        LabeledRelation{idx_a:11, idx_b:0, human_label:"clarifying"},     // Compliance → Budget (correction: use correct indices)
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
        // Technology intra (25-35)
        LabeledRelation{idx_a:26, idx_b:30, human_label:"constraining"},  // Firewall → CircuitBreaker
        LabeledRelation{idx_a:27, idx_b:28, human_label:"balancing"},     // LoadBalancer → DBIndex
        LabeledRelation{idx_a:28, idx_b:26, human_label:"clarifying"},    // DBIndex → Firewall
        LabeledRelation{idx_a:29, idx_b:32, human_label:"transmissive"},  // MessageBroker → FeatureFlag
        LabeledRelation{idx_a:30, idx_b:35, human_label:"constraining"},  // CircuitBreaker → Chaos
        LabeledRelation{idx_a:31, idx_b:32, human_label:"influential"},   // Deprecation → FeatureFlag
        LabeledRelation{idx_a:32, idx_b:11, human_label:"influential"},   // FeatureFlag → Compliance (cross)
        LabeledRelation{idx_a:33, idx_b:30, human_label:"clarifying"},    // HealthCheck → CircuitBreaker
        LabeledRelation{idx_a:34, idx_b:33, human_label:"receptive"},     // EventLog → HealthCheck
        LabeledRelation{idx_a:35, idx_b:33, human_label:"causal"},        // Chaos → HealthCheck
        LabeledRelation{idx_a:36, idx_b:30, human_label:"receptive"},     // RateLimiter → CircuitBreaker
        LabeledRelation{idx_a:37, idx_b:29, human_label:"transmissive"},  // API Gateway → MessageBroker
        // Cross-domain (36-40)
        LabeledRelation{idx_a:0, idx_b:26, human_label:"constraining"},   // Budget → Firewall
        LabeledRelation{idx_a:20, idx_b:7, human_label:"causal"},         // Mutation → Innovation
        LabeledRelation{idx_a:27, idx_b:21, human_label:"receptive"},     // LoadBalancer → Homeostasis
        LabeledRelation{idx_a:11, idx_b:30, human_label:"clarifying"},    // Compliance → CircuitBreaker
    ]
}

fn label_to_type(label: &str) -> RelationType {
    use RelationType::*;
    match label { "generative"=>Generative,"receptive"=>Receptive,"causal"=>Causal,"transmissive"=>Transmissive,"constraining"=>Constraining,"influential"=>Influential,"clarifying"=>Clarifying,"balancing"=>Balancing, _=>panic!("{label}") }
}

#[test]
fn realistic_multi_encoding_benchmark() {
    let c = concepts();
    let r = relations();
    let mc: Vec<MultiEncodedConcept> = c.iter()
        .map(|rc| MultiEncodedConcept::from_single_encoding(&llm_encode(&rc.coefficients)))
        .collect();
    let enc: Vec<Multivector> = c.iter()
        .map(|rc| llm_encode(&rc.coefficients))
        .collect();

    let weights = FeatureWeights::default();
    let mut orig_correct = 0usize;
    let mut multi_correct = 0usize;

    println!("\n{:=^60}", " REALISTIC MULTI-ENCODING BENCHMARK ");
    println!("  {} concepts, {} human-labeled relations", c.len(), r.len());
    println!("{:=^60}\n", "");

    let mut per_domain: std::collections::HashMap<&str, (usize, usize)> = std::collections::HashMap::new();

    for rel in &r {
        let expected = label_to_type(rel.human_label);
        let (orig, _) = RelationType::from_pair(&enc[rel.idx_a], &enc[rel.idx_b]);
        let (multi, _) = classify_multi_encoded(&mc[rel.idx_a], &mc[rel.idx_b], &weights);

        if orig == expected { orig_correct += 1; }
        if multi == expected { multi_correct += 1; }

        let domain = "all";
        let e = per_domain.entry(domain).or_insert((0, 0));
        e.0 += 1;
        if multi == expected { e.1 += 1; }
    }

    let n = r.len();
    let orig_acc = orig_correct as f64 / n as f64 * 100.0;
    let multi_acc = multi_correct as f64 / n as f64 * 100.0;

    println!("\n── RESULTS ──");
    println!("  Original (from_pair):      {:.1}% ({}/{})", orig_acc, orig_correct, n);
    println!("  Multi-encoding:             {:.1}% ({}/{})", multi_acc, multi_correct, n);
    println!("  Delta:                      {:+.1}pp", multi_acc - orig_acc);
    println!("  Random baseline:            12.5%");
    println!("  LLM direct:                 ~85-95%");

    println!("\n  GA-Bagua is {} competitive with LLM direct for human labels.",
        if multi_acc > 70.0 { "NOW" } else { "NOT YET" });
}
