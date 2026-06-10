// Analogy benchmark: GA-Bagua analogy() vs baselines on 20 quadruplets

use ga_semantics_core::prelude::*;
use ga_semantics_core::relation_type::RelationType;
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

fn c(v: &[f64; 8]) -> Multivector { llm_encode(v) }

#[test]
fn analogy_benchmark() {
    let ds = load();
    let enc: Vec<Multivector> = ds.concepts.iter().map(|c| {
        let coeffs: [f64; 8] = c.coefficients.as_slice().try_into().unwrap();
        llm_encode(&coeffs)
    }).collect();

    // 20 analogy quadruplets: (A_idx, B_idx, C_idx, expected_D_role)
    let analogies: Vec<(usize, usize, usize, &str)> = vec![
        // Software domain
        (0, 10, 7, "influential"),    // RateLimiter:AuthProvider::FeatureFlag:?
        (1, 3, 11, "transmissive"),   // MessageQueue:APIGateway::CacheLayer:?
        (4, 1, 3, "balancing"),        // LoadBalancer:MQ::APIGateway:?
        (5, 0, 13, "constraining"),    // CircuitBreaker:RateLimiter::DBTransaction:?
        (6, 8, 2, "clarifying"),       // MonitorDash:Logging::DBIndex:?
        // Business domain
        (18, 21, 24, "generative"),    // SalesPipeline:Revenue::InnovationFund:?
        (19, 20, 28, "causal"),        // SupportTicket:Quarterly :: ComplianceAudit:?
        (22, 33, 17, "constraining"),  // EmployeeHandbook:HiringFreeze::MarketingBudget:?
        (25, 27, 31, "balancing"),     // FeedbackLoop:TrendAnalysis::TeamStandup:?
        (30, 22, 14, "influential"),   // IndustryReg:Handbook::DeprecationPolicy:?
        // Biology domain
        (36, 40, 37, "generative"),    // Photosynthesis:Homeostasis::WaterCycle:?
        (39, 41, 34, "causal"),        // Mutation:NaturalSelection::Predator:?
        (38, 42, 43, "influential"),   // Keystone:Succession::Symbiosis:?
        (35, 36, 44, "receptive"),     // Decomposer:Photosynthesis::DNAReplication:?
        (47, 48, 32, "transmissive"),  // Hormone:CellMembrane::SupplyChain:?
        // Cross-domain
        (0, 19, 34, "constraining"),   // RateLimiter→SupportTicket :: Predator→?
        (39, 24, 18, "causal"),        // Mutation→InnovationFund :: Pipeline→?
        (4, 40, 31, "balancing"),      // LoadBalancer→Homeostasis :: Standup→?
        (12, 6, 20, "clarifying"),     // EventStream→Monitoring :: QuarterlyReport→?
        (44, 40, 46, "generative"),    // DNAReplication→Homeostasis :: Enzyme→?
    ];

    println!("\n{:=^70}", " ANALOGY BENCHMARK ");
    println!("  {} quadruplets across 3 domains + cross-domain", analogies.len());
    println!("{:=^70}\n", "");

    let mut correct = 0usize;
    let mut exact_role = 0usize;

    println!("── GA-BAGUA ANALOGY ──");
    println!("  {:<30} x {:<30} :: {:<30} x {:?}", "A", "B", "C", "Expected");
    println!("  {:-<30}-+-{:-<30}-+-{:-<30}-+-{:-<15}", "", "", "", "");

    for &(ai, bi, ci, exp_role) in &analogies {
        let a = &enc[ai]; let b = &enc[bi]; let c = &enc[ci];
        let result = analogy(a, b, c);
        let got = result.as_ref().map(|mv| mv.dominant_role().role_name()).unwrap_or("none");
        let expected = exp_role;
        let ok = got == expected;
        if ok { correct += 1; }
        if result.is_some() && result.unwrap().dominant_role().role_name() == expected { exact_role += 1; }

        println!("  {:<30} x {:<30} :: {:<30} x {:<15} | {}",
            &ds.concepts[ai].name, &ds.concepts[bi].name,
            &ds.concepts[ci].name, format!("{expected}({got})"),
            if ok { "OK" } else { "FAIL" });
    }

    let acc = correct as f64 / analogies.len() as f64 * 100.0;
    println!("\n  Accuracy: {:.1}% ({}/{})", acc, correct, analogies.len());
    println!("  Random baseline (8-way): 12.5%");
    println!("  LLM analogy (prompted):   ~85-95%");

    // ── 3CosAdd baseline ──
    println!("\n── 3CosAdd BASELINE ──");
    let mut cos_add_ok = 0usize;
    for &(ai, bi, ci, exp_role) in &analogies {
        let a = enc[ai].coefficients(); let b = enc[bi].coefficients(); let c = enc[ci].coefficients();
        // 3CosAdd: D = B - A + C, find closest concept's dominant role among all concepts
        let target: [f64; 8] = [b[0]-a[0]+c[0], b[1]-a[1]+c[1], b[2]-a[2]+c[2], b[3]-a[3]+c[3],
            b[4]-a[4]+c[4], b[5]-a[5]+c[5], b[6]-a[6]+c[6], b[7]-a[7]+c[7]];
        let target_mv = llm_encode(&target);
        let mut best_idx = 0usize; let mut best_sim = -2.0f64;
        for (j, mv) in enc.iter().enumerate() {
            let ca = mv.coefficients();
            let dot: f64 = target_mv.coefficients().iter().zip(ca.iter()).map(|(x,y)| x*y).sum();
            if dot > best_sim { best_sim = dot; best_idx = j; }
        }
        let got = enc[best_idx].dominant_role().role_name();
        let ok = got == exp_role;
        if ok { cos_add_ok += 1; }
    }
    println!("  3CosAdd accuracy: {:.1}% ({}/{})", cos_add_ok as f64/analogies.len() as f64*100., cos_add_ok, analogies.len());

    // ── Comparison ──
    println!("\n── COMPARISON ──");
    println!("  {:<25} | {:<10}", "Method", "Accuracy");
    println!("  {:-<25}-+-{:-<10}", "", "");
    println!("  {:<25} | {:.1}%", "GA-Bagua (WuXing cycle)", acc);
    println!("  {:<25} | {:.1}%", "3CosAdd (vector algebra)", cos_add_ok as f64/analogies.len() as f64*100.);
    println!("  {:<25} | {:.1}%", "Random (8-way uniform)", 12.5);

    assert!(acc > 12.5, "Analogy accuracy ({:.1}%) should beat random (12.5%)", acc);
}
