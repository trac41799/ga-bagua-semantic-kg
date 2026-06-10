use ga_semantics_core::prelude::*;

// ═══════════════════════════════════════════════════════════════════
// Concept 1: freedom / liberté / 自由 — generative dominant (Qian)
// ═══════════════════════════════════════════════════════════════════
const FREEDOM_EN: [f64; 8] = [0.10, 0.30, 0.25, -0.05, 0.15, 0.10, 0.15, 0.70];
const FREEDOM_FR: [f64; 8] = [0.12, 0.28, 0.23, -0.06, 0.14, 0.12, 0.17, 0.68];
const FREEDOM_JA: [f64; 8] = [0.08, 0.32, 0.27, -0.04, 0.16, 0.08, 0.13, 0.72];

// ═══════════════════════════════════════════════════════════════════
// Concept 2: justice / justice / 正義 — balancing dominant (Dui)
// ═══════════════════════════════════════════════════════════════════
const JUSTICE_EN: [f64; 8] = [0.15, 0.10, 0.20, -0.15, 0.10, 0.25, 0.72, 0.10];
const JUSTICE_FR: [f64; 8] = [0.14, 0.12, 0.18, -0.14, 0.12, 0.23, 0.70, 0.11];
const JUSTICE_JA: [f64; 8] = [0.16, 0.08, 0.22, -0.13, 0.08, 0.27, 0.74, 0.09];

// ═══════════════════════════════════════════════════════════════════
// Concept 3: harmony / harmonie / 調和 — receptive dominant (Kun)
// ═══════════════════════════════════════════════════════════════════
const HARMONY_EN: [f64; 8] = [0.68, 0.10, 0.15, 0.20, 0.30, 0.10, 0.05, 0.05];
const HARMONY_FR: [f64; 8] = [0.66, 0.12, 0.14, 0.22, 0.28, 0.08, 0.07, 0.06];
const HARMONY_JA: [f64; 8] = [0.70, 0.08, 0.16, 0.18, 0.32, 0.12, 0.04, 0.04];

// ═══════════════════════════════════════════════════════════════════
// Concept 4: innovation / innovation / 革新 — causal dominant (Zhen)
// ═══════════════════════════════════════════════════════════════════
const INNOVATION_EN: [f64; 8] = [0.10, 0.65, 0.30, -0.10, 0.15, 0.20, 0.10, 0.25];
const INNOVATION_FR: [f64; 8] = [0.12, 0.63, 0.28, -0.08, 0.14, 0.22, 0.11, 0.24];
const INNOVATION_JA: [f64; 8] = [0.08, 0.67, 0.32, -0.12, 0.16, 0.18, 0.09, 0.26];

// ═══════════════════════════════════════════════════════════════════
// Concept 5: tradition / tradition / 伝統 — constraining dominant (Gen)
// ═══════════════════════════════════════════════════════════════════
const TRADITION_EN: [f64; 8] = [0.20, -0.10, -0.25, 0.65, 0.30, -0.15, 0.10, 0.05];
const TRADITION_FR: [f64; 8] = [0.22, -0.08, -0.27, 0.63, 0.28, -0.14, 0.12, 0.06];
const TRADITION_JA: [f64; 8] = [0.18, -0.12, -0.23, 0.67, 0.32, -0.16, 0.08, 0.04];

#[test]
fn b5_cross_lingual_benchmark() {
    println!("\n=== B5: Cross-Lingual Concept Alignment ===");

    #[allow(dead_code)]
    struct ConceptGroup {
        name: &'static str,
        expected_dominant: &'static str,
        encodings: [(&'static str, [f64; 8]); 3],
    }

    let concepts: [ConceptGroup; 5] = [
        ConceptGroup {
            name: "freedom",
            expected_dominant: "generative",
            encodings: [
                ("freedom_EN", FREEDOM_EN),
                ("freedom_FR", FREEDOM_FR),
                ("freedom_JA", FREEDOM_JA),
            ],
        },
        ConceptGroup {
            name: "justice",
            expected_dominant: "balancing",
            encodings: [
                ("justice_EN", JUSTICE_EN),
                ("justice_FR", JUSTICE_FR),
                ("justice_JA", JUSTICE_JA),
            ],
        },
        ConceptGroup {
            name: "harmony",
            expected_dominant: "receptive",
            encodings: [
                ("harmony_EN", HARMONY_EN),
                ("harmony_FR", HARMONY_FR),
                ("harmony_JA", HARMONY_JA),
            ],
        },
        ConceptGroup {
            name: "innovation",
            expected_dominant: "causal",
            encodings: [
                ("innovation_EN", INNOVATION_EN),
                ("innovation_FR", INNOVATION_FR),
                ("innovation_JA", INNOVATION_JA),
            ],
        },
        ConceptGroup {
            name: "tradition",
            expected_dominant: "constraining",
            encodings: [
                ("tradition_EN", TRADITION_EN),
                ("tradition_FR", TRADITION_FR),
                ("tradition_JA", TRADITION_JA),
            ],
        },
    ];

    // ── Dominant trigram consistency check ──
    println!("\n  Dominant trigram consistency across languages:");
    let mut all_dominant_consistent = true;
    for group in &concepts {
        let mut trigrams: Vec<String> = Vec::new();
        for (label, enc) in &group.encodings {
            let mv = Multivector::new(*enc);
            let role = mv.dominant_role().role_name().to_string();
            trigrams.push(format!("{}({})", label, role));
        }
        let roles: Vec<&str> = trigrams.iter()
            .map(|s| s.split('(').nth(1).unwrap().trim_end_matches(')'))
            .collect();
        let consistent = roles.windows(2).all(|w| w[0] == w[1]);
        if !consistent {
            all_dominant_consistent = false;
        }
        println!("    {}: {} | {}",
            group.name,
            trigrams.join(", "),
            if consistent { "OK" } else { "MISMATCH" }
        );
    }

    // ── INTRA-CONCEPT similarity ──
    println!("\n  INTRA-concept similarities (same concept across languages):");
    let mut intra_sims: Vec<f64> = Vec::new();
    let mut intra_details: Vec<String> = Vec::new();

    for group in &concepts {
        let mvs: Vec<(&&str, Multivector)> = group.encodings.iter()
            .map(|(label, enc)| (label, Multivector::new(*enc)))
            .collect();
        let pairs = [(0, 1), (0, 2), (1, 2)];
        for &(i, j) in &pairs {
            let sim = semantic_similarity(&mvs[i].1, &mvs[j].1);
            intra_sims.push(sim);
            intra_details.push(format!(
                "    {:<12} ↔ {:<12} : {:.4}",
                *mvs[i].0, *mvs[j].0, sim
            ));
        }
    }
    for d in &intra_details {
        println!("{}", d);
    }

    // ── INTER-CONCEPT similarity ──
    println!("\n  INTER-concept similarities (different concepts within same language):");
    let languages = ["EN", "FR", "JA"];
    #[allow(clippy::type_complexity)]
    let lang_encs: [Vec<(&str, [f64; 8])>; 3] = [
        vec![("freedom",    FREEDOM_EN), ("justice", JUSTICE_EN), ("harmony", HARMONY_EN), ("innovation", INNOVATION_EN), ("tradition", TRADITION_EN)],
        vec![("freedom",    FREEDOM_FR), ("justice", JUSTICE_FR), ("harmony", HARMONY_FR), ("innovation", INNOVATION_FR), ("tradition", TRADITION_FR)],
        vec![("freedom",    FREEDOM_JA), ("justice", JUSTICE_JA), ("harmony", HARMONY_JA), ("innovation", INNOVATION_JA), ("tradition", TRADITION_JA)],
    ];

    let mut inter_sims: Vec<f64> = Vec::new();
    let mut inter_details: Vec<String> = Vec::new();

    for (lang_idx, lang) in languages.iter().enumerate() {
        println!("\n    --- {} ---", lang);
        let entries = &lang_encs[lang_idx];
        let mvs: Vec<(&str, Multivector)> = entries.iter()
            .map(|(name, enc)| (*name, Multivector::new(*enc)))
            .collect();
        for i in 0..mvs.len() {
            for j in (i + 1)..mvs.len() {
                let sim = semantic_similarity(&mvs[i].1, &mvs[j].1);
                inter_sims.push(sim);
                let line = format!("      {:<12} ↔ {:<12} : {:.4}", mvs[i].0, mvs[j].0, sim);
                println!("{}", line);
                inter_details.push(line);
            }
        }
    }

    // ── Similarity matrix ──
    println!("\n  Similarity matrix (15 concepts × 15 concepts):");
    let all_encodings: Vec<(&str, [f64; 8])> = vec![
        ("freedom_EN",    FREEDOM_EN),    ("freedom_FR",    FREEDOM_FR),    ("freedom_JA",    FREEDOM_JA),
        ("justice_EN",    JUSTICE_EN),    ("justice_FR",    JUSTICE_FR),    ("justice_JA",    JUSTICE_JA),
        ("harmony_EN",    HARMONY_EN),    ("harmony_FR",    HARMONY_FR),    ("harmony_JA",    HARMONY_JA),
        ("innovation_EN", INNOVATION_EN), ("innovation_FR", INNOVATION_FR), ("innovation_JA", INNOVATION_JA),
        ("tradition_EN",  TRADITION_EN),  ("tradition_FR",  TRADITION_FR),  ("tradition_JA",  TRADITION_JA),
    ];
    let all_mvs: Vec<Multivector> = all_encodings.iter()
        .map(|(_, enc)| Multivector::new(*enc))
        .collect();

    print!("    {:>14}", "");
    for (name, _) in &all_encodings {
        let short = if name.len() > 11 { &name[..11] } else { name };
        print!(" {:>8}", short);
    }
    println!();
    for (i, (name_a, _)) in all_encodings.iter().enumerate() {
        let short = if name_a.len() > 14 { &name_a[..14] } else { name_a };
        print!("    {:>14}", short);
        for (j, _) in all_encodings.iter().enumerate() {
            let sim = semantic_similarity(&all_mvs[i], &all_mvs[j]);
            print!(" {:>8.3}", sim);
        }
        println!();
    }

    // ── Metrics ──
    let mean_intra = intra_sims.iter().sum::<f64>() / intra_sims.len() as f64;
    let mean_inter = inter_sims.iter().sum::<f64>() / inter_sims.len() as f64;
    let separation_ratio = if mean_inter > f64::EPSILON { mean_intra / mean_inter } else { f64::MAX };

    println!();
    println!("  ── B5 METRICS ──");
    println!("  mean_intra_similarity : {:.4}", mean_intra);
    println!("  mean_inter_similarity : {:.4}", mean_inter);
    println!("  separation_ratio      : {:.4}", separation_ratio);
    println!("  dominant_trigram_consistent : {}", all_dominant_consistent);

    let threshold_intra = 0.70;
    let threshold_inter = 0.50;
    let threshold_ratio = 1.4;

    let intra_ok = mean_intra >= threshold_intra;
    let inter_ok = mean_inter <= threshold_inter;
    let ratio_ok = separation_ratio >= threshold_ratio;

    let passed = intra_ok && inter_ok && ratio_ok && all_dominant_consistent;

    println!(
        "BENCH: cross_lingual: intra={:.4} (>= {:.2}) inter={:.4} (<= {:.2}) ratio={:.4} (>= {:.2}) dominant_ok={} | {}",
        mean_intra, threshold_intra,
        mean_inter, threshold_inter,
        separation_ratio, threshold_ratio,
        all_dominant_consistent,
        if passed { "PASS" } else { "FAIL" }
    );

    assert!(intra_ok, "mean_intra={:.4} below threshold {:.2}", mean_intra, threshold_intra);
    assert!(inter_ok, "mean_inter={:.4} above threshold {:.2}", mean_inter, threshold_inter);
    assert!(ratio_ok, "separation_ratio={:.4} below threshold {:.2}", separation_ratio, threshold_ratio);
    assert!(all_dominant_consistent, "Dominant trigram mismatch across languages");
}
