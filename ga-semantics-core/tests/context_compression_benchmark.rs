use std::time::Instant;

fn estimate_tokens(text: &str) -> usize {
    (text.len() as f64 / 4.0).ceil() as usize
}

fn context_compression_analysis() {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║        CONTEXT COMPRESSION EFFICIENCY BENCHMARK                  ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Measures token usage across approaches for a 50K-token doc     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // ── PARAMETERS ──
    let n_concepts = 100usize;              // Distinct concepts in a 50K-token document
    let concept_desc_tokens = 80;           // Average tokens to describe one concept
    let n_queries = vec![1, 5, 10, 20, 50, 100]; // Number of semantic queries made by the agent
    let bagua_encode_tokens = 200;          // SKILL.md prompt + encode instruction
    let bagua_interpret_tokens = 30;        // Tokens for LLM to interpret algebra result
    let llm_direct_query_tokens = 300;      // "How does X relate to Y?" full-text query
    let summarize_compress_tokens = 2000;   // Summarize 50K doc to ~2K tokens
    let summarize_ratio = 0.04;             // 2K/50K = 4% compression

    // ── APPROACHES ──
    let approaches = [
        ("LLM + Full Context (no tool)", "Read entire 50K doc into context each query"),
        ("LLM + GA-Bagua (encoding)", "Encode 100 concepts once (~200 tok/ea), then query algebraically"),
        ("LLM + Naive Summarization", "Summarize 50K doc to 2K tokens, query against summary"),
        ("LLM + GA-Bagua + Summary", "Summarize doc then encode concepts from summary"),
        ("LLM + Vector DB + RAG", "Chunk doc into embeddable segments, retrieve + query"),
    ];

    println!("  Document: 50,000 tokens, ~100 distinct concepts");
    println!("  GA-Bagua encode cost: ~{} tokens per concept (one-time)", bagua_encode_tokens);
    println!("  LLM direct query cost: ~{} tokens per query", llm_direct_query_tokens);
    println!();
    println!("  Approach                                         │ Encode │ Query │ 1Q │ 5Q │ 10Q │ 20Q │ 50Q │ 100Q");
    println!("  ────────────────────────────────────────────────┼────────┼───────┼─────┼─────┼──────┼──────┼──────┼───────");

    for (name, _desc) in &approaches {
        let (encode_cost, query_cost) = match *name {
            "LLM + Full Context (no tool)" => (0usize, 50_000 + 500),
            "LLM + GA-Bagua (encoding)" => (n_concepts * bagua_encode_tokens, bagua_interpret_tokens),
            "LLM + Naive Summarization" => (summarize_compress_tokens + 500, summarize_compress_tokens + 300),
            "LLM + GA-Bagua + Summary" => (summarize_compress_tokens + n_concepts * bagua_encode_tokens, bagua_interpret_tokens),
            "LLM + Vector DB + RAG" => (50_000, 500 + 500), // Embed + retrieve + LLM query
            _ => (0, 0),
        };

        print!("  {:50} │ {:>5}K │ {:>4}K │", name, encode_cost / 1000, query_cost / 1000);
        for &q in &n_queries {
            let total = encode_cost + q * query_cost;
            let total_k = total as f64 / 1000.0;
            print!(" {:>4.0}K", total_k);
        }
        println!();
    }

    // ── BREAK-EVEN ANALYSIS ──
    println!();
    println!("  ── BREAK-EVEN ANALYSIS ──");
    println!("  How many queries before GA-Bagua saves more tokens than Full Context?");

    let full_query_cost = 50_000 + 500;      // Full doc + LLM prompt per query
    let bagua_encode = n_concepts * bagua_encode_tokens;
    let bagua_query = bagua_interpret_tokens;

    for q in &[1, 2, 3, 5, 10, 20] {
        let full_total = *q * full_query_cost;
        let bagua_total = bagua_encode + *q * bagua_query;
        let saved = full_total as i64 - bagua_total as i64;
        let ratio = bagua_total as f64 / full_total as f64;

        println!("    Queries={:>3}: Full={:>7}K  Bagua={:>7}K  Saved={:>7}K  Ratio={:.1}%  {}",
            q,
            full_total / 1000, bagua_total / 1000, saved / 1000,
            ratio * 100.0,
            if saved > 0 { "Cheaper" } else { "More expensive" }
        );
    }

    // ── STORAGE DENSITY ──
    println!();
    println!("  ── STORAGE DENSITY COMPARISON ──");
    println!("  Method                      │ Bytes/Concept │ 1K Concepts │ 10K │ 100K │ 1M");
    println!("  ────────────────────────────┼───────────────┼─────────────┼─────┼──────┼──────");

    let storage_comparisons = [
        ("GA-Bagua (8 x f64)", 64usize),
        ("GA-Bagua (8 x f16 hypothetical)", 16usize),
        ("BERT base (768 x f32)", 768 * 4),
        ("OpenAI ada-002 (1536 x f32)", 1536 * 4),
        ("TransE (200 x f32)", 200 * 4),
        ("Raw text description (~80 tok)", 80 * 4),
        ("Summarized text (~20 tok)", 20 * 4),
    ];

    for (name, bytes_per) in &storage_comparisons {
        let bp = *bytes_per;
        let k1 = bp * 1_000;
        let k10 = bp * 10_000;
        let k100 = bp * 100_000;
        let m1 = bp as f64 * 1_000_000.0;

        let fmt = |b: usize| -> String {
            if b < 1024 { format!("{} B", b) }
            else if b < 1024 * 1024 { format!("{:.0} KB", b as f64 / 1024.0) }
            else if b < 1024 * 1024 * 1024 { format!("{:.1} MB", b as f64 / 1048576.0) }
            else { format!("{:.1} GB", b as f64 / 1073741824.0) }
        };

        println!("  {:>28} | {:>10} B | {:>10} | {:>9} | {:>8} | {:>6}",
            name, bp,
            fmt(k1), fmt(k10), fmt(k100),
            if m1 < 1073741824.0 { fmt(m1 as usize) } else { format!("{:.1} GB", m1 / 1073741824.0) }
        );
    }

    // ── LATENCY COMPARISON ──
    println!();
    println!("  ── QUERY LATENCY COMPARISON ──");
    println!("  Method                     │ Single Query │ 100 Queries │ 10K Queries");
    println!("  ───────────────────────────┼──────────────┼─────────────┼────────────");

    let latency_comparisons = [
        ("GA-Bagua algebraic (ns-us)", 10.0, 1_000.0, 100_000.0),
        ("BERT/ada-002 cosine (us)", 10.0, 1_000.0, 100_000.0),
        ("KGE model scoring (us)", 50.0, 5_000.0, 500_000.0),
        ("LLM query (~500 tok, ms-ms)", 500.0 * 1000.0, 500.0 * 100_000.0, 500.0 * 10_000_000.0),
    ];

    for (name, single, h100, k10) in &latency_comparisons {
        let fmt_us = |us: f64| -> String {
            if us < 1_000.0 { format!("{:.0} ns", us * 1_000.0) }
            else if us < 1_000_000.0 { format!("{:.1} us", us) }
            else { format!("{:.1} ms", us / 1_000.0) }
        };

        println!("  {:>27} │ {:>12} │ {:>11} │ {:>9}",
            name, fmt_us(*single), fmt_us(*h100), fmt_us(*k10));
    }

    // ── REAL-WORLD SCENARIO ──
    println!();
    println!("  ── REAL-WORLD SCENARIO: CODEBASE EXPLORATION ──");
    println!("  Task: Agent analyzes a 200-module codebase, making 200 relationship queries.");
    println!();
    println!("  Pipeline                           │ Tokens Used │ Cost ($0.01/1K) │ Latency");
    println!("  ───────────────────────────────────┼─────────────┼────────────────┼─────────");

    let scenarios = [
        ("AI reads all code each query", 200 * 50500, 200.0 * 50500.0 / 1000.0 * 0.01, "200 × 3s = 600s"),
        ("AI + GA-Bagua (200 encodes)", 200 * 200 + 200 * 30, (200.0 * 200.0 + 200.0 * 30.0) / 1000.0 * 0.01, "200 × 0.5us + 200 × 0.5s = 100s"),
        ("AI + summarization", 2000 + 200 * 2500, (2000.0 + 200.0 * 2500.0) / 1000.0 * 0.01, "1 × 2s + 200 × 1s = 202s"),
        ("AI + GA-Bagua + summary", 2000 + 200 * 200 + 200 * 30, (2000.0 + 200.0 * 230.0) / 1000.0 * 0.01, "1 × 2s + 200 × 0.5us + 200 × 0.5s = 102s"),
    ];

    for (name, tokens, cost, latency) in &scenarios {
        println!("  {:>35} │ {:>9}K │ ${:>13.2} │ {}", name, tokens / 1000, cost, latency);
    }

    // ── LONG-CONTEXT WINDOW UTILIZATION ──
    println!();
    println!("  ── CONTEXT WINDOW UTILIZATION ──");
    println!("  Window size: 128K tokens (GPT-4o / Claude 3)");
    println!();
    println!("  What fits in the context window?");
    println!();
    println!("  Approach                           │ Context Used │ Remaining for Reasoning");
    println!("  ───────────────────────────────────┼──────────────┼────────────────────────");

    let window = 128_000usize;
    let full_ctx = 50_000 + 500;
    let bagua_ctx = n_concepts * 200 / 5 + 30; // Encode once, reuse; only active encodings in context
    let summary_ctx = 2_500;

    println!("  Full document in context           │ {:>9}K     │ ~{:>5}K tokens", full_ctx / 1000, (window - full_ctx) / 1000);
    println!("  GA-Bagua (encodings + query)       │ {:>9}K     │ ~{:>5}K tokens", bagua_ctx / 1000, (window - bagua_ctx) / 1000);
    println!("  Summarized context                 │ {:>9}K     │ ~{:>5}K tokens", summary_ctx / 1000, (window - summary_ctx) / 1000);

    // ── SPEED BENCHMARK (ACTUAL) ──
    println!();
    println!("  ── ACTUAL ALGEBRA SPEED (this machine) ──");

    use ga_semantics_core::prelude::*;

    let mut seed = 0xCAFEFEED_BEEFBABE;
    fn rand_mv(seed: &mut u64) -> Multivector {
        *seed ^= *seed << 13; *seed ^= *seed >> 17; *seed ^= *seed << 5;
        let mut raw = [0.0; 8];
        for i in 0..8 {
            *seed ^= *seed << 13; *seed ^= *seed >> 17; *seed ^= *seed << 5;
            raw[i] = ((*seed as f64) / (u64::MAX as f64)) * 2.0 - 1.0;
        }
        llm_encode(&raw)
    }

    // Single similarity
    let a = rand_mv(&mut seed);
    let b = rand_mv(&mut seed);
    let start = Instant::now();
    for _ in 0..100_000 {
        let _ = semantic_similarity(&a, &b);
    }
    let elapsed = start.elapsed();
    let ns_per = elapsed.as_nanos() as f64 / 100_000.0;

    // Classification
    let start = Instant::now();
    for _ in 0..100_000 {
        let _ = RelationType::from_pair(&a, &b);
    }
    let cls_elapsed = start.elapsed();
    let cls_ns = cls_elapsed.as_nanos() as f64 / 100_000.0;

    // Batch 100
    let candidates: Vec<Multivector> = (0..100).map(|_| rand_mv(&mut seed)).collect();
    let start = Instant::now();
    for _ in 0..10_000 {
        let _: Vec<_> = candidates.iter().map(|c| semantic_similarity(&a, c)).collect();
    }
    let batch_elapsed = start.elapsed();
    let batch_us = batch_elapsed.as_micros() as f64 / 10_000.0;

    println!("  Single similarity:   {:.1} ns/op  → {:.0}M queries/sec on 1 CPU core",
        ns_per, 1_000.0 / ns_per);
    println!("  Relation classify:   {:.1} ns/op  → {:.0}M classifications/sec",
        cls_ns, 1_000.0 / cls_ns);
    println!("  Batch 100 search:    {:.1} us     → {:.0} full-scans/sec",
        batch_us, 1_000_000.0 / batch_us);
    println!("  = {:.0}x faster than LLM query (~500ms/query)", 500_000.0 / batch_us);
}

#[test]
fn context_compression_benchmark() {
    context_compression_analysis();
}
