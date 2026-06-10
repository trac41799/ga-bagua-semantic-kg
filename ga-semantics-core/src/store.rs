use crate::multivector::Multivector;
use crate::relation_type::RelationType;
use crate::semantics::{relation_strength, semantic_similarity};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredConcept {
    pub id: i64,
    pub name: String,
    pub text: String,
    pub encoding: [f64; 8],
    pub created_at: String,
    #[serde(default)]
    pub document_id: Option<i64>,
    #[serde(default)]
    pub agent_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredRelation {
    pub id: i64,
    pub from_id: i64,
    pub to_id: i64,
    pub relation_type: String,
    pub confidence: f64,
    pub strength: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentMeta {
    pub id: i64,
    pub name: String,
    pub source: Option<String>,
    pub language: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GraphData {
    next_concept_id: i64,
    next_relation_id: i64,
    concepts: Vec<StoredConcept>,
    relations: Vec<StoredRelation>,
    #[serde(default)]
    documents: Vec<DocumentMeta>,
    #[serde(default)]
    next_document_id: i64,
}

pub struct ConceptStore {
    data: GraphData,
    path: Option<String>,
    dirty: bool,
}

impl ConceptStore {
    pub fn open(path: &str) -> Result<Self, String> {
        let data = if std::path::Path::new(path).exists() {
            let json = std::fs::read_to_string(path).map_err(|e| format!("read: {e}"))?;
            serde_json::from_str(&json).map_err(|e| format!("parse: {e}"))?
        } else {
            GraphData {
                next_concept_id: 1, next_relation_id: 1, next_document_id: 1,
                concepts: vec![], relations: vec![], documents: vec![],
            }
        };
        Ok(ConceptStore { data, path: Some(path.to_string()), dirty: false })
    }

    pub fn open_memory() -> Self {
        ConceptStore {
            data: GraphData {
                next_concept_id: 1, next_relation_id: 1, next_document_id: 1,
                concepts: vec![], relations: vec![], documents: vec![],
            },
            path: None,
            dirty: false,
        }
    }

    fn save(&mut self) -> Result<(), String> {
        if let Some(ref p) = self.path {
            let json = serde_json::to_string_pretty(&self.data).map_err(|e| format!("serialize: {e}"))?;
            std::fs::write(p, json).map_err(|e| format!("write: {e}"))?;
            self.dirty = false;
        }
        Ok(())
    }

    pub fn store_concept(&mut self, name: &str, text: &str, encoding: &[f64; 8]) -> Result<i64, String> {
        let id = self.data.next_concept_id;
        self.data.next_concept_id += 1;
        self.data.concepts.push(StoredConcept {
            id, name: name.to_string(), text: text.to_string(),
            encoding: *encoding, created_at: chrono_now(),
            document_id: None,
            agent_id: None,
        });
        self.dirty = true;
        self.save()?;
        Ok(id)
    }

    pub fn get_concept(&self, id: i64) -> Option<StoredConcept> {
        self.data.concepts.iter().find(|c| c.id == id).cloned()
    }

    pub fn get_concept_by_name(&self, name: &str) -> Option<StoredConcept> {
        self.data.concepts.iter().find(|c| c.name == name).cloned()
    }

    pub fn delete_concept(&mut self, id: i64) -> Result<bool, String> {
        let before = self.data.concepts.len();
        self.data.concepts.retain(|c| c.id != id);
        self.data.relations.retain(|r| r.from_id != id && r.to_id != id);
        self.dirty = self.data.concepts.len() != before;
        self.save()?;
        Ok(self.data.concepts.len() != before)
    }

    pub fn concept_count(&self) -> usize { self.data.concepts.len() }

    pub fn all_concepts(&self) -> Vec<StoredConcept> { self.data.concepts.clone() }

    pub fn store_document(&mut self, name: &str, source: Option<&str>, language: Option<&str>) -> Result<i64, String> {
        let id = self.data.next_document_id;
        self.data.next_document_id += 1;
        self.data.documents.push(DocumentMeta {
            id,
            name: name.to_string(),
            source: source.map(|s| s.to_string()),
            language: language.map(|s| s.to_string()),
        });
        self.dirty = true;
        self.save()?;
        Ok(id)
    }

    pub fn get_document(&self, id: i64) -> Option<DocumentMeta> {
        self.data.documents.iter().find(|d| d.id == id).cloned()
    }

    pub fn list_documents(&self) -> Vec<DocumentMeta> {
        self.data.documents.clone()
    }

    pub fn store_concept_with_doc(&mut self, name: &str, text: &str, encoding: &[f64; 8], document_id: i64) -> Result<i64, String> {
        let id = self.data.next_concept_id;
        self.data.next_concept_id += 1;
        self.data.concepts.push(StoredConcept {
            id, name: name.to_string(), text: text.to_string(),
            encoding: *encoding, created_at: chrono_now(),
            document_id: Some(document_id),
            agent_id: None,
        });
        self.dirty = true;
        self.save()?;
        Ok(id)
    }

    pub fn store_concept_with_agent(&mut self, name: &str, text: &str, encoding: &[f64; 8], agent_id: &str) -> Result<i64, String> {
        let id = self.data.next_concept_id;
        self.data.next_concept_id += 1;
        self.data.concepts.push(StoredConcept {
            id, name: name.to_string(), text: text.to_string(),
            encoding: *encoding, created_at: chrono_now(),
            document_id: None,
            agent_id: Some(agent_id.to_string()),
        });
        self.dirty = true;
        self.save()?;
        Ok(id)
    }

    pub fn query_concepts_by_document(&self, document_id: i64) -> Vec<StoredConcept> {
        self.data.concepts.iter().filter(|c| c.document_id == Some(document_id)).cloned().collect()
    }

    pub fn query_concepts_by_agent(&self, agent_id: &str) -> Vec<StoredConcept> {
        self.data.concepts.iter().filter(|c| c.agent_id.as_deref() == Some(agent_id)).cloned().collect()
    }

    pub fn query_similar(&self, query_mv: &Multivector, top_k: usize) -> Vec<(StoredConcept, f64)> {
        let mut scored: Vec<_> = self.data.concepts.iter().map(|c| {
            let mv = Multivector::new(c.encoding);
            let score = crate::semantics::dominant_similarity(query_mv, &mv);
            (c.clone(), score)
        }).collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        if scored.len() > top_k { scored.truncate(top_k); }
        scored
    }

    pub fn query_exact_similar(&self, query_mv: &Multivector, threshold: f64) -> Vec<(StoredConcept, f64)> {
        let mut scored: Vec<_> = self.data.concepts.iter().filter_map(|c| {
            let mv = Multivector::new(c.encoding);
            let score = semantic_similarity(query_mv, &mv);
            if score.abs() >= threshold { Some((c.clone(), score)) } else { None }
        }).collect();
        scored.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap_or(std::cmp::Ordering::Equal));
        scored
    }

    pub fn add_relation(&mut self, from_id: i64, to_id: i64) -> Result<i64, String> {
        let a = self.get_concept(from_id).ok_or("concept A not found")?;
        let b = self.get_concept(to_id).ok_or("concept B not found")?;
        let mv_a = Multivector::new(a.encoding);
        let mv_b = Multivector::new(b.encoding);
        let (role, confidence) = RelationType::from_pair(&mv_a, &mv_b);
        let strength = relation_strength(&mv_a, &mv_b);
        let id = self.data.next_relation_id;
        self.data.next_relation_id += 1;
        self.data.relations.push(StoredRelation {
            id, from_id, to_id, relation_type: role.role_name().to_string(),
            confidence, strength,
        });
        self.dirty = true;
        self.save()?;
        Ok(id)
    }

    pub fn get_relations_from(&self, from_id: i64) -> Vec<StoredRelation> {
        self.data.relations.iter().filter(|r| r.from_id == from_id).cloned().collect()
    }

    pub fn get_all_relations(&self) -> Vec<StoredRelation> {
        self.data.relations.clone()
    }

    pub fn export_graph(&self) -> serde_json::Value {
        let nodes: Vec<_> = self.data.concepts.iter().map(|c| {
            serde_json::json!({"id": c.id, "name": c.name, "text": c.text, "encoding": c.encoding})
        }).collect();
        let edges: Vec<_> = self.data.relations.iter().map(|r| {
            serde_json::json!({"id": r.id, "from": r.from_id, "to": r.to_id, "type": r.relation_type, "confidence": r.confidence, "strength": r.strength})
        }).collect();
        serde_json::json!({"nodes": nodes, "edges": edges, "node_count": nodes.len(), "edge_count": edges.len()})
    }
}

fn chrono_now() -> String {
    use std::time::SystemTime;
    let dur = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", 1970, 1, 1, 0, 0, dur.as_secs() % 86400 / 3600)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_and_retrieve_concept() {
        let mut store = ConceptStore::open_memory();
        let enc = [0.1, 0.2, 0.3, 0.4, -0.1, -0.2, -0.3, 0.5];
        let id = store.store_concept("test", "test concept", &enc).unwrap();
        let c = store.get_concept(id).unwrap();
        assert_eq!(c.name, "test");
        for i in 0..8 { assert!((c.encoding[i] - enc[i]).abs() < 1e-10); }
    }

    #[test]
    fn concept_count_increases() {
        let mut store = ConceptStore::open_memory();
        store.store_concept("a", "", &[0.0; 8]).unwrap();
        store.store_concept("b", "", &[0.0; 8]).unwrap();
        assert_eq!(store.concept_count(), 2);
    }

    #[test]
    fn query_similar_orders_correctly() {
        let mut store = ConceptStore::open_memory();
        store.store_concept("near", "", &[0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
        store.store_concept("far", "", &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]).unwrap();
        let q = Multivector::new([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let results = store.query_similar(&q, 2);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0.name, "near");
    }

    #[test]
    fn add_and_retrieve_relation() {
        let mut store = ConceptStore::open_memory();
        let id1 = store.store_concept("a", "", &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
        let id2 = store.store_concept("b", "", &[0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
        store.add_relation(id1, id2).unwrap();
        assert_eq!(store.get_relations_from(id1).len(), 1);
    }

    #[test]
    fn export_produces_valid_json() {
        let mut store = ConceptStore::open_memory();
        store.store_concept("x", "", &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
        let exported = store.export_graph();
        assert!(exported["nodes"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn store_and_retrieve_document() {
        let mut store = ConceptStore::open_memory();
        let doc_id = store.store_document("test_doc", Some("source.txt"), Some("en")).unwrap();
        assert_eq!(doc_id, 1);
        let doc = store.get_document(doc_id).unwrap();
        assert_eq!(doc.name, "test_doc");
        assert_eq!(doc.source, Some("source.txt".to_string()));
        assert_eq!(doc.language, Some("en".to_string()));
    }

    #[test]
    fn list_multiple_documents() {
        let mut store = ConceptStore::open_memory();
        store.store_document("doc1", None, None).unwrap();
        store.store_document("doc2", None, None).unwrap();
        let docs = store.list_documents();
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn store_concept_with_document_id() {
        let mut store = ConceptStore::open_memory();
        let doc_id = store.store_document("doc", None, None).unwrap();
        let enc = [0.1, 0.2, 0.3, 0.4, -0.1, -0.2, -0.3, 0.5];
        let cid = store.store_concept_with_doc("c1", "concept in doc", &enc, doc_id).unwrap();
        let c = store.get_concept(cid).unwrap();
        assert_eq!(c.document_id, Some(doc_id));
        assert_eq!(c.agent_id, None);
    }

    #[test]
    fn store_concept_with_agent_id() {
        let mut store = ConceptStore::open_memory();
        let enc = [0.1, 0.2, 0.3, 0.4, -0.1, -0.2, -0.3, 0.5];
        let cid = store.store_concept_with_agent("c2", "concept from agent", &enc, "agent-42").unwrap();
        let c = store.get_concept(cid).unwrap();
        assert_eq!(c.agent_id, Some("agent-42".to_string()));
        assert_eq!(c.document_id, None);
    }

    #[test]
    fn query_concepts_by_document() {
        let mut store = ConceptStore::open_memory();
        let doc_id = store.store_document("doc", None, None).unwrap();
        let enc = [0.1; 8];
        store.store_concept_with_doc("a", "", &enc, doc_id).unwrap();
        store.store_concept_with_doc("b", "", &enc, doc_id).unwrap();
        store.store_concept("orphan", "", &enc).unwrap();
        let results = store.query_concepts_by_document(doc_id);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn query_concepts_by_agent() {
        let mut store = ConceptStore::open_memory();
        let enc = [0.1; 8];
        store.store_concept_with_agent("a", "", &enc, "agent-1").unwrap();
        store.store_concept_with_agent("b", "", &enc, "agent-1").unwrap();
        store.store_concept("orphan", "", &enc).unwrap();
        let results = store.query_concepts_by_agent("agent-1");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn backward_compat_store_concept_has_none_ids() {
        let mut store = ConceptStore::open_memory();
        let enc = [1.0; 8];
        let id = store.store_concept("old", "legacy concept", &enc).unwrap();
        let c = store.get_concept(id).unwrap();
        assert_eq!(c.document_id, None);
        assert_eq!(c.agent_id, None);
    }

    #[test]
    fn backward_compat_parse_old_json_no_doc_id() {
        let old_json = r#"{"next_concept_id":2,"next_relation_id":1,"concepts":[{"id":1,"name":"legacy","text":"old data","encoding":[1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0],"created_at":"1970-01-01T00:00:00Z"}],"relations":[]}"#;
        let data: GraphData = serde_json::from_str(old_json).unwrap();
        assert_eq!(data.concepts.len(), 1);
        assert_eq!(data.concepts[0].document_id, None);
        assert_eq!(data.concepts[0].agent_id, None);
        assert!(data.documents.is_empty());
        assert_eq!(data.next_document_id, 0); // serde(default) for i64 is 0
    }
}
