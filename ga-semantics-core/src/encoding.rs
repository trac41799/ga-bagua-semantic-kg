use crate::Multivector;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn llm_encode(raw_coefficients: &[f64; 8]) -> Multivector {
    let raw = Multivector::new(*raw_coefficients);
    let n = raw.norm();
    if n > f64::EPSILON { raw * (1.0 / n) } else { Multivector::one() }
}

pub fn hash_encode(text: &str) -> Multivector {
    let mut coeffs = [0.0f64; 8];
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() { return Multivector::one(); }
    let word_count = words.len();
    if word_count == 1 { return word_to_multivector(words[0]); }
    for (_i, word) in words.iter().enumerate() {
        let mut h = DefaultHasher::new(); word.hash(&mut h);
        let seed = h.finish();
        let magic = 0x9E3779B97F4A7C15u64;
        for s in 0..8 {
            let shifted = seed.wrapping_mul(magic).wrapping_add((s as u64 + 1).wrapping_mul(0x517CC1B727220A95u64));
            coeffs[s] += ((shifted as f64) / (u64::MAX as f64)) * 2.0 - 1.0;
        }
    }
    let n = (coeffs.iter().map(|c| c * c).sum::<f64>()).sqrt();
    if n < f64::EPSILON { return Multivector::one(); }
    for c in coeffs.iter_mut() { *c /= n; }
    Multivector::new(coeffs)
}

#[deprecated(since = "0.2.0", note = "Use llm_encode for LLM-provided coefficients. hash_encode is available for lexical-only encoding but produces 0% semantic accuracy.")]
pub fn text_to_multivector(text: &str) -> Multivector {
    hash_encode(text)
}

pub fn multivector_to_roles(mv: &Multivector) -> Vec<(String, f64, String)> {
    let coeffs = mv.coefficients();
    let roles = [
        ("receptive", "Accepts, follows, grounds; adopts conventions"),
        ("causal", "Triggers, starts a chain reaction; event-driven"),
        ("transmissive", "Channels, flows, transmits; data pipelines"),
        ("constraining", "Limits, bounds, restricts; permissions, capacity"),
        ("clarifying", "Reveals, illuminates, makes visible; introspection"),
        ("influential", "Pervades, gradually affects; convention spreading"),
        ("balancing", "Mirrors, equilibrates, reflects; feedback loops"),
        ("generative", "Introduces, creates, initiates new patterns"),
    ];
    let mut entries: Vec<_> = roles
        .iter()
        .enumerate()
        .map(|(i, (name, desc))| (name.to_string(), coeffs[i], desc.to_string()))
        .collect();
    entries.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap_or(std::cmp::Ordering::Equal));
    entries
}

pub fn multivector_describe(mv: &Multivector) -> String {
    let roles = multivector_to_roles(mv);
    let mut parts = vec![];
    for (name, weight, desc) in &roles {
        let intensity = if weight.abs() > 0.5 {
            "strongly"
        } else if weight.abs() > 0.3 {
            "moderately"
        } else if weight.abs() > 0.15 {
            "slightly"
        } else {
            continue;
        };
        let prefix = if *weight > 0.0 { "" } else { "counter-" };
        parts.push(format!("{intensity} {prefix}{name} ({desc})"));
    }
    if parts.is_empty() {
        "neutral — no strongly activated semantic roles".to_string()
    } else {
        parts.join("; ")
    }
}

pub fn word_to_multivector(word: &str) -> Multivector {
    let mut h = DefaultHasher::new();
    word.hash(&mut h);
    let mut coeffs = [0.0f64; 8];
    let seed = h.finish();
    let magic = 0x9E3779B97F4A7C15u64;
    for i in 0..8 {
        let shifted = seed
            .wrapping_mul(magic)
            .wrapping_add((i as u64 + 1).wrapping_mul(0x517CC1B727220A95u64));
        let val = ((shifted as f64) / (u64::MAX as f64)) * 2.0 - 1.0;
        coeffs[i] = val;
    }
    let n = (coeffs.iter().map(|c| c * c).sum::<f64>()).sqrt();
    if n < f64::EPSILON {
        return Multivector::one();
    }
    for c in coeffs.iter_mut() {
        *c /= n;
    }
    Multivector::new(coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_to_mv_is_deterministic() {
        let a = text_to_multivector("hello world");
        let b = text_to_multivector("hello world");
        assert!(a.approx_eq(&b, 1e-10));
    }

    #[test]
    fn text_to_mv_is_unit_norm() {
        let mv = text_to_multivector("the quick brown fox jumps over the lazy dog");
        assert!((mv.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn different_texts_different_mv() {
        let a = text_to_multivector("artificial intelligence");
        let b = text_to_multivector("deep learning");
        assert!(!a.approx_eq(&b, 1e-10));
    }

    #[test]
    fn empty_text_is_one() {
        let mv = text_to_multivector("");
        assert!((mv.scalar() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn multivector_describe_non_empty() {
        let mv = text_to_multivector("causal event trigger initiate chain reaction");
        let desc = multivector_describe(&mv);
        assert!(!desc.is_empty());
    }

    #[test]
    fn word_to_mv_deterministic() {
        let a = word_to_multivector("knowledge");
        let b = word_to_multivector("knowledge");
        assert!(a.approx_eq(&b, 1e-10));
    }
}
