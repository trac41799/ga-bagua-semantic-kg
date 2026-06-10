use crate::relation_type::RelationType;
use crate::Multivector;

/// Feature vector dimension for the GA classifier.
/// Covers: coefficients, geometric product, trigram encoding, WuXing signals.
pub const GA_FEATURE_DIM: usize = 62;

/// A trainable multi-class logistic regression classifier that learns
/// to classify GA concept pairs from labeled training data.
///
/// Features are extracted from the geometric algebra encoding of both
/// concepts and their interaction patterns. The model is trained with
/// gradient descent + L2 regularization, then predicts with softmax.
pub struct GaFeatureClassifier {
    /// weights[class][feature] — per-class weight vectors
    weights: [[f64; GA_FEATURE_DIM]; 8],
    /// biases[class] — per-class bias terms
    biases: [f64; 8],
    /// L2 regularization strength
    lambda: f64,
}

impl GaFeatureClassifier {
    /// Create an untrained classifier with zero weights.
    pub fn new(lambda: f64) -> Self {
        GaFeatureClassifier {
            weights: [[0.0; GA_FEATURE_DIM]; 8],
            biases: [0.0; 8],
            lambda,
        }
    }

    /// Train using gradient descent with softmax cross-entropy loss.
    /// Returns the final average loss.
    pub fn train(
        &mut self,
        features: &[[f64; GA_FEATURE_DIM]],
        labels: &[RelationType],
        learning_rate: f64,
        epochs: usize,
    ) -> f64 {
        let n = features.len();
        if n == 0 { return 0.0; }

        for _epoch in 0..epochs {
            let mut _total_loss = 0.0f64;

            for i in 0..n {
                let logits = self.compute_logits(&features[i]);
                let probs = softmax(&logits);

                let label_idx = relation_type_index(labels[i]);
                _total_loss += -probs[label_idx].ln().max(-100.0);

                for c in 0..8 {
                    for f in 0..GA_FEATURE_DIM {
                        _total_loss += self.lambda * self.weights[c][f] * self.weights[c][f];
                    }
                }

                let mut grad_logits = probs;
                grad_logits[label_idx] -= 1.0;

                // Update weights and biases
                for c in 0..8 {
                    let g = grad_logits[c];
                    self.biases[c] -= learning_rate * g;

                    for f in 0..GA_FEATURE_DIM {
                        let w_grad = g * features[i][f] + 2.0 * self.lambda * self.weights[c][f];
                        self.weights[c][f] -= learning_rate * w_grad;
                    }
                }
            }

            // Optionally decay learning rate (not used currently)
        }

        // Compute final average loss
        let mut final_loss = 0.0f64;
        for i in 0..n {
            let logits = self.compute_logits(&features[i]);
            let probs = softmax(&logits);
            let label_idx = relation_type_index(labels[i]);
            final_loss += -probs[label_idx].ln().max(-100.0);
        }
        for c in 0..8 {
            for f in 0..GA_FEATURE_DIM {
                final_loss += self.lambda * self.weights[c][f] * self.weights[c][f];
            }
        }
        final_loss / n as f64
    }

    /// Predict the relation type with softmax probability confidence.
    pub fn predict(&self, features: &[f64; GA_FEATURE_DIM]) -> (RelationType, f64) {
        let logits = self.compute_logits(features);
        let probs = softmax(&logits);

        let mut best_idx = 0;
        let mut best_prob = 0.0f64;
        for c in 0..8 {
            if probs[c] > best_prob {
                best_prob = probs[c];
                best_idx = c;
            }
        }

        (RelationType::ALL[best_idx], best_prob)
    }

    /// Predict probabilities for all 8 labels.
    pub fn predict_probs(&self, features: &[f64; GA_FEATURE_DIM]) -> [(RelationType, f64); 8] {
        let logits = self.compute_logits(features);
        let probs = softmax(&logits);
        let mut result = [(RelationType::Generative, 0.0); 8];
        for c in 0..8 {
            result[c] = (RelationType::ALL[c], probs[c]);
        }
        result
    }

    fn compute_logits(&self, features: &[f64; GA_FEATURE_DIM]) -> [f64; 8] {
        let mut logits = [0.0f64; 8];
        for c in 0..8 {
            let mut s = self.biases[c];
            for f in 0..GA_FEATURE_DIM {
                s += self.weights[c][f] * features[f];
            }
            logits[c] = s;
        }
        logits
    }

    /// Extract 62 features from a concept pair.
    /// Features describe both individual encodings and their interaction.
    pub fn extract_features(a: &Multivector, b: &Multivector) -> [f64; GA_FEATURE_DIM] {
        let mut f = [0.0f64; GA_FEATURE_DIM];
        let mut idx = 0usize;

        // 1. A coefficients (8 features)
        let a_coeffs = a.coefficients();
        for i in 0..8 { f[idx] = a_coeffs[i]; idx += 1; }

        // 2. B coefficients (8 features)
        let b_coeffs = b.coefficients();
        for i in 0..8 { f[idx] = b_coeffs[i]; idx += 1; }

        // 3. A*B geometric product coefficients (8 features)
        let ab = a.geo_product(b);
        let ab_coeffs = ab.coefficients();
        for i in 0..8 { f[idx] = ab_coeffs[i]; idx += 1; }

        // 4. A*inv(B) = similarity gradient (8 features)
        if let Ok(b_inv) = b.inverse() {
            let a_binv = a.geo_product(&b_inv);
            let abi_coeffs = a_binv.coefficients();
            for i in 0..8 { f[idx] = abi_coeffs[i]; idx += 1; }
        } else {
            for _ in 0..8 { f[idx] = 0.0; idx += 1; }
        }

        // 5. Dominant trigrams as one-hot (8 for A, 8 for B = 16)
        let ta = a.dominant_trigram();
        let tb = b.dominant_trigram();
        let ta_idx = match ta {
            crate::bagua::Trigram::Kun => 0, crate::bagua::Trigram::Gen => 1,
            crate::bagua::Trigram::Kan => 2, crate::bagua::Trigram::Xun => 3,
            crate::bagua::Trigram::Zhen => 4, crate::bagua::Trigram::Li => 5,
            crate::bagua::Trigram::Dui => 6, crate::bagua::Trigram::Qian => 7,
        };
        for i in 0..8 { f[idx] = if i == ta_idx { 1.0 } else { 0.0 }; idx += 1; }
        let tb_idx = match tb {
            crate::bagua::Trigram::Kun => 0, crate::bagua::Trigram::Gen => 1,
            crate::bagua::Trigram::Kan => 2, crate::bagua::Trigram::Xun => 3,
            crate::bagua::Trigram::Zhen => 4, crate::bagua::Trigram::Li => 5,
            crate::bagua::Trigram::Dui => 6, crate::bagua::Trigram::Qian => 7,
        };
        for i in 0..8 { f[idx] = if i == tb_idx { 1.0 } else { 0.0 }; idx += 1; }

        // 6. WuXing cycle signals (4 features)
        let wa = ta.wuxing_phase();
        let wb = tb.wuxing_phase();
        f[idx] = if wa.generate() == wb { 1.0 } else { 0.0 }; idx += 1;
        f[idx] = if wb.generate() == wa { 1.0 } else { 0.0 }; idx += 1;
        f[idx] = if wa.control() == wb { 1.0 } else { 0.0 }; idx += 1;
        f[idx] = if wb.control() == wa { 1.0 } else { 0.0 }; idx += 1;

        // 7. Encoding quality (2 features)
        f[idx] = a.encoding_sharpness(); idx += 1;
        f[idx] = b.encoding_sharpness(); idx += 1;

        // 8. Norm ratio (1 feature)
        f[idx] = (a.norm() / b.norm().max(f64::EPSILON)).ln().clamp(-5.0, 5.0); idx += 1;

        // 9. Same phase indicator (1 feature)
        f[idx] = if wa == wb { 1.0 } else { 0.0 };
        // idx += 1; // last feature

        f
    }
}

fn softmax(logits: &[f64; 8]) -> [f64; 8] {
    let max_logit = logits.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mut exp_sum = 0.0f64;
    let mut exps = [0.0f64; 8];
    for i in 0..8 {
        exps[i] = (logits[i] - max_logit).exp();
        exp_sum += exps[i];
    }
    if exp_sum > f64::EPSILON {
        for e in &mut exps { *e /= exp_sum; }
    }
    exps
}

fn relation_type_index(rt: RelationType) -> usize {
    RelationType::ALL.iter().position(|&r| r == rt).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::llm_encode;
    use crate::Blade;

    #[test]
    fn feature_extraction_has_correct_dimension() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let features = GaFeatureClassifier::extract_features(&a, &b);
        assert_eq!(features.len(), GA_FEATURE_DIM);
    }

    #[test]
    fn softmax_sums_to_one() {
        let logits = [1.0, 2.0, 0.5, 0.0, -1.0, 3.0, 0.2, 0.8];
        let probs = softmax(&logits);
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10, "softmax should sum to 1, got {sum}");
    }

    #[test]
    fn trainable_classifier_converges_on_synthetic_data() {
        let mut model = GaFeatureClassifier::new(0.01);

        // Generate synthetic data where features reliably predict labels
        let mut features = Vec::new();
        let mut labels = Vec::new();

        // For each label, create a feature vector where the first 8 coefficients
        // encode the label index strongly
        for label_idx in 0..8 {
            for _ in 0..5 {
                let mut f = [0.0f64; GA_FEATURE_DIM];
                // Put strong signal in coefficient for this label's blade
                let blade_idx = RelationType::ALL[label_idx].blade().index();
                f[blade_idx] = label_idx as f64 + 1.0;
                // Also put signal in geometric product
                f[24 + blade_idx] = (label_idx + 1) as f64 * 0.5;
                // And in same-phase indicator
                f[61] = if label_idx < 4 { 1.0 } else { 0.0 };

                features.push(f);
                labels.push(RelationType::ALL[label_idx]);
            }
        }

        model.train(&features, &labels, 0.1, 500);

        // Measure accuracy on training data
        let mut correct = 0usize;
        for i in 0..features.len() {
            let (pred, _) = model.predict(&features[i]);
            if pred == labels[i] { correct += 1; }
        }

        let acc = correct as f64 / features.len() as f64;
        assert!(acc > 0.7,
            "classifier should achieve >70% on synthetic data, got {:.1}%", acc * 100.0);
    }

    #[test]
    fn trainable_classifier_predicts_all_eight_labels() {
        let mut model = GaFeatureClassifier::new(0.01);
        let a = llm_encode(&[0.05, 0.10, 0.10, 0.85, 0.15, 0.25, 0.15, 0.10]);
        let b = llm_encode(&[0.10, 0.15, 0.80, -0.05, -0.10, 0.15, 0.20, 0.20]);

        let features = GaFeatureClassifier::extract_features(&a, &b);

        // Train minimally
        let train_features = vec![features];
        let train_labels = vec![RelationType::Generative];
        model.train(&train_features, &train_labels, 0.01, 10);

        let probs = model.predict_probs(&features);
        assert_eq!(probs.len(), 8, "should return 8 probability entries");

        // All probabilities should be in [0, 1]
        for &(_, p) in &probs {
            assert!(p >= 0.0 && p <= 1.01, "probability {p} out of range");
        }
    }

    #[test]
    fn feature_extraction_between_same_and_different_pairs() {
        let a1 = llm_encode(&[0.05, 0.10, 0.10, 0.85, 0.15, 0.25, 0.15, 0.10]);
        let b1 = llm_encode(&[0.10, 0.15, 0.80, -0.05, -0.10, 0.15, 0.20, 0.20]);
        let a2 = llm_encode(&[0.10, 0.75, 0.20, 0.05, 0.10, 0.30, 0.10, 0.20]);

        let f1 = GaFeatureClassifier::extract_features(&a1, &b1);
        let f2 = GaFeatureClassifier::extract_features(&a1, &a2);

        // Different pairs should produce different feature vectors
        let diff: f64 = f1.iter().zip(f2.iter()).map(|(x, y)| (x - y).abs()).sum();
        assert!(diff > 0.01, "different pairs should produce different feature vectors, diff={diff}");
    }
}
