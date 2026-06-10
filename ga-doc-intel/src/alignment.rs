use ga_semantics_core::prelude::*;
use ga_semantics_core::store::ConceptStore;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClaimAlignment {
    pub claim_a_id: i64,
    pub claim_b_id: i64,
    pub similarity: f64,
    pub relation_type: String,
    pub is_match: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlignmentReport {
    pub doc_a_name: String,
    pub doc_b_name: String,
    pub alignments: Vec<ClaimAlignment>,
    pub matched_count: usize,
    pub conflicting_count: usize,
    pub supporting_count: usize,
}

const MATCH_THRESHOLD: f64 = 0.7;

pub fn align_documents(store: &ConceptStore, doc_a_id: i64, doc_b_id: i64) -> AlignmentReport {
    let claims_a = store.query_concepts_by_document(doc_a_id);
    let claims_b = store.query_concepts_by_document(doc_b_id);

    let doc_a_name = store
        .get_document(doc_a_id)
        .map(|d| d.name)
        .unwrap_or_else(|| format!("doc_{}", doc_a_id));
    let doc_b_name = store
        .get_document(doc_b_id)
        .map(|d| d.name)
        .unwrap_or_else(|| format!("doc_{}", doc_b_id));

    let mut alignments = Vec::new();
    let mut matched_count = 0usize;
    let mut conflicting_count = 0usize;
    let mut supporting_count = 0usize;

    for ca in &claims_a {
        for cb in &claims_b {
            let mv_a = Multivector::new(ca.encoding);
            let mv_b = Multivector::new(cb.encoding);
            let similarity = dominant_similarity(&mv_a, &mv_b);
            let (rel, _confidence) = RelationType::from_pair(&mv_a, &mv_b);
            let relation_type = rel.role_name().to_string();
            let is_match = similarity > MATCH_THRESHOLD;

            if is_match {
                matched_count += 1;
            }

            if rel == RelationType::Constraining || (similarity < -MATCH_THRESHOLD) {
                conflicting_count += 1;
            }

            if rel == RelationType::Generative || rel == RelationType::Clarifying {
                supporting_count += 1;
            }

            alignments.push(ClaimAlignment {
                claim_a_id: ca.id,
                claim_b_id: cb.id,
                similarity,
                relation_type,
                is_match,
            });
        }
    }

    AlignmentReport {
        doc_a_name,
        doc_b_name,
        alignments,
        matched_count,
        conflicting_count,
        supporting_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ga_semantics_core::store::ConceptStore;

    #[test]
    fn align_similar_documents_produces_matches() {
        let mut store = ConceptStore::open_memory();
        let d1 = store.store_document("doc_a", None, None).unwrap();
        let d2 = store.store_document("doc_b", None, None).unwrap();

        let enc_similar1 = [0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let enc_similar2 = [0.85, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        store.store_concept_with_doc("a1", "claim a1", &enc_similar1, d1).unwrap();
        store.store_concept_with_doc("b1", "claim b1", &enc_similar2, d2).unwrap();

        let report = align_documents(&store, d1, d2);
        assert_eq!(report.doc_a_name, "doc_a");
        assert_eq!(report.doc_b_name, "doc_b");
        assert_eq!(report.alignments.len(), 1);
        assert!(report.alignments[0].similarity > 0.9);
        assert!(report.alignments[0].is_match);
    }

    #[test]
    fn align_dissimilar_documents_count_conflicts() {
        let mut store = ConceptStore::open_memory();
        let d1 = store.store_document("doc_a", None, None).unwrap();
        let d2 = store.store_document("doc_b", None, None).unwrap();

        let enc_wood = [0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let enc_earth = [0.0, 0.0, 0.0, 0.9, 0.0, 0.0, 0.0, 0.0];
        store.store_concept_with_doc("a1", "wood claim", &enc_wood, d1).unwrap();
        store.store_concept_with_doc("b1", "earth claim", &enc_earth, d2).unwrap();

        let report = align_documents(&store, d1, d2);
        assert_eq!(report.alignments.len(), 1);
    }

    #[test]
    fn align_multiple_claims_per_document() {
        let mut store = ConceptStore::open_memory();
        let d1 = store.store_document("doc_a", None, None).unwrap();
        let d2 = store.store_document("doc_b", None, None).unwrap();

        let encs_a = [
            [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ];
        let encs_b = [
            [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ];

        for (i, enc) in encs_a.iter().enumerate() {
            store.store_concept_with_doc(&format!("a{}", i), "text", enc, d1).unwrap();
        }
        for (i, enc) in encs_b.iter().enumerate() {
            store.store_concept_with_doc(&format!("b{}", i), "text", enc, d2).unwrap();
        }

        let report = align_documents(&store, d1, d2);
        assert_eq!(report.alignments.len(), 4);
        assert!(report.matched_count >= 1);
    }
}
