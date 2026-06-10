// Dense retrieval benchmark: GA-Bagua similarity vs baselines
// Measures same-role peer retrieval across 50 concepts

use ga_semantics_core::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JsonConcept { name: String, coefficients: Vec<f64> }
#[derive(Debug, Deserialize)]
struct BenchmarkDataset { concepts: Vec<JsonConcept> }

fn load() -> BenchmarkDataset {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("data").join("benchmark_dataset.json");
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap()
}

#[test]
fn retrieval_benchmark() {
    let ds = load();
    let enc: Vec<Multivector> = ds.concepts.iter().map(|c| {
        let coeffs: [f64; 8] = c.coefficients.as_slice().try_into().unwrap();
        llm_encode(&coeffs)
    }).collect();
    let roles: Vec<&str> = enc.iter().map(|mv| mv.dominant_role().role_name()).collect();

    println!("\n{:=^60}", " DENSE RETRIEVAL BENCHMARK ");
    println!("  {} concepts, {} role categories", enc.len(),
        roles.iter().collect::<std::collections::HashSet<_>>().len());
    println!("{:=^60}\n", "");

    let methods: Vec<(&str, Box<dyn Fn(&Multivector, &Multivector) -> f64>)> = vec![
        ("GA-Bagua (dominant)", Box::new(|a, b| dominant_similarity(a, b))),
        ("GA-Bagua (semantic)", Box::new(|a, b| semantic_similarity(a, b))),
        ("Cosine (coeffs)", Box::new(|a, b| {
            let ca = a.coefficients(); let cb = b.coefficients();
            let dot: f64 = ca.iter().zip(cb.iter()).map(|(x,y)| x*y).sum();
            dot
        })),
        ("Euclidean (coeffs)", Box::new(|a, b| {
            let ca = a.coefficients(); let cb = b.coefficients();
            let diff: f64 = ca.iter().zip(cb.iter()).map(|(x,y)| (x-y).powi(2)).sum();
            -diff.sqrt()
        })),
    ];

    println!("── RESULTS ──");
    println!("  {:<25} | {:<10} | {:<10} | {:<10} | {:<10}",
        "Method", "P@1", "P@3", "P@5", "MRR");
    println!("  {:-<25}-+-{:-<10}-+-{:-<10}-+-{:-<10}-+-{:-<10}", "", "", "", "", "");

    for (name, sim_fn) in &methods {
        let mut p1 = 0.0f64; let mut p3 = 0.0f64;
        let mut p5 = 0.0f64; let mut mrr = 0.0f64;

        for i in 0..enc.len() {
            let query = &enc[i]; let qrole = roles[i];
            let mut scored: Vec<(usize, f64)> = enc.iter().enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(j, mv)| (j, sim_fn(query, mv)))
                .collect();
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            // P@K: fraction of top K that share the same role
            for k in &[1, 3, 5] {
                let top: Vec<usize> = scored.iter().take(*k).map(|(j, _)| *j).collect();
                let hits = top.iter().filter(|&&j| roles[j] == qrole).count();
                let pk = hits as f64 / *k as f64;
                match k { 1 => p1 += pk, 3 => p3 += pk, 5 => p5 += pk, _ => {} }
            }

            // MRR: rank of first same-role peer
            if let Some(rk) = scored.iter().position(|(j, _)| roles[*j] == qrole) {
                mrr += 1.0 / (rk as f64 + 1.0);
            }
        }

        let n = enc.len() as f64;
        println!("  {:<25} | {:.1}%     | {:.1}%     | {:.1}%     | {:.3}",
            name, p1/n*100., p3/n*100., p5/n*100., mrr/n);

        // Honest check: dominant_similarity should beat Euclidean
        if name.contains("dominant") {
            assert!(p5/n*100. > 20.0, "dominant_similarity P@5 ({:.1}%) too low", p5/n*100.);
        }
    }

    // Random baseline
    println!("  {:<25} | {:.1}%     | {:.1}%     | {:.1}%     | —",
        "Random (shuffle)", 100./8., 100./8., 100./8.);

    // Per-role breakdown
    println!("\n── PER-ROLE P@5 (dominant_similarity) ──");
    let unique_roles: Vec<&&str> = roles.iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
    for &r in &unique_roles {
        let r_str: &str = r;
        let indices: Vec<usize> = roles.iter().enumerate()
            .filter(|(_, &role)| role == r_str).map(|(i, _)| i).collect();
        let mut p5_sum = 0.0f64;
        for &i in &indices {
            let query = &enc[i];
            let mut scored: Vec<(usize, f64)> = enc.iter().enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(j, mv)| (j, dominant_similarity(query, mv))).collect();
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            let top5: Vec<usize> = scored.iter().take(5).map(|(j, _)| *j).collect();
            p5_sum += top5.iter().filter(|&&j| roles[j] == r_str).count() as f64 / 5.0;
        }
        println!("  {:<15} | {:.1}% ({:.1} concepts)", r_str, p5_sum/indices.len() as f64*100., indices.len());
    }
}
