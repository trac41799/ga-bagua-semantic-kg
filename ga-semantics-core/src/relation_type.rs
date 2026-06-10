use crate::bagua::{Trigram, WuXing};
use crate::blade::Blade;
use crate::Multivector;
use std::fmt;
use std::str::FromStr;

/// Configurable feature weights for the weighted multi-hypothesis classifier.
#[derive(Clone, Copy, Debug)]
pub struct FeatureWeights {
    pub f1: f64, // WuXing exact cycle match
    pub f2: f64, // WuXing partial alignment
    pub f3: f64, // Trigram quality (A's coefficient at label's blade)
    pub f4: f64, // Geometric product (A*B coefficient at label's blade)
}

impl Default for FeatureWeights {
    fn default() -> Self {
        FeatureWeights { f1: 0.5, f2: 0.1, f3: 0.2, f4: 0.2 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RelationType {
    Generative,
    Receptive,
    Causal,
    Transmissive,
    Constraining,
    Influential,
    Clarifying,
    Balancing,
}

impl RelationType {
    pub const ALL: [RelationType; 8] = [
        RelationType::Generative,
        RelationType::Receptive,
        RelationType::Causal,
        RelationType::Transmissive,
        RelationType::Constraining,
        RelationType::Influential,
        RelationType::Clarifying,
        RelationType::Balancing,
    ];

    pub fn role_name(self) -> &'static str {
        match self {
            RelationType::Generative => "generative",
            RelationType::Receptive => "receptive",
            RelationType::Causal => "causal",
            RelationType::Transmissive => "transmissive",
            RelationType::Constraining => "constraining",
            RelationType::Influential => "influential",
            RelationType::Clarifying => "clarifying",
            RelationType::Balancing => "balancing",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            RelationType::Generative => "Introduces, creates, initiates new patterns",
            RelationType::Receptive => "Accepts, follows, grounds; adopts conventions",
            RelationType::Causal => "Triggers, starts a chain reaction; event-driven",
            RelationType::Transmissive => "Channels, flows, transmits; data pipelines",
            RelationType::Constraining => "Limits, bounds, restricts; permissions, capacity",
            RelationType::Influential => "Pervades, gradually affects; convention spreading",
            RelationType::Clarifying => "Reveals, illuminates, makes visible; introspection",
            RelationType::Balancing => "Mirrors, equilibrates, reflects; feedback loops",
        }
    }

    pub fn bagua(self) -> Trigram {
        match self {
            RelationType::Generative => Trigram::Qian,
            RelationType::Receptive => Trigram::Kun,
            RelationType::Causal => Trigram::Zhen,
            RelationType::Transmissive => Trigram::Kan,
            RelationType::Constraining => Trigram::Gen,
            RelationType::Influential => Trigram::Xun,
            RelationType::Clarifying => Trigram::Li,
            RelationType::Balancing => Trigram::Dui,
        }
    }

    pub fn from_trigram(t: Trigram) -> Self {
        match t {
            Trigram::Qian => RelationType::Generative,
            Trigram::Kun => RelationType::Receptive,
            Trigram::Zhen => RelationType::Causal,
            Trigram::Kan => RelationType::Transmissive,
            Trigram::Gen => RelationType::Constraining,
            Trigram::Xun => RelationType::Influential,
            Trigram::Li => RelationType::Clarifying,
            Trigram::Dui => RelationType::Balancing,
        }
    }

    pub fn wuxing_phase(self) -> WuXing {
        self.bagua().wuxing_phase()
    }

    pub fn blade(self) -> Blade {
        self.bagua().blade()
    }

    /// Classify the relationship between two multivectors using WuXing cycle dynamics
    /// and hexagram stacking. Deterministic, no training, no algebraic transformation error.
    ///
    /// Classification logic (in priority order):
    ///
    /// Trigram-specific overrides (each trigram has a characteristic dynamic quality
    /// that takes precedence over the generic WuXing phase label):
    /// 0a. A=Zhen(causal) generates/controls B → "causal" (initiating/triggering)
    /// 0b. A=Kan(transmissive) generates B → "transmissive" (channeling/flowing)
    /// 0c. A=Li(clarifying) generates B → "clarifying" (illuminating/revealing)
    /// 0d. A=Xun(influential) generates/controls B → "influential" (pervading/shaping)
    ///
    /// Generic WuXing cycle rules:
    /// 1. A generates B → "generative"
    /// 2. B generates A → "receptive"
    /// 3. A controls B → "constraining"
    /// 4. B controls A → "influential"
    /// 5. Same phase + complementary trigrams → "balancing"
    /// 6. Same phase + different trigrams → "clarifying"
    /// 7. Same trigram → "receptive"
    /// 8. Fallback: hexagram-based classification
    pub fn from_pair(a: &Multivector, b: &Multivector) -> (Self, f64) {
        use crate::bagua::Trigram;

        let ta = a.dominant_trigram();
        let tb = b.dominant_trigram();
        let wa = ta.wuxing_phase();
        let wb = tb.wuxing_phase();

        const SHARPNESS_THRESHOLD: f64 = 0.25;
        let quality = a.encoding_sharpness().min(b.encoding_sharpness());
        if quality < SHARPNESS_THRESHOLD {
            let rel = if wa.generate() == wb {
                RelationType::Generative
            } else if wb.generate() == wa {
                RelationType::Receptive
            } else if wa.control() == wb {
                RelationType::Constraining
            } else if wb.control() == wa {
                RelationType::Influential
            } else {
                RelationType::Receptive
            };
            return (rel, 0.0);
        }

        // ── Trigram-specific generate overrides: when A generates B,
        //     the specific trigram's characteristic quality refines the label.
        //     Control relationships remain generic ("constraining" is Gen's domain). ──

        // Zhen (causal/initiating) generates → triggering creation = "causal"
        if ta == Trigram::Zhen && wa.generate() == wb {
            return (RelationType::Causal, 0.9);
        }
        // Kan (transmissive/flowing) generates → nourishing creation = "transmissive"
        if ta == Trigram::Kan && wa.generate() == wb {
            return (RelationType::Transmissive, 0.9);
        }
        // Li (clarifying/revealing) generates → illuminating creation = "clarifying"
        if ta == Trigram::Li && wa.generate() == wb {
            return (RelationType::Clarifying, 0.9);
        }
        // Xun (influential/pervading) generates → shaping creation = "influential"
        if ta == Trigram::Xun && wa.generate() == wb {
            return (RelationType::Influential, 0.9);
        }

        // ── Generic WuXing cycle rules ──
        if wa.generate() == wb {
            return (RelationType::Generative, 1.0);
        }
        if wb.generate() == wa {
            return (RelationType::Receptive, 1.0);
        }
        if wa.control() == wb {
            return (RelationType::Constraining, 1.0);
        }
        if wb.control() == wa {
            return (RelationType::Influential, 1.0);
        }
        if wa == wb && ta.complementary() == tb {
            return (RelationType::Balancing, 0.9);
        }
        if wa == wb && ta != tb {
            return (RelationType::Clarifying, 0.7);
        }
        if ta == tb {
            return (RelationType::Receptive, 0.6);
        }

        let hex = crate::bagua::Hexagram::from_multivector_pair(a, b);
        let hex_rel = match (hex.upper(), hex.lower()) {
            (crate::bagua::Trigram::Qian, _) => RelationType::Generative,
            (crate::bagua::Trigram::Kun, _) => RelationType::Receptive,
            (crate::bagua::Trigram::Zhen, _) => RelationType::Causal,
            (crate::bagua::Trigram::Kan, _) => RelationType::Transmissive,
            (crate::bagua::Trigram::Gen, _) => RelationType::Constraining,
            (crate::bagua::Trigram::Xun, _) => RelationType::Influential,
            (crate::bagua::Trigram::Li, _) => RelationType::Clarifying,
            (crate::bagua::Trigram::Dui, _) => RelationType::Balancing,
        };
        let product = a.geo_product(b);
        let conf = product.norm() / (a.norm() * b.norm()).max(f64::EPSILON);
        (hex_rel, conf.clamp(0.0, 1.0))
    }

    /// Multi-hypothesis classification: evaluates all 8 relation types
    /// simultaneously using WuXing cycle alignment, trigram quality, and
    /// geometric product evidence. Returns the best label with honest
    /// confidence that reflects how strongly the evidence supports one
    /// interpretation over others.
    ///
    /// Unlike `from_pair()` which uses a rigid priority chain, this method
    /// scores every possible label and selects the highest-scoring one.
    /// Confidence is computed as the margin between the best and second-best
    /// scores, so ambiguous cases get appropriately low confidence.
    pub fn from_pair_multi(a: &Multivector, b: &Multivector) -> (Self, f64) {
        let ta = a.dominant_trigram();
        let tb = b.dominant_trigram();
        let wa = ta.wuxing_phase();
        let wb = tb.wuxing_phase();

        // Score each label
        let mut scores: [(Self, f64); 8] = [
            (RelationType::Generative, 0.0),
            (RelationType::Receptive, 0.0),
            (RelationType::Causal, 0.0),
            (RelationType::Transmissive, 0.0),
            (RelationType::Constraining, 0.0),
            (RelationType::Influential, 0.0),
            (RelationType::Clarifying, 0.0),
            (RelationType::Balancing, 0.0),
        ];

        for (label, score) in &mut scores {
            *score = Self::score_weighted(ta, tb, wa, wb, a, b, *label, &FeatureWeights::default());
        }

        // Sort by score descending
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let (best_label, best_score) = scores[0];
        let second_score = scores[1].1;

        // Confidence = normalized margin: how much better is the top choice?
        let margin = best_score - second_score;
        let conf = if best_score > f64::EPSILON {
            (margin / best_score).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // If no label has a meaningful score, fall back to sensible defaults
        if best_score < 0.02 {
            // Same trigram → both concepts share the same role → receptive
            if ta == tb {
                return (RelationType::Receptive, 0.6);
            }
            let (fallback, _) = Self::from_pair(a, b);
            return (fallback, 0.0);
        }

        (best_label, conf)
    }

    /// Multi-hypothesis classification with geometric confidence.
    /// Like from_pair_multi but uses geometric_confidence() for calibration.
    pub fn from_pair_with_geom_conf(a: &Multivector, b: &Multivector) -> (Self, f64) {
        let (label, _score_conf) = Self::from_pair_multi(a, b);
        let geoms = Self::geometric_confidence(a, b);

        // Find the probability for the predicted label
        let geom_prob = geoms.iter()
            .find(|(l, _)| *l == label)
            .map(|(_, p)| *p)
            .unwrap_or(0.0);

        // Blend geometric probability with score-margin confidence
        let conf = (geom_prob * 0.7 + _score_conf * 0.3).clamp(0.0, 1.0);

        (label, conf)
    }

    /// Multi-hypothesis classification with configurable feature weights.
    /// Each label gets scored using 4 features multiplied by per-feature weights.
    /// Returns the best label with confidence margin.
    pub fn from_pair_weighted(
        a: &Multivector, b: &Multivector, weights: &FeatureWeights,
    ) -> (Self, f64) {
        let ta = a.dominant_trigram();
        let tb = b.dominant_trigram();
        let wa = ta.wuxing_phase();
        let wb = tb.wuxing_phase();

        let mut scores: [(Self, f64); 8] = RelationType::ALL.map(|l| (l, 0.0));
        for (label, score) in &mut scores {
            *score = Self::score_weighted(ta, tb, wa, wb, a, b, *label, weights);
        }
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let (best_label, best_score) = scores[0];
        let second_score = scores[1].1;
        let conf = if best_score > f64::EPSILON {
            ((best_score - second_score) / best_score).clamp(0.0, 1.0)
        } else { 0.0 };

        if best_score < 0.01 { let (fb, _) = Self::from_pair(a, b); return (fb, 0.0); }
        (best_label, conf)
    }

    fn score_weighted(
        ta: crate::bagua::Trigram, tb: crate::bagua::Trigram,
        wa: crate::bagua::WuXing, wb: crate::bagua::WuXing,
        a: &Multivector, b: &Multivector, label: Self, w: &FeatureWeights,
    ) -> f64 {
        use crate::bagua::Trigram;

        // Feature 1: WuXing exact cycle match
        let f1: f64 = match label {
            RelationType::Generative if wa.generate() == wb => 1.0,
            RelationType::Receptive if wb.generate() == wa => 1.0,
            RelationType::Constraining if wa.control() == wb => 1.0,
            RelationType::Influential if wb.control() == wa => 1.0,
            RelationType::Causal if ta == Trigram::Zhen && wa.generate() == wb => 1.0,
            RelationType::Transmissive if ta == Trigram::Kan && wa.generate() == wb => 1.0,
            RelationType::Clarifying if ta == Trigram::Li && wa.generate() == wb => 1.0,
            RelationType::Influential if ta == Trigram::Xun && wa.generate() == wb => 1.0,
            RelationType::Balancing if wa == wb && ta.complementary() == tb => 0.9,
            RelationType::Clarifying if wa == wb && ta != tb => 0.7,
            _ => 0.0,
        };

        // Feature 2: WuXing partial alignment (2-step)
        let f2: f64 = match (label, wa, wb) {
            (RelationType::Generative, ap, bp) if ap.generate().generate() == bp => 0.4,
            (RelationType::Receptive, ap, bp) if bp.generate().generate() == ap => 0.4,
            (RelationType::Constraining, ap, bp) if ap.control().control() == bp => 0.25,
            (RelationType::Influential, ap, bp) if bp.control().control() == ap => 0.25,
            _ => 0.0,
        };

        // Feature 3: Trigram quality (A's coefficient at label's blade)
        let a_wt = a.coefficient(label.blade().index()).abs();
        let f3 = 1.0 / (1.0 + (-6.0 * (a_wt - 0.25)).exp());

        // Feature 4: Geometric product (A*B coefficient at label's blade)
        let product = a.geo_product(b);
        let f4 = (product.coefficient(label.blade().index()).abs()
            / product.norm().max(f64::EPSILON)).min(1.0);

        // Weighted sum
        let s = w.f1 * f1 + w.f2 * f2 + w.f3 * f3 + w.f4 * f4;
        s.clamp(0.0, 1.0)
    }

    /// Simple grid-search weight optimization from training pairs.
    /// Returns the FeatureWeights that maximize F1 on training data.
    pub fn optimize_weights(
        training_pairs: &[(&Multivector, &Multivector, RelationType)],
    ) -> FeatureWeights {
        let steps = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
        let mut best = FeatureWeights::default();
        let mut best_f1 = 0.0;

        // Compute per-label F1 for a given weight config
        let eval = |w: &FeatureWeights| -> f64 {
            let mut tp = [0usize; 8]; let mut fp = [0usize; 8]; let mut fn_ = [0usize; 8];
            for &(a, b, expected) in training_pairs {
                let (pred, _) = RelationType::from_pair_weighted(a, b, w);
                let ei = RelationType::ALL.iter().position(|&r| r == expected).unwrap();
                let pi = RelationType::ALL.iter().position(|&r| r == pred).unwrap();
                if pred == expected { tp[ei] += 1; }
                else { fn_[ei] += 1; fp[pi] += 1; }
            }
            let mut total_f1 = 0.0f64;
            for i in 0..8 {
                let p = if tp[i] + fp[i] > 0 { tp[i] as f64 / (tp[i] + fp[i]) as f64 } else { 0.0 };
                let r = if tp[i] + fn_[i] > 0 { tp[i] as f64 / (tp[i] + fn_[i]) as f64 } else { 0.0 };
                if p + r > 0.0 { total_f1 += 2.0 * p * r / (p + r); }
            }
            total_f1 / 8.0 // macro-averaged F1
        };

        // Grid search over f1/f2/f3/f4
        for &f1 in &steps {
            for &f2 in &steps {
                for &f3 in &steps {
                    for &f4 in &steps {
                        let w = FeatureWeights { f1, f2, f3, f4 };
                        let f1_score = eval(&w);
                        if f1_score > best_f1 {
                            best_f1 = f1_score;
                            best = w;
                        }
                    }
                }
            }
        }

        best
    }

    /// Compute confidence as a probability distribution over all 8 labels
    /// based on the geometric product A*B pattern. The geometric product
    /// captures the actual compound dynamics — if A generates B, the product
    /// should have a strong generative blade component.
    ///
    /// Returns 8 pairs of (RelationType, probability) summing to 1.0.
    pub fn geometric_confidence(a: &Multivector, b: &Multivector) -> [(Self, f64); 8] {
        let product = a.geo_product(b);
        let prod_coeffs = product.coefficients();
        let total = prod_coeffs.iter().map(|c| c.abs()).sum::<f64>().max(f64::EPSILON);

        let mut probs = [(RelationType::Generative, 0.0); 8];
        for (i, label) in RelationType::ALL.iter().enumerate() {
            let blade_idx = label.blade().index();
            // How strongly does the product express this label's blade?
            let raw = prod_coeffs[blade_idx].abs() / total;

            // Blend with WuXing cycle evidence for stability
            let ta = a.dominant_trigram();
            let tb = b.dominant_trigram();
            let wa = ta.wuxing_phase();
            let wb = tb.wuxing_phase();
            let cycle_bonus = match *label {
                RelationType::Generative if wa.generate() == wb => 0.15,
                RelationType::Receptive if wb.generate() == wa => 0.15,
                RelationType::Constraining if wa.control() == wb => 0.15,
                RelationType::Influential if wb.control() == wa => 0.15,
                _ => 0.0,
            };

            probs[i] = (*label, (raw + cycle_bonus).min(1.0));
        }

        // Normalize to sum to 1.0
        let sum: f64 = probs.iter().map(|(_, p)| p).sum();
        if sum > f64::EPSILON {
            for (_, p) in &mut probs { *p /= sum; }
        }

        probs
    }

    /// Generate a corrective encoding suggestion for an LLM feedback loop.
    /// Given a misclassified pair, provides a natural-language suggestion
    /// for how to re-encode both concepts so the classifier produces the
    /// expected label.
    pub fn corrective_prompt(
        a_name: &str, b_name: &str,
        a: &Multivector, b: &Multivector,
        expected: RelationType,
    ) -> Option<String> {
        let (actual, _) = Self::from_pair(a, b);
        if actual == expected { return None; }

        let ta = a.dominant_trigram();
        let tb = b.dominant_trigram();
        let wa = ta.wuxing_phase();
        let wb = tb.wuxing_phase();

        let target_phase_for_a = Self::phase_needed_for(expected, wb, true);
        let target_phase_for_b = Self::phase_needed_for(expected, wa, false);

        let mut prompt = format!(
            "RELATION CLASSIFICATION FAILURE:\n\
               {a_name} -> {b_name}\n\
               Expected: {expected} ({}), Got: {actual} ({})\n\n\
               Current encodings:\n  {a_name}: {ta:?} ({wa:?} phase)\n  {b_name}: {tb:?} ({wb:?} phase)\n\n",
            expected.description(), actual.description(),
        );

        if let Some(tgt) = target_phase_for_a {
            let trigrams = tgt.trigrams();
            prompt.push_str(&format!(
                "FIX: Re-encode \"{a_name}\" into {tgt:?} phase. Available trigrams: {trigrams:?}.\n\
                 Reason: For an {expected} relationship with B in {wb:?} phase, A must be in {tgt:?}.\n\
                 Ask: What does \"{a_name}\" DO in the {tgt:?} roles? \
                 Encode the strongest matching role as dominant.\n"
            ));
        }
        if let Some(tgt) = target_phase_for_b {
            let trigrams = tgt.trigrams();
            prompt.push_str(&format!(
                "ALTERNATIVE: Re-encode \"{b_name}\" into {tgt:?} phase (trigrams: {trigrams:?}).\n\
                 Reason: With A in {wa:?}, B must be in {tgt:?} for {expected}.\n"
            ));
        }

        Some(prompt)
    }

    /// Determine the WuXing phase needed for a concept to satisfy a relation.
    fn phase_needed_for(
        relation: RelationType,
        known_phase: crate::bagua::WuXing,
        need_a: bool,
    ) -> Option<crate::bagua::WuXing> {
        use crate::bagua::WuXing;
        let all = [WuXing::Wood, WuXing::Fire, WuXing::Earth, WuXing::Metal, WuXing::Water];
        match (relation, need_a) {
            (RelationType::Generative, true) => all.iter().find(|&&w| w.generate() == known_phase).copied(),
            (RelationType::Generative, false) => Some(known_phase.generate()),
            (RelationType::Receptive, true) => Some(known_phase.generate()),
            (RelationType::Receptive, false) => all.iter().find(|&&w| w.generate() == known_phase).copied(),
            (RelationType::Constraining, true) => all.iter().find(|&&w| w.control() == known_phase).copied(),
            (RelationType::Constraining, false) => Some(known_phase.control()),
            (RelationType::Influential, true) => Some(known_phase.control()),
            (RelationType::Influential, false) => all.iter().find(|&&w| w.control() == known_phase).copied(),
            (RelationType::Causal, true) => Some(WuXing::Wood.generate()),
            (RelationType::Causal, false) => all.iter().find(|&&w| w.generate() == known_phase).copied(),
            (RelationType::Transmissive, true) => Some(WuXing::Water.generate()),
            (RelationType::Transmissive, false) => all.iter().find(|&&w| w.generate() == known_phase).copied(),
            (RelationType::Balancing, _) | (RelationType::Clarifying, _) => Some(known_phase),
        }
    }
}

impl fmt::Display for RelationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.role_name())
    }
}

impl FromStr for RelationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "generative" | "qian" | "乾" => Ok(RelationType::Generative),
            "receptive" | "kun" | "坤" => Ok(RelationType::Receptive),
            "causal" | "zhen" | "震" => Ok(RelationType::Causal),
            "transmissive" | "kan" | "坎" => Ok(RelationType::Transmissive),
            "constraining" | "gen" | "艮" => Ok(RelationType::Constraining),
            "influential" | "xun" | "巽" => Ok(RelationType::Influential),
            "clarifying" | "li" | "離" | "离" => Ok(RelationType::Clarifying),
            "balancing" | "dui" | "兌" | "兑" => Ok(RelationType::Balancing),
            _ => Err(format!("unknown relation type: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Blade;

    #[test]
    fn all_variants_have_unique_role_names() {
        let mut names: Vec<&str> = RelationType::ALL.iter().map(|r| r.role_name()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 8);
    }

    #[test]
    fn from_trigram_roundtrip() {
        for t in &Trigram::ALL {
            let rt = RelationType::from_trigram(*t);
            assert_eq!(rt.bagua(), *t);
        }
    }

    #[test]
    fn all_variants_have_descriptions() {
        for r in &RelationType::ALL {
            assert!(!r.description().is_empty());
        }
    }

    #[test]
    fn from_pair_e1_e2_is_receptive() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let (rt, conf) = RelationType::from_pair(&a, &b);
        assert_eq!(rt, RelationType::Receptive);
        assert!((conf - 1.0).abs() < 1e-10);
    }

    #[test]
    fn from_str_role_names_work() {
        assert_eq!("generative".parse::<RelationType>().unwrap(), RelationType::Generative);
        assert_eq!("causal".parse::<RelationType>().unwrap(), RelationType::Causal);
        assert_eq!("constraining".parse::<RelationType>().unwrap(), RelationType::Constraining);
    }

    #[test]
    fn from_str_bagua_names_work() {
        assert_eq!("qian".parse::<RelationType>().unwrap(), RelationType::Generative);
        assert_eq!("zhen".parse::<RelationType>().unwrap(), RelationType::Causal);
        assert_eq!("li".parse::<RelationType>().unwrap(), RelationType::Clarifying);
    }

    #[test]
    fn from_str_unknown_returns_err() {
        assert!("bogus".parse::<RelationType>().is_err());
    }

    #[test]
    fn display_uses_role_name() {
        assert_eq!(RelationType::Causal.to_string(), "causal");
        assert_eq!(RelationType::Generative.to_string(), "generative");
    }

    #[test]
    fn wuxing_consistent_with_bagua() {
        for r in &RelationType::ALL {
            assert_eq!(r.wuxing_phase(), r.bagua().wuxing_phase());
        }
    }

    #[test]
    fn blade_consistent_with_bagua() {
        for r in &RelationType::ALL {
            assert_eq!(r.blade(), r.bagua().blade());
        }
    }

    #[test]
    fn from_pair_diffuse_encodings_get_low_confidence() {
        let a = Multivector::new([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let b = Multivector::new([1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0]);
        let (_, conf) = RelationType::from_pair(&a, &b);
        assert!(conf < 0.25, "diffuse encodings should get low confidence, got {}", conf);
    }

    #[test]
    fn from_pair_sharp_encodings_keep_high_confidence() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let (rel, conf) = RelationType::from_pair(&a, &b);
        assert_eq!(rel, RelationType::Receptive);
        assert!((conf - 1.0).abs() < 1e-10,
            "sharp blade encodings generate/receive cycle should give conf=1.0, got {}", conf);
    }

    #[test]
    fn from_pair_random_encodings_filtered_by_gate() {
        use crate::encoding::llm_encode;
        let mut high_conf = 0usize;
        let mut seed: u64 = 0xBEEF;
        for _ in 0..1000 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let s1 = seed;
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let s2 = seed;
            let make = |s: u64| {
                let raw = [
                    ((s as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(3) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(7) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(11) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(13) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(17) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(19) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(23) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                ];
                llm_encode(&raw)
            };
            let (_, conf) = RelationType::from_pair(&make(s1), &make(s2));
            if conf > 0.3 { high_conf += 1; }
        }
        let high_pct = high_conf as f64 / 1000.0 * 100.0;
        assert!(high_pct < 15.0,
            "<15% of random pairs should get >0.3 confidence (got {:.1}%)", high_pct);
    }

    // ── Multi-hypothesis classifier tests ──

    #[test]
    fn multi_hypothesis_matches_original_on_hand_tuned_pairs() {
        // For blade-encoded pairs (sharp, single-trigram), both classifiers
        // should agree since the evidence is unambiguous.
        let pairs: Vec<(Multivector, Multivector)> = vec![
            (Multivector::from_blade(Blade::E1, 1.0), Multivector::from_blade(Blade::E2, 1.0)),
            (Multivector::from_blade(Blade::E3, 1.0), Multivector::from_blade(Blade::E2, 1.0)),
            (Multivector::from_blade(Blade::E12, 1.0), Multivector::from_blade(Blade::E1, 1.0)),
        ];
        for (a, b) in &pairs {
            let (orig_label, _) = RelationType::from_pair(a, b);
            let (multi_label, _) = RelationType::from_pair_multi(a, b);
            assert_eq!(orig_label, multi_label,
                "multi-hypothesis should match original on blade pairs. \
                 Original={orig_label:?}, Multi={multi_label:?}");
        }
    }

    #[test]
    fn multi_hypothesis_confidence_on_ambiguous_is_lower() {
        // Same trigram both sides → ambiguous → multi should give lower confidence
        let a = Multivector::from_blade(Blade::E12, 1.0);
        let b = Multivector::from_blade(Blade::E12, 1.0); // same blade
        let (_, orig_conf) = RelationType::from_pair(&a, &b);
        let (_, multi_conf) = RelationType::from_pair_multi(&a, &b);

        // Original would give 0.6 (same trigram → receptive with 0.6)
        // Multi should give <= 0.6 since it's ambiguous
        assert!(multi_conf <= orig_conf + 0.01,
            "multi confidence ({multi_conf}) should not exceed original ({orig_conf}) for ambiguous case");
    }

    #[test]
    fn multi_hypothesis_returns_all_eight_possible_labels() {
        use crate::encoding::llm_encode;
        let mut seed: u64 = 0xFEED;
        let mut labels_seen = std::collections::HashSet::new();

        for _ in 0..500 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let s1 = seed;
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let s2 = seed;
            let make = |s: u64| {
                let raw = [
                    ((s as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(3) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(7) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(11) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(13) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(17) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(19) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(23) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                ];
                llm_encode(&raw)
            };
            let (label, conf) = RelationType::from_pair_multi(&make(s1), &make(s2));
            if conf > 0.1 {
                labels_seen.insert(label);
            }
        }

        // With 500 random pairs, we should see at least 5 different labels
        // (not all 8 may appear due to random coverage, but at least 5 should)
        assert!(labels_seen.len() >= 5,
            "should see at least 5 distinct labels from random pairs, got {}", labels_seen.len());
    }

    #[test]
    fn multi_hypothesis_confidence_in_range() {
        use crate::encoding::llm_encode;
        let mut seed: u64 = 0xABCD;
        for _ in 0..200 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let s1 = seed;
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let s2 = seed;
            let make = |s: u64| {
                let raw = [
                    ((s as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(3) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(7) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(11) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(13) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(17) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(19) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                    ((s.wrapping_mul(23) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                ];
                llm_encode(&raw)
            };
            let (_, conf) = RelationType::from_pair_multi(&make(s1), &make(s2));
            assert!(conf >= 0.0 && conf <= 1.0,
                "confidence must be in [0, 1], got {conf}");
        }
    }

    #[test]
    fn multi_hypothesis_agrees_on_dataset_passing_pairs() {
        // Use the correct pairs from the benchmark that are known to pass
        let rate_limiter = crate::encoding::llm_encode(&[
            0.0428, -0.0856, -0.5136, 0.6847, 0.214, -0.2568, 0.1712, -0.3424,
        ]);
        let api_gateway = crate::encoding::llm_encode(&[
            0.2247, 0.337, 0.8425, 0.0562, -0.1123, 0.1685, 0.2808, 0.0562,
        ]);

        // Rate Limiter(Gen/Earth) → API Gateway(Kan/Water): Earth controls Water → constraining
        let (multi_label, multi_conf) = RelationType::from_pair_multi(&rate_limiter, &api_gateway);
        assert_eq!(multi_label, RelationType::Constraining,
            "multi should correctly identify constraining");
        assert!(multi_conf > 0.3,
            "confidence should be meaningful for correct pair, got {multi_conf}");
    }

    #[test]
    fn multi_hypothesis_confidence_lower_on_failing_pairs() {
        // Pipeline(Kan/Water) → Revenue Target(Qian/Metal)
        // Known to fail: Metal generates Water → receptive (wrong direction)
        let pipeline = crate::encoding::llm_encode(&[
            0.115, 0.1725, 0.8971, -0.0575, -0.115, 0.1725, 0.23, 0.23,
        ]);
        let revenue = crate::encoding::llm_encode(&[
            0.1086, 0.3259, 0.1086, -0.1086, 0.1629, 0.1086, 0.1629, 0.8908,
        ]);

        let (_, orig_conf) = RelationType::from_pair(&pipeline, &revenue);
        let (_, multi_conf) = RelationType::from_pair_multi(&pipeline, &revenue);

        // Original gives 1.0 confidence (wrongly! Metal generates Water)
        // Multi should give lower confidence since the evidence is conflicting
        assert!(multi_conf <= orig_conf,
            "multi confidence ({multi_conf}) should be <= original ({orig_conf}) for known failure. \
             Original gives false 1.0; multi should be more honest.");
    }

    // ── Geometric confidence tests ──

    #[test]
    fn geometric_confidence_sums_to_one() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let conf = RelationType::geometric_confidence(&a, &b);
        let sum: f64 = conf.iter().map(|(_, p)| p).sum();
        assert!((sum - 1.0).abs() < 1e-10,
            "geometric confidence should sum to 1.0, got {sum}");
    }

    #[test]
    fn geometric_confidence_all_eight_labels_present() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let conf = RelationType::geometric_confidence(&a, &b);
        assert_eq!(conf.len(), 8);
        // For pure blade pairs, the product is sparse — not all labels get mass.
        // This is honest behavior: the geometry doesn't support every label.
        let nonzero = conf.iter().filter(|(_, p)| *p > 1e-10).count();
        assert!(nonzero >= 2, "at least 2 labels should get probability mass, got {nonzero}");
    }

    #[test]
    fn geometric_confidence_distinguishes_control_from_generate() {
        // Wood(E1/Zhen) controls Earth(E3/Gen): E1*E3 = -E31 (bivector)
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E3, 1.0);
        let conf = RelationType::geometric_confidence(&a, &b);
        // Constraining (Gen/Earth, E3 blade) should get meaningful probability
        let constraining_prob = conf.iter()
            .find(|(l, _)| *l == RelationType::Constraining)
            .map(|(_, p)| *p).unwrap_or(0.0);
        let generative_prob = conf.iter()
            .find(|(l, _)| *l == RelationType::Generative)
            .map(|(_, p)| *p).unwrap_or(0.0);
        // Wood controls Earth → constraining should score higher than generative
        // because Earth doesn't generate Wood
        assert!(constraining_prob > generative_prob,
            "E1→E3 (Wood controls Earth): constraining={constraining_prob:.3} should > generative={generative_prob:.3}");
    }

    #[test]
    fn corrective_prompt_for_known_failure() {
        let pipeline = crate::encoding::llm_encode(&[
            0.115, 0.1725, 0.8971, -0.0575, -0.115, 0.1725, 0.23, 0.23,
        ]);
        let revenue = crate::encoding::llm_encode(&[
            0.1086, 0.3259, 0.1086, -0.1086, 0.1629, 0.1086, 0.1629, 0.8908,
        ]);

        let prompt = RelationType::corrective_prompt(
            "Sales Pipeline", "Revenue Target",
            &pipeline, &revenue,
            RelationType::Generative,
        );

        assert!(prompt.is_some(), "known failure should generate a corrective prompt");
        let p = prompt.unwrap();
        assert!(p.contains("Sales Pipeline"), "prompt should name concept A");
        assert!(p.contains("Revenue Target"), "prompt should name concept B");
        assert!(p.contains("generative") || p.contains("Generative"),
            "prompt should mention the expected label");
        assert!(p.contains("phase") || p.contains("Phase"),
            "prompt should mention phase changes needed");
    }

    #[test]
    fn corrective_prompt_none_for_correct_pair() {
        let rl = crate::encoding::llm_encode(&[
            0.0428, -0.0856, -0.5136, 0.6847, 0.214, -0.2568, 0.1712, -0.3424,
        ]);
        let gw = crate::encoding::llm_encode(&[
            0.2247, 0.337, 0.8425, 0.0562, -0.1123, 0.1685, 0.2808, 0.0562,
        ]);

        let prompt = RelationType::corrective_prompt(
            "Rate Limiter", "API Gateway",
            &rl, &gw,
            RelationType::Constraining,
        );

        assert!(prompt.is_none(), "correct pairs should not need correction prompts");
    }

    #[test]
    fn geom_conf_better_for_correct_pairs() {
        let rl = crate::encoding::llm_encode(&[
            0.0428, -0.0856, -0.5136, 0.6847, 0.214, -0.2568, 0.1712, -0.3424,
        ]);
        let gw = crate::encoding::llm_encode(&[
            0.2247, 0.337, 0.8425, 0.0562, -0.1123, 0.1685, 0.2808, 0.0562,
        ]);
        // RL(Earth/Gen) → GW(Water/Kan): Earth controls Water → constraining
        let (label, conf) = RelationType::from_pair_with_geom_conf(&rl, &gw);
        assert_eq!(label, RelationType::Constraining);
        assert!(conf > 0.0 && conf <= 1.0, "geom confidence should be in (0,1], got {conf}");
    }

    #[test]
    fn geom_conf_lower_for_ambiguous_pairs() {
        // Two identical blade encodings: E12 vs E12 — same trigram, ambiguous
        let a = Multivector::from_blade(Blade::E12, 1.0);
        let b = Multivector::from_blade(Blade::E12, 1.0);
        let (_, _multi_conf) = RelationType::from_pair_multi(&a, &b);
        let (_, geom_conf) = RelationType::from_pair_with_geom_conf(&a, &b);
        // For ambiguous pairs, confidence should be moderate at most
        assert!(geom_conf < 0.8, "ambiguous pairs should not get high geom conf, got {geom_conf}");
    }

    #[test]
    fn geom_conf_fallback_works_for_diffuse() {
        let a = Multivector::new([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let b = Multivector::new([1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0]);
        let (_, conf) = RelationType::from_pair_with_geom_conf(&a, &b);
        assert!(conf < 0.3, "diffuse encodings should get low geom conf, got {conf}");
    }
}
