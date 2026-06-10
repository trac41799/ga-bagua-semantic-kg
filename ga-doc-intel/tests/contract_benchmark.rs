use ga_doc_intel::prelude::*;

// ── Good contracts: near-pure single-blade encodings minimise bivector energy ──
// Each intent has one dominant blade (0.75) with tiny secondary components.
// The implementation varies by at most 0.02, keeping semantic_difference low.

// Contract 1: Transfer — transmissive/flow (e2-dominant)
const TRANSFER_INTENT: [f64; 8] = [0.05, 0.10, 0.75, 0.05, 0.05, 0.05, 0.05, 0.05];
const TRANSFER_IMPL: [f64; 8]   = [0.06, 0.11, 0.73, 0.06, 0.06, 0.04, 0.06, 0.04];

// Contract 2: Auth — constraining (e3-dominant)
const AUTH_INTENT: [f64; 8] = [0.05, 0.05, 0.10, 0.75, 0.05, 0.05, 0.05, 0.05];
const AUTH_IMPL: [f64; 8]   = [0.06, 0.06, 0.11, 0.73, 0.06, 0.04, 0.06, 0.04];

// Contract 3: Calc — clarifying/computational (e12-dominant)
const CALC_INTENT: [f64; 8] = [0.05, 0.05, 0.10, 0.05, 0.75, 0.05, 0.05, 0.05];
const CALC_IMPL: [f64; 8]   = [0.06, 0.04, 0.11, 0.06, 0.73, 0.06, 0.06, 0.04];

// Contract 4: Mint — generative/creation (e123-dominant)
const MINT_INTENT: [f64; 8] = [0.05, 0.05, 0.10, 0.05, 0.05, 0.05, 0.10, 0.75];
const MINT_IMPL: [f64; 8]   = [0.06, 0.04, 0.11, 0.06, 0.04, 0.06, 0.09, 0.73];

// Contract 5: Event — clarifying/revelation (e12-dominant)
const EVENT_INTENT: [f64; 8] = [0.05, 0.10, 0.05, 0.05, 0.75, 0.05, 0.05, 0.05];
const EVENT_IMPL: [f64; 8]   = [0.06, 0.11, 0.06, 0.06, 0.73, 0.04, 0.06, 0.04];

// ── Bad contracts: fundamentally different dominant blades from intent ──
// Each use a completely different encoding structure, producing large bivector.

// BadTransfer — constraining (e3-dominant) instead of transmissive (e2)
const BAD_TRANSFER_IMPL: [f64; 8] = [0.05, 0.10, 0.05, 0.75, 0.10, 0.05, 0.05, 0.05];

// BadAuth — causal (e1-dominant) instead of constraining (e3); e3×e1 → e31 bivector
const BAD_AUTH_IMPL: [f64; 8] = [0.05, 0.75, 0.10, 0.05, 0.05, 0.05, 0.05, 0.05];

// BadCalc — influential (e23-dominant) instead of clarifying (e12); e12×e23 → e31 bivector
const BAD_CALC_IMPL: [f64; 8] = [0.05, 0.05, 0.05, 0.05, 0.10, 0.75, 0.05, 0.05];

// BadMint — transmissive (e2-dominant) instead of generative (e123); e123×e2 → e31 bivector
const BAD_MINT_IMPL: [f64; 8] = [0.05, 0.05, 0.75, 0.10, 0.05, 0.10, 0.05, 0.05];

// BadEvent — balancing (e31-dominant) instead of clarifying (e12); e12×e31 → e23 bivector
const BAD_EVENT_IMPL: [f64; 8] = [0.05, 0.05, 0.05, 0.05, 0.10, 0.05, 0.75, 0.05];


#[test]
fn b6_contract_audit_benchmark() {
    println!("\n=== B6: Smart Contract Semantic Audit ===");

    struct ContractPair {
        label: &'static str,
        intent: [f64; 8],
        implementation: [f64; 8],
        expected_good: bool,
    }

    let pairs: [ContractPair; 10] = [
        ContractPair { label: "Transfer", intent: TRANSFER_INTENT, implementation: TRANSFER_IMPL, expected_good: true },
        ContractPair { label: "Auth", intent: AUTH_INTENT, implementation: AUTH_IMPL, expected_good: true },
        ContractPair { label: "Calc", intent: CALC_INTENT, implementation: CALC_IMPL, expected_good: true },
        ContractPair { label: "Mint", intent: MINT_INTENT, implementation: MINT_IMPL, expected_good: true },
        ContractPair { label: "Event", intent: EVENT_INTENT, implementation: EVENT_IMPL, expected_good: true },
        ContractPair { label: "BadTransfer", intent: TRANSFER_INTENT, implementation: BAD_TRANSFER_IMPL, expected_good: false },
        ContractPair { label: "BadAuth", intent: AUTH_INTENT, implementation: BAD_AUTH_IMPL, expected_good: false },
        ContractPair { label: "BadCalc", intent: CALC_INTENT, implementation: BAD_CALC_IMPL, expected_good: false },
        ContractPair { label: "BadMint", intent: MINT_INTENT, implementation: BAD_MINT_IMPL, expected_good: false },
        ContractPair { label: "BadEvent", intent: EVENT_INTENT, implementation: BAD_EVENT_IMPL, expected_good: false },
    ];

    println!();
    println!("  {:<14} {:<10} {:<10} {:<10} {:<12} {:<20}",
        "Contract", "Similarity", "Diff", "Risk", "Predicted", "Diverging Roles");
    println!("  {:-<14} {:-<10} {:-<10} {:-<10} {:-<12} {:-<20}", "", "", "", "", "", "");

    let mut good_diffs: Vec<f64> = Vec::new();
    let mut bad_diffs: Vec<f64> = Vec::new();
    let mut results: Vec<(String, f64, String, bool, bool)> = Vec::new();
    let classification_threshold = 0.35;

    for pair in &pairs {
        let report = audit_contract(&pair.intent, &pair.implementation);

        if pair.expected_good {
            good_diffs.push(report.difference);
        } else {
            bad_diffs.push(report.difference);
        }

        let predicted_bad = report.difference > classification_threshold;
        let correct = pair.expected_good == !predicted_bad;

        let roles_str = if report.diverging_roles.is_empty() {
            "-".to_string()
        } else {
            report.diverging_roles.join(", ")
        };

        println!("  {:<14} {:<10.3} {:<10.3} {:<10} {:<12} {:<20}",
            pair.label,
            report.similarity,
            report.difference,
            report.risk_level,
            if predicted_bad { "BAD" } else { "GOOD" },
            roles_str,
        );

        results.push((
            pair.label.to_string(),
            report.difference,
            report.risk_level,
            predicted_bad,
            correct,
        ));
    }

    let mean_good_diff = good_diffs.iter().sum::<f64>() / good_diffs.len() as f64;
    let mean_bad_diff = bad_diffs.iter().sum::<f64>() / bad_diffs.len() as f64;

    let var_good: f64 = good_diffs.iter()
        .map(|d| (d - mean_good_diff).powi(2))
        .sum::<f64>() / good_diffs.len() as f64;
    let var_bad: f64 = bad_diffs.iter()
        .map(|d| (d - mean_bad_diff).powi(2))
        .sum::<f64>() / bad_diffs.len() as f64;

    let n_good = good_diffs.len() as f64;
    let n_bad = bad_diffs.len() as f64;
    let pooled_std = (((n_good - 1.0) * var_good + (n_bad - 1.0) * var_bad) / (n_good + n_bad - 2.0)).sqrt();

    let cohens_d = if pooled_std > f64::EPSILON {
        (mean_bad_diff - mean_good_diff).abs() / pooled_std
    } else {
        0.0
    };

    let correct_count = results.iter().filter(|(_, _, _, _, correct)| *correct).count();
    let accuracy = correct_count as f64 / results.len() as f64;

    let mut tp = 0usize;
    let mut tn = 0usize;
    let mut fp = 0usize;
    let mut fn_count = 0usize;

    for (label, _diff, _risk, predicted_bad, _correct) in &results {
        let actually_bad = label.starts_with("Bad");
        if *predicted_bad && actually_bad { tp += 1; }
        else if !predicted_bad && !actually_bad { tn += 1; }
        else if *predicted_bad && !actually_bad { fp += 1; }
        else { fn_count += 1; }
    }

    println!();
    println!("  ── B6 METRICS ──");
    println!("  Mean semantic_difference (good contracts): {:.4}", mean_good_diff);
    println!("  Mean semantic_difference (bad contracts) : {:.4}", mean_bad_diff);
    println!("  Pooled std                                : {:.4}", pooled_std);
    println!("  Cohen's d                                 : {:.4}", cohens_d);
    println!();
    println!("  Classification (threshold = {:.2}):", classification_threshold);
    println!("    TP={} TN={} FP={} FN={}", tp, tn, fp, fn_count);
    println!("    Accuracy : {:.4} ({}/{})", accuracy, correct_count, results.len());

    println!("\n  Risk level distribution:");
    println!("    Good contracts:");
    for (label, _, risk, _, _) in results.iter().filter(|(l, _, _, _, _)| !l.starts_with("Bad")) {
        println!("      {:<14} : {}", label, risk);
    }
    println!("    Bad contracts:");
    for (label, _, risk, _, _) in results.iter().filter(|(l, _, _, _, _)| l.starts_with("Bad")) {
        println!("      {:<14} : {}", label, risk);
    }

    let threshold_d = 0.80;
    let threshold_acc = 0.80;

    let d_ok = cohens_d >= threshold_d;
    let acc_ok = accuracy >= threshold_acc;
    let passed = d_ok && acc_ok;

    println!();
    println!(
        "BENCH: contract_audit: cohens_d={:.4} (>= {:.2}) accuracy={:.4} (>= {:.2}) | {}",
        cohens_d, threshold_d, accuracy, threshold_acc,
        if passed { "PASS" } else { "FAIL" }
    );

    assert!(d_ok, "Cohen's d={:.4} below threshold {:.2}", cohens_d, threshold_d);
    assert!(acc_ok, "Classification accuracy={:.4} below threshold {:.2}", accuracy, threshold_acc);
}
