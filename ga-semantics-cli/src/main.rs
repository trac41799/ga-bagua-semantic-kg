use clap::{Parser, Subcommand, Args, ValueEnum};
use ga_semantics_core::prelude::*;
use ga_semantics_core::advanced::{Hexagram, Trigram, WuXing, trigram_transform_details, wuxing_generating_chain, wuxing_controlling_chain};
use ga_semantics_core::RelationType;
use std::str::FromStr;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "ga-semantics",
    version = VERSION,
    about = "GA-Bagua Semantic KG — CLI for encoding, classification, analogy, and knowledge graph management")]
struct Cli {
    #[arg(short = 'j', long, global = true, help = "Output as JSON (machine-readable)")]
    json: bool,
    #[arg(long, global = true, help = "Output as CSV")]
    csv: bool,
    #[arg(short = 'q', long, global = true, help = "Quiet mode (values only)")]
    quiet: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Encode a concept from 8 coefficient weights
    Encode(EncodeArgs),
    /// Describe a multivector in human-readable terms
    Describe(DescribeArgs),
    /// Show role strength breakdown of a multivector
    Roles(RolesArgs),
    /// Compute semantic similarity [-1, 1]
    Sim(SimArgs),
    /// Compute semantic difference [0, 1]
    Diff(DiffArgs),
    /// Classify the relationship between two concepts
    Classify(ClassifyArgs),
    /// Solve analogy: A is to B as C is to ?
    Analogy(AnalogyArgs),
    /// Detect contradiction between two concepts
    Contradict(ContradictArgs),
    /// Compose two rotors
    Compose(ComposeArgs),
    /// Look up relation type metadata
    #[command(name = "rel-info")]
    RelInfo(RelInfoArgs),
    /// Inspect a trigram (I-Ching Bagua)
    Trigram(TrigramArgs),
    /// Classify a concept pair as a hexagram (64 states)
    #[command(name = "hexagram")]
    Hexagram(HexagramArgs),
    /// Explore WuXing phase cycles
    Wuxing(WuxingArgs),
    /// Manage the knowledge graph store
    #[command(subcommand)]
    Store(StoreCommand),
    /// Run benchmarks
    #[command(subcommand)]
    Bench(BenchCommand),
}

// ── Encoding ────────────────────────────────────────────────────────

#[derive(Args)]
struct EncodeArgs {
    /// 8 coefficients: [receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative]
    coefficients: Vec<f64>,
    #[arg(short, long, help = "Concept name")]
    name: Option<String>,
}

#[derive(Args)]
struct DescribeArgs {
    /// 8 coefficients as JSON array or comma-separated
    multivector: String,
}

#[derive(Args)]
struct RolesArgs {
    /// 8 coefficients as JSON array or comma-separated
    multivector: String,
    #[arg(short = 'n', long, default_value = "0", help = "Show only top N roles (0 = all)")]
    top: usize,
}

// ── Semantics ───────────────────────────────────────────────────────

#[derive(Args)]
struct SimArgs { a: String, b: String }
#[derive(Args)]
struct DiffArgs { a: String, b: String }
#[derive(Args)]
struct ClassifyArgs { a: String, b: String }
#[derive(Args)]
struct AnalogyArgs { a: String, b: String, c: String }
#[derive(Args)]
struct ContradictArgs { a: String, b: String, #[arg(default_value = "0.5")] threshold: f64 }
#[derive(Args)]
struct ComposeArgs { r1: String, r2: String }
#[derive(Args)]
struct RelInfoArgs { role: String }

// ── Bagua ───────────────────────────────────────────────────────────

#[derive(Args)]
struct TrigramArgs {
    /// Trigram: kun, gen, kan, xun, zhen, li, dui, qian
    trigram: String,
    #[arg(short, long, help = "Show line transformations")]
    transforms: bool,
}

#[derive(Args)]
struct HexagramArgs { a: String, b: String }

#[derive(Args)]
struct WuxingArgs {
    /// Phase: wood, fire, earth, metal, water
    phase: String,
    #[arg(short, long, value_enum, default_value_t)]
    cycle: CycleKind,
}

#[derive(Clone, ValueEnum, Default)]
enum CycleKind { #[default] #[value(name = "generating")] Generating, #[value(name = "controlling")] Controlling }

impl std::fmt::Display for CycleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Self::Generating => write!(f, "generating"), Self::Controlling => write!(f, "controlling") }
    }
}

// ── Store ───────────────────────────────────────────────────────────

#[derive(Subcommand)]
enum StoreCommand {
    /// Add a concept from raw coefficients
    #[command(name = "add")]
    Add {
        name: String,
        #[arg(short, long)]
        text: Option<String>,
        coefficients: Vec<f64>,
    },
    /// Query similar concepts
    #[command(name = "query")]
    Query {
        query: String,
        #[arg(short = 'n', long, default_value = "10")]
        top_k: usize,
    },
    /// Add a relation between concepts
    #[command(name = "relate")]
    Relate {
        from_id: i64,
        to_id: i64,
    },
    /// List all stored concepts
    #[command(name = "list")]
    List,
    /// Export graph as JSON
    #[command(name = "export")]
    Export,
    /// Show concept details
    #[command(name = "get")]
    Get {
        id: i64,
    },
}

// ── Benchmarks ──────────────────────────────────────────────────────

#[derive(Subcommand)]
enum BenchCommand {
    /// Run timing benchmarks (500K iterations per op)
    Timing,
    /// Run semantic accuracy benchmark
    Semantic,
}

// ── Parse helpers ───────────────────────────────────────────────────

fn parse_mv(s: &str) -> Multivector {
    let s = s.trim();
    if s.starts_with('[') {
        if let Ok(arr) = serde_json::from_str::<Vec<f64>>(s) {
            let mut c = [0.0; 8];
            for (i, v) in arr.iter().enumerate().take(8) { c[i] = *v; }
            return llm_encode(&c);
        }
    }
    let parts: Vec<f64> = s.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    if parts.len() == 8 {
        let mut c = [0.0; 8];
        c.copy_from_slice(&parts);
        return llm_encode(&c);
    }
    ga_semantics_core::encoding::hash_encode(s)
}

fn parse_trigram(s: &str) -> Result<Trigram, String> {
    match s.to_lowercase().as_str() {
        "kun" | "\u{5764}" => Ok(Trigram::Kun),
        "gen" | "\u{826e}" => Ok(Trigram::Gen),
        "kan" | "\u{574e}" => Ok(Trigram::Kan),
        "xun" | "\u{5dcf}" => Ok(Trigram::Xun),
        "zhen" | "\u{9707}" => Ok(Trigram::Zhen),
        "li" | "\u{96e2}" => Ok(Trigram::Li),
        "dui" | "\u{5151}" => Ok(Trigram::Dui),
        "qian" | "\u{4e7e}" => Ok(Trigram::Qian),
        other => Err(format!("unknown trigram '{}'. Use: kun, gen, kan, xun, zhen, li, dui, qian", other)),
    }
}

fn parse_wuxing(s: &str) -> Result<WuXing, String> {
    match s.to_lowercase().as_str() {
        "wood" => Ok(WuXing::Wood), "fire" => Ok(WuXing::Fire),
        "earth" => Ok(WuXing::Earth), "metal" => Ok(WuXing::Metal),
        "water" => Ok(WuXing::Water),
        other => Err(format!("unknown phase '{}'. Use: wood, fire, earth, metal, water", other)),
    }
}

fn emit(cli: &Cli, val: serde_json::Value) {
    if cli.json {
        println!("{}", serde_json::to_string(&val).unwrap_or_default());
    } else if cli.csv {
        if let Some(obj) = val.as_object() {
            let keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
            let vals: Vec<String> = obj.values().map(fmt_val).collect();
            println!("{}", keys.join(","));
            println!("{}", vals.join(","));
        } else {
            println!("{}", fmt_val(&val));
        }
    } else if let Some(obj) = val.as_object() {
        let max_key = obj.keys().map(|k| k.len()).max().unwrap_or(0);
        for (k, v) in obj {
            println!("  {:<max_key$}  {}", format!("{}:", k), fmt_val(v), max_key = max_key + 1);
        }
    } else if let Some(arr) = val.as_array() {
        for v in arr { println!("  - {}", fmt_val(v)); }
    } else {
        println!("{}", fmt_val(&val));
    }
}

fn fmt_val(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => format!("{:.4}", n.as_f64().unwrap_or(0.0)),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Array(arr) => arr.iter().map(fmt_val).collect::<Vec<_>>().join(", "),
        serde_json::Value::Null => "null".into(),
        _ => v.to_string(),
    }
}

fn store_path() -> String { "ga_semantics_graph.json".to_string() }

// ── Main ────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run_command(&cli) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run_command(cli: &Cli) -> Result<(), String> {
    match &cli.command {
        Command::Encode(args) => {
            let mut c = [0.0; 8];
            for (i, v) in args.coefficients.iter().enumerate().take(8) { c[i] = *v; }
            if c.iter().all(|x| *x == 0.0) { return Err("at least one coefficient must be non-zero".into()); }
            let mv = llm_encode(&c);
            let role = mv.dominant_role();
            let dom = role.bagua();
            let mut out = serde_json::json!({
                "name": args.name.as_deref().unwrap_or("unnamed"),
                "norm": mv.norm(),
                "coefficients": mv.coefficients(),
                "dominant_role": role.role_name(),
                "dominant_trigram": dom.name(),
                "wuxing_phase": format!("{:?}", dom.wuxing_phase()),
            });
            if !cli.quiet {
                out["description"] = serde_json::json!(multivector_describe(&mv));
                let role_list: Vec<_> = multivector_to_roles(&mv).iter().map(|(n, w, d)| {
                    serde_json::json!({"role": n, "weight": w, "description": d})
                }).collect();
                out["all_roles"] = serde_json::json!(role_list);
            }
            emit(cli, out);
        }
        Command::Describe(args) => {
            let mv = parse_mv(&args.multivector);
            emit(cli, serde_json::json!({ "description": multivector_describe(&mv) }));
        }
        Command::Roles(args) => {
            let mv = parse_mv(&args.multivector);
            let all = multivector_to_roles(&mv);
            let roles: Vec<_> = (if args.top > 0 { &all[..args.top.min(all.len())] } else { &all })
                .iter().map(|(n, w, d)| serde_json::json!({"role": n, "weight": w, "description": d}))
                .collect();
            emit(cli, serde_json::json!({ "roles": roles }));
        }
        Command::Sim(args) => {
            let a = parse_mv(&args.a); let b = parse_mv(&args.b);
            let cosine = semantic_similarity(&a, &b);
            let dom = dominant_similarity(&a, &b);
            emit(cli, serde_json::json!({
                "cosine_similarity": cosine, "dominant_similarity": dom,
                "interpretation": if dom > 0.7 { "very similar" } else if dom > 0.3 { "moderate" } else if dom > 0.0 { "weak" } else { "dissimilar" }
            }));
        }
        Command::Diff(args) => {
            let a = parse_mv(&args.a); let b = parse_mv(&args.b);
            emit(cli, serde_json::json!({ "difference": semantic_difference(&a, &b) }));
        }
        Command::Classify(args) => {
            let a = parse_mv(&args.a); let b = parse_mv(&args.b);
            let (role, conf) = RelationType::from_pair(&a, &b);
            let hex = Hexagram::from_multivector_pair(&a, &b);
            emit(cli, serde_json::json!({
                "relation": role.role_name(), "description": role.description(),
                "confidence": conf, "strength": relation_strength(&a, &b),
                "hexagram": hex.name(), "hexagram_pinyin": hex.pinyin(),
                "hexagram_interpretation": hex.interpretation(),
                "pair": hex.role_pair_name(),
            }));
        }
        Command::Analogy(args) => {
            let a = parse_mv(&args.a); let b = parse_mv(&args.b); let c = parse_mv(&args.c);
            match analogy(&a, &b, &c) {
                Some(result) => {
                    let roles = multivector_to_roles(&result);
                    let role = result.dominant_role();
                    emit(cli, serde_json::json!({
                        "coefficients": result.coefficients(),
                        "dominant_role": role.role_name(),
                        "top_roles": roles.iter().take(3).map(|(n,w,_)| serde_json::json!({"role":n,"weight":w})).collect::<Vec<_>>(),
                    }));
                }
                None => eprintln!("error: could not solve analogy"),
            }
        }
        Command::Contradict(args) => {
            let a = parse_mv(&args.a); let b = parse_mv(&args.b);
            let gp = a.geo_product(&b);
            let total = gp.norm();
            let mag = if total > f64::EPSILON { gp.grade_projection(2).norm() / total } else { 0.0 };
            emit(cli, serde_json::json!({
                "is_contradiction": is_contradictory(&a, &b, args.threshold),
                "magnitude": mag, "threshold": args.threshold,
            }));
        }
        Command::Compose(args) => {
            let r1_mv = parse_mv(&args.r1); let r2_mv = parse_mv(&args.r2);
            let r1 = Rotor::from_multivector(r1_mv).unwrap_or(Rotor::identity());
            let r2 = Rotor::from_multivector(r2_mv).unwrap_or(Rotor::identity());
            let composed = compose_relations(&r1, &r2);
            emit(cli, serde_json::json!({ "result": format!("{}", composed.multivector()), "coefficients": composed.multivector().coefficients() }));
        }
        Command::RelInfo(args) => {
            let rt = RelationType::from_str(&args.role)?;
            let t = rt.bagua();
            let wu = t.wuxing_phase();
            emit(cli, serde_json::json!({
                "role": rt.role_name(), "description": rt.description(),
                "trigram": t.name(), "trigram_translation": t.translation(),
                "wuxing_phase": format!("{:?}", wu),
                "generates": format!("{:?}", wu.generate()),
                "controls": format!("{:?}", wu.control()),
            }));
        }
        Command::Trigram(args) => {
            let t = parse_trigram(&args.trigram)?;
            let wu = t.wuxing_phase();
            let comp = t.complementary();
            let mut out = serde_json::json!({
                "trigram": t.name(), "translation": t.translation(), "binary": format!("{:?}", t.binary()),
                "grade": t.grade(), "wuxing_phase": format!("{:?}", wu),
                "generates": format!("{:?}", wu.generate()), "controls": format!("{:?}", wu.control()),
                "complement": comp.name(), "complement_translation": comp.translation(),
            });
            if args.transforms {
                let details = trigram_transform_details(t);
                let xforms: Vec<_> = details.iter().map(|(tr, desc)| {
                    serde_json::json!({"trigram": tr.name(), "translation": tr.translation(), "change": desc})
                }).collect();
                out["line_transforms"] = serde_json::json!(xforms);
            }
            emit(cli, out);
        }
        Command::Hexagram(args) => {
            let a = parse_mv(&args.a); let b = parse_mv(&args.b);
            let hex = Hexagram::from_multivector_pair(&a, &b);
            let up = RelationType::from_trigram(hex.upper());
            let lo = RelationType::from_trigram(hex.lower());
            emit(cli, serde_json::json!({
                "name": hex.name(), "pinyin": hex.pinyin(), "number": hex.binary_number() + 1,
                "interpretation": hex.interpretation(), "pair": hex.role_pair_name(),
                "upper": {"trigram": hex.upper().name(), "translation": hex.upper().translation(), "role": up.role_name()},
                "lower": {"trigram": hex.lower().name(), "translation": hex.lower().translation(), "role": lo.role_name()},
            }));
        }
        Command::Wuxing(args) => {
            let p = parse_wuxing(&args.phase)?;
            let next = match args.cycle {
                CycleKind::Generating => p.generate(),
                CycleKind::Controlling => p.control(),
            };
            let trigrams: Vec<_> = p.trigrams().iter().map(|t| t.name()).collect();
            let mut out = serde_json::json!({
                "phase": p.name(), "trigrams": trigrams,
                "cycle": args.cycle.to_string(),
                "next_phase": format!("{:?}", next),
            });
            if !cli.quiet {
                let gen: Vec<_> = wuxing_generating_chain(WuXing::Wood).iter().map(|(a,b)| {
                    serde_json::json!({"from": format!("{:?}", a), "to": format!("{:?}", b)})
                }).collect();
                let ctrl: Vec<_> = wuxing_controlling_chain(WuXing::Wood).iter().map(|(a,b)| {
                    serde_json::json!({"from": format!("{:?}", a), "to": format!("{:?}", b)})
                }).collect();
                out["generating_chain"] = serde_json::json!(gen);
                out["controlling_chain"] = serde_json::json!(ctrl);
            }
            emit(cli, out);
        }
        Command::Store(sub) => run_store(cli, sub)?,
        Command::Bench(sub) => run_bench(cli, sub)?,
    }
    Ok(())
}

fn run_store(cli: &Cli, cmd: &StoreCommand) -> Result<(), String> {
    use ga_semantics_core::store::ConceptStore;
    let path = store_path();

    match cmd {
        StoreCommand::Add { name, text, coefficients } => {
            let mut c = [0.0; 8];
            for (i, v) in coefficients.iter().enumerate().take(8) { c[i] = *v; }
            let mv = llm_encode(&c);
            let mut store = ConceptStore::open(&path)?;
            let id = store.store_concept(name, text.as_deref().unwrap_or(""), mv.coefficients())?;
            if !cli.quiet { eprintln!("stored #{id}: {name}"); }
            emit(cli, serde_json::json!({ "id": id, "name": name }));
        }
        StoreCommand::Query { query, top_k } => {
            let q = parse_mv(query);
            let store = ConceptStore::open(&path)?;
            let results = store.query_similar(&q, *top_k);
            let items: Vec<_> = results.iter().map(|(c, s)| {
                serde_json::json!({"id": c.id, "name": c.name, "similarity": s})
            }).collect();
            emit(cli, serde_json::json!({ "results": items, "count": items.len() }));
        }
        StoreCommand::Relate { from_id, to_id } => {
            let mut store = ConceptStore::open(&path)?;
            let id = store.add_relation(*from_id, *to_id)?;
            if !cli.quiet { eprintln!("relation #{id}: {from_id} -> {to_id}"); }
            emit(cli, serde_json::json!({ "relation_id": id, "from": from_id, "to": to_id }));
        }
        StoreCommand::List => {
            let store = ConceptStore::open(&path)?;
            let all = store.all_concepts();
            let items: Vec<_> = all.iter().map(|c| {
                serde_json::json!({"id": c.id, "name": c.name, "created_at": c.created_at})
            }).collect();
            emit(cli, serde_json::json!({ "concepts": items, "count": items.len() }));
        }
        StoreCommand::Export => {
            let store = ConceptStore::open(&path)?;
            println!("{}", serde_json::to_string_pretty(&store.export_graph()).unwrap_or_default());
        }
        StoreCommand::Get { id } => {
            let store = ConceptStore::open(&path)?;
            match store.get_concept(*id) {
                Some(c) => {
                    let mv = Multivector::new(c.encoding);
                    let role = mv.dominant_role();
                    emit(cli, serde_json::json!({
                        "id": c.id, "name": c.name, "text": c.text,
                        "encoding": c.encoding, "created_at": c.created_at,
                        "dominant_role": role.role_name(),
                        "dominant_trigram": role.bagua().name(),
                        "wuxing_phase": format!("{:?}", role.bagua().wuxing_phase()),
                    }));
                }
                None => return Err(format!("concept {} not found", id)),
            }
        }
    }
    Ok(())
}

fn run_bench(_cli: &Cli, cmd: &BenchCommand) -> Result<(), String> {
    match cmd {
        BenchCommand::Timing => {
            println!("Timing benchmarks (500k iterations each):\n");
            bench_one("multivector_new", || { Multivector::new([1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0]); });
            let a = Multivector::new([1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0]);
            let b = Multivector::new([0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0]);
            bench_one("geo_product", || { a.geo_product(&b); });
            bench_one("norm", || { a.norm(); });
            bench_one("cosine_sim", || { semantic_similarity(&a, &b); });
            bench_one("dominant_sim", || { dominant_similarity(&a, &b); });
            bench_one("from_pair", || { RelationType::from_pair(&a, &b); });
            bench_one("llm_encode", || { llm_encode(&[0.25,0.15,-0.10,0.55,0.40,0.05,0.30,0.20]); });
            println!("\nDone.");
        }
        BenchCommand::Semantic => {
            println!("Semantic accuracy: dominant role detection\n");
            let fixtures: Vec<(&str, [f64; 8])> = vec![
                ("constraining", [0.04,-0.09,-0.51,0.68,0.21,-0.26,0.17,-0.34]),
                ("transmissive",  [0.15,0.25,0.81,-0.20,-0.25,0.10,0.36,0.05]),
                ("constraining",  [0.28,0.05,0.14,0.79,0.32,0.18,0.37,0.09]),
                ("constraining",  [0.25,0.15,-0.10,0.55,0.40,0.05,0.30,0.20]),
                ("transmissive",  [0.30,0.10,0.60,-0.25,-0.30,0.15,0.35,0.10]),
                ("clarifying",    [0.15,0.05,0.10,0.30,0.85,0.05,0.25,-0.15]),
                ("influential",   [0.10,0.20,0.20,-0.10,0.10,0.78,0.35,0.40]),
                ("balancing",     [0.30,-0.10,0.45,-0.05,0.10,0.15,0.80,0.10]),
                ("generative",    [0.15,0.55,0.30,-0.15,0.10,-0.10,0.25,0.60]),
                ("transmissive",  [0.20,0.30,0.65,0.05,-0.10,0.15,0.25,0.05]),
                ("clarifying",    [0.20,0.25,0.05,0.15,0.75,0.10,0.30,-0.05]),
                ("constraining",  [0.05,-0.30,-0.20,0.75,0.25,-0.10,0.25,-0.20]),
                ("receptive",     [0.65,0.05,0.10,0.15,0.30,0.15,0.10,0.00]),
                ("transmissive",  [0.10,0.30,0.60,-0.10,0.15,0.30,0.15,0.45]),
                ("clarifying",    [0.05,-0.10,-0.30,-0.05,-0.70,-0.65,-0.15,0.05]),
                ("generative",    [-0.30,0.25,0.15,-0.55,0.15,0.30,0.10,0.85]),
                ("constraining",  [0.20,0.05,-0.15,0.80,0.35,0.10,-0.05,-0.05]),
                ("balancing",     [0.05,0.10,0.35,-0.25,0.15,0.25,0.80,0.15]),
                ("influential",   [0.10,0.40,0.45,-0.05,0.30,0.50,0.10,0.10]),
                ("receptive",     [0.50,0.05,0.15,0.20,0.45,0.10,-0.05,-0.15]),
            ];
            let mut correct = 0usize;
            for (expected, coeffs) in &fixtures {
                let mv = llm_encode(coeffs);
                let actual = mv.dominant_role().role_name();
                let ok = actual == *expected;
                println!("  {:<20} -> {:<15} {}", mv.dominant_role().bagua().name(), actual, if ok { "OK" } else { "FAIL" });
                if ok { correct += 1; }
            }
            let pct = correct as f64 / fixtures.len() as f64 * 100.0;
            println!("\n  Accuracy: {}/{} ({:.0}%)", correct, fixtures.len(), pct);
        }
    }
    Ok(())
}

fn bench_one(name: &str, f: impl Fn()) {
    use std::time::Instant;
    let n = 500_000u64;
    let start = Instant::now();
    for _ in 0..n { f(); }
    let elapsed = start.elapsed();
    let per_op = elapsed.as_nanos() as f64 / n as f64;
    println!("  {:<20} {:>8.0} ns/op", format!("{}:", name), per_op);
}

#[cfg(test)]
mod tests {
    use super::{parse_mv, parse_trigram};

    #[test]
    fn parse_json_array() {
        let mv = parse_mv("[0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20]");
        assert!((mv.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn parse_csv() {
        let mv = parse_mv("0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20");
        assert!((mv.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn parse_trigram_valid() {
        assert!(parse_trigram("kun").is_ok());
        assert!(parse_trigram("qian").is_ok());
        assert!(parse_trigram("li").is_ok());
        assert!(parse_trigram("bad").is_err());
    }
}
