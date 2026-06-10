use ga_semantics_core::prelude::*;
use ga_semantics_core::store::ConceptStore;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoherenceReport {
    pub doc_id: i64,
    pub doc_name: String,
    pub total_claims: usize,
    pub contradictory_pairs: Vec<(i64, i64, f64)>,
    pub coherence_score: f64,
}

const CONTRADICTION_THRESHOLD: f64 = 0.4;

pub fn intra_coherence(store: &ConceptStore, doc_id: i64) -> CoherenceReport {
    let claims = store.query_concepts_by_document(doc_id);
    let total_claims = claims.len();
    let doc_name = store
        .get_document(doc_id)
        .map(|d| d.name)
        .unwrap_or_else(|| format!("doc_{}", doc_id));

    let mut contradictory_pairs = Vec::new();
    let total_possible_pairs = if total_claims >= 2 {
        total_claims * (total_claims - 1) / 2
    } else {
        0
    };

    for i in 0..total_claims {
        for j in (i + 1)..total_claims {
            let mv_a = Multivector::new(claims[i].encoding);
            let mv_b = Multivector::new(claims[j].encoding);
            if is_contradictory(&mv_a, &mv_b, CONTRADICTION_THRESHOLD) {
                let sim = dominant_similarity(&mv_a, &mv_b);
                contradictory_pairs.push((claims[i].id, claims[j].id, sim));
            }
        }
    }

    let coherence_score = if total_possible_pairs > 0 {
        1.0 - (contradictory_pairs.len() as f64 / total_possible_pairs as f64)
    } else {
        1.0
    };

    CoherenceReport {
        doc_id,
        doc_name,
        total_claims,
        contradictory_pairs,
        coherence_score,
    }
}

pub fn inter_coherence(store: &ConceptStore, doc_a_id: i64, doc_b_id: i64) -> CoherenceReport {
    let claims_a = store.query_concepts_by_document(doc_a_id);
    let claims_b = store.query_concepts_by_document(doc_b_id);
    let total_pairs = claims_a.len() * claims_b.len();

    let doc_name = format!(
        "{}_vs_{}",
        store
            .get_document(doc_a_id)
            .map(|d| d.name)
            .unwrap_or_else(|| format!("doc_{}", doc_a_id)),
        store
            .get_document(doc_b_id)
            .map(|d| d.name)
            .unwrap_or_else(|| format!("doc_{}", doc_b_id)),
    );

    let mut contradictory_pairs = Vec::new();

    for ca in &claims_a {
        for cb in &claims_b {
            let mv_a = Multivector::new(ca.encoding);
            let mv_b = Multivector::new(cb.encoding);
            if is_contradictory(&mv_a, &mv_b, CONTRADICTION_THRESHOLD) {
                let sim = dominant_similarity(&mv_a, &mv_b);
                contradictory_pairs.push((ca.id, cb.id, sim));
            }
        }
    }

    let coherence_score = if total_pairs > 0 {
        1.0 - (contradictory_pairs.len() as f64 / total_pairs as f64)
    } else {
        1.0
    };

    CoherenceReport {
        doc_id: doc_a_id,
        doc_name,
        total_claims: claims_a.len() + claims_b.len(),
        contradictory_pairs,
        coherence_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ga_semantics_core::store::ConceptStore;

    #[test]
    fn intra_coherence_identical_claims_is_perfect() {
        let mut store = ConceptStore::open_memory();
        let doc_id = store.store_document("coherent_doc", None, None).unwrap();

        let enc = [0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        store.store_concept_with_doc("c1", "claim 1", &enc, doc_id).unwrap();
        store.store_concept_with_doc("c2", "claim 2", &enc, doc_id).unwrap();

        let report = intra_coherence(&store, doc_id);
        assert_eq!(report.total_claims, 2);
        assert!(report.contradictory_pairs.is_empty());
        assert!((report.coherence_score - 1.0).abs() < 1e-10);
    }

    #[test]
    fn intra_coherence_detects_contradictions() {
        let mut store = ConceptStore::open_memory();
        let doc_id = store.store_document("conflicting_doc", None, None).unwrap();

        let enc_e1 = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let enc_e2 = [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        store.store_concept_with_doc("c1", "e1 claim", &enc_e1, doc_id).unwrap();
        store.store_concept_with_doc("c2", "e2 claim", &enc_e2, doc_id).unwrap();

        let report = intra_coherence(&store, doc_id);
        assert_eq!(report.total_claims, 2);
        assert!(report.coherence_score < 1.0);
    }

    #[test]
    fn inter_coherence_between_documents() {
        let mut store = ConceptStore::open_memory();
        let d1 = store.store_document("doc_a", None, None).unwrap();
        let d2 = store.store_document("doc_b", None, None).unwrap();

        let enc_a = [0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let enc_b = [0.85, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        store.store_concept_with_doc("a1", "text a", &enc_a, d1).unwrap();
        store.store_concept_with_doc("b1", "text b", &enc_b, d2).unwrap();

        let report = inter_coherence(&store, d1, d2);
        assert_eq!(report.total_claims, 2);
        assert!(report.coherence_score >= 0.0 && report.coherence_score <= 1.0);
    }

    #[test]
    fn single_claim_document_has_perfect_coherence() {
        let mut store = ConceptStore::open_memory();
        let doc_id = store.store_document("solo_doc", None, None).unwrap();

        let enc = [0.5; 8];
        store.store_concept_with_doc("c1", "solo claim", &enc, doc_id).unwrap();

        let report = intra_coherence(&store, doc_id);
        assert_eq!(report.total_claims, 1);
        assert_eq!(report.coherence_score, 1.0);
        assert!(report.contradictory_pairs.is_empty());
    }
}
