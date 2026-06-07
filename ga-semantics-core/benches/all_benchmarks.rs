use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ga_semantics_core::prelude::*;
use ga_semantics_core::RelationType;

fn random_multivector() -> Multivector {
    Multivector::new([
        rand_val(), rand_val(), rand_val(), rand_val(),
        rand_val(), rand_val(), rand_val(), rand_val(),
    ])
}
fn rand_val() -> f64 { (rand::random::<f64>() * 2.0 - 1.0) * 5.0 }

fn bench_geometric_product(c: &mut Criterion) {
    let a = random_multivector();
    let b = random_multivector();
    c.bench_function("geo_product", |bench| {
        bench.iter(|| { black_box(&a).geo_product(black_box(&b)) })
    });
}

fn bench_inner_product(c: &mut Criterion) {
    let a = random_multivector();
    let b = random_multivector();
    c.bench_function("inner_product", |bench| {
        bench.iter(|| { black_box(&a).inner_product(black_box(&b)) })
    });
}

fn bench_wedge_product(c: &mut Criterion) {
    let a = random_multivector();
    let b = random_multivector();
    c.bench_function("wedge_product", |bench| {
        bench.iter(|| { black_box(&a).wedge_product(black_box(&b)) })
    });
}

fn bench_norm(c: &mut Criterion) {
    let a = random_multivector();
    c.bench_function("norm", |bench| {
        bench.iter(|| { black_box(&a).norm() })
    });
}

fn bench_inverse(c: &mut Criterion) {
    let a = random_multivector();
    c.bench_function("inverse", |bench| {
        bench.iter(|| { black_box(&a).inverse() })
    });
}

fn bench_dualize(c: &mut Criterion) {
    let a = random_multivector();
    c.bench_function("dualize", |bench| {
        bench.iter(|| { black_box(&a).dualize() })
    });
}

fn bench_grade_projection(c: &mut Criterion) {
    let a = random_multivector();
    c.bench_function("grade_projection", |bench| {
        bench.iter(|| { black_box(&a).grade_projection(2) })
    });
}

fn bench_rotor_construct(c: &mut Criterion) {
    c.bench_function("rotor_construct", |bench| {
        bench.iter(|| { Rotor::new(black_box(1.5), black_box(Blade::E12)) })
    });
}

fn bench_rotor_apply(c: &mut Criterion) {
    let r = Rotor::new(0.8, Blade::E12).unwrap();
    let v = random_multivector();
    c.bench_function("rotor_apply", |bench| {
        bench.iter(|| { black_box(&r).apply(black_box(&v)) })
    });
}

fn bench_rotor_compose(c: &mut Criterion) {
    let r1 = Rotor::new(0.5, Blade::E12).unwrap();
    let r2 = Rotor::new(0.3, Blade::E23).unwrap();
    c.bench_function("rotor_compose", |bench| {
        bench.iter(|| { black_box(&r1).compose(black_box(&r2)) })
    });
}

fn bench_semantic_similarity(c: &mut Criterion) {
    let a = random_multivector();
    let b = random_multivector();
    c.bench_function("semantic_similarity", |bench| {
        bench.iter(|| semantic_similarity(black_box(&a), black_box(&b)))
    });
}

fn bench_semantic_difference(c: &mut Criterion) {
    let a = random_multivector();
    let b = random_multivector();
    c.bench_function("semantic_difference", |bench| {
        bench.iter(|| semantic_difference(black_box(&a), black_box(&b)))
    });
}

fn bench_analogy(c: &mut Criterion) {
    let a = random_multivector();
    let b = random_multivector();
    let c = random_multivector();
    c.bench_function("analogy", |bench| {
        bench.iter(|| analogy(black_box(&a), black_box(&b), black_box(&c)))
    });
}

fn bench_classify_relation(c: &mut Criterion) {
    let a = random_multivector();
    let b = random_multivector();
    c.bench_function("classify_relation", |bench| {
        bench.iter(|| RelationType::from_pair(black_box(&a), black_box(&b)))
    });
}

fn bench_contradiction(c: &mut Criterion) {
    let a = random_multivector();
    let b = random_multivector();
    c.bench_function("detect_contradiction", |bench| {
        bench.iter(|| is_contradictory(black_box(&a), black_box(&b), 0.5))
    });
}

fn bench_text_to_mv(c: &mut Criterion) {
    c.bench_function("text_to_multivector_10words", |bench| {
        bench.iter(|| text_to_multivector(black_box("a triggering event that initiates a chain of causal consequences and propagates effects")))
    });
}

fn bench_word_to_mv(c: &mut Criterion) {
    c.bench_function("word_to_multivector", |bench| {
        bench.iter(|| word_to_multivector(black_box("causality")))
    });
}

fn bench_mv_describe(c: &mut Criterion) {
    let mv = random_multivector();
    c.bench_function("multivector_describe", |bench| {
        bench.iter(|| multivector_describe(black_box(&mv)))
    });
}

fn bench_relation_strength(c: &mut Criterion) {
    let a = random_multivector();
    let b = random_multivector();
    c.bench_function("relation_strength", |bench| {
        bench.iter(|| relation_strength(black_box(&a), black_box(&b)))
    });
}

fn bench_compose_chain_5(c: &mut Criterion) {
    let rotors: Vec<Rotor> = (0..5)
        .map(|i| Rotor::new((i as f64) * 0.3 + 0.1, Blade::E12).unwrap())
        .collect();
    c.bench_function("compose_chain_5_hops", |bench| {
        bench.iter(|| compose_chain(black_box(&rotors)))
    });
}

fn bench_dominant_role(c: &mut Criterion) {
    let mv = random_multivector();
    c.bench_function("dominant_role", |bench| {
        bench.iter(|| black_box(&mv).dominant_role())
    });
}

fn bench_context_apply(c: &mut Criterion) {
    let r = Rotor::new(0.7, Blade::E23).unwrap();
    let ctx = Context::new(r);
    let entity = random_multivector();
    c.bench_function("context_apply", |bench| {
        bench.iter(|| black_box(&ctx).apply(black_box(&entity)))
    });
}

fn bench_batch_100_similarity(c: &mut Criterion) {
    let base = random_multivector();
    let candidates: Vec<Multivector> = (0..100).map(|_| random_multivector()).collect();
    c.bench_function("batch_100_similarity", |bench| {
        bench.iter(|| {
            candidates.iter().map(|c| semantic_similarity(&base, c)).collect::<Vec<_>>()
        })
    });
}

fn bench_batch_50_analogy(c: &mut Criterion) {
    let triples: Vec<(Multivector, Multivector, Multivector)> =
        (0..50).map(|_| (random_multivector(), random_multivector(), random_multivector())).collect();
    c.bench_function("batch_50_analogy", |bench| {
        bench.iter(|| {
            triples.iter().filter_map(|(a, b, c)| analogy(a, b, c)).collect::<Vec<_>>()
        })
    });
}

criterion_group!(
    benches,
    bench_geometric_product,
    bench_inner_product,
    bench_wedge_product,
    bench_norm,
    bench_inverse,
    bench_dualize,
    bench_grade_projection,
    bench_rotor_construct,
    bench_rotor_apply,
    bench_rotor_compose,
    bench_semantic_similarity,
    bench_semantic_difference,
    bench_analogy,
    bench_classify_relation,
    bench_contradiction,
    bench_relation_strength,
    bench_dominant_role,
    bench_compose_chain_5,
    bench_context_apply,
    bench_text_to_mv,
    bench_word_to_mv,
    bench_mv_describe,
    bench_batch_100_similarity,
    bench_batch_50_analogy,
);
criterion_main!(benches);
