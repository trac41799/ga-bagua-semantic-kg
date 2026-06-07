use clap::{Parser, Subcommand, Args};
use ga_semantics_core::prelude::*;
use ga_semantics_core::RelationType;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "ga-semantics", about = "Geometric Algebra semantic layer CLI")]
struct Cli {
    #[arg(long, global = true, help = "Output as JSON")]
    json: bool,

    #[arg(long, global = true, help = "Output as CSV")]
    csv: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create, inspect multivectors
    Mv(MvArgs),
    /// Semantic similarity between two concepts
    Sim(SimArgs),
    /// Semantic difference between two concepts
    Diff(DiffArgs),
    /// Solve analogies: A:B :: C:?
    Analogy(AnalogyArgs),
    /// Classify relation between two concepts (returns role label)
    Classify(ClassifyArgs),
    /// Compose relations
    Compose(ComposeArgs),
    /// Check for contradiction
    Contradict(ContradictArgs),
    /// Look up relation type metadata (role label, Bagua, WuXing)
    RelationType(RelationTypeArgs),
    /// Query five-phase cycles
    Wuxing(WuxingArgs),
    /// Apply context transformation
    Context(ContextArgs),
    /// Run benchmark evaluations
    Eval(EvalArgs),
}

#[derive(Args)]
struct SimArgs {
    a: String,
    b: String,
}

#[derive(Args)]
struct DiffArgs {
    a: String,
    b: String,
}

#[derive(Args)]
struct AnalogyArgs {
    a: String,
    b: String,
    c: String,
}

#[derive(Args)]
struct ClassifyArgs {
    a: String,
    b: String,
}

#[derive(Args)]
struct ComposeArgs {
    r1: String,
    r2: String,
}

#[derive(Args)]
struct ContradictArgs {
    a: String,
    b: String,
    #[arg(default_value = "0.5")]
    threshold: f64,
}

#[derive(Args)]
struct RelationTypeArgs {
    name: String,
}

#[derive(Args)]
struct WuxingArgs {
    phase: String,
    #[arg(long)]
    cycle: Option<String>,
}

#[derive(Args)]
struct ContextArgs {
    context: String,
    entity: String,
}

#[derive(Args)]
struct EvalArgs {
    file: String,
}

#[derive(Args)]
struct MvArgs {
    coefficients: Vec<f64>,
}

fn parse_multivector(s: &str) -> Multivector {
    let parts: Vec<f64> = s.trim_matches(&['[', ']'][..])
        .split(',')
        .filter_map(|x| x.trim().parse().ok())
        .collect();
    let mut coeffs = [0.0; 8];
    for (i, v) in parts.iter().enumerate().take(8) {
        coeffs[i] = *v;
    }
    Multivector::new(coeffs)
}

fn print_value<T: serde::Serialize + std::fmt::Display>(cli: &Cli, label: &str, value: T) {
    if cli.json {
        let map = serde_json::json!({ label: value });
        println!("{}", serde_json::to_string(&map).unwrap_or_default());
    } else if cli.csv {
        println!("{},{}", label, value);
    } else {
        println!("{}: {}", label, value);
    }
}

fn print_map(cli: &Cli, map: &serde_json::Value) {
    if cli.json {
        println!("{}", serde_json::to_string(map).unwrap_or_default());
    } else if cli.csv {
        if let Some(obj) = map.as_object() {
            let keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
            println!("{}", keys.join(","));
            let vals: Vec<String> = obj.values().map(|v| match v {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => format!("{}", v),
            }).collect();
            println!("{}", vals.join(","));
        }
    } else {
        if let Some(obj) = map.as_object() {
            for (k, v) in obj {
                let val = match v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => format!("{}", v),
                };
                println!("{}: {}", k, val);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Mv(ref args) => {
            let mut coeffs = [0.0; 8];
            for (i, v) in args.coefficients.iter().enumerate().take(8) {
                coeffs[i] = *v;
            }
            let mv = Multivector::new(coeffs);
            print_value(&cli, "multivector", format!("{}", mv));
        }
        Command::Sim(ref args) => {
            let a = parse_multivector(&args.a);
            let b = parse_multivector(&args.b);
            let score = semantic_similarity(&a, &b);
            print_value(&cli, "similarity", score);
        }
        Command::Diff(ref args) => {
            let a = parse_multivector(&args.a);
            let b = parse_multivector(&args.b);
            let score = semantic_difference(&a, &b);
            print_value(&cli, "difference", score);
        }
        Command::Analogy(ref args) => {
            let a = parse_multivector(&args.a);
            let b = parse_multivector(&args.b);
            let c = parse_multivector(&args.c);
            match analogy(&a, &b, &c) {
                Some(result) => print_value(&cli, "result", format!("{}", result)),
                None => eprintln!("error: degenerate multivector in analogy"),
            }
        }
        Command::Classify(ref args) => {
            let a = parse_multivector(&args.a);
            let b = parse_multivector(&args.b);
            let (role, confidence) = RelationType::from_pair(&a, &b);
            print_map(&cli, &serde_json::json!({
                "relation_type": role.role_name(),
                "confidence": confidence,
            }));
        }
        Command::Compose(ref args) => {
            let r1_mv = parse_multivector(&args.r1);
            let r2_mv = parse_multivector(&args.r2);
            let r1 = Rotor::from_multivector(r1_mv).unwrap_or(Rotor::identity());
            let r2 = Rotor::from_multivector(r2_mv).unwrap_or(Rotor::identity());
            let composed = compose_relations(&r1, &r2);
            print_value(&cli, "composed", format!("{}", composed.multivector()));
        }
        Command::Contradict(ref args) => {
            let a = parse_multivector(&args.a);
            let b = parse_multivector(&args.b);
            let result = is_contradictory(&a, &b, args.threshold);
            print_value(&cli, "contradiction", result);
        }
        Command::RelationType(ref args) => {
            match RelationType::from_str(&args.name) {
                Ok(rt) => {
                    print_map(&cli, &serde_json::json!({
                        "role": rt.role_name(),
                        "description": rt.description(),
                        "bagua_trigram": format!("{:?}", rt.bagua()),
                        "wuxing_phase": format!("{:?}", rt.wuxing_phase()),
                    }));
                }
                Err(e) => eprintln!("error: {e}"),
            }
        }
        Command::Wuxing(ref args) => {
            let p = match args.phase.to_lowercase().as_str() {
                "wood" => ga_semantics_core::advanced::WuXing::Wood,
                "fire" => ga_semantics_core::advanced::WuXing::Fire,
                "earth" => ga_semantics_core::advanced::WuXing::Earth,
                "metal" => ga_semantics_core::advanced::WuXing::Metal,
                "water" => ga_semantics_core::advanced::WuXing::Water,
                _ => { eprintln!("unknown phase: {}", args.phase); return; }
            };
            print_map(&cli, &serde_json::json!({
                "phase": p.name(),
                "generates": format!("{:?}", p.generate()),
                "controls": format!("{:?}", p.control()),
            }));
        }
        Command::Context(ref args) => {
            let ctx_mv = parse_multivector(&args.context);
            let entity = parse_multivector(&args.entity);
            if let Some(rotor) = Rotor::from_multivector(ctx_mv) {
                let ctx = Context::new(rotor);
                let result = ctx.apply(&entity);
                print_value(&cli, "result", format!("{}", result));
            } else {
                eprintln!("error: invalid context rotor");
            }
        }
        Command::Eval(ref args) => {
            let content = match std::fs::read_to_string(&args.file) {
                Ok(c) => c,
                Err(e) => { eprintln!("error reading {}: {}", args.file, e); return; }
            };
            let cases: Vec<Vec<f64>> = serde_json::from_str(&content).unwrap_or_default();
            print_value(&cli, "loaded_cases", cases.len());
        }
    }
}
