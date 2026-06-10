use ga_semantics_core::advanced::WuXing;
use ga_semantics_core::prelude::*;
use ga_semantics_core::store::ConceptStore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SynthesisReport {
    pub topic_name: String,
    pub papers_analyzed: usize,
    pub phase_coverage: HashMap<String, Vec<String>>,
    pub gaps: Vec<String>,
    pub coverage_score: f64,
}

pub fn find_gaps(store: &ConceptStore, doc_ids: &[i64]) -> SynthesisReport {
    let mut doc_phases: HashMap<String, WuXing> = HashMap::new();
    let mut papers_analyzed = 0usize;

    for &doc_id in doc_ids {
        let claims = store.query_concepts_by_document(doc_id);
        if claims.is_empty() {
            continue;
        }

        let doc_name = store
            .get_document(doc_id)
            .map(|d| d.name)
            .unwrap_or_else(|| format!("doc_{}", doc_id));

        let mut phase_counts: Vec<(WuXing, usize)> =
            WuXing::ALL.iter().map(|&p| (p, 0)).collect();
        for claim in &claims {
            let mv = Multivector::new(claim.encoding);
            let role = mv.dominant_role();
            let phase = role.wuxing_phase();
            for (p, count) in &mut phase_counts {
                if *p == phase {
                    *count += 1;
                    break;
                }
            }
        }

        let dominant_phase = phase_counts
            .iter()
            .copied()
            .max_by_key(|(_, count)| *count)
            .map(|(phase, _)| phase)
            .unwrap_or_else(|| {
                let mv = Multivector::new(claims[0].encoding);
                mv.dominant_role().wuxing_phase()
            });

        doc_phases.insert(doc_name, dominant_phase);
        papers_analyzed += 1;
    }

    let all_phases = WuXing::ALL;
    let mut phase_coverage: HashMap<String, Vec<String>> = HashMap::new();
    let mut gaps: Vec<String> = Vec::new();
    let mut covered_count = 0usize;

    for &phase in &all_phases {
        let phase_name = phase.name().to_string();
        let papers: Vec<String> = doc_phases
            .iter()
            .filter(|(_, &p)| p == phase)
            .map(|(name, _)| name.clone())
            .collect();

        if papers.is_empty() {
            gaps.push(phase_name.clone());
        } else {
            covered_count += 1;
        }

        phase_coverage.insert(phase_name, papers);
    }

    let coverage_score = covered_count as f64 / all_phases.len() as f64;

    SynthesisReport {
        topic_name: format!("topic_{}", papers_analyzed),
        papers_analyzed,
        phase_coverage,
        gaps,
        coverage_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ga_semantics_core::store::ConceptStore;

    #[test]
    fn full_coverage_all_phases_present() {
        let mut store = ConceptStore::open_memory();

        let wood_enc = [0.0, 0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]; // E1(1) -> Zhen -> Wood
        let fire_enc = [0.0, 0.0, 0.0, 0.0, 0.9, 0.0, 0.0, 0.0]; // E12(4) -> Li -> Fire
        let earth_enc = [0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]; // Scalar(0) -> Kun -> Earth
        let metal_enc = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9]; // E123(7) -> Qian -> Metal
        let water_enc = [0.0, 0.0, 0.9, 0.0, 0.0, 0.0, 0.0, 0.0]; // E2(2) -> Kan -> Water

        let encodings = [wood_enc, fire_enc, earth_enc, metal_enc, water_enc];
        let names = ["wood_paper", "fire_paper", "earth_paper", "metal_paper", "water_paper"];
        let mut doc_ids = Vec::new();

        for (i, (name, enc)) in names.iter().zip(encodings.iter()).enumerate() {
            let doc_id = store.store_document(name, None, None).unwrap();
            store.store_concept_with_doc(&format!("c{}", i), "text", enc, doc_id).unwrap();
            doc_ids.push(doc_id);
        }

        let report = find_gaps(&store, &doc_ids);
        assert_eq!(report.papers_analyzed, 5);
        assert!((report.coverage_score - 1.0).abs() < 1e-10);
        assert!(report.gaps.is_empty());
    }

    #[test]
    fn gap_detection_with_missing_phases() {
        let mut store = ConceptStore::open_memory();

        let wood_enc = [0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let fire_enc = [0.0, 0.0, 0.0, 0.0, 0.9, 0.0, 0.0, 0.0];

        let d1 = store.store_document("wood_paper", None, None).unwrap();
        let d2 = store.store_document("fire_paper", None, None).unwrap();

        store.store_concept_with_doc("c1", "text", &wood_enc, d1).unwrap();
        store.store_concept_with_doc("c2", "text", &fire_enc, d2).unwrap();

        let report = find_gaps(&store, &[d1, d2]);
        assert_eq!(report.papers_analyzed, 2);
        assert!((report.coverage_score - 0.4).abs() < 1e-10);
        assert_eq!(report.gaps.len(), 3);
    }

    #[test]
    fn empty_document_list_yields_zero_coverage() {
        let store = ConceptStore::open_memory();
        let report = find_gaps(&store, &[]);
        assert_eq!(report.papers_analyzed, 0);
        assert_eq!(report.coverage_score, 0.0);
        assert_eq!(report.gaps.len(), 5);
    }

    #[test]
    fn doc_without_claims_is_skipped() {
        let mut store = ConceptStore::open_memory();
        let d1 = store.store_document("empty_doc", None, None).unwrap();

        let fire_enc = [0.0, 0.0, 0.0, 0.0, 0.9, 0.0, 0.0, 0.0];
        let d2 = store.store_document("fire_paper", None, None).unwrap();
        store.store_concept_with_doc("c1", "text", &fire_enc, d2).unwrap();

        let report = find_gaps(&store, &[d1, d2]);
        assert_eq!(report.papers_analyzed, 1);
        assert!((report.coverage_score - 0.2).abs() < 1e-10);
    }
}
