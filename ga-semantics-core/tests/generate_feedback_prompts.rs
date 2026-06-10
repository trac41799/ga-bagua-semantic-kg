/// Generates corrective prompts for all failing pairs in the human-labeled dataset.
/// Outputs a JSON file that the LLM feedback PowerShell script can read.
/// This is Phase 1 of the LLM re-encoding loop.
use ga_semantics_core::prelude::*;
use ga_semantics_core::multi_encoding::{MultiEncodedConcept, classify_multi_encoded};
use ga_semantics_core::relation_type::{FeatureWeights, RelationType};
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct FeedbackPrompt {
    concept_a: String,
    concept_b: String,
    expected_label: String,
    current_label: String,
    a_coefficients: Vec<f64>,
    b_coefficients: Vec<f64>,
    prompt: String,
    /// Target phase for concept A (if re-encoding A would fix it)
    target_phase_for_a: Option<String>,
    target_phase_for_b: Option<String>,
}

#[derive(Serialize)]
struct FeedbackOutput {
    concepts: Vec<ConceptEntry>,
    failing_pairs: Vec<FeedbackPrompt>,
    usage_instructions: String,
}

#[derive(Serialize)]
struct ConceptEntry {
    index: usize,
    name: String,
    coefficients: Vec<f64>,
}

const CONCEPT_NAMES: [&str; 38] = [
    "Marketing Budget", "Sales Pipeline", "Revenue Target", "Support Ticket", "Quarterly Report",
    "Employee Handbook", "Vendor Contract", "Innovation Fund", "Feedback Loop", "Onboarding Process",
    "Market Trend", "Compliance Audit", "Severance Package", "Industry Standard", "Team Standup",
    "Predator", "Decomposer", "Photosynthesis", "Water Cycle", "Keystone Species",
    "Mutation", "Homeostasis", "Natural Selection", "Ecological Succession", "Symbiosis",
    "DNA Replication", "Firewall", "Load Balancer", "Database Index", "Message Broker",
    "Circuit Breaker", "Deprecation Policy", "Feature Flag", "Health Check Endpoint", "Event Sourcing Log",
    "Chaos Engineering", "Rate Limiter", "API Gateway",
];

const CONCEPT_COEFFS: [[f64; 8]; 38] = [
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

fn label_to_type(label: &str) -> RelationType {
    use RelationType::*;
    match label {"generative"=>Generative,"receptive"=>Receptive,"causal"=>Causal,"transmissive"=>Transmissive,"constraining"=>Constraining,"influential"=>Influential,"clarifying"=>Clarifying,"balancing"=>Balancing,_=>Receptive}
}

#[test]
fn generate_feedback_prompts() {
    let enc: Vec<Multivector> = CONCEPT_COEFFS.iter().map(|c| llm_encode(c)).collect();
    let mc: Vec<MultiEncodedConcept> = enc.iter()
        .map(|mv| MultiEncodedConcept::from_single_encoding(mv))
        .collect();
    let weights = FeatureWeights::default();

    let relations: Vec<(usize, usize, &str)> = vec![
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

    let mut concepts_out = Vec::new();
    for i in 0..38 {
        concepts_out.push(ConceptEntry {
            index: i,
            name: CONCEPT_NAMES[i].to_string(),
            coefficients: CONCEPT_COEFFS[i].to_vec(),
        });
    }

    let mut failing_pairs = Vec::new();

    for (ia, ib, label_str) in &relations {
        let expected = label_to_type(label_str);
        let (pred, _) = classify_multi_encoded(&mc[*ia], &mc[*ib], &weights);

        if pred == expected { continue; }

        let a = &enc[*ia];
        let b = &enc[*ib];
        let prompt = RelationType::corrective_prompt(
            CONCEPT_NAMES[*ia], CONCEPT_NAMES[*ib],
            a, b, expected,
        );

        let ta = a.dominant_trigram();
        let tb = b.dominant_trigram();
        let wa = ta.wuxing_phase();
        let wb = tb.wuxing_phase();

        let target_phase_a = (match expected {
            RelationType::Generative => if wa.generate() != wb { Some(format!("{:?}", wb)) } else { None },
            RelationType::Receptive => if wb.generate() != wa { Some(format!("{:?}", wb)) } else { None },
            RelationType::Constraining => if wa.control() != wb { Some(format!("{:?}", wb.control())) } else { None },
            RelationType::Influential => if wb.control() != wa { Some(format!("{:?}", wb)) } else { None },
            _ => None,
        });

        let prompt_text = prompt.unwrap_or_else(|| format!(
            "Re-encode {} and {} to align with {:?} relationship.",
            CONCEPT_NAMES[*ia], CONCEPT_NAMES[*ib], expected
        ));

        failing_pairs.push(FeedbackPrompt {
            concept_a: CONCEPT_NAMES[*ia].to_string(),
            concept_b: CONCEPT_NAMES[*ib].to_string(),
            expected_label: label_str.to_string(),
            current_label: pred.role_name().to_string(),
            a_coefficients: CONCEPT_COEFFS[*ia].to_vec(),
            b_coefficients: CONCEPT_COEFFS[*ib].to_vec(),
            prompt: prompt_text,
            target_phase_for_a: target_phase_a,
            target_phase_for_b: None,
        });
    }

    let output = FeedbackOutput {
        concepts: concepts_out,
        failing_pairs,
        usage_instructions: format!(
            "For each failing pair, call the LLM with the prompt to re-encode both concepts.\n\
             The LLM should respond with a JSON object: \
             {{\"a\": [coeff0,...,coeff7], \"b\": [coeff0,...,coeff7]}}\n\
             Each coefficient array must be 8 floats in [-1.0, 1.0].\n\
             Save all responses as data/llm_feedback_results.json"
        ),
    };

    let json = serde_json::to_string_pretty(&output).unwrap();
    let path = "../data/llm_feedback_prompts.json";
    fs::write(path, &json).unwrap();
    println!("Generated {} corrective prompts → {}", output.failing_pairs.len(), path);
    println!("Run: scripts/run_llm_feedback.ps1 to process them with OpenRouter LLM");
}
