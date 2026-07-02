use crate::evaluator::AnswerEvaluator;
use crate::mcp_client::{self, McpClient};
use crate::types::*;
use anyhow::Result;
use std::time::Instant;

pub struct BenchmarkHarness {
    pub mcp: Option<McpClient>,
    pub encoding_records: Vec<EncodingRecord>,
    pub ga_bagua_calls: Vec<GaBaguaToolCall>,
}

impl BenchmarkHarness {
    pub fn new() -> Self {
        Self {
            mcp: None,
            encoding_records: Vec::new(),
            ga_bagua_calls: Vec::new(),
        }
    }

    pub fn with_mcp(mut self) -> Result<Self> {
        self.mcp = Some(McpClient::spawn()?);
        Ok(self)
    }

    pub fn encode_concept(
        &mut self,
        name: &str,
        coefficients: &[f64; 8],
    ) -> Result<EncodingRecord> {
        let mcp = self
            .mcp
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("MCP not initialized"))?;

        let (result, latency) = mcp.llm_encode(name, coefficients)?;

        let text = mcp_client::parse_tool_text(&result);

        let norm_val: f64 = result
            .get("normalized_norm")
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        let sharpness: f64 = result
            .get("sharpness")
            .and_then(|v| v.as_f64())
            .or_else(|| {
                text.lines()
                    .find(|l| l.contains("sharpness"))
                    .and_then(|l| l.split(|c: char| !c.is_ascii_digit() && c != '.').find(|s| !s.is_empty()))
                    .and_then(|s| s.parse::<f64>().ok())
            })
            .unwrap_or(0.0);

        let dominant_role = result
            .get("dominant_role")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Self::extract_field(&text, "dominant_role"));

        let dominant_trigram = result
            .get("trigram")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Self::extract_field(&text, "trigram"));

        let wuxing_phase = result
            .get("wuxing_phase")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Self::extract_field(&text, "wuxing_phase"));

        let record = EncodingRecord {
            concept_name: name.to_string(),
            coefficients: *coefficients,
            dominant_role,
            dominant_trigram,
            wuxing_phase,
            sharpness,
            norm: norm_val,
        };

        self.ga_bagua_calls.push(GaBaguaToolCall {
            tool_name: "llm_encode".to_string(),
            arguments: serde_json::json!({ "name": name, "coefficients": coefficients }),
            result: Some(result),
            latency_us: latency,
        });

        self.encoding_records.push(record.clone());
        Ok(record)
    }

    pub fn store_and_encode_concepts(
        &mut self,
        annotations: &[ConceptAnnotation],
    ) -> Result<Vec<EncodingRecord>> {
        let mut records = Vec::new();
        for annotation in annotations {
            let record = self.encode_concept(
                &annotation.name,
                &annotation.suggested_coefficients,
            )?;

            let mcp = self.mcp.as_mut().unwrap();
            let (_result, _latency) = mcp.store_llm_concept(
                &annotation.name,
                &annotation.suggested_coefficients,
                &annotation.description,
            )?;

            records.push(record);
        }
        Ok(records)
    }

    pub fn query_similar(
        &mut self,
        query_coeffs: &[f64; 8],
        top_k: usize,
    ) -> Result<(Vec<(String, f64)>, u64)> {
        let mcp = self.mcp.as_mut().unwrap();
        let (result, latency) = mcp.store_query_similar(query_coeffs, top_k)?;

        let concepts: Vec<(String, f64)> = result
            .get("similar_concepts")
            .and_then(|sc| sc.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let name = item["name"].as_str().unwrap_or("").to_string();
                        let sim = item["similarity"].as_f64().unwrap_or(0.0);
                        if name.is_empty() {
                            None
                        } else {
                            Some((name, sim))
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        self.ga_bagua_calls.push(GaBaguaToolCall {
            tool_name: "store_query_similar".to_string(),
            arguments: serde_json::json!({ "query": query_coeffs, "top_k": top_k }),
            result: Some(result),
            latency_us: latency,
        });

        Ok((concepts, latency))
    }

    pub fn classify_pair(
        &mut self,
        a_coeffs: &[f64; 8],
        b_coeffs: &[f64; 8],
    ) -> Result<String> {
        let mcp = self.mcp.as_mut().unwrap();
        let (result, _latency) = mcp.classify_relation(a_coeffs, b_coeffs)?;

        let relation = result
            .get("relation_type")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(relation)
    }

    pub fn detect_contradiction(
        &mut self,
        a_coeffs: &[f64; 8],
        b_coeffs: &[f64; 8],
    ) -> Result<bool> {
        let mcp = self.mcp.as_mut().unwrap();
        let (result, _latency) = mcp.detect_contradiction(a_coeffs, b_coeffs, 0.5)?;

        let is_contradiction = result
            .get("is_contradiction")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(is_contradiction)
    }

    pub fn avg_sharpness(&self) -> f64 {
        if self.encoding_records.is_empty() {
            return 0.0;
        }
        self.encoding_records.iter().map(|r| r.sharpness).sum::<f64>()
            / self.encoding_records.len() as f64
    }

    pub fn phase_distribution(&self) -> Vec<(String, usize)> {
        let mut phases: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for record in &self.encoding_records {
            *phases.entry(record.wuxing_phase.clone()).or_insert(0) += 1;
        }
        let mut result: Vec<_> = phases.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }

    pub fn list_concepts(&mut self) -> Result<Vec<(String, [f64; 8])>> {
        let mcp = self.mcp.as_mut().unwrap();
        let (result, _latency) = mcp.store_list_concepts()?;

        let concepts: Vec<(String, [f64; 8])> = result
            .get("concepts")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let name = item["name"].as_str().unwrap_or("").to_string();
                        let enc = item["encoding"].as_array().map(|a| {
                            let mut coeffs = [0.0f64; 8];
                            for (i, v) in a.iter().take(8).enumerate() {
                                coeffs[i] = v.as_f64().unwrap_or(0.0);
                            }
                            coeffs
                        });
                        match enc {
                            Some(coeffs) => Some((name, coeffs)),
                            None => None,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(concepts)
    }

    fn extract_field(text: &str, field: &str) -> String {
        for line in text.lines() {
            if line.to_lowercase().contains(&field.to_lowercase()) {
                if let Some(part) = line.split(':').nth(1) {
                    return part.trim().trim_matches('"').trim_matches('\'').to_string();
                }
            }
        }
        String::new()
    }
}

pub fn compute_token_efficiency(
    encoding_tokens: u64,
    query_tokens: u64,
    baseline_per_query_tokens: u64,
    num_queries: usize,
    num_concepts: usize,
) -> TokenEfficiencyResult {
    let total_ga = encoding_tokens + query_tokens;
    let total_baseline = baseline_per_query_tokens * num_queries as u64;
    let savings = total_baseline.saturating_sub(total_ga);
    let ratio = if total_ga > 0 {
        total_baseline as f64 / total_ga as f64
    } else {
        0.0
    };

    let mut break_even = None;
    if baseline_per_query_tokens > 0 {
        let encode_cost = encoding_tokens;
        let query_saving = baseline_per_query_tokens.saturating_sub(query_tokens / num_queries.max(1) as u64);
        if query_saving > 0 {
            let be = (encode_cost as f64 / query_saving as f64).ceil() as usize;
            break_even = Some(be);
        }
    }

    TokenEfficiencyResult {
        total_ga_bagua_tokens: total_ga,
        total_baseline_tokens: total_baseline,
        token_savings: savings,
        savings_ratio: ratio,
        break_even_queries: break_even,
        encoding_tokens,
        query_tokens,
        encoding_percentage: if total_ga > 0 {
            encoding_tokens as f64 / total_ga as f64
        } else {
            0.0
        },
    }
}

#[derive(Debug, Clone)]
pub struct TokenEfficiencyResult {
    pub total_ga_bagua_tokens: u64,
    pub total_baseline_tokens: u64,
    pub token_savings: u64,
    pub savings_ratio: f64,
    pub break_even_queries: Option<usize>,
    pub encoding_tokens: u64,
    pub query_tokens: u64,
    pub encoding_percentage: f64,
}

pub fn compute_retrieval_metrics(
    retrieved: &[(String, f64)],
    expected: &[String],
    all_same_role: &[String],
) -> RetrievalMetrics {
    let k = retrieved.len();
    let hits_at_1 = if k > 0 && expected.iter().any(|e| e.to_lowercase() == retrieved[0].0.to_lowercase()) {
        1.0
    } else {
        0.0
    };

    let precision_at_5: f64 = retrieved.iter().take(5).filter(|(name, _)| {
        expected.iter().any(|e| e.to_lowercase() == name.to_lowercase())
    }).count() as f64 / 5.0_f64.min(retrieved.len() as f64);

    let recall_at_10: f64 = if expected.is_empty() {
        0.0
    } else {
        let found: usize = retrieved.iter().take(10).filter(|(name, _)| {
            expected.iter().any(|e| e.to_lowercase() == name.to_lowercase())
        }).count();
        found as f64 / expected.len() as f64
    };

    let mut reciprocal_rank = 0.0;
    for (i, (name, _)) in retrieved.iter().enumerate() {
        if expected.iter().any(|e| e.to_lowercase() == name.to_lowercase()) {
            reciprocal_rank = 1.0 / (i as f64 + 1.0);
            break;
        }
    }

    let same_role_in_results = retrieved.iter().filter(|(name, _)| {
        all_same_role.iter().any(|r| r.to_lowercase() == name.to_lowercase())
    }).count();

    RetrievalMetrics {
        hits_at_1,
        precision_at_5,
        recall_at_10,
        mrr: reciprocal_rank,
        num_retrieved: retrieved.len(),
        same_role_found: same_role_in_results,
    }
}

#[derive(Debug, Clone)]
pub struct RetrievalMetrics {
    pub hits_at_1: f64,
    pub precision_at_5: f64,
    pub recall_at_10: f64,
    pub mrr: f64,
    pub num_retrieved: usize,
    pub same_role_found: usize,
}
