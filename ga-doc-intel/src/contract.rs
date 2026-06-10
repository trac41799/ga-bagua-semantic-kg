use ga_semantics_core::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractAuditReport {
    pub intent_name: String,
    pub implementation_name: String,
    pub similarity: f64,
    pub difference: f64,
    pub is_contradictory: bool,
    pub diverging_roles: Vec<String>,
    pub risk_level: String,
}

const CONTRADICTION_THRESHOLD: f64 = 0.4;

const BLADE_NAMES: [&str; 8] = ["scalar", "e1", "e2", "e3", "e12", "e23", "e31", "e123"];

pub fn audit_contract(
    intent_encoding: &[f64; 8],
    impl_encoding: &[f64; 8],
) -> ContractAuditReport {
    let intent_name = "intent".to_string();
    let implementation_name = "implementation".to_string();

    audit_contract_named(&intent_name, intent_encoding, &implementation_name, impl_encoding)
}

pub fn audit_contract_named(
    intent_name: &str,
    intent_encoding: &[f64; 8],
    implementation_name: &str,
    impl_encoding: &[f64; 8],
) -> ContractAuditReport {
    let mv_intent = Multivector::new(*intent_encoding);
    let mv_impl = Multivector::new(*impl_encoding);

    let similarity = dominant_similarity(&mv_intent, &mv_impl);
    let difference = semantic_difference(&mv_intent, &mv_impl);
    let is_contradictory = is_contradictory(&mv_intent, &mv_impl, CONTRADICTION_THRESHOLD);

    let mut role_diffs: Vec<(usize, f64)> = (0..8)
        .map(|i| (i, (intent_encoding[i] - impl_encoding[i]).abs()))
        .collect();
    role_diffs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let diverging_roles: Vec<String> = role_diffs
        .iter()
        .take(3)
        .filter(|(_, diff)| *diff > 0.1)
        .map(|(i, diff)| format!("{} ({:.3})", BLADE_NAMES[*i], diff))
        .collect();

    let risk_level = determine_risk_level(similarity, difference, is_contradictory).to_string();

    ContractAuditReport {
        intent_name: intent_name.to_string(),
        implementation_name: implementation_name.to_string(),
        similarity,
        difference,
        is_contradictory,
        diverging_roles,
        risk_level,
    }
}

fn determine_risk_level(similarity: f64, difference: f64, is_contradictory: bool) -> &'static str {
    if is_contradictory {
        return "critical";
    }
    if similarity < 0.3 {
        return "high";
    }
    if similarity < 0.5 {
        return "high";
    }
    if similarity < 0.7 {
        return "medium";
    }
    if difference > 0.5 {
        return "medium";
    }
    if difference > 0.3 {
        return "low";
    }
    "low"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_identical_encodings() {
        let enc = [0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let report = audit_contract_named("intent", &enc, "impl", &enc);

        assert!((report.similarity - 1.0).abs() < 1e-10);
        assert!((report.difference - 0.0).abs() < 1e-10);
        assert!(!report.is_contradictory);
        assert_eq!(report.risk_level, "low");
    }

    #[test]
    fn audit_contradictory_encodings() {
        let enc_intent = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let enc_impl = [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        let report = audit_contract_named("intent", &enc_intent, "impl", &enc_impl);
        assert!(report.similarity < 0.5);
        assert!(report.difference > 0.0);
        assert_eq!(report.risk_level, "critical");
    }

    #[test]
    fn audit_diverging_roles_detected() {
        let enc_intent = [0.9, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let enc_impl = [0.2, 0.1, 0.8, 0.0, 0.0, 0.0, 0.0, 0.0];

        let report = audit_contract_named("intent", &enc_intent, "impl", &enc_impl);
        assert!(!report.diverging_roles.is_empty());
        assert!(report.similarity < 1.0);
    }

    #[test]
    fn audit_risk_level_progression() {
        let enc_base = [0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        let enc_close = [0.85, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let report_close = audit_contract_named("intent", &enc_base, "impl", &enc_close);
        assert_eq!(report_close.risk_level, "low");

        let enc_far = [0.1, 0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let report_far = audit_contract_named("intent", &enc_base, "impl", &enc_far);
        assert!(report_far.risk_level == "medium" || report_far.risk_level == "high");
    }
}
