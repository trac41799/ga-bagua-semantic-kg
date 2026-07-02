use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptAnnotation {
    pub concept_id: String,
    pub name: String,
    pub description: String,
    pub document_id: String,
    pub section: Option<String>,
    pub dominant_trigram: String,
    pub dominant_role: String,
    pub wuxing_phase: String,
    pub secondary_roles: Vec<String>,
    pub related_concepts: Vec<String>,
    pub relation_types: HashMap<String, String>,
    pub contradicts: Vec<String>,
    pub suggested_coefficients: [f64; 8],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySpec {
    pub query_id: String,
    pub text: String,
    pub query_type: String,
    pub expected_concepts: Vec<String>,
    pub expected_answer_fragments: Vec<String>,
    pub min_tokens_if_alone: u64,
    pub requires_multi_hop: bool,
    pub requires_cross_document: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSpec {
    pub test_id: String,
    pub name: String,
    pub description: String,
    pub document_paths: Vec<String>,
    pub annotation_path: String,
    pub queries: Vec<QuerySpec>,
    pub metrics: Vec<String>,
    pub baselines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaBaguaToolCall {
    pub tool_name: String,
    pub arguments: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub latency_us: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub query_id: String,
    pub query_text: String,
    pub expected_concepts: Vec<String>,
    pub retrieved_concepts: Vec<(String, f64)>,
    pub accuracy_score: f64,
    pub tokens_consumed: u64,
    pub ga_bagua_calls: Vec<GaBaguaToolCall>,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    pub test_id: String,
    pub name: String,
    pub configuration: String,
    pub encoding_tokens: u64,
    pub encoding_concepts: usize,
    pub query_results: Vec<QueryResult>,
    pub total_tokens: u64,
    pub total_latency_ms: u64,
    pub accuracy: f64,
    pub encoding_sharpness_avg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineResult {
    pub configuration: String,
    pub query_results: Vec<QueryResult>,
    pub total_tokens: u64,
    pub accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcResult {
    pub ac_id: String,
    pub criterion: String,
    pub actual: String,
    pub expected: String,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QaResult {
    pub qa_id: String,
    pub scenario: String,
    pub observed: String,
    pub verdict: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    pub test_id: String,
    pub name: String,
    pub status: String,
    pub experiment: SessionResult,
    pub baseline: Option<BaselineResult>,
    pub ac_results: Vec<AcResult>,
    pub qa_results: Vec<QaResult>,
    pub token_savings_ratio: f64,
    pub break_even_query: Option<usize>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateReport {
    pub timestamp: String,
    pub test_reports: Vec<TestReport>,
    pub overall_status: String,
    pub total_ac_passed: usize,
    pub total_ac_failed: usize,
    pub key_findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodingRecord {
    pub concept_name: String,
    pub coefficients: [f64; 8],
    pub dominant_role: String,
    pub dominant_trigram: String,
    pub wuxing_phase: String,
    pub sharpness: f64,
    pub norm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenModel {
    pub encode_tokens_per_concept: u64,
    pub verify_tokens_per_candidate: u64,
    pub base_answer_tokens: u64,
    pub document_tokens: u64,
}

impl Default for TokenModel {
    fn default() -> Self {
        Self {
            encode_tokens_per_concept: 200,
            verify_tokens_per_candidate: 15,
            base_answer_tokens: 50,
            document_tokens: 64000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodingDiagnostic {
    pub concept_name: String,
    pub coefficients: [f64; 8],
    pub sharpness: f64,
    pub dominant_coefficient: f64,
    pub secondary_coefficient: f64,
    pub num_negative: usize,
    pub norm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalDiagnostic {
    pub query_name: String,
    pub top_5_concepts: Vec<(String, f64)>,
    pub precision_at_5: f64,
    pub recall_at_10: f64,
    pub mrr: f64,
    pub same_role_concepts_found: usize,
}
