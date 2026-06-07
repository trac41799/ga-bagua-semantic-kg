use crate::blade::Blade;
use crate::Multivector;

// ── Trigram ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trigram {
    Kun, Gen, Kan, Xun,
    Zhen, Li, Dui, Qian,
}

impl Trigram {
    pub const ALL: [Trigram; 8] = [
        Trigram::Kun, Trigram::Gen, Trigram::Kan, Trigram::Xun,
        Trigram::Zhen, Trigram::Li, Trigram::Dui, Trigram::Qian,
    ];

    pub fn blade(self) -> Blade {
        match self {
            Trigram::Kun => Blade::Scalar,
            Trigram::Zhen => Blade::E1,
            Trigram::Kan => Blade::E2,
            Trigram::Gen => Blade::E3,
            Trigram::Li => Blade::E12,
            Trigram::Xun => Blade::E23,
            Trigram::Dui => Blade::E31,
            Trigram::Qian => Blade::E123,
        }
    }

    pub fn from_blade(blade: Blade) -> Option<Self> {
        Some(match blade {
            Blade::Scalar => Trigram::Kun,
            Blade::E1 => Trigram::Zhen,
            Blade::E2 => Trigram::Kan,
            Blade::E3 => Trigram::Gen,
            Blade::E12 => Trigram::Li,
            Blade::E23 => Trigram::Xun,
            Blade::E31 => Trigram::Dui,
            Blade::E123 => Trigram::Qian,
        })
    }

    pub fn index(self) -> usize { self.blade().index() }

    pub fn from_index(i: usize) -> Option<Self> {
        Blade::from_index(i).and_then(Self::from_blade)
    }

    pub fn name(self) -> &'static str {
        match self {
            Trigram::Kun => "坤 Kūn",
            Trigram::Gen => "艮 Gèn",
            Trigram::Kan => "坎 Kǎn",
            Trigram::Xun => "巽 Xùn",
            Trigram::Zhen => "震 Zhèn",
            Trigram::Li => "離 Lí",
            Trigram::Dui => "兌 Duì",
            Trigram::Qian => "乾 Qián",
        }
    }

    pub fn translation(self) -> &'static str {
        match self {
            Trigram::Kun => "Earth", Trigram::Gen => "Mountain",
            Trigram::Kan => "Water", Trigram::Xun => "Wind",
            Trigram::Zhen => "Thunder", Trigram::Li => "Fire",
            Trigram::Dui => "Lake", Trigram::Qian => "Heaven",
        }
    }

    pub fn binary(self) -> [bool; 3] {
        match self {
            Trigram::Kun => [false, false, false],
            Trigram::Gen => [false, false, true],
            Trigram::Kan => [false, true, false],
            Trigram::Xun => [false, true, true],
            Trigram::Zhen => [true, false, false],
            Trigram::Li => [true, false, true],
            Trigram::Dui => [true, true, false],
            Trigram::Qian => [true, true, true],
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Trigram::Kun => "The Receptive, yielding, ground state",
            Trigram::Gen => "Keeping Still, stillness, boundary",
            Trigram::Kan => "The Abyssal, danger, flow",
            Trigram::Xun => "The Gentle, penetration, flexibility",
            Trigram::Zhen => "The Arousing, initiative, excitation",
            Trigram::Li => "The Clinging, clarity, radiance",
            Trigram::Dui => "The Joyous, pleasure, reflection",
            Trigram::Qian => "The Creative, force, persistence",
        }
    }

    pub fn wuxing_phase(self) -> WuXing {
        match self {
            Trigram::Zhen | Trigram::Xun => WuXing::Wood,
            Trigram::Li => WuXing::Fire,
            Trigram::Kun | Trigram::Gen => WuXing::Earth,
            Trigram::Qian | Trigram::Dui => WuXing::Metal,
            Trigram::Kan => WuXing::Water,
        }
    }

    pub fn complementary(self) -> Trigram {
        match self {
            Trigram::Kun => Trigram::Qian, Trigram::Gen => Trigram::Dui,
            Trigram::Kan => Trigram::Li, Trigram::Xun => Trigram::Zhen,
            Trigram::Zhen => Trigram::Xun, Trigram::Li => Trigram::Kan,
            Trigram::Dui => Trigram::Gen, Trigram::Qian => Trigram::Kun,
        }
    }

    pub fn transform_line(self, line: usize) -> Option<Trigram> {
        if line > 2 { return None; }
        let mut bits = self.binary();
        bits[line] = !bits[line];
        Some(Self::from_binary(bits))
    }

    pub fn all_transforms(self) -> [Trigram; 3] {
        [self.transform_line(0).unwrap(), self.transform_line(1).unwrap(), self.transform_line(2).unwrap()]
    }

    fn from_binary(bits: [bool; 3]) -> Trigram {
        match bits {
            [false, false, false] => Trigram::Kun,
            [false, false, true] => Trigram::Gen,
            [false, true, false] => Trigram::Kan,
            [false, true, true] => Trigram::Xun,
            [true, false, false] => Trigram::Zhen,
            [true, false, true] => Trigram::Li,
            [true, true, false] => Trigram::Dui,
            [true, true, true] => Trigram::Qian,
        }
    }

    pub fn grade(self) -> usize { self.blade().grade() }

    pub fn as_rotor(self, theta: f64) -> Option<crate::Rotor> {
        if self.grade() != 2 { return None; }
        crate::Rotor::new(theta, self.blade())
    }
}

// ── WuXing ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WuXing { Wood, Fire, Earth, Metal, Water }

impl WuXing {
    pub const ALL: [WuXing; 5] = [WuXing::Wood, WuXing::Fire, WuXing::Earth, WuXing::Metal, WuXing::Water];

    pub fn trigrams(self) -> &'static [Trigram] {
        match self {
            WuXing::Wood => &[Trigram::Zhen, Trigram::Xun],
            WuXing::Fire => &[Trigram::Li],
            WuXing::Earth => &[Trigram::Kun, Trigram::Gen],
            WuXing::Metal => &[Trigram::Qian, Trigram::Dui],
            WuXing::Water => &[Trigram::Kan],
        }
    }

    pub fn generate(self) -> WuXing {
        match self {
            WuXing::Wood => WuXing::Fire, WuXing::Fire => WuXing::Earth,
            WuXing::Earth => WuXing::Metal, WuXing::Metal => WuXing::Water,
            WuXing::Water => WuXing::Wood,
        }
    }

    pub fn control(self) -> WuXing {
        match self {
            WuXing::Wood => WuXing::Earth, WuXing::Fire => WuXing::Metal,
            WuXing::Earth => WuXing::Water, WuXing::Metal => WuXing::Wood,
            WuXing::Water => WuXing::Fire,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            WuXing::Wood => "Wood (木)", WuXing::Fire => "Fire (火)",
            WuXing::Earth => "Earth (土)", WuXing::Metal => "Metal (金)",
            WuXing::Water => "Water (水)",
        }
    }
}

// ── Hexagram ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hexagram { upper: Trigram, lower: Trigram }

impl Hexagram {
    pub fn new(upper: Trigram, lower: Trigram) -> Self { Hexagram { upper, lower } }

    pub fn upper(self) -> Trigram { self.upper }
    pub fn lower(self) -> Trigram { self.lower }
    pub fn upper_name(self) -> &'static str { self.upper.name() }
    pub fn lower_name(self) -> &'static str { self.lower.name() }

    /// Binary number (0..63) = upper(3 bits) << 3 | lower(3 bits)
    /// Uses binary trigram order: Kun=0, Gen=1, Kan=2, Xun=3, Zhen=4, Li=5, Dui=6, Qian=7
    pub fn binary_number(self) -> u8 {
        (trigram_binary_index(self.upper) << 3) | trigram_binary_index(self.lower)
    }

    /// Traditional hexagram name from the I-Ching
    pub fn name(self) -> &'static str {
        let u = trigram_binary_index(self.upper) as usize;
        let l = trigram_binary_index(self.lower) as usize;
        HEXAGRAM_NAMES[u][l]
    }

    pub fn from_multivector_pair(a: &Multivector, b: &Multivector) -> Self {
        let upper = a.dominant_trigram();
        let product = a.geo_product(b);
        let lower = product.dominant_trigram();
        Hexagram { upper, lower }
    }

    pub fn interpretation(self) -> &'static str {
        let u = trigram_binary_index(self.upper) as usize;
        let l = trigram_binary_index(self.lower) as usize;
        HEXAGRAM_INTERPRETATIONS[u][l]
    }

    pub fn pinyin(self) -> &'static str {
        let u = trigram_binary_index(self.upper) as usize;
        let l = trigram_binary_index(self.lower) as usize;
        HEXAGRAM_PINYIN[u][l]
    }

    pub fn role_pair_name(self) -> String {
        format!("{} over {}", self.upper.translation(), self.lower.translation())
    }
}

/// Binary trigram index (0-7): Kun=0, Gen=1, Kan=2, Xun=3, Zhen=4, Li=5, Dui=6, Qian=7
fn trigram_binary_index(t: Trigram) -> u8 {
    match t {
        Trigram::Kun => 0, Trigram::Gen => 1, Trigram::Kan => 2, Trigram::Xun => 3,
        Trigram::Zhen => 4, Trigram::Li => 5, Trigram::Dui => 6, Trigram::Qian => 7,
    }
}

/// Traditional hexagram names indexed by [trigram_binary_index(upper)][trigram_binary_index(lower)]
/// Row/col order: Kun, Gen, Kan, Xun, Zhen, Li, Dui, Qian
const HEXAGRAM_NAMES: [[&str; 8]; 8] = [
    ["坤", "復", "師", "謙", "豫", "比", "剥", "晉"],
    ["屯", "震", "解", "蹇", "小過", "恒", "豐", "歸妹"],
    ["蒙", "渙", "坎", "解", "蹇", "井", "困", "未濟"],
    ["升", "恆", "井", "巽", "蠱", "鼎", "姤", "大過"],
    ["萃", "豐", "困", "咸", "大過", "蠱", "兌", "履"],
    ["晉", "旅", "比", "鼎", "離", "革", "豐", "噬嗑"],
    ["否", "咸", "困", "姤", "履", "兌", "履", "夬"],
    ["否", "无妄", "訟", "遯", "革", "姤", "履", "乾"],
];

const HEXAGRAM_PINYIN: [[&str; 8]; 8] = [
    ["Kun", "Fu", "Shi", "Qian", "Yu", "Bi", "Bo", "Jin"],
    ["Zhun", "Zhen", "Xie", "Jian", "XiaoGuo", "Heng", "Feng", "GuiMei"],
    ["Meng", "Huan", "Kan", "Xie", "Jian", "Jing", "Kun", "WeiJi"],
    ["Sheng", "Heng", "Jing", "Xun", "Gu", "Ding", "Gou", "DaGuo"],
    ["Cui", "Feng", "Kun", "Xian", "DaGuo", "Gu", "Dui", "Lu"],
    ["Jin", "Lu", "Bi", "Ding", "Li", "Ge", "Feng", "ShiKe"],
    ["Pi", "Xian", "Kun", "Gou", "Lu", "Dui", "Lu", "Guai"],
    ["Pi", "WuWang", "Song", "Dun", "Ge", "Gou", "Lu", "Qian"],
];

const HEXAGRAM_INTERPRETATIONS: [[&str; 8]; 8] = [
    ["Pure Earth — total receptivity; ground state; passive acceptance of structure",
     "Return — the turning point; cyclic renewal; a pattern re-emerging after dormancy",
     "The Army — organized collective action; disciplined mobilization of resources",
     "Modesty — balanced self-assessment; neither over-nor-under-valuing one's position",
     "Enthusiasm — forward momentum; excited engagement with emerging possibilities",
     "Holding Together — mutual affiliation; voluntary association around a center",
     "Splitting Apart — decay of structure; the outer shell crumbles revealing inner truth",
     "Progress — steady advancement; gradual illumination of what was hidden"],

    ["Difficulty at the Beginning — initial chaos before order; the hard first step",
     "Pure Thunder — arousing force; sudden excitation that breaks stasis",
     "Deliverance — release from tension; resolution of accumulated pressure",
     "Obstruction — a blockage that demands circumvention; path temporarily closed",
     "Small Excess — slight overcorrection; fine-tuning with restraint",
     "Duration — enduring persistence; sustained effort through changing conditions",
     "Abundance — fullness and plenty; resources exceeding immediate need",
     "The Marrying Maiden — asymmetric partnership; integration of unequal elements"],

    ["Youthful Folly — inexperience seeking guidance; the teachable moment before mastery",
     "Dispersion — dissolving of rigid structures; flow overcoming solid barriers",
     "Pure Water — dangerous depth; the abyss that forces adaptation",
     "Deliverance — liberation from confinement; breaking free of constraints",
     "Obstruction — barriers requiring indirect approach; patience over force",
     "The Well — deep resource that sustains others; infrastructure beneath the surface",
     "Oppression — exhaustion under external pressure; the breaking point before renewal",
     "Before Completion — the final step before the goal; anticipation with preparation"],

    ["Pushing Upward — organic growth; rising by serving the structure beneath",
     "Duration — persistent steady state; the constant amid changing conditions",
     "The Well — foundational infrastructure; what others draw from without noticing",
     "Pure Wind — gentle penetration; pervasive influence without confrontation",
     "Decay — corruption of what was once sound; rot exposing structural weakness",
     "The Cauldron — transformation through refinement; raw material becoming nourishment",
     "Coming to Meet — unexpected encounter; the meeting of complementary forces",
     "Great Excess — overreach beyond capacity; the beam that sags under its own weight"],

    ["Gathering Together — convergence around a shared purpose; mass alignment",
     "Abundance — overflowing richness; when influence exceeds expectations",
     "Oppression — being trapped by circumstances; the crucible that forces change",
     "Influence — mutual attraction and response; resonance between separate systems",
     "Great Excess — structural overextension; the point where strength becomes weakness",
     "Decay — deterioration revealing hidden order; what must be cleared for renewal",
     "Pure Lake — joyous reflection; mirroring that creates mutual understanding",
     "Treading — careful navigation of dangerous ground; measured step through risk"],

    ["Progress — dawn breaking; the moment illumination overtakes obscurity",
     "The Wanderer — the outsider's perspective; clarity from detachment",
     "Holding Together — affiliation through shared qualities; clustering by affinity",
     "The Cauldron — the vessel of transformation; alchemy of the collective",
     "Pure Fire — radiant clarity; dependence on a fuel source for sustained light",
     "Revolution — systematic change; molting the old skin to reveal the new",
     "Abundance — peak illumination; when everything is visible and nothing hidden",
     "Biting Through — decisive action cutting through obstruction; the clean break"],

    ["Standstill — stagnation when creative force meets passive inertia; mutual blockage",
     "Influence — resonance across distance; attraction without direct contact",
     "Oppression — constraint that forces resourcefulness; limitation as catalyst",
     "Coming to Meet — the fateful encounter; two systems intersecting unexpectedly",
     "Treading — cautious advance through uncertain territory; measured risk-taking",
     "Pure Lake — joyful exchange; mutual reflection creating shared understanding",
     "Treading — continued careful navigation; persistent caution through complexity",
     "Breakthrough — decisive resolution; the dam breaking after long pressure"],

    ["Standstill — creative above, receptive below; intention blocked by passivity",
     "Innocence — spontaneous right action; uncalculated natural correctness",
     "Conflict — tension demanding resolution; the productive argument",
     "Retreat — strategic withdrawal; conservation through distance",
     "Revolution — fundamental transformation; the cycle completing and beginning anew",
     "Coming to Meet — convergence of complementary opposites; synthesis moment",
     "Treading — conducting oneself through hierarchy; the dance of protocol",
     "Pure Heaven — creative force unbounded; total generative potential realized"],
];

pub fn wuxing_generating_chain(start: WuXing) -> Vec<(WuXing, WuXing)> {
    let mut chain = Vec::with_capacity(5);
    let mut current = start;
    for _ in 0..5 {
        let next = current.generate();
        chain.push((current, next));
        current = next;
    }
    chain
}

pub fn wuxing_controlling_chain(start: WuXing) -> Vec<(WuXing, WuXing)> {
    let mut chain = Vec::with_capacity(5);
    let mut current = start;
    for _ in 0..5 {
        let next = current.control();
        chain.push((current, next));
        current = next;
    }
    chain
}

pub fn trigram_transform_details(t: Trigram) -> Vec<(Trigram, &'static str)> {
    let line_names = ["bottom-line (heaven/intent) shifts", "middle-line (human/method) shifts", "top-line (earth/outcome) shifts"];
    t.all_transforms().iter().enumerate().map(|(i, &x)| (x, line_names[i])).collect()
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigram_blade_bidirectional() {
        for t in &Trigram::ALL {
            assert_eq!(Trigram::from_blade(t.blade()).unwrap(), *t);
        }
    }

    #[test]
    fn all_blades_map_to_trigrams() {
        for blade in &Blade::ALL {
            let t = Trigram::from_blade(*blade).unwrap();
            assert_eq!(t.blade(), *blade);
        }
    }

    #[test]
    fn trigram_binary_encoding() {
        assert_eq!(Trigram::Kun.binary(),  [false, false, false]);
        assert_eq!(Trigram::Gen.binary(),  [false, false, true]);
        assert_eq!(Trigram::Kan.binary(),  [false, true, false]);
        assert_eq!(Trigram::Xun.binary(),  [false, true, true]);
        assert_eq!(Trigram::Zhen.binary(), [true, false, false]);
        assert_eq!(Trigram::Li.binary(),   [true, false, true]);
        assert_eq!(Trigram::Dui.binary(),  [true, true, false]);
        assert_eq!(Trigram::Qian.binary(), [true, true, true]);
    }

    #[test]
    fn trigram_metadata_not_empty() {
        for t in &Trigram::ALL {
            assert!(!t.name().is_empty());
            assert!(!t.translation().is_empty());
            assert!(!t.description().is_empty());
        }
    }

    #[test]
    fn complementary_pairs() {
        assert_eq!(Trigram::Kun.complementary(),  Trigram::Qian);
        assert_eq!(Trigram::Qian.complementary(), Trigram::Kun);
        assert_eq!(Trigram::Gen.complementary(),  Trigram::Dui);
        assert_eq!(Trigram::Zhen.complementary(), Trigram::Xun);
    }

    #[test]
    fn trigram_transforms_change_one_line() {
        for t in &Trigram::ALL {
            let xforms = t.all_transforms();
            for (i, x) in xforms.iter().enumerate() {
                assert!(*x != *t, "transform {i} of {t:?} should differ");
                let diff = t.binary().iter().zip(x.binary().iter()).filter(|(a, b)| a != b).count();
                assert_eq!(diff, 1, "transform {i} of {t:?} changes exactly 1 line");
            }
        }
    }

    #[test]
    fn hexagram_binary_number() {
        let h = Hexagram::new(Trigram::Qian, Trigram::Kun);
        assert_eq!(h.binary_number(), 56);
    }

    #[test]
    fn hexagram_all_64_have_names() {
        for u in &Trigram::ALL {
            for l in &Trigram::ALL {
                let h = Hexagram::new(*u, *l);
                assert!(!h.name().is_empty(), "no name for ({u:?}, {l:?})");
            }
        }
    }

    #[test]
    fn wuxing_trigram_mapping() {
        assert_eq!(Trigram::Zhen.wuxing_phase(), WuXing::Wood);
        assert_eq!(Trigram::Xun.wuxing_phase(),  WuXing::Wood);
        assert_eq!(Trigram::Li.wuxing_phase(),    WuXing::Fire);
        assert_eq!(Trigram::Kun.wuxing_phase(),   WuXing::Earth);
        assert_eq!(Trigram::Gen.wuxing_phase(),   WuXing::Earth);
        assert_eq!(Trigram::Qian.wuxing_phase(),  WuXing::Metal);
        assert_eq!(Trigram::Dui.wuxing_phase(),   WuXing::Metal);
        assert_eq!(Trigram::Kan.wuxing_phase(),   WuXing::Water);
    }

    #[test]
    fn wuxing_generating_cycle() {
        let mut p = WuXing::Wood;
        for exp in [WuXing::Fire, WuXing::Earth, WuXing::Metal, WuXing::Water, WuXing::Wood] {
            p = p.generate();
            assert_eq!(p, exp);
        }
    }

    #[test]
    fn wuxing_controlling_cycle() {
        let mut p = WuXing::Wood;
        for exp in [WuXing::Earth, WuXing::Water, WuXing::Fire, WuXing::Metal, WuXing::Wood] {
            p = p.control();
            assert_eq!(p, exp);
        }
    }

    #[test]
    fn wuxing_trigrams_consistent() {
        for phase in &WuXing::ALL {
            for t in phase.trigrams() {
                assert_eq!(t.wuxing_phase(), *phase);
            }
        }
    }

    #[test]
    fn as_rotor_only_bivector_trigrams() {
        assert!(Trigram::Li.as_rotor(1.0).is_some());
        assert!(Trigram::Xun.as_rotor(1.0).is_some());
        assert!(Trigram::Dui.as_rotor(1.0).is_some());
        assert!(Trigram::Kun.as_rotor(1.0).is_none());
        assert!(Trigram::Zhen.as_rotor(1.0).is_none());
    }

    #[test]
    fn hexagram_all_have_interpretations() {
        for u in &Trigram::ALL {
            for l in &Trigram::ALL {
                let h = Hexagram::new(*u, *l);
                assert!(!h.interpretation().is_empty(), "no interpretation for ({u:?}, {l:?})");
                assert!(!h.pinyin().is_empty());
            }
        }
    }
}
