use ga_semantics_core::prelude::*;
use ga_semantics_core::advanced::hexagram_explore;
use std::collections::HashSet;

#[test]
fn b11_creative_ideation_benchmark() {
    println!("\n=== B11: Creative Ideation Quality ===");

    let seed_coeffs: [f64; 8] = [0.0, 1.0, 0.1, 0.1, 0.05, 0.05, 0.05, 0.05];
    let seed = llm_encode(&seed_coeffs);

    println!("  Seed: causal/Wood dominant");
    println!("  Dominant trigram: {}", seed.dominant_role().bagua().name());
    println!("  Seed coefficients: {:?}", seed.coefficients());

    let results = hexagram_explore(&seed, 64);
    assert_eq!(results.len(), 64, "Should explore all 64 hexagrams");
    println!("  Results count: {}", results.len());

    let mut all_mvs: Vec<Multivector> = results.iter().map(|(_, mv, _)| mv.clone()).collect();
    all_mvs.insert(0, seed.clone());

    let mut total_dist = 0.0f64;
    let mut pairs = 0usize;
    for i in 0..all_mvs.len() {
        for j in (i + 1)..all_mvs.len() {
            total_dist += ga_semantics_core::semantics::semantic_difference(&all_mvs[i], &all_mvs[j]);
            pairs += 1;
        }
    }
    let mean_dist = total_dist / pairs as f64;
    println!("  Mean pairwise geometric distance: {:.4}", mean_dist);

    let mut trigrams_seen = HashSet::new();
    for (_, mv, _) in &results {
        trigrams_seen.insert(mv.dominant_role().bagua().name().to_string());
    }
    let coverage = trigrams_seen.len();
    println!("  Unique dominant trigrams: {}/8 ({:?})", coverage, trigrams_seen.iter().collect::<Vec<_>>());

    let mut unique_mvs = HashSet::new();
    for (_, mv, _) in &results {
        let coeffs = mv.coefficients();
        let key: Vec<i64> = coeffs.iter().map(|&c| (c * 1e6) as i64).collect();
        unique_mvs.insert(key);
    }
    println!("  Unique result multivectors: {}", unique_mvs.len());

    let first = results[0].1.coefficients();
    let last = results[63].1.coefficients();
    let first_dist = ga_semantics_core::semantics::semantic_difference(&seed, &Multivector::new(*first));
    let last_dist = ga_semantics_core::semantics::semantic_difference(&seed, &Multivector::new(*last));
    let sorted_desc = first_dist >= last_dist;
    println!("  Sorted by distance descending: {} ({:.4} >= {:.4})", sorted_desc, first_dist, last_dist);
    
    println!("  First 3 results:");
    for (i, (hex, mv, _)) in results.iter().enumerate().take(3) {
        let d = ga_semantics_core::semantics::semantic_difference(&seed, mv);
        println!("    {}: {} (upper={}, dist={:.4}, dom={})",
            i + 1, hex.name(), hex.upper_name(), d, mv.dominant_role().bagua().name());
    }

    assert!(mean_dist > 0.1, "Mean distance should be > 0.1 (meaningful diversity)");
    assert!(coverage >= 3, "Should cover at least 3 different trigrams");

    let passed = mean_dist > 0.1 && coverage >= 3;
    println!(
        "  BENCH: B11: mean_dist={:.4}, trigram_coverage={}/8 | threshold=0.10,3/8 | {}",
        mean_dist, coverage,
        if passed { "PASS" } else { "FAIL" }
    );

    assert!(passed, "Ideation must produce diverse perspectives");
}
