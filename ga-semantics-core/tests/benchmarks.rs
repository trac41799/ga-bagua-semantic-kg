use ga_semantics_core::prelude::*;
use ga_semantics_core::RelationType;
use std::time::Instant;

#[allow(deprecated)]

fn xorshift(seed: &mut u64) -> f64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    ((*seed as f64) / (u64::MAX as f64)) * 2.0 - 1.0
}
fn random_multivector(seed: &mut u64) -> Multivector {
    Multivector::new([
        xorshift(seed), xorshift(seed), xorshift(seed), xorshift(seed),
        xorshift(seed), xorshift(seed), xorshift(seed), xorshift(seed),
    ])
}

struct BenchResult { name: &'static str, ns_per_op: f64, ops_per_sec: f64 }

fn time<F>(name: &'static str, iterations: u64, mut f: F) -> BenchResult
where F: FnMut() {
    let start = Instant::now();
    for _ in 0..iterations { f(); }
    let elapsed = start.elapsed();
    let ns = elapsed.as_nanos() as f64 / iterations as f64;
    BenchResult { name, ns_per_op: ns, ops_per_sec: 1_000_000_000.0 / ns }
}

fn report(results: &[BenchResult]) {
    println!("\n{:=^70}", " GA-SEMANTICS BENCHMARKS ");
    println!("{:<38} {:>12} {:>16}", "OPERATION", "ns/op", "ops/sec");
    println!("{:-<38} {:-<12} {:-<16}", "", "", "");
    for r in results {
        if r.ns_per_op < 1_000.0 {
            println!("{:<38} {:>8.1} ns {:>13.0}", r.name, r.ns_per_op, r.ops_per_sec);
        } else if r.ns_per_op < 1_000_000.0 {
            println!("{:<38} {:>8.1} us {:>13.0}", r.name, r.ns_per_op / 1000.0, r.ops_per_sec);
        } else {
            println!("{:<38} {:>8.1} ms {:>13.0}", r.name, r.ns_per_op / 1_000_000.0, r.ops_per_sec);
        }
    }
    println!("{:=<70}", "");
}

#[test]
fn run_benchmarks() {
    let iters = 500_000u64;
    let iters_heavy = 50_000u64;
    let mut seed = 0xDEADBEEF_CAFE_BABE;
    let a = random_multivector(&mut seed);
    let b = random_multivector(&mut seed);
    let c = random_multivector(&mut seed);
    let r1 = Rotor::new(0.5, Blade::E12).unwrap();
    let r2 = Rotor::new(0.3, Blade::E23).unwrap();
    let ctx = Context::new(Rotor::new(0.7, Blade::E31).unwrap());
    let rotors_5: Vec<Rotor> = (0..5).map(|i| Rotor::new(i as f64 * 0.3 + 0.1, Blade::E12).unwrap()).collect();
    let candidates_100: Vec<Multivector> = (0..100).map(|_| random_multivector(&mut seed)).collect();
    let triples_50: Vec<(Multivector, Multivector, Multivector)> =
        (0..50).map(|_| (random_multivector(&mut seed), random_multivector(&mut seed), random_multivector(&mut seed))).collect();

    let results = [
        time("geo_product", iters, || { let _ = a.geo_product(&b); }),
        time("inner_product", iters, || { let _ = a.inner_product(&b); }),
        time("wedge_product", iters, || { let _ = a.wedge_product(&b); }),
        time("norm", iters, || { let _ = a.norm(); }),
        time("reverse", iters, || { let _ = a.reverse(); }),
        time("inverse", iters, || { let _ = a.inverse(); }),
        time("grade_projection", iters, || { let _ = a.grade_projection(2); }),
        time("dualize", iters, || { let _ = a.dualize(); }),
        time("dominant_role", iters, || { let _ = a.dominant_role(); }),
        time("rotor_construct", iters, || { let _ = Rotor::new(1.5, Blade::E23); }),
        time("rotor_apply", iters, || { let _ = r1.apply(&a); }),
        time("rotor_compose", iters, || { let _ = r1.compose(&r2); }),
        time("semantic_similarity", iters, || { let _ = semantic_similarity(&a, &b); }),
        time("semantic_difference", iters, || { let _ = semantic_difference(&a, &b); }),
        time("classify_relation", iters, || { let _ = RelationType::from_pair(&a, &b); }),
        time("relation_strength", iters, || { let _ = relation_strength(&a, &b); }),
        time("detect_contradiction", iters, || { let _ = is_contradictory(&a, &b, 0.5); }),
        time("analogy", iters, || { let _ = analogy(&a, &b, &c); }),
        time("compose_chain(5)", iters, || { let _ = compose_chain(&rotors_5); }),
        time("context_apply", iters, || { let _ = ctx.apply(&a); }),
        time("word_to_multivector", iters_heavy, || { let _ = word_to_multivector("causality"); }),
        time("text_to_multivector(10w)", iters_heavy, || { let _ = text_to_multivector("a triggering event that initiates a chain of causal consequences and propagates"); }),
        time("multivector_describe", iters_heavy, || { let _ = multivector_describe(&a); }),
        time("batch_100_similarity", iters_heavy / 100, || { let _: Vec<_> = candidates_100.iter().map(|c| semantic_similarity(&a, c)).collect(); }),
        time("batch_50_analogy", iters_heavy / 50, || { let _: Vec<_> = triples_50.iter().filter_map(|(a,b,c)| analogy(a,b,c)).collect(); }),
    ];

    report(&results);

    let storage = 8usize * 8; // 8 f64 = 64 bytes
    let total_ns: f64 = results.iter().map(|r| r.ns_per_op).sum();
    println!("\n  Storage per concept:     {:>6} bytes ({:.1} bits)", storage, storage as f64 * 8.0);
    println!("  Total time all ops:     {:>8.1} ns", total_ns);
    println!("  All 25 ops + 1KB text:  ~200 LLM tokens vs ~50 algebra ops");

    assert!(results.iter().all(|r| r.ns_per_op < 10_000_000.0), "All ops under 10ms");
}
