use crate::multi_encoding::{classify_multi_encoded, MultiEncodedConcept};
use crate::relation_type::{FeatureWeights, RelationType};
use crate::Multivector;

/// Ensemble classifier that combines multiple classification methods
/// via voting to overcome individual classifier weaknesses.
///
/// Each classifier makes different errors. A voting ensemble captures
/// the wisdom of the crowd — the patterns that multiple classifiers
/// agree on are more likely to be correct than any single one.
pub struct EnsembleClassifier {
    pub weights: FeatureWeights,
}

impl Default for EnsembleClassifier {
    fn default() -> Self {
        EnsembleClassifier::new()
    }
}

impl EnsembleClassifier {
    pub fn new() -> Self {
        EnsembleClassifier {
            weights: FeatureWeights::default(),
        }
    }

    pub fn with_weights(weights: FeatureWeights) -> Self {
        EnsembleClassifier { weights }
    }

    /// Collect predictions from all 5 classifiers.
    /// Returns Vec of (label, confidence) — one per classifier.
    pub fn collect_predictions(
        &self,
        a: &Multivector,
        b: &Multivector,
        mc_a: Option<&MultiEncodedConcept>,
        mc_b: Option<&MultiEncodedConcept>,
    ) -> Vec<(RelationType, f64)> {
        let mut preds = vec![
            RelationType::from_pair(a, b),
            RelationType::from_pair_multi(a, b),
            RelationType::from_pair_weighted(a, b, &self.weights),
            RelationType::from_pair_with_geom_conf(a, b),
        ];

        if let (Some(mca), Some(mcb)) = (mc_a, mc_b) {
            preds.push(classify_multi_encoded(mca, mcb, &self.weights));
        }

        preds
    }

    /// Majority vote: pick the label that appears most often.
    /// Tiebreaks by summing confidences per label.
    pub fn classify_majority(
        &self,
        a: &Multivector,
        b: &Multivector,
        mc_a: Option<&MultiEncodedConcept>,
        mc_b: Option<&MultiEncodedConcept>,
    ) -> (RelationType, f64) {
        let preds = self.collect_predictions(a, b, mc_a, mc_b);
        let total = preds.len();

        let mut label_counts: std::collections::HashMap<RelationType, usize> =
            std::collections::HashMap::new();
        let mut label_confidence_sums: std::collections::HashMap<RelationType, f64> =
            std::collections::HashMap::new();

        for &(label, conf) in &preds {
            *label_counts.entry(label).or_insert(0) += 1;
            *label_confidence_sums
                .entry(label)
                .or_insert(0.0) += conf;
        }

        let max_count = label_counts.values().max().copied().unwrap_or(0);
        let tied: Vec<RelationType> = label_counts
            .iter()
            .filter(|(_, &c)| c == max_count)
            .map(|(&l, _)| l)
            .collect();

        // Tiebreak: highest total confidence among tied labels
        let best_label = tied
            .iter()
            .max_by(|&&a, &&b| {
                let ca = label_confidence_sums.get(&a).copied().unwrap_or(0.0);
                let cb = label_confidence_sums.get(&b).copied().unwrap_or(0.0);
                ca.partial_cmp(&cb).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
            .unwrap_or(RelationType::Receptive);

        // Confidence = proportion of classifiers that agree
        let agreement = max_count as f64 / total as f64;

        (best_label, agreement)
    }

    /// Confidence-weighted vote: each classifier casts a confidence-weighted
    /// ballot for its predicted label. The label with the highest total
    /// confidence mass wins.
    pub fn classify_weighted(
        &self,
        a: &Multivector,
        b: &Multivector,
        mc_a: Option<&MultiEncodedConcept>,
        mc_b: Option<&MultiEncodedConcept>,
    ) -> (RelationType, f64) {
        let preds = self.collect_predictions(a, b, mc_a, mc_b);

        let mut mass: [f64; 8] = [0.0; 8];
        let total_conf: f64 = preds.iter().map(|(_, c)| c).sum();

        for &(label, conf) in &preds {
            let idx = RelationType::ALL
                .iter()
                .position(|&r| r == label)
                .unwrap_or(0);
            mass[idx] += conf;
        }

        let best_idx = mass
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        let best_mass = mass[best_idx];
        let confidence = if total_conf > f64::EPSILON {
            (best_mass / total_conf).clamp(0.0, 1.0)
        } else {
            0.0
        };

        (RelationType::ALL[best_idx], confidence)
    }

    /// Smart ensemble: multi-encoding is the primary signal (56.4% baseline).
    /// Other classifiers provide tiebreaking/supplementary evidence.
    /// This avoids the problem where 4 similar from_pair variants outvote
    /// the genuinely different multi-encoding classifier.
    pub fn classify_smart(
        &self,
        a: &Multivector,
        b: &Multivector,
        mc_a: Option<&MultiEncodedConcept>,
        mc_b: Option<&MultiEncodedConcept>,
    ) -> (RelationType, f64) {
        // If we have multi-encoding, use it as primary
        if let (Some(mca), Some(mcb)) = (mc_a, mc_b) {
            let (multi_label, multi_conf) = classify_multi_encoded(mca, mcb, &self.weights);

            // Hexagram classifier as independent second opinion
            let ta = a.dominant_trigram();
            let product = a.geo_product(b);
            let lower = product.dominant_trigram();
            let hex = crate::bagua::Hexagram::new(ta, lower);
            let (hex_label, hex_conf) = hex.relation_type();

            // from_pair_multi as third opinion
            let (pair_label, pair_conf) = RelationType::from_pair_multi(a, b);

            // Collect unique opinions (count per label, NOT per classifier)
            let mut votes: std::collections::HashMap<RelationType, f64> =
                std::collections::HashMap::new();

            *votes.entry(multi_label).or_insert(0.0) += multi_conf * 2.0; // primary ×2 weight
            *votes.entry(hex_label).or_insert(0.0) += hex_conf;
            *votes.entry(pair_label).or_insert(0.0) += pair_conf;

            // Pick label with highest total confidence mass
            let best = votes.iter()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(&l, _)| l)
                .unwrap_or(multi_label);

            let total_mass: f64 = votes.values().sum();
            let best_mass = votes.get(&best).copied().unwrap_or(0.0);
            let conf = if total_mass > f64::EPSILON {
                (best_mass / total_mass).clamp(0.0, 1.0)
            } else { 0.0 };

            return (best, conf);
        }

        // Without multi-encoding, use from_pair_multi as primary
        RelationType::from_pair_multi(a, b)
    }

    fn classify_best_on_train(
        &self,
        a: &Multivector,
        b: &Multivector,
        mc_a: Option<&MultiEncodedConcept>,
        mc_b: Option<&MultiEncodedConcept>,
        classifier_idx: usize,
    ) -> (RelationType, f64) {
        match classifier_idx {
            0 => RelationType::from_pair(a, b),
            1 => RelationType::from_pair_multi(a, b),
            2 => RelationType::from_pair_weighted(a, b, &self.weights),
            3 => RelationType::from_pair_with_geom_conf(a, b),
            4 => {
                if let (Some(mca), Some(mcb)) = (mc_a, mc_b) {
                    classify_multi_encoded(mca, mcb, &self.weights)
                } else {
                    RelationType::from_pair_multi(a, b)
                }
            }
            _ => RelationType::from_pair(a, b),
        }
    }

    /// Select the best classifier index by evaluating accuracy on training data.
    pub fn select_best_classifier(
        &self,
        training: &[(&Multivector, &Multivector, RelationType)],
        mc_pairs: &[(&MultiEncodedConcept, &MultiEncodedConcept)],
    ) -> usize {
        let n_methods = 5;
        let mut best_idx = 0;
        let mut best_acc = 0.0f64;

        for method in 0..n_methods {
            let mut correct = 0usize;
            for (i, &(a, b, expected)) in training.iter().enumerate() {
                let (mc_a, mc_b) = if i < mc_pairs.len() {
                    (Some(mc_pairs[i].0), Some(mc_pairs[i].1))
                } else {
                    (None, None)
                };
                let (pred, _) =
                    self.classify_best_on_train(a, b, mc_a, mc_b, method);
                if pred == expected {
                    correct += 1;
                }
            }
            let acc = correct as f64 / training.len() as f64;
            if acc > best_acc {
                best_acc = acc;
                best_idx = method;
            }
        }

        best_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::llm_encode;
    use crate::Blade;

    fn make_mc(coeffs: &[f64; 8]) -> MultiEncodedConcept {
        let mv = llm_encode(coeffs);
        MultiEncodedConcept::from_single_encoding(&mv)
    }

    // Rate Limiter: constraining (Earth/Gen)
    fn rl_coeffs() -> [f64; 8] {
        [0.0428, -0.0856, -0.5136, 0.6847, 0.214, -0.2568, 0.1712, -0.3424]
    }

    // API Gateway: transmissive (Water/Kan)
    fn gw_coeffs() -> [f64; 8] {
        [0.2247, 0.337, 0.8425, 0.0562, -0.1123, 0.1685, 0.2808, 0.0562]
    }

    // Pipeline: transmissive (Water/Kan)
    fn pl_coeffs() -> [f64; 8] {
        [0.115, 0.1725, 0.8971, -0.0575, -0.115, 0.1725, 0.23, 0.23]
    }

    // Innovation Fund: generative (Metal/Qian)
    fn if_coeffs() -> [f64; 8] {
        [0.05, 0.25, 0.15, -0.15, 0.10, 0.15, 0.10, 0.88]
    }

    #[test]
    fn ensemble_collects_all_predictions() {
        let e = EnsembleClassifier::new();
        let a = llm_encode(&rl_coeffs());
        let b = llm_encode(&gw_coeffs());

        let preds = e.collect_predictions(&a, &b, None, None);
        assert_eq!(preds.len(), 4, "should have 4 predictions without multi-encoding");

        let mc_a = make_mc(&rl_coeffs());
        let mc_b = make_mc(&gw_coeffs());
        let preds_with_mc = e.collect_predictions(&a, &b, Some(&mc_a), Some(&mc_b));
        assert_eq!(preds_with_mc.len(), 5, "should have 5 predictions with multi-encoding");
    }

    #[test]
    fn majority_vote_on_blade_pairs_is_unanimous() {
        let e = EnsembleClassifier::new();
        let a = Multivector::from_blade(Blade::E1, 1.0); // Zhen (causal)
        let b = Multivector::from_blade(Blade::E2, 1.0); // Kan (transmissive)
        // E1→E2: Wood generates Water → generative ... actually wait
        // E1=Zhen(Wood), E2=Kan(Water): Wood generates Water → generative
        // but the trigram override: ta=Zhen,wa.generate()=Fire, not Water...
        // Actually wa=Wood, wb=Water. Wood generates Fire, not Water...
        // So Wood->Water: neither generate nor control cycle matching
        // wa.generate()==wb? Wood→Fire, not Water. No.
        // wb.generate()==wa? Water→Wood, YES! Water generates Wood → receptive
        // So from_pair should say receptive

        let (label, conf) = e.classify_majority(&a, &b, None, None);
        // For E1→E2: Water(Wood is wrong... E1=Zhen=Wood, E2=Kan=Water)
        // Water generates Wood → wb.generate() == wa → receptive
        assert_eq!(label, RelationType::Receptive,
            "E1(Zhen/Wood)→E2(Kan/Water): Water generates Wood → receptive");
        assert!(conf > 0.5,
            "majority should have high agreement on blade pairs, got {conf}");
    }

    #[test]
    fn weighted_vote_on_known_pair() {
        let e = EnsembleClassifier::new();
        let rl = llm_encode(&rl_coeffs());
        let gw = llm_encode(&gw_coeffs());

        // RL(Earth/Gen) → GW(Water/Kan): Earth controls Water → constraining
        let (label, conf) = e.classify_weighted(&rl, &gw, None, None);
        assert_eq!(label, RelationType::Constraining,
            "RL→GW should be constraining, got {:?}", label);
        assert!(conf > 0.0 && conf <= 1.0, "confidence should be in (0,1]");
    }

    #[test]
    fn weighted_vote_confidence_in_range() {
        let e = EnsembleClassifier::new();
        let rl = llm_encode(&rl_coeffs());
        let gw = llm_encode(&gw_coeffs());

        // Run weighted vote on several pairs, always produce valid confidence
        let pairs = [
            (&rl, &gw),
        ];
        for (a, b) in &pairs {
            let (_, conf) = e.classify_weighted(a, b, None, None);
            assert!(conf >= 0.0 && conf <= 1.0, "confidence {conf} out of range");
        }
    }

    #[test]
    fn select_best_classifier_returns_valid_index() {
        let e = EnsembleClassifier::new();
        let rl = llm_encode(&rl_coeffs());
        let gw = llm_encode(&gw_coeffs());
        let pl = llm_encode(&pl_coeffs());
        let ifv = llm_encode(&if_coeffs());

        let training = vec![
            (&rl, &gw, RelationType::Constraining),
            (&pl, &ifv, RelationType::Generative),
        ];

        let mc_pairs: Vec<(&MultiEncodedConcept, &MultiEncodedConcept)> = vec![];

        let idx = e.select_best_classifier(&training, &mc_pairs);
        assert!(idx < 5, "should return valid classifier index 0-4, got {idx}");
    }

    #[test]
    fn ensemble_majority_with_multi_encoding() {
        let e = EnsembleClassifier::new();

        let rl = llm_encode(&rl_coeffs());
        let gw = llm_encode(&gw_coeffs());
        let mc_rl = make_mc(&rl_coeffs());
        let mc_gw = make_mc(&gw_coeffs());

        // RL→GW: constraining
        let (label, conf) = e.classify_majority(&rl, &gw, Some(&mc_rl), Some(&mc_gw));
        assert_eq!(label, RelationType::Constraining,
            "ensemble+multi should agree on constraining, got {:?}", label);
        assert!(conf > 0.0);

        // Weighted should also work
        let (label2, conf2) = e.classify_weighted(&rl, &gw, Some(&mc_rl), Some(&mc_gw));
        assert_eq!(label2, RelationType::Constraining,
            "weighted ensemble+multi should agree, got {:?}", label2);
        assert!(conf2 > 0.0);
    }
}
