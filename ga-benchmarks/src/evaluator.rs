use anyhow::Result;
use std::collections::HashSet;

pub struct AnswerEvaluator;

impl AnswerEvaluator {
    /// Score an LLM answer against expected concept names and answer fragments.
    /// Returns a score in [0.0, 1.0].
    pub fn score(
        answer: &str,
        expected_concepts: &[String],
        expected_fragments: &[String],
    ) -> f64 {
        let answer_lower = answer.to_lowercase();

        let concept_score = if expected_concepts.is_empty() {
            0.5
        } else {
            let found: usize = expected_concepts
                .iter()
                .filter(|c| answer_lower.contains(&c.to_lowercase()))
                .count();
            found as f64 / expected_concepts.len() as f64
        };

        let fragment_score = if expected_fragments.is_empty() {
            0.5
        } else {
            let found: usize = expected_fragments
                .iter()
                .filter(|f| answer_lower.contains(&f.to_lowercase()))
                .count();
            found as f64 / expected_fragments.len() as f64
        };

        0.6 * concept_score + 0.4 * fragment_score
    }

    /// Check if the answer mentions at least one expected concept.
    pub fn concepts_found(answer: &str, concepts: &[String]) -> Vec<String> {
        let answer_lower = answer.to_lowercase();
        concepts
            .iter()
            .filter(|c| answer_lower.contains(&c.to_lowercase()))
            .cloned()
            .collect()
    }

    /// Count unique concept names mentioned across the answer.
    pub fn count_mentioned_concepts(answer: &str, known_concepts: &HashSet<String>) -> usize {
        let answer_lower = answer.to_lowercase();
        known_concepts
            .iter()
            .filter(|c| answer_lower.contains(&c.to_lowercase()))
            .count()
    }

    /// Extract concept names from answer via simple heuristic (proper-noun-like patterns).
    pub fn extract_concept_mentions(answer: &str) -> Vec<String> {
        answer
            .split(|c: char| !c.is_alphanumeric() && c != ' ' && c != '-')
            .flat_map(|segment| {
                segment
                    .split_whitespace()
                    .filter(|w| {
                        w.len() > 2
                            && w.chars().next().map_or(false, |c| c.is_uppercase())
                    })
                    .map(|w| w.to_string())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_perfect_match() {
        let answer = "The Rate Limiter constrains throughput by limiting requests per time window.";
        let concepts = vec!["Rate Limiter".to_string()];
        let fragments = vec!["throughput".to_string(), "requests".to_string()];
        let score = AnswerEvaluator::score(answer, &concepts, &fragments);
        assert!(score > 0.8, "Expected > 0.8, got {}", score);
    }

    #[test]
    fn test_score_no_match() {
        let answer = "This is an unrelated answer about something else.";
        let concepts = vec!["Rate Limiter".to_string()];
        let fragments = vec!["throughput".to_string()];
        let score = AnswerEvaluator::score(answer, &concepts, &fragments);
        assert!(score < 0.3, "Expected < 0.3, got {}", score);
    }

    #[test]
    fn test_score_empty_expectations() {
        let answer = "Any answer here.";
        let score = AnswerEvaluator::score(answer, &[], &[]);
        assert!((score - 0.5).abs() < 0.01, "Expected 0.5, got {}", score);
    }

    #[test]
    fn test_concepts_found() {
        let answer = "The Rate Limiter and Circuit Breaker both constrain requests.";
        let concepts = vec![
            "Rate Limiter".to_string(),
            "Circuit Breaker".to_string(),
            "Token Bucket".to_string(),
        ];
        let found = AnswerEvaluator::concepts_found(answer, &concepts);
        assert_eq!(found.len(), 2);
        assert!(found.contains(&"Rate Limiter".to_string()));
        assert!(found.contains(&"Circuit Breaker".to_string()));
    }

    #[test]
    fn test_concepts_found_case_insensitive() {
        let answer = "the rate limiter governs throughput";
        let concepts = vec!["Rate Limiter".to_string()];
        let found = AnswerEvaluator::concepts_found(answer, &concepts);
        assert_eq!(found.len(), 1);
    }
}
