use crate::types::*;
use std::fs;
use std::path::Path;

pub struct Reporter;

impl Reporter {
    /// Generate a per-test report in Markdown.
    pub fn generate_test_report(report: &TestReport) -> String {
        let mut md = String::new();

        md.push_str(&format!(
            "## Test: {} | Status: {}\n\n",
            report.test_id, report.status
        ));

        md.push_str("### Acceptance Criteria\n\n");
        md.push_str("| AC | Criterion | Actual | Expected | Pass |\n");
        md.push_str("|----|-----------|--------|----------|------|\n");
        for ac in &report.ac_results {
            md.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                ac.ac_id,
                ac.criterion,
                ac.actual,
                ac.expected,
                if ac.passed { "YES" } else { "NO" }
            ));
        }

        md.push_str("\n### Token Breakdown\n\n");
        let exp = &report.experiment;
        md.push_str(&format!("- Encoding: {} tokens\n", exp.encoding_tokens));
        md.push_str(&format!("- Queries: {} queries, {} tokens\n",
            exp.query_results.len(),
            exp.query_results.iter().map(|r| r.tokens_consumed).sum::<u64>()
        ));
        md.push_str(&format!("- Total (GA-Bagua): {} tokens\n", exp.total_tokens));

        if let Some(ref baseline) = report.baseline {
            md.push_str(&format!("- Total (Alone): {} tokens\n", baseline.total_tokens));
            md.push_str(&format!("- Savings: {:.1}%\n", (1.0 - report.token_savings_ratio) * 100.0));
            if report.token_savings_ratio > 0.0 {
                md.push_str(&format!("- Token ratio: {:.1}x fewer tokens with GA-Bagua\n", 1.0 / report.token_savings_ratio.max(0.001)));
            }
        }

        if let Some(be) = report.break_even_query {
            md.push_str(&format!("- Break-even: {} queries\n", be));
        }

        md.push_str("\n### QA Results\n\n");
        for qa in &report.qa_results {
            md.push_str(&format!("- **{}**: {} → {}\n", qa.qa_id, qa.scenario, qa.verdict));
        }

        if !report.warnings.is_empty() {
            md.push_str("\n### Warnings\n\n");
            for w in &report.warnings {
                md.push_str(&format!("- {}\n", w));
            }
        }

        md.push('\n');
        md
    }

    /// Generate an aggregate scorecard in Markdown.
    pub fn generate_aggregate_report(aggregate: &AggregateReport) -> String {
        let mut md = String::new();

        md.push_str(&format!(
            "# GA-Bagua LLM Integration Benchmark Report\n\n**Date:** {}\n\n**Overall Status:** {}\n\n",
            aggregate.timestamp, aggregate.overall_status
        ));

        md.push_str("## Scorecard\n\n");
        md.push_str("| Test ID | Name | Status | ACs Passed | Token Savings | Accuracy (GA-Bagua) | Accuracy (Alone) |\n");
        md.push_str("|---------|------|--------|------------|---------------|---------------------|--------------------|\n");

        for report in &aggregate.test_reports {
            let ac_passed = report.ac_results.iter().filter(|a| a.passed).count();
            let ac_total = report.ac_results.len();
            let savings = if report.token_savings_ratio > 0.0 {
                format!("{:.1}x", 1.0 / report.token_savings_ratio.max(0.001))
            } else {
                "N/A".to_string()
            };
            let acc_exp = format!("{:.1}%", report.experiment.accuracy * 100.0);
            let acc_base = if let Some(ref b) = report.baseline {
                format!("{:.1}%", b.accuracy * 100.0)
            } else {
                "N/A".to_string()
            };

            md.push_str(&format!(
                "| {} | {} | {} | {}/{} | {} | {} | {} |\n",
                report.test_id,
                report.name,
                report.status,
                ac_passed,
                ac_total,
                savings,
                acc_exp,
                acc_base
            ));
        }

        md.push_str(&format!(
            "\n**Total:** {} ACs passed, {} ACs failed\n\n",
            aggregate.total_ac_passed, aggregate.total_ac_failed
        ));

        if !aggregate.key_findings.is_empty() {
            md.push_str("## Key Findings\n\n");
            for finding in &aggregate.key_findings {
                md.push_str(&format!("- {}\n", finding));
            }
            md.push('\n');
        }

        for report in &aggregate.test_reports {
            md.push_str(&Self::generate_test_report(report));
        }

        md
    }

    /// Write report to file.
    pub fn write_report(path: &str, content: &str) -> std::io::Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)
    }

    /// Load test spec from JSON file.
    pub fn load_test_spec(path: &str) -> anyhow::Result<TestSpec> {
        let content = fs::read_to_string(path)?;
        let spec: TestSpec = serde_json::from_str(&content)?;
        Ok(spec)
    }

    /// Load concept annotations from JSON file.
    pub fn load_annotations(path: &str) -> anyhow::Result<Vec<ConceptAnnotation>> {
        let content = fs::read_to_string(path)?;
        let annotations: Vec<ConceptAnnotation> = serde_json::from_str(&content)?;
        Ok(annotations)
    }

    /// Load a document from file.
    pub fn load_document(path: &str) -> anyhow::Result<String> {
        fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Failed to read {}: {}", path, e))
    }
}
