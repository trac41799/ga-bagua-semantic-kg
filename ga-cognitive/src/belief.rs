use ga_semantics_core::prelude::*;
use ga_semantics_core::semantics::belief_revise;

pub struct BeliefTimeline {
    pub history: Vec<BeliefSnapshot>,
}

pub struct BeliefSnapshot {
    pub name: String,
    pub encoding: [f64; 8],
    pub timestamp: String,
    pub revision_from_previous: Option<String>,
}

impl Default for BeliefTimeline {
    fn default() -> Self { Self::new() }
}

impl BeliefTimeline {
    pub fn new() -> Self { BeliefTimeline { history: vec![] } }

    pub fn record(&mut self, name: &str, encoding: &[f64; 8]) {
        let prev_rotor = if let Some(last) = self.history.last() {
            let old_mv = Multivector::new(last.encoding);
            let new_mv = Multivector::new(*encoding);
            belief_revise(&old_mv, &new_mv).map(|r| format!("{}", r.multivector()))
        } else { None };

        self.history.push(BeliefSnapshot {
            name: name.to_string(),
            encoding: *encoding,
            timestamp: chrono_now(),
            revision_from_previous: prev_rotor,
        });
    }

    pub fn drift_magnitude(&self) -> f64 {
        if self.history.len() < 2 { return 0.0; }
        let first = Multivector::new(self.history[0].encoding);
        let last = Multivector::new(self.history.last().unwrap().encoding);
        ga_semantics_core::semantics::semantic_difference(&first, &last)
    }
}

fn chrono_now() -> String {
    format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ga_semantics_core::blade::Blade;

    #[test]
    fn record_single_snapshot_no_revision() {
        let mut timeline = BeliefTimeline::new();
        let enc = [0.1, 0.2, 0.3, 0.4, 0.0, 0.0, 0.0, 0.0];
        timeline.record("test", &enc);
        assert_eq!(timeline.history.len(), 1);
        assert!(timeline.history[0].revision_from_previous.is_none());
    }

    #[test]
    fn record_two_snapshots_has_revision() {
        let mut timeline = BeliefTimeline::new();
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        timeline.record("first", a.coefficients());
        timeline.record("second", b.coefficients());
        assert_eq!(timeline.history.len(), 2);
        assert!(timeline.history[1].revision_from_previous.is_some());
    }

    #[test]
    fn drift_magnitude_same_encoding_is_zero() {
        let mut timeline = BeliefTimeline::new();
        let enc = [0.1, 0.2, 0.3, 0.4, 0.0, 0.0, 0.0, 0.0];
        timeline.record("a", &enc);
        timeline.record("b", &enc);
        let drift = timeline.drift_magnitude();
        assert!((drift - 0.0).abs() < 1e-10);
    }

    #[test]
    fn drift_magnitude_different_encodings_positive() {
        let mut timeline = BeliefTimeline::new();
        let enc1 = Multivector::from_blade(Blade::E1, 1.0);
        let enc2 = Multivector::from_blade(Blade::E2, 1.0);
        timeline.record("a", enc1.coefficients());
        timeline.record("b", enc2.coefficients());
        let drift = timeline.drift_magnitude();
        assert!(drift > 0.0);
    }
}
