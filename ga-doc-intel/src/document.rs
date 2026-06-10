#[allow(unused_imports)]
use ga_semantics_core::prelude::*;
use ga_semantics_core::store::{ConceptStore, DocumentMeta, StoredConcept};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: i64,
    pub name: String,
    pub source: Option<String>,
    pub language: Option<String>,
    pub claim_ids: Vec<i64>,
}

pub struct DocumentStore {
    store: ConceptStore,
}

impl DocumentStore {
    pub fn new(store_path: &str) -> Result<Self, String> {
        let store = ConceptStore::open(store_path)?;
        Ok(DocumentStore { store })
    }

    pub fn from_memory() -> Self {
        DocumentStore {
            store: ConceptStore::open_memory(),
        }
    }

    pub fn create_document(
        &mut self,
        name: &str,
        source: Option<&str>,
        language: Option<&str>,
    ) -> Result<i64, String> {
        self.store.store_document(name, source, language)
    }

    pub fn add_claim(
        &mut self,
        doc_id: i64,
        name: &str,
        text: &str,
        encoding: &[f64; 8],
    ) -> Result<i64, String> {
        self.store.store_concept_with_doc(name, text, encoding, doc_id)
    }

    pub fn get_claims(&self, doc_id: i64) -> Vec<StoredConcept> {
        self.store.query_concepts_by_document(doc_id)
    }

    pub fn get_document(&self, id: i64) -> Option<DocumentMeta> {
        self.store.get_document(id)
    }

    pub fn list_documents(&self) -> Vec<DocumentMeta> {
        self.store.list_documents()
    }

    pub fn claim_count(&self, doc_id: i64) -> usize {
        self.store.query_concepts_by_document(doc_id).len()
    }

    pub fn store(&self) -> &ConceptStore {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut ConceptStore {
        &mut self.store
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_retrieve_document() {
        let mut ds = DocumentStore::from_memory();
        let doc_id = ds.create_document("test_doc", Some("source.txt"), Some("en")).unwrap();
        assert!(doc_id > 0);

        let doc = ds.get_document(doc_id).unwrap();
        assert_eq!(doc.name, "test_doc");
        assert_eq!(doc.source, Some("source.txt".to_string()));
        assert_eq!(doc.language, Some("en".to_string()));
    }

    #[test]
    fn add_and_retrieve_claims() {
        let mut ds = DocumentStore::from_memory();
        let doc_id = ds.create_document("paper", None, None).unwrap();

        let enc1 = [0.8, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let enc2 = [0.0, 0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let c1 = ds.add_claim(doc_id, "claim1", "first claim", &enc1).unwrap();
        let c2 = ds.add_claim(doc_id, "claim2", "second claim", &enc2).unwrap();
        assert_ne!(c1, c2);

        let claims = ds.get_claims(doc_id);
        assert_eq!(claims.len(), 2);

        assert_eq!(ds.claim_count(doc_id), 2);
    }

    #[test]
    fn list_multiple_documents() {
        let mut ds = DocumentStore::from_memory();
        ds.create_document("doc1", None, None).unwrap();
        ds.create_document("doc2", None, None).unwrap();
        ds.create_document("doc3", None, None).unwrap();

        let docs = ds.list_documents();
        assert_eq!(docs.len(), 3);
        let names: Vec<_> = docs.iter().map(|d| d.name.clone()).collect();
        assert!(names.contains(&"doc1".to_string()));
        assert!(names.contains(&"doc2".to_string()));
        assert!(names.contains(&"doc3".to_string()));
    }

    #[test]
    fn claims_isolated_per_document() {
        let mut ds = DocumentStore::from_memory();
        let d1 = ds.create_document("doc_a", None, None).unwrap();
        let d2 = ds.create_document("doc_b", None, None).unwrap();

        let enc = [0.5; 8];
        ds.add_claim(d1, "a1", "text a1", &enc).unwrap();
        ds.add_claim(d1, "a2", "text a2", &enc).unwrap();
        ds.add_claim(d2, "b1", "text b1", &enc).unwrap();

        assert_eq!(ds.get_claims(d1).len(), 2);
        assert_eq!(ds.get_claims(d2).len(), 1);
        assert_eq!(ds.claim_count(d1), 2);
        assert_eq!(ds.claim_count(d2), 1);
    }
}
