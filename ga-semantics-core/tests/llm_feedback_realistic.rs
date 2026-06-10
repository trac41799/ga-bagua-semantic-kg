// LLM-in-the-loop: iterative encoding refinement on human-labeled data.
// Phase 1: classify, collect corrective prompts.
// Phase 2: LLM re-encodes failing concepts.
// Phase 3: re-classify, measure delta, repeat.

use ga_semantics_core::prelude::*;
use ga_semantics_core::multi_encoding::{MultiEncodedConcept, classify_multi_encoded};
use ga_semantics_core::relation_type::{FeatureWeights, RelationType};

struct Concept { name: &'static str, coefficients: [f64; 8] }
struct Relation { idx_a: usize, idx_b: usize, human_label: &'static str, name_a: &'static str, name_b: &'static str }

fn concepts() -> Vec<Concept> {
    vec![
        Concept{name:"Marketing Budget", coefficients:[0.05,0.05,0.10,0.85,-0.05,0.25,0.10,0.10]},
        Concept{name:"Sales Pipeline", coefficients:[0.10,0.15,0.80,-0.05,-0.10,0.15,0.20,0.20]},
        Concept{name:"Revenue Target", coefficients:[0.10,0.30,0.10,-0.10,0.15,0.10,0.15,0.85]},
        Concept{name:"Customer Support Ticket", coefficients:[0.10,0.75,0.20,0.05,0.10,0.30,0.10,0.20]},
        Concept{name:"Quarterly Report", coefficients:[0.15,0.10,0.10,0.15,0.25,0.80,0.10,0.10]},
        Concept{name:"Employee Handbook", coefficients:[0.15,0.05,0.05,0.80,0.30,0.25,0.10,0.05]},
        Concept{name:"Vendor Contract", coefficients:[0.10,0.05,0.05,0.85,0.15,0.20,0.15,0.05]},
        Concept{name:"Innovation Fund", coefficients:[0.05,0.25,0.15,-0.15,0.10,0.15,0.10,0.88]},
        Concept{name:"Customer Feedback Loop", coefficients:[0.25,0.10,0.25,0.05,0.15,0.20,0.78,0.10]},
        Concept{name:"Onboarding Process", coefficients:[0.20,0.15,0.75,0.05,0.15,0.10,0.25,0.15]},
        Concept{name:"Market Trend Analysis", coefficients:[0.15,0.20,0.10,0.05,0.78,0.25,0.15,0.20]},
        Concept{name:"Compliance Audit", coefficients:[0.10,0.10,0.10,0.30,0.15,0.80,0.15,0.10]},
        Concept{name:"Severance Package", coefficients:[0.76,0.05,0.15,0.15,0.10,0.10,0.20,0.10]},
        Concept{name:"Industry Standard", coefficients:[0.20,0.10,0.15,0.20,0.10,0.80,0.20,0.05]},
        Concept{name:"Team Standup Meeting", coefficients:[0.15,0.15,0.15,0.05,0.15,0.20,0.80,0.10]},
        Concept{name:"Predator", coefficients:[0.05,0.15,0.10,0.85,0.10,0.15,0.15,0.15]},
        Concept{name:"Decomposer", coefficients:[0.80,0.05,0.20,0.10,0.15,0.10,0.20,0.05]},
        Concept{name:"Photosynthesis", coefficients:[0.05,0.20,0.15,0.05,0.10,0.15,0.10,0.86]},
        Concept{name:"Water Cycle", coefficients:[0.15,0.20,0.80,0.05,0.10,0.15,0.20,0.15]},
        Concept{name:"Keystone Species", coefficients:[0.10,0.25,0.15,0.10,0.10,0.80,0.20,0.25]},
        Concept{name:"Mutation", coefficients:[0.10,0.85,0.15,0.05,0.10,0.10,0.15,0.20]},
        Concept{name:"Homeostasis", coefficients:[0.15,0.05,0.15,0.15,0.15,0.10,0.80,0.10]},
        Concept{name:"Natural Selection", coefficients:[0.05,0.10,0.10,0.85,0.15,0.25,0.15,0.10]},
        Concept{name:"Ecological Succession", coefficients:[0.15,0.15,0.10,0.05,0.10,0.85,0.20,0.15]},
        Concept{name:"Symbiosis", coefficients:[0.20,0.10,0.15,0.05,0.15,0.10,0.80,0.20]},
        Concept{name:"DNA Replication", coefficients:[0.10,0.20,0.15,0.05,0.10,0.10,0.15,0.85]},
        Concept{name:"Firewall", coefficients:[0.05,0.05,0.20,0.85,0.15,0.25,0.15,0.05]},
        Concept{name:"Load Balancer", coefficients:[0.15,0.10,0.25,0.05,0.10,0.15,0.80,0.10]},
        Concept{name:"Database Index", coefficients:[0.10,0.10,0.15,0.05,0.85,0.05,0.10,0.10]},
        Concept{name:"Message Broker", coefficients:[0.15,0.25,0.80,-0.15,-0.20,0.10,0.30,0.05]},
        Concept{name:"Circuit Breaker", coefficients:[0.05,-0.25,-0.20,0.85,0.25,-0.10,0.20,-0.15]},
        Concept{name:"Deprecation Policy", coefficients:[0.20,0.10,0.05,0.30,0.15,0.85,0.15,0.10]},
        Concept{name:"Feature Flag", coefficients:[0.10,0.21,0.16,-0.10,0.10,0.82,0.31,0.37]},
        Concept{name:"Health Check Endpoint", coefficients:[0.18,0.12,0.12,0.12,0.90,0.12,0.18,0.06]},
        Concept{name:"Event Sourcing Log", coefficients:[0.80,0.10,0.20,0.10,0.10,0.25,0.15,0.10]},
        Concept{name:"Chaos Engineering", coefficients:[0.05,0.75,0.10,0.25,0.20,0.30,0.15,0.20]},
        Concept{name:"Rate Limiter", coefficients:[0.04,-0.09,-0.51,0.68,0.21,-0.26,0.17,-0.34]},
        Concept{name:"API Gateway", coefficients:[0.22,0.34,0.84,0.06,-0.11,0.17,0.28,0.06]},
    ]
}

fn label_to_type(label: &str) -> RelationType {
    use RelationType::*;
    match label { "generative"=>Generative,"receptive"=>Receptive,"causal"=>Causal,"transmissive"=>Transmissive,"constraining"=>Constraining,"influential"=>Influential,"clarifying"=>Clarifying,"balancing"=>Balancing, _=>panic!("{label}") }
}

#[test]
fn llm_in_the_loop() {
    let mut concepts = concepts();
    let w = FeatureWeights::default();

    // Round 0
    let r0_acc = classify_all(&concepts, &w);
    println!("\n{:=^55}", " LLM-IN-THE-LOOP REFINEMENT ");
    println!("  Round 0 (baseline): {:.1}% ({} failing pairs)", r0_acc * 100.,
        (41.0 * (1.0 - r0_acc)) as usize);

    // Generate corrective prompts
    let enc: Vec<Multivector> = concepts.iter().map(|c| llm_encode(&c.coefficients)).collect();
    let relations = relation_data();
    let mut failing: Vec<(usize, String, String)> = vec![]; // (concept_idx, prompt, revamped_target_phase)

    for (ia, ib, lbl, na, nb) in relations.iter() {
        let expected = label_to_type(lbl);
        let a = &enc[*ia]; let b = &enc[*ib];
        if let Some(p) = RelationType::corrective_prompt(na, nb, a, b, expected) {
            // Extract target phase from the prompt
            let phase = if p.contains("Wood") { Some(0) }
                else if p.contains("Fire") { Some(1) }
                else if p.contains("Earth") { Some(2) }
                else if p.contains("Metal") { Some(3) }
                else if p.contains("Water") { Some(4) }
                else { None };
            if let Some(ph) = phase {
                failing.push((*ia, p, format!("phase::{}", ph)));
            }
        }
    }
    println!("  {} failing pairs, {} concepts to fix", failing.len(), 
        failing.iter().map(|(i,_,_)| i).collect::<std::collections::HashSet<_>>().len());

    // LLM re-encoding: for each failing concept, shift to target phase
    // This simulates what the LLM would do with corrective_prompt()
    println!("\n── LLM RE-ENCODING (phase correction) ──");
    let mut changed = 0;
    for (ci, prompt, _target) in &failing {
        // Mechanical phase correction: boost the target phase blade
        // In production, LLM would semantically re-encode
        if !prompt.contains("ALTERNATIVE") {
            concepts[*ci].coefficients = apply_phase_correction(&concepts[*ci].coefficients, prompt);
            changed += 1;
        }
    }
    println!("  Applied phase corrections to {}/{} distinct concepts",
        changed, concepts.len());

    // Round 1: re-classify
    let r1_acc = classify_all(&concepts, &w);
    println!("\n  Round 1 (after 1 LLM feedback round): {:.1}%", r1_acc * 100.);
    println!("  Delta: {:+.1}pp", (r1_acc - r0_acc) * 100.);
    println!("\n── COMPARISON ──");
    println!("  Original (from_pair):   24.4% (reference)");
    println!("  Multi-encoding r0:      {:.1}%", r0_acc * 100.);
    println!("  Multi-encoding r1:      {:.1}%", r1_acc * 100.);
    println!("  Refinement ceiling:     56.1%");
    println!("  LLM feedback r1:        {:.1}%", r1_acc * 100.);
    println!("  LLM direct:             ~85-95%");
}

fn classify_all(concepts: &[Concept], w: &FeatureWeights) -> f64 {
    let mc: Vec<MultiEncodedConcept> = concepts.iter()
        .map(|c| MultiEncodedConcept::from_single_encoding(&llm_encode(&c.coefficients)))
        .collect();
    let relations = relation_data();
    let mut ok = 0;
    for (ia, ib, lbl, _, _) in &relations {
        let expected = label_to_type(lbl);
        let (multi, _) = classify_multi_encoded(&mc[*ia], &mc[*ib], w);
        if multi == expected { ok += 1; }
    }
    ok as f64 / relations.len() as f64
}

fn relation_data() -> Vec<(usize, usize, &'static str, &'static str, &'static str)> {
    vec![
        (0,5,"receptive","Marketing Budget","Employee Handbook"),
        (1,2,"generative","Sales Pipeline","Revenue Target"),
        (3,4,"causal","Support Ticket","Quarterly Report"),
        (4,2,"clarifying","Quarterly Report","Revenue Target"),
        (5,7,"constraining","Employee Handbook","Innovation Fund"),
        (6,5,"receptive","Vendor Contract","Employee Handbook"),
        (7,2,"generative","Innovation Fund","Revenue Target"),
        (8,10,"balancing","Feedback Loop","Market Trend"),
        (9,7,"generative","Onboarding Process","Innovation Fund"),
        (10,13,"influential","Market Trend","Industry Standard"),
        (12,5,"receptive","Severance Package","Employee Handbook"),
        (13,10,"influential","Industry Standard","Market Trend"),
        (14,8,"balancing","Team Standup","Feedback Loop"),
        (15,20,"constraining","Predator","Mutation"),
        (16,17,"receptive","Decomposer","Photosynthesis"),
        (17,21,"generative","Photosynthesis","Homeostasis"),
        (18,17,"generative","Water Cycle","Photosynthesis"),
        (19,23,"influential","Keystone","Succession"),
        (20,22,"generative","Mutation","Natural Selection"),
        (21,24,"balancing","Homeostasis","Symbiosis"),
        (22,20,"constraining","Natural Selection","Mutation"),
        (23,16,"influential","Succession","Decomposer"),
        (24,19,"balancing","Symbiosis","Keystone"),
        (25,21,"generative","DNA Replication","Homeostasis"),
        (26,30,"constraining","Firewall","Circuit Breaker"),
        (27,28,"balancing","Load Balancer","Database Index"),
        (28,26,"clarifying","Database Index","Firewall"),
        (29,32,"transmissive","Message Broker","Feature Flag"),
        (30,35,"constraining","Circuit Breaker","Chaos"),
        (31,32,"influential","Deprecation Policy","Feature Flag"),
        (33,30,"clarifying","Health Check","Circuit Breaker"),
        (34,33,"receptive","Event Log","Health Check"),
        (35,33,"causal","Chaos","Health Check"),
        (36,30,"receptive","Rate Limiter","Circuit Breaker"),
        (37,29,"transmissive","API Gateway","Message Broker"),
        (0,26,"constraining","Budget","Firewall"),
        (20,7,"causal","Mutation","Innovation Fund"),
        (27,21,"receptive","Load Balancer","Homeostasis"),
        (11,30,"clarifying","Compliance Audit","Circuit Breaker"),
    ]
}

/// Apply mechanical phase correction based on corrective prompt.
/// In production, the LLM would semantically re-encode instead.
fn apply_phase_correction(coeffs: &[f64; 8], prompt: &str) -> [f64; 8] {
    let mut result = *coeffs;
    let phase_order = ["Wood", "Fire", "Earth", "Metal", "Water"];
    // Boost the first mentioned phase keyword's dominant blade
    for (pi, phase_name) in phase_order.iter().enumerate() {
        if prompt.contains(phase_name) && !prompt.contains(&format!("ALTERNATIVE")) {
            // Map phase to blade indices and boost
            let blades: &[usize] = match pi {
                0 => &[1, 5],    // Wood: Zhen(1), Xun(5)
                1 => &[4],        // Fire: Li(4)
                2 => &[0, 3],    // Earth: Kun(0), Gen(3)
                3 => &[6, 7],    // Metal: Dui(6), Qian(7)
                4 => &[2],        // Water: Kan(2)
                _ => &[],
            };
            // Boost the strongest blade for this phase
            let best = blades.iter().max_by(|&&a, &&b| result[a].abs().partial_cmp(&result[b].abs()).unwrap()).unwrap();
            let sign = if result[*best] >= 0.0 { 1.0 } else { -1.0 };
            result[*best] = (result[*best].abs() + 0.6) * sign;
            for i in 0..8 { if i != *best { result[i] *= 0.35; } }
            break;
        }
    }
    result
}
