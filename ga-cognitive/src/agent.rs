use ga_semantics_core::store::{ConceptStore, StoredConcept};

pub struct AgentStore {
    #[allow(dead_code)]
    store_path: String,
    store: ConceptStore,
}

impl AgentStore {
    pub fn new(store_path: &str) -> Result<Self, String> {
        let store = ConceptStore::open(store_path)?;
        Ok(AgentStore { store_path: store_path.to_string(), store })
    }

    pub fn open_memory() -> Self {
        AgentStore { store_path: String::new(), store: ConceptStore::open_memory() }
    }

    pub fn create_agent(&mut self, agent_name: &str) -> Result<String, String> {
        let agent_id = format!("agent_{}_{}", chrono_now_simple(), agent_counter());
        self.store.store_concept_with_agent(agent_name, &format!("Agent: {}", agent_name), &[0.0; 8], &agent_id)?;
        Ok(agent_id)
    }

    pub fn add_belief(&mut self, agent_id: &str, name: &str, text: &str, encoding: &[f64; 8]) -> Result<i64, String> {
        self.store.store_concept_with_agent(name, text, encoding, agent_id)
    }

    pub fn list_beliefs(&self, agent_id: &str) -> Vec<StoredConcept> {
        self.store.query_concepts_by_agent(agent_id)
    }

    pub fn get_belief(&self, id: i64) -> Option<StoredConcept> {
        self.store.get_concept(id)
    }

    pub fn belief_count(&self, agent_id: &str) -> usize {
        self.list_beliefs(agent_id).len()
    }
}

fn chrono_now_simple() -> String {
    format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
}

fn agent_counter() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_agent_and_add_belief() {
        let mut store = AgentStore::open_memory();
        let agent_id = store.create_agent("TestAgent").unwrap();
        assert!(agent_id.starts_with("agent_"));
        let enc = [0.1, 0.2, 0.3, 0.4, -0.1, -0.2, -0.3, 0.5];
        let id = store.add_belief(&agent_id, "belief1", "test belief", &enc).unwrap();
        assert!(id > 0);
        let belief = store.get_belief(id).unwrap();
        assert_eq!(belief.name, "belief1");
    }

    #[test]
    fn belief_count_tracks_beliefs() {
        let mut store = AgentStore::open_memory();
        let agent_id = store.create_agent("Agent2").unwrap();
        assert_eq!(store.belief_count(&agent_id), 1);
        store.add_belief(&agent_id, "b1", "", &[0.1; 8]).unwrap();
        store.add_belief(&agent_id, "b2", "", &[0.2; 8]).unwrap();
        assert_eq!(store.belief_count(&agent_id), 3);
    }

    #[test]
    fn multiple_agents_independent() {
        let mut store = AgentStore::open_memory();
        let a1 = store.create_agent("AgentA").unwrap();
        let a2 = store.create_agent("AgentB").unwrap();
        store.add_belief(&a1, "alpha", "", &[0.1; 8]).unwrap();
        store.add_belief(&a2, "beta", "", &[0.2; 8]).unwrap();
        assert_eq!(store.belief_count(&a1), 2);
        assert_eq!(store.belief_count(&a2), 2);
    }
}
