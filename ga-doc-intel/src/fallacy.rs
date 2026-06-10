use ga_semantics_core::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClaimNode {
    pub id: i64,
    pub text: String,
    pub role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inference {
    pub from: i64,
    pub to: i64,
    pub relation_type: String,
    pub is_valid: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgumentGraph {
    pub nodes: Vec<ClaimNode>,
    pub edges: Vec<Inference>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FallacyResult {
    pub fallacy_type: String,
    pub description: String,
    pub confidence: f64,
}

const CIRCULAR_THRESHOLD: f64 = 0.98;
const CONTRADICTION_THRESHOLD: f64 = 0.4;

pub fn analyze_argument(encodings: &[(String, [f64; 8])]) -> Vec<FallacyResult> {
    let mut results = Vec::new();

    if encodings.len() < 2 {
        return results;
    }

    let n = encodings.len();

    for i in 0..(n - 1) {
        let (_premise_name, premise_enc) = &encodings[i];
        let (_conclusion_name, conclusion_enc) = &encodings[i + 1];

        let mv_premise = Multivector::new(*premise_enc);
        let mv_conclusion = Multivector::new(*conclusion_enc);

        let similarity = dominant_similarity(&mv_premise, &mv_conclusion);

        if similarity > CIRCULAR_THRESHOLD {
            results.push(FallacyResult {
                fallacy_type: "circular".to_string(),
                description: format!(
                    "Premise and conclusion are nearly identical (similarity={:.4}), indicating circular reasoning",
                    similarity
                ),
                confidence: similarity,
            });
        }

        let phase_premise = mv_premise.dominant_role().wuxing_phase();
        let phase_conclusion = {
            let mv_c = Multivector::new(*conclusion_enc);
            mv_c.dominant_role().wuxing_phase()
        };

        let is_generating = phase_premise.generate() == phase_conclusion;
        let is_controlling = phase_premise.control() == phase_conclusion;
        let is_generated_by = phase_conclusion.generate() == phase_premise;
        let is_controlled_by = phase_conclusion.control() == phase_premise;

        let has_wuxing_edge = is_generating || is_controlling || is_generated_by || is_controlled_by;

        if !has_wuxing_edge {
            results.push(FallacyResult {
                fallacy_type: "non_sequitur".to_string(),
                description: format!(
                    "No WuXing generating or controlling edge between premise ({:?}) and conclusion ({:?}) phases",
                    phase_premise, phase_conclusion
                ),
                confidence: 0.8,
            });
        }

        if is_contradictory(&mv_premise, &mv_conclusion, CONTRADICTION_THRESHOLD) {
            results.push(FallacyResult {
                fallacy_type: "contradiction".to_string(),
                description: "Premise and conclusion are contradictory".to_string(),
                confidence: 0.9,
            });
        }
    }

    results
}

pub fn build_argument_graph(encodings: &[(String, [f64; 8])]) -> ArgumentGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for (i, (text, _encoding)) in encodings.iter().enumerate() {
        let role = if i == encodings.len() - 1 {
            "conclusion"
        } else {
            "premise"
        };

        nodes.push(ClaimNode {
            id: i as i64,
            text: text.clone(),
            role: role.to_string(),
        });
    }

    for i in 0..(encodings.len().saturating_sub(1)) {
        let (_text_a, enc_a) = &encodings[i];
        let (_text_b, enc_b) = &encodings[i + 1];

        let mv_a = Multivector::new(*enc_a);
        let mv_b = Multivector::new(*enc_b);

        let (rel_type, _confidence) = RelationType::from_pair(&mv_a, &mv_b);
        let relation_type = rel_type.role_name().to_string();

        let sim = dominant_similarity(&mv_a, &mv_b);
        let is_valid = sim > CIRCULAR_THRESHOLD
            || rel_type == RelationType::Generative
            || rel_type == RelationType::Clarifying
            || rel_type == RelationType::Receptive;

        edges.push(Inference {
            from: i as i64,
            to: (i + 1) as i64,
            relation_type,
            is_valid,
        });
    }

    ArgumentGraph { nodes, edges }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_circular_argument() {
        let encodings = vec![
            ("premise".to_string(), [0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            ("conclusion".to_string(), [0.89, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        ];

        let results = analyze_argument(&encodings);
        let has_circular = results.iter().any(|r| r.fallacy_type == "circular");
        assert!(has_circular, "should detect circular argument in nearly identical encodings");
    }

    #[test]
    fn detect_contradiction() {
        let encodings = vec![
            ("premise".to_string(), [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            ("conclusion".to_string(), [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        ];

        let results = analyze_argument(&encodings);
        let has_contradiction = results.iter().any(|r| r.fallacy_type == "contradiction");
        assert!(has_contradiction, "should detect contradiction between opposing encodings");
    }

    #[test]
    fn build_graph_with_valid_inferences() {
        let encodings = vec![
            ("premise_a".to_string(), [0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            ("conclusion".to_string(), [0.85, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        ];

        let graph = build_argument_graph(&encodings);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.nodes[0].role, "premise");
        assert_eq!(graph.nodes[1].role, "conclusion");
    }

    #[test]
    fn single_element_no_fallacies() {
        let encodings = vec![
            ("only_claim".to_string(), [0.5; 8]),
        ];

        let results = analyze_argument(&encodings);
        assert!(results.is_empty());
    }

    #[test]
    fn empty_argument_no_panic() {
        let encodings: Vec<(String, [f64; 8])> = vec![];
        let results = analyze_argument(&encodings);
        assert!(results.is_empty());
    }
}
