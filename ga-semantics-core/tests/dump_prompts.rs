// Phase 1 of LLM feedback loop: dump all corrective prompts for failing pairs.
// The LLM will then provide re-encodings which go into benchmark_dataset_v3.json.

use ga_semantics_core::prelude::*;
use ga_semantics_core::relation_type::RelationType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JsonConcept { index: usize, name: String, description: String, domain: String, coefficients: Vec<f64> }
#[derive(Debug, Deserialize)]
struct JsonRelation { index: usize, idx_a: usize, idx_b: usize, label: String, confidence: String, cross_domain: bool }
#[derive(Debug, Deserialize)]
struct BenchmarkDataset { concepts: Vec<JsonConcept>, relations: Vec<JsonRelation> }

fn label_to_type(label: &str) -> RelationType {
    use RelationType::*;
    match label { "generative"=>Generative,"receptive"=>Receptive,"causal"=>Causal,"transmissive"=>Transmissive,"constraining"=>Constraining,"influential"=>Influential,"clarifying"=>Clarifying,"balancing"=>Balancing, _=>panic!("{label}") }
}

#[test]
fn dump_corrective_prompts() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("data").join("benchmark_dataset.json");
    let content = std::fs::read_to_string(&path).unwrap();
    let ds: BenchmarkDataset = serde_json::from_str(&content).unwrap();

    let encoded: Vec<Multivector> = ds.concepts.iter().map(|c| {
        let coeffs: [f64; 8] = c.coefficients.as_slice().try_into().unwrap();
        llm_encode(&coeffs)
    }).collect();

    let mut prompts: Vec<String> = vec![];
    let mut correct_count = 0usize;
    let mut fail_count = 0usize;

    for r in &ds.relations {
        let expected = label_to_type(&r.label);
        let a = &encoded[r.idx_a];
        let b = &encoded[r.idx_b];

        let prompt = RelationType::corrective_prompt(
            &ds.concepts[r.idx_a].name, &ds.concepts[r.idx_b].name,
            a, b, expected,
        );

        match prompt {
            None => { correct_count += 1; }
            Some(p) => {
                fail_count += 1;
                prompts.push(format!("// Pair {}: {} -> {}\n{}", r.index,
                    &ds.concepts[r.idx_a].name, &ds.concepts[r.idx_b].name, p));
            }
        }
    }

    println!("\nCorrect: {correct_count}/{} | Failing: {fail_count}/{}\n",
        ds.relations.len(), ds.relations.len());
    for p in &prompts {
        println!("{p}\n");
    }

    // Print concept reference for re-encoding
    println!("\n// ── CONCEPT INDEX REFERENCE ──");
    for c in &ds.concepts {
        let mv = &encoded[c.index];
        println!("// {}: {} [{}] — dominant: {:?}({:?}) coords: {:?}",
            c.index, c.name, c.domain,
            mv.dominant_role().role_name(),
            mv.dominant_role().wuxing_phase().name(),
            &c.coefficients.iter().map(|x| format!("{:.3}", x)).collect::<Vec<_>>());
    }
}
