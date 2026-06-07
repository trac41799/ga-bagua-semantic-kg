use crate::bagua::{Trigram, WuXing};
use crate::blade::Blade;
use crate::Multivector;
use std::fmt;
use std::str::FromStr;

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
    /// 1. A's WuXing generates B's WuXing → generative (confidence 1.0)
    /// 2. B's WuXing generates A's WuXing → receptive (confidence 1.0)
    /// 3. A's WuXing controls B's WuXing → constraining (confidence 1.0)
    /// 4. B's WuXing controls A's WuXing → influential (confidence 1.0)
    /// 5. Same WuXing phase + complementary trigrams → balancing (confidence 0.9)
    /// 6. Same WuXing phase, different same-phase trigrams → clarifying (confidence 0.7)
    /// 7. Same trigram → receptive (confidence 0.6) — self-relation
    /// 8. Fallback: hexagram-based classification
    pub fn from_pair(a: &Multivector, b: &Multivector) -> (Self, f64) {
        let ta = a.dominant_trigram();
        let tb = b.dominant_trigram();
        let wa = ta.wuxing_phase();
        let wb = tb.wuxing_phase();

        // Encoding quality gate: if either concept is near-random (diffuse
        // across all 8 roles), the WuXing cycle match is unreliable.
        // Random uniform-normalised vectors average ~0.16 sharpness;
        // LLM-encoded concepts average ~0.30-0.47.
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
}
