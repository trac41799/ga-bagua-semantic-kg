use clap::Parser;
use ga_semantics_core::prelude::*;
use ga_semantics_core::store::ConceptStore;
use ga_semantics_core::RelationType;
use std::io::{self, BufRead, Write};
use std::sync::Mutex;

static STORE: Mutex<Option<ConceptStore>> = Mutex::new(None);

#[derive(Parser)]
#[command(name = "ga-semantics-mcp", about = "GA-Bagua Semantic KG MCP server")]
struct Args {
    /// Run as HTTP server instead of stdio MCP
    #[arg(long)]
    http: bool,

    /// Port for HTTP server (default: 3100)
    #[arg(long, default_value = "3100")]
    port: u16,
}

fn main() {
    let args = Args::parse();

    if args.http {
        start_http_server(args.port);
    } else {
        run_stdio();
    }
}

fn run_stdio() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }
        let response = handle_request(&line);
        let mut out = stdout.lock();
        let _ = writeln!(out, "{}", response);
        let _ = out.flush();
    }
}

fn start_http_server(port: u16) {
    use axum::http::Method;
    use axum::routing::{get, post};
    use axum::{Json, Router};
    use std::net::SocketAddr;
    use tower_http::cors::{Any, CorsLayer};

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route(
            "/mcp",
            post(|body: String| async move {
                let response = handle_request(&body);
                Json(
                    serde_json::from_str::<serde_json::Value>(&response)
                        .unwrap_or(serde_json::json!({"error": "internal error"})),
                )
            }),
        )
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("MCP HTTP server listening on http://0.0.0.0:{}/mcp", port);

    let rt = tokio::runtime::Runtime::new().expect("Failed to start tokio runtime");
    rt.block_on(async {
        axum::serve(
            tokio::net::TcpListener::bind(addr).await.unwrap(),
            app,
        )
        .await
        .unwrap();
    });
}

fn handle_request(body: &str) -> String {
    let req: serde_json::Value = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => return json_error(None, -32700, format!("Parse error: {e}")),
    };
    let id = req.get("id").cloned();
    let method = req["method"].as_str().unwrap_or("").to_string();
    let params = req.get("params").cloned().unwrap_or(serde_json::Value::Null);

    match method.as_str() {
        "initialize" => json_result(id, serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": { "tools": {} },
            "serverInfo": { "name": "ga-semantics-mcp", "version": "0.1.1" }
        })),
        "tools/list" => json_result(id, serde_json::json!({ "tools": get_tools() })),
        "tools/call" => handle_tool_call(id, &params),
        _ => json_error(id, -32601, format!("Method not found: {method}")),
    }
}

fn handle_tool_call(id: Option<serde_json::Value>, params: &serde_json::Value) -> String {
    let name = params["name"].as_str().unwrap_or("").to_string();
    let args = &params["arguments"];

    let result = match name.as_str() {
        "create_multivector" => {
            let coeffs: [f64; 8] = serde_json::from_value(args["coefficients"].clone()).unwrap_or([0.0; 8]);
            let mv = Multivector::new(coeffs);
            serde_json::json!({ "multivector": format!("{mv}"), "coefficients": coeffs, "norm": mv.norm() })
        }
        "text_to_multivector" => {
            let text = args["text"].as_str().unwrap_or("");
            let mv = text_to_multivector(text);
            let desc = multivector_describe(&mv);
            let roles = multivector_to_roles(&mv);
            serde_json::json!({
                "multivector": format!("{mv}"),
                "coefficients": mv.coefficients(),
                "norm": mv.norm(),
                "description": desc,
                "top_roles": roles.iter().take(3).map(|(n,w,d)| serde_json::json!({"role":n,"weight":w,"description":d})).collect::<Vec<_>>()
            })
        }
        "word_to_multivector" => {
            let word = args["word"].as_str().unwrap_or("");
            let mv = word_to_multivector(word);
            serde_json::json!({ "multivector": format!("{mv}"), "coefficients": mv.coefficients(), "norm": mv.norm() })
        }
        "multivector_describe" => {
            let mv = parse_mv(&args["multivector"]);
            let desc = multivector_describe(&mv);
            let roles = multivector_to_roles(&mv);
            serde_json::json!({
                "description": desc,
                "all_roles": roles.iter().map(|(n,w,d)| serde_json::json!({"role":n,"weight":w,"description":d})).collect::<Vec<_>>()
            })
        }
        "semantic_similarity" => {
            let a = parse_mv(&args["a"]);
            let b = parse_mv(&args["b"]);
            let score = semantic_similarity(&a, &b);
            let relation = semantic_relation(&a, &b);
            serde_json::json!({
                "score": score,
                "interpretation": interpret_similarity(score),
                "underlying_relation": relation.role_name()
            })
        }
        "semantic_difference" => {
            let a = parse_mv(&args["a"]);
            let b = parse_mv(&args["b"]);
            let score = semantic_difference(&a, &b);
            serde_json::json!({ "score": score, "interpretation": interpret_difference(score) })
        }
        "analogy" => {
            let a = parse_mv(&args["a"]);
            let b = parse_mv(&args["b"]);
            let c = parse_mv(&args["c"]);
            match analogy(&a, &b, &c) {
                Some(result) => {
                    let desc = multivector_describe(&result);
                    serde_json::json!({ "result": format!("{result}"), "coefficients": result.coefficients(), "description": desc })
                }
                None => serde_json::json!({ "error": "degenerate input" }),
            }
        }
        "classify_relation" => {
            let a = parse_mv(&args["a"]);
            let b = parse_mv(&args["b"]);
            let (role, confidence) = RelationType::from_pair(&a, &b);
            let strength = relation_strength(&a, &b);
            serde_json::json!({
                "relation_type": role.role_name(),
                "description": role.description(),
                "confidence": confidence,
                "strength": strength,
                "bagua_trigram": format!("{:?}", role.bagua()),
                "wuxing_phase": format!("{:?}", role.wuxing_phase()),
            })
        }
        "compose_relations" => {
            let r1 = parse_rotor(&args["r1"]);
            let r2 = parse_rotor(&args["r2"]);
            let composed = compose_relations(&r1, &r2);
            serde_json::json!({ "result": format!("{}", composed.multivector()), "coefficients": composed.multivector().coefficients() })
        }
        "detect_contradiction" => {
            let a = parse_mv(&args["a"]);
            let b = parse_mv(&args["b"]);
            let threshold = args["threshold"].as_f64().unwrap_or(0.5);
            let result = is_contradictory(&a, &b, threshold);
            let mag = a.geo_product(&b).grade_projection(2).norm();
            let sim = semantic_similarity(&a, &b);
            serde_json::json!({ "is_contradiction": result, "contradiction_magnitude": mag, "similarity": sim })
        }
        "relation_type_info" => {
            let name = args["role"].as_str().unwrap_or("");
            match name.parse::<RelationType>() {
                Ok(rt) => serde_json::json!({
                    "role": rt.role_name(), "description": rt.description(),
                    "bagua_trigram": format!("{:?}", rt.bagua()),
                    "wuxing_phase": format!("{:?}", rt.wuxing_phase()),
                }),
                Err(e) => serde_json::json!({ "error": e }),
            }
        }
        "wuxing_query" => {
            let phase = args["phase"].as_str().unwrap_or("");
            let p = parse_wuxing(phase);
            let cycle_type = args["cycle"].as_str().unwrap_or("generating");
            let next = if cycle_type == "controlling" { p.control() } else { p.generate() };
            serde_json::json!({
                "phase": format!("{:?}", p), "phase_name": p.name(),
                "next_in_cycle": format!("{:?}", next),
                "trigrams": p.trigrams().iter().map(|t| t.name()).collect::<Vec<_>>(),
            })
        }
        "context_apply" => {
            let ctx_mv = parse_mv(&args["context"]);
            let entity = parse_mv(&args["entity"]);
            if let Some(rotor) = Rotor::from_multivector(ctx_mv) {
                let ctx = Context::new(rotor);
                let result = ctx.apply(&entity);
                let desc = multivector_describe(&result);
                serde_json::json!({ "result": format!("{result}"), "coefficients": result.coefficients(), "description": desc })
            } else {
                serde_json::json!({ "error": "invalid context rotor" })
            }
        }
        "batch_process" => {
            if let Some(ops) = args.get("operations").and_then(|v| v.as_array()) {
                let results: Vec<serde_json::Value> = ops.iter().map(|op| {
                    let tn = op["name"].as_str().unwrap_or("");
                    let tp = serde_json::to_string(&serde_json::json!({ "method": "tools/call", "params": { "name": tn, "arguments": op.get("arguments").cloned().unwrap_or(serde_json::json!(null)) } })).unwrap_or_default();
                    serde_json::from_str(&handle_request(&tp)).unwrap_or(serde_json::json!(null))
                }).collect();
                serde_json::json!({ "results": results })
            } else {
                serde_json::json!({ "error": "missing operations array" })
            }
        }
        "semantic_explore" => {
            let concept = args["concept"].as_str().unwrap_or("");
            let related: Vec<String> = args["related"].as_array()
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();

            let main_mv = text_to_multivector(concept);
            let main_desc = multivector_describe(&main_mv);
            let main_roles = multivector_to_roles(&main_mv);

            let comparisons: Vec<serde_json::Value> = related.iter().map(|rel| {
                let rel_mv = text_to_multivector(rel);
                let sim = semantic_similarity(&main_mv, &rel_mv);
                let diff = semantic_difference(&main_mv, &rel_mv);
                let (role, _) = RelationType::from_pair(&main_mv, &rel_mv);
                let strength = relation_strength(&main_mv, &rel_mv);
                serde_json::json!({
                    "concept": rel, "similarity": sim, "difference": diff,
                    "relation_type": role.role_name(), "relation_description": role.description(),
                    "strength": strength
                })
            }).collect();

            serde_json::json!({
                "concept": concept, "concept_encoding": main_mv.coefficients(),
                "concept_description": main_desc,
                "top_roles": main_roles.iter().take(3).map(|(n,w,d)| serde_json::json!({"role":n,"weight":w,"description":d})).collect::<Vec<_>>(),
                "comparisons": comparisons, "encoded_size_bytes": 64,
            })
        }
        "llm_encode" => {
            let name = args["name"].as_str().unwrap_or("unnamed");
            let coeffs_arr: Vec<f64> = args["coefficients"].as_array()
                .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                .unwrap_or_default();
            if coeffs_arr.len() != 8 {
                return json_error(id, -32602, "coefficients must be exactly 8 floats".to_string());
            }
            let mut coeff = [0.0f64; 8];
            coeff.copy_from_slice(&coeffs_arr[0..8]);
            let raw_mv = Multivector::new(coeff);
            let n = raw_mv.norm();
            let normalized = if n > f64::EPSILON { raw_mv * (1.0 / n) } else { Multivector::one() };
            let desc = multivector_describe(&normalized);
            let roles = multivector_to_roles(&normalized);
            let dominant = normalized.dominant_role();
            serde_json::json!({
                "concept": name, "raw_coefficients": coeff,
                "normalized_coefficients": normalized.coefficients(),
                "norm_before_normalization": n,
                "dominant_role": dominant.role_name(),
                "dominant_role_description": dominant.description(),
                "description": desc,
                "all_roles": roles.iter().map(|(n,w,d)| serde_json::json!({"role":n,"weight":w,"description":d})).collect::<Vec<_>>(),
                "bagua_trigram": format!("{:?}", dominant.bagua()),
                "wuxing_phase": format!("{:?}", dominant.wuxing_phase()),
                "encoded_size_bytes": 64,
            })
        }
        "classify_hexagram" => {
            let a = parse_mv(&args["a"]);
            let b = parse_mv(&args["b"]);
            let hex = ga_semantics_core::advanced::Hexagram::from_multivector_pair(&a, &b);
            let upper_role = RelationType::from_trigram(hex.upper());
            let lower_role = RelationType::from_trigram(hex.lower());
            serde_json::json!({
                "hexagram_number": hex.binary_number() + 1,
                "hexagram_name": hex.name(),
                "hexagram_pinyin": hex.pinyin(),
                "hexagram_interpretation": hex.interpretation(),
                "pair_name": hex.role_pair_name(),
                "upper_trigram": {
                    "name": hex.upper().name(), "translation": hex.upper().translation(),
                    "role": upper_role.role_name(), "description": hex.upper().description(),
                    "wuxing_phase": format!("{:?}", hex.upper().wuxing_phase()),
                    "binary": format!("{:?}", hex.upper().binary()),
                },
                "lower_trigram": {
                    "name": hex.lower().name(), "translation": hex.lower().translation(),
                    "role": lower_role.role_name(), "description": hex.lower().description(),
                    "wuxing_phase": format!("{:?}", hex.lower().wuxing_phase()),
                    "binary": format!("{:?}", hex.lower().binary()),
                },
            })
        }
        "bagua_dynamics" => {
            let mode = args["mode"].as_str().unwrap_or("explore");
            match mode {
                "trigram_transforms" => {
                    let t = parse_trigram(args["trigram"].as_str().unwrap_or("qian"));
                    let transforms = ga_semantics_core::advanced::trigram_transform_details(t);
                    let complementary = t.complementary();
                    serde_json::json!({
                        "trigram": t.name(), "translation": t.translation(), "description": t.description(),
                        "binary": format!("{:?}", t.binary()),
                        "wuxing_phase": format!("{:?}", t.wuxing_phase()),
                        "complement": complementary.name(), "complement_translation": complementary.translation(),
                        "line_transforms": transforms.iter().map(|(trig, desc)| {
                            serde_json::json!({"trigram":trig.name(),"translation":trig.translation(),"change":desc})
                        }).collect::<Vec<_>>(),
                    })
                }
                "wuxing_chain" => {
                    let p = parse_wuxing(args["phase"].as_str().unwrap_or("wood"));
                    let gen_chain = ga_semantics_core::advanced::wuxing_generating_chain(p);
                    let ctrl_chain = ga_semantics_core::advanced::wuxing_controlling_chain(p);
                    let all_phases: Vec<_> = [ga_semantics_core::advanced::WuXing::Wood, ga_semantics_core::advanced::WuXing::Fire, ga_semantics_core::advanced::WuXing::Earth, ga_semantics_core::advanced::WuXing::Metal, ga_semantics_core::advanced::WuXing::Water].iter().map(|w| {
                        serde_json::json!({"phase":format!("{:?}",w),"name":w.name(),"trigrams":w.trigrams().iter().map(|t|t.name()).collect::<Vec<_>>()})
                    }).collect::<Vec<_>>();
                    serde_json::json!({
                        "starting_phase": p.name(),
                        "generating_cycle": gen_chain.iter().map(|(a,b)| serde_json::json!({"from":format!("{:?}",a),"to":format!("{:?}",b)})).collect::<Vec<_>>(),
                        "controlling_cycle": ctrl_chain.iter().map(|(a,b)| serde_json::json!({"from":format!("{:?}",a),"to":format!("{:?}",b)})).collect::<Vec<_>>(),
                        "all_phases": all_phases,
                    })
                }
                _ => {
                    let t = parse_trigram(args["trigram"].as_str().unwrap_or("qian"));
                    let complementary = t.complementary();
                    let transforms = ga_semantics_core::advanced::trigram_transform_details(t);
                    let wu = t.wuxing_phase();
                    serde_json::json!({
                        "trigram": t.name(), "translation": t.translation(), "description": t.description(),
                        "binary": format!("{:?}", t.binary()), "grade": t.grade(),
                        "wuxing_phase": format!("{:?}", wu), "wuxing_name": wu.name(),
                        "wuxing_generates": format!("{:?}", wu.generate()),
                        "wuxing_controls": format!("{:?}", wu.control()),
                        "complement": complementary.name(), "complement_translation": complementary.translation(),
                        "line_transforms": transforms.iter().map(|(trig, desc)| {
                            serde_json::json!({"trigram":trig.name(),"translation":trig.translation(),"change":desc})
                        }).collect::<Vec<_>>(),
                    })
                }
            }
        }
        "validate_encoding" => {
            let a = parse_mv(&args["a"]);
            let b = parse_mv(&args["b"]);
            let expected_role = args["expected_relation"].as_str().unwrap_or("");
            let (actual_role, confidence) = RelationType::from_pair(&a, &b);
            let sim = semantic_similarity(&a, &b);
            let diff = semantic_difference(&a, &b);
            let strength = relation_strength(&a, &b);
            let expected_parsed = expected_role.parse::<RelationType>().ok();
            let matches = expected_parsed.map(|r| r == actual_role).unwrap_or(false);
            serde_json::json!({
                "actual_relation": actual_role.role_name(), "actual_description": actual_role.description(),
                "expected_relation": expected_role, "match": matches, "confidence": confidence,
                "similarity": sim, "difference": diff, "strength": strength,
                "verdict": if matches && confidence > 0.5 { "PASS" } else if matches { "WEAK_PASS" } else if confidence > 0.5 { "MISMATCH" } else { "UNCLEAR" },
            })
        }
        "encoding_benchmark" => {
            let test_cases: Vec<(&str, &str, &str)> = vec![
                ("a triggering event that initiates a chain of causal consequences", "a boundary condition that limits and constrains possible outcomes", "causal"),
                ("a flowing channel that transmits information between nodes", "a rigid boundary that restricts movement and limits possibilities", "transmissive"),
                ("an innovative creation that introduces entirely new patterns", "an accepted convention that grounds and stabilizes existing practices", "generative"),
                ("a monitoring system that observes and reveals internal state", "a black box that hides implementation details from consumers", "clarifying"),
                ("a feedback loop that mirrors outputs back to inputs for adjustment", "a one-way pipeline that transmits data without reflection", "balancing"),
            ];
            let mut results = vec![];
            for (desc_a, desc_b, expected) in &test_cases {
                let a = text_to_multivector(desc_a);
                let b = text_to_multivector(desc_b);
                let (actual, confidence) = RelationType::from_pair(&a, &b);
                let matched = actual.role_name() == *expected;
                results.push(serde_json::json!({"pair":[desc_a,desc_b],"expected":expected,"actual":actual.role_name(),"confidence":confidence,"pass":matched}));
            }
            let passed = results.iter().filter(|r| r["pass"].as_bool().unwrap_or(false)).count();
            serde_json::json!({"total_cases":results.len(), "passed":passed, "failed":results.len()-passed, "accuracy_pct": (passed as f64 / results.len() as f64 * 100.0) as u32, "note":"Hash-based encoding. Use llm_encode for better results.", "results":results})
        }
        "store_open" => {
            let path = args["path"].as_str().unwrap_or("ga_semantics_graph.json");
            match ConceptStore::open(path) {
                Ok(store) => {
                    let count = store.concept_count();
                    *STORE.lock().unwrap() = Some(store);
                    serde_json::json!({"status":"opened","path":path,"concept_count":count})
                }
                Err(e) => serde_json::json!({"error":e}),
            }
        }
        "store_concept" => {
            let name = args["name"].as_str().unwrap_or("");
            let text = args["text"].as_str().unwrap_or("");
            let enc = parse_mv(&args["encoding"]);
            let mut store_guard = STORE.lock().unwrap();
            match store_guard.as_mut() {
                Some(store) => match store.store_concept(name, text, enc.coefficients()) {
                    Ok(id) => serde_json::json!({"id":id,"name":name,"encoding":enc.coefficients(),"text":text}),
                    Err(e) => serde_json::json!({"error":e}),
                },
                None => serde_json::json!({"error":"No store open. Call store_open first."}),
            }
        }
        "store_llm_concept" => {
            let name = args["name"].as_str().unwrap_or("");
            let text = args["text"].as_str().unwrap_or("");
            let coeffs_arr: Vec<f64> = args["coefficients"].as_array()
                .map(|a| a.iter().filter_map(|v| v.as_f64()).collect()).unwrap_or_default();
            if coeffs_arr.len() != 8 { return json_error(id, -32602, "need 8 coefficients".to_string()); }
            let mut c = [0.0; 8]; c.copy_from_slice(&coeffs_arr[0..8]);
            let raw = Multivector::new(c);
            let n = raw.norm();
            let norm = if n > f64::EPSILON { raw * (1.0/n) } else { Multivector::one() };
            let mut store_guard = STORE.lock().unwrap();
            match store_guard.as_mut() {
                Some(store) => match store.store_concept(name, text, norm.coefficients()) {
                    Ok(id) => {
                        let desc = multivector_describe(&norm);
                        let dom = norm.dominant_role();
                        serde_json::json!({"id":id,"name":name,"encoding":norm.coefficients(),"dominant_role":dom.role_name(),"description":desc})
                    },
                    Err(e) => serde_json::json!({"error":e}),
                },
                None => serde_json::json!({"error":"No store open. Call store_open first."}),
            }
        }
        "store_query_similar" => {
            let query = parse_mv(&args["query"]);
            let top_k = args["top_k"].as_u64().unwrap_or(10) as usize;
            let store_guard = STORE.lock().unwrap();
            match store_guard.as_ref() {
                Some(store) => {
                    let results = store.query_similar(&query, top_k);
                    serde_json::json!({"results": results.iter().map(|(c,s)| {
                        serde_json::json!({"id":c.id,"name":c.name,"similarity":s})
                    }).collect::<Vec<_>>()})
                },
                None => serde_json::json!({"error":"No store open."}),
            }
        }
        "store_get_concept" => {
            let id = args["id"].as_i64().unwrap_or(-1);
            let store_guard = STORE.lock().unwrap();
            match store_guard.as_ref().and_then(|s| s.get_concept(id)) {
                Some(c) => serde_json::json!({"id":c.id,"name":c.name,"text":c.text,"encoding":c.encoding,"created_at":c.created_at}),
                None => serde_json::json!({"error":"concept not found"}),
            }
        }
        "store_list_concepts" => {
            let store_guard = STORE.lock().unwrap();
            match store_guard.as_ref() {
                Some(store) => {
                    let all = store.all_concepts();
                    serde_json::json!({"count":all.len(),"concepts":all.iter().map(|c| serde_json::json!({"id":c.id,"name":c.name})).collect::<Vec<_>>()})
                },
                None => serde_json::json!({"error":"No store open."}),
            }
        }
        "store_add_relation" => {
            let from_id = args["from_id"].as_i64().unwrap_or(-1);
            let to_id = args["to_id"].as_i64().unwrap_or(-1);
            let mut store_guard = STORE.lock().unwrap();
            match store_guard.as_mut() {
                Some(store) => match store.add_relation(from_id, to_id) {
                    Ok(rel_id) => serde_json::json!({"relation_id":rel_id}),
                    Err(e) => serde_json::json!({"error":e}),
                },
                None => serde_json::json!({"error":"No store open."}),
            }
        }
        "store_export" => {
            let store_guard = STORE.lock().unwrap();
            match store_guard.as_ref() {
                Some(store) => store.export_graph(),
                None => serde_json::json!({"error":"No store open."}),
            }
        }
        "store_close" => {
            *STORE.lock().unwrap() = None;
            serde_json::json!({"status":"closed"})
        }
        "ideate_seed" => {
            let name = args["name"].as_str().unwrap_or("problem");
            let coeffs_arr: Vec<f64> = args["coefficients"].as_array()
                .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                .unwrap_or_default();
            if coeffs_arr.len() != 8 {
                return json_error(id, -32602, "coefficients must be exactly 8 floats".to_string());
            }
            let mut coeff = [0.0f64; 8];
            coeff.copy_from_slice(&coeffs_arr[0..8]);
            let raw_mv = Multivector::new(coeff);
            let n = raw_mv.norm();
            let seed_mv = if n > f64::EPSILON { raw_mv * (1.0 / n) } else { Multivector::one() };
            let dominant_trigram = seed_mv.dominant_role().bagua();
            serde_json::json!({
                "concept": name,
                "seed_coefficients": seed_mv.coefficients(),
                "dominant_trigram": dominant_trigram.name(),
                "dominant_phase": format!("{:?}", dominant_trigram.wuxing_phase()),
                "norm": seed_mv.norm(),
            })
        }
        "ideate_step" => {
            let seed = parse_mv(&args["seed"]);
            let hex_num = args["hexagram"].as_u64().unwrap_or(1).min(64).max(1);
            use ga_semantics_core::advanced::{Hexagram, hexagram_step};
            let hex = Hexagram::from_number(hex_num as u8);
            match hexagram_step(&seed, &hex) {
                Some(result) => {
                    let roles = multivector_to_roles(&result);
                    serde_json::json!({
                        "hexagram_number": hex_num,
                        "hexagram_name": hex.name(),
                        "hexagram_pinyin": hex.pinyin(),
                        "interpretation": hex.interpretation(),
                        "shifted_perspective": result.coefficients(),
                        "top_roles": roles.iter().take(3).map(|(n,w,d)| serde_json::json!({"role":n,"weight":w,"description":d})).collect::<Vec<_>>(),
                    })
                }
                None => serde_json::json!({ "error": "degenerate seed — cannot step" }),
            }
        }
        "ideate_explore" => {
            let seed = parse_mv(&args["seed"]);
            let top_n = args["top_n"].as_u64().unwrap_or(8).min(64).max(1) as usize;
            use ga_semantics_core::advanced::hexagram_explore;
            let results = hexagram_explore(&seed, top_n);
            let ranked: Vec<serde_json::Value> = results.iter().enumerate().map(|(i, (hex, mv, interp))| {
                let roles = multivector_to_roles(mv);
                serde_json::json!({
                    "rank": i + 1,
                    "hexagram_number": hex.binary_number() + 1,
                    "hexagram_name": hex.name(),
                    "interpretation": interp,
                    "coefficients": mv.coefficients(),
                    "top_roles": roles.iter().take(2).map(|(n,w,_d)| serde_json::json!({"role":n,"weight":w})).collect::<Vec<_>>(),
                })
            }).collect();
            serde_json::json!({
                "seed_concept": multivector_describe(&seed),
                "perspectives_explored": results.len(),
                "perspectives": ranked,
            })
        }
        "ideate_blend" => {
            let a = parse_mv(&args["a"]);
            let b = parse_mv(&args["b"]);
            let blend = a.geo_product(&b);
            let normalized = if blend.norm() > f64::EPSILON { blend * (1.0 / blend.norm()) } else { Multivector::one() };
            use ga_semantics_core::advanced::Hexagram;
            let hex = Hexagram::from_multivector_pair(&a, &b);
            let roles = multivector_to_roles(&normalized);
            serde_json::json!({
                "blend_coefficients": normalized.coefficients(),
                "hexagram_number": hex.binary_number() + 1,
                "hexagram_name": hex.name(),
                "hexagram_pinyin": hex.pinyin(),
                "interpretation": hex.interpretation(),
                "top_roles": roles.iter().take(3).map(|(n,w,d)| serde_json::json!({"role":n,"weight":w,"description":d})).collect::<Vec<_>>(),
            })
        }
        _ => return json_error(id, -32601, format!("Unknown tool: {name}")),
    };
    json_result(id, result)
}

fn parse_mv(v: &serde_json::Value) -> Multivector {
    if let Some(arr) = v.as_array() {
        let mut coeffs = [0.0; 8];
        for (i, val) in arr.iter().enumerate().take(8) {
            coeffs[i] = val.as_f64().unwrap_or(0.0);
        }
        Multivector::new(coeffs)
    } else if let Some(s) = v.as_str() {
        hash_encode(s)
    } else {
        Multivector::zero()
    }
}

fn parse_rotor(v: &serde_json::Value) -> Rotor {
    Rotor::from_multivector(parse_mv(v)).unwrap_or(Rotor::identity())
}

fn parse_wuxing(s: &str) -> ga_semantics_core::advanced::WuXing {
    match s.to_lowercase().as_str() {
        "wood" => ga_semantics_core::advanced::WuXing::Wood,
        "fire" => ga_semantics_core::advanced::WuXing::Fire,
        "earth" => ga_semantics_core::advanced::WuXing::Earth,
        "metal" => ga_semantics_core::advanced::WuXing::Metal,
        "water" => ga_semantics_core::advanced::WuXing::Water,
        _ => ga_semantics_core::advanced::WuXing::Wood,
    }
}

fn parse_trigram(s: &str) -> ga_semantics_core::advanced::Trigram {
    use ga_semantics_core::advanced::Trigram;
    match s.to_lowercase().as_str() {
        "kun" => Trigram::Kun, "gen" => Trigram::Gen, "kan" => Trigram::Kan, "xun" => Trigram::Xun,
        "zhen" => Trigram::Zhen, "li" => Trigram::Li, "dui" => Trigram::Dui,
        _ => Trigram::Qian,
    }
}

fn json_result(id: Option<serde_json::Value>, result: serde_json::Value) -> String {
    serde_json::to_string(&serde_json::json!({ "jsonrpc": "2.0", "id": id.unwrap_or(serde_json::json!(null)), "result": result })).unwrap_or_default()
}

fn json_error(id: Option<serde_json::Value>, code: i32, message: String) -> String {
    serde_json::to_string(&serde_json::json!({ "jsonrpc": "2.0", "id": id.unwrap_or(serde_json::json!(null)), "error": { "code": code, "message": message } })).unwrap_or_default()
}

fn mv8_schema(desc: &str) -> serde_json::Value {
    serde_json::json!({"type":"array","description":desc,"items":{"type":"number"},"minItems":8,"maxItems":8})
}

fn interpret_similarity(score: f64) -> &'static str {
    if score > 0.7 { "strongly aligned" } else if score > 0.3 { "moderately aligned" } else if score > -0.3 { "weakly related" } else if score > -0.7 { "moderately opposed" } else { "strongly opposed" }
}

fn interpret_difference(score: f64) -> &'static str {
    if score < 0.2 { "very similar" } else if score < 0.4 { "somewhat different" } else if score < 0.6 { "clearly different" } else { "highly divergent" }
}

fn get_tools() -> Vec<serde_json::Value> {
    vec![
        tool("text_to_multivector",
            "Encode free-form text into a unit-norm multivector (8 f64). Returns coefficients, human-readable semantic description, and top activated roles. Primary entry point for LLMs to convert concepts into algebraic form.",
            serde_json::json!({"type":"object","properties":{"text":{"type":"string","description":"Free text description to encode"}},"required":["text"]})),
        tool("word_to_multivector",
            "Deterministically encode a single word into a unit-norm multivector using FNV hashing. Same word always produces same encoding.",
            serde_json::json!({"type":"object","properties":{"word":{"type":"string","description":"A single word or short phrase"}},"required":["word"]})),
        tool("multivector_describe",
            "Decode a multivector back into human-readable semantic role descriptions showing which roles are activated.",
            serde_json::json!({"type":"object","properties":{"multivector":mv8_schema("8 coefficients of the multivector")},"required":["multivector"]})),
        tool("semantic_similarity",
            "Compute semantic alignment between two concepts. Returns score [-1,1] plus interpretation and underlying relation type. 1=same, 0=unrelated, -1=opposite.",
            serde_json::json!({"type":"object","properties":{"a":{"oneOf":[mv8_schema("First concept"),{"type":"string","description":"Text to auto-encode"}]},"b":{"oneOf":[mv8_schema("Second concept"),{"type":"string","description":"Text to auto-encode"}]}},"required":["a","b"]})),
        tool("semantic_difference",
            "Compute how different two concepts are. Returns score [0,1] with interpretation. 0=identical, 1=completely distinct.",
            serde_json::json!({"type":"object","properties":{"a":{"oneOf":[mv8_schema(""),{"type":"string"}]},"b":{"oneOf":[mv8_schema(""),{"type":"string"}]}},"required":["a","b"]})),
        tool("analogy",
            "Solve a : b :: c : ? via rotor application. Classic reasoning: king:queen :: man:woman.",
            serde_json::json!({"type":"object","properties":{"a":{"oneOf":[mv8_schema(""),{"type":"string"}]},"b":{"oneOf":[mv8_schema(""),{"type":"string"}]},"c":{"oneOf":[mv8_schema(""),{"type":"string"}]}},"required":["a","b","c"]})),
        tool("classify_relation",
            "Classify the semantic relationship between two concepts into 8 interpretable roles: generative, receptive, causal, transmissive, constraining, influential, clarifying, balancing. Returns role, description, confidence, and strength.",
            serde_json::json!({"type":"object","properties":{"a":{"oneOf":[mv8_schema(""),{"type":"string"}]},"b":{"oneOf":[mv8_schema(""),{"type":"string"}]}},"required":["a","b"]})),
        tool("compose_relations",
            "Compose two relation rotors for multi-hop reasoning. Models transitive chains like A causes B triggers C.",
            serde_json::json!({"type":"object","properties":{"r1":mv8_schema("First rotor"),"r2":mv8_schema("Second rotor")},"required":["r1","r2"]})),
        tool("detect_contradiction",
            "Check if two concepts logically contradict. High bivector magnitude => contradiction.",
            serde_json::json!({"type":"object","properties":{"a":{"oneOf":[mv8_schema(""),{"type":"string"}]},"b":{"oneOf":[mv8_schema(""),{"type":"string"}]},"threshold":{"type":"number","default":0.5}},"required":["a","b"]})),
        tool("relation_type_info",
            "Get metadata about a semantic role type: description, Bagua trigram mapping, WuXing phase.",
            serde_json::json!({"type":"object","properties":{"role":{"type":"string","description":"Role name: generative, causal, constraining, etc."}},"required":["role"]})),
        tool("wuxing_query",
            "Query five-phase (WuXing) generating/controlling cycle. Wood->Fire->Earth->Metal->Water->Wood.",
            serde_json::json!({"type":"object","properties":{"phase":{"type":"string","enum":["wood","fire","earth","metal","water"]},"cycle":{"type":"string","enum":["generating","controlling"],"default":"generating"}},"required":["phase"]})),
        tool("context_apply",
            "Apply a context transformation (rotor) to shift semantic perspective of a concept.",
            serde_json::json!({"type":"object","properties":{"context":mv8_schema("Context rotor"),"entity":mv8_schema("Entity to transform")},"required":["context","entity"]})),
        tool("semantic_explore",
            "Explore a concept against related concepts in one call. Encodes main concept, returns role breakdown, and pairwise comparisons. Ideal for LLM concept exploration with minimal tokens.",
            serde_json::json!({"type":"object","properties":{"concept":{"type":"string","description":"The main concept to explore"},"related":{"type":"array","items":{"type":"string"},"description":"Related concepts to compare against"}},"required":["concept","related"]})),
        tool("llm_encode",
            "Register concept encoding from LLM-provided coefficients (bypasses hash encoder). Accepts concept name + 8 float coefficients array. Normalizes to unit norm if needed. Returns full role breakdown, descriptions, Bagua trigram, WuXing phase. This is the PRIMARY tool for LLM-assisted semantic encoding.",
            serde_json::json!({"type":"object","properties":{"name":{"type":"string","description":"Concept name"},"coefficients":{"type":"array","items":{"type":"number"},"minItems":8,"maxItems":8,"description":"8 semantic role weights [-1,1] assigned by the LLM using the Bagua encoding skill"}},"required":["name","coefficients"]})),
        tool("classify_hexagram",
            "Classify a pair of concepts using the full 64 Hexagram taxonomy. Returns hexagram number (1-64), Chinese name, pinyin, English interpretation, upper/lower trigram details with roles and WuXing phases. Provides richer classification than the 8-role classify_relation.",
            serde_json::json!({"type":"object","properties":{"a":{"oneOf":[mv8_schema(""),{"type":"string"}]},"b":{"oneOf":[mv8_schema(""),{"type":"string"}]}},"required":["a","b"]})),
        tool("bagua_dynamics",
            "Explore Bagua dynamics: trigram line-change transforms, WuXing generating/controlling cycles, complementary trigram pairs. Mode='explore' for single trigram details, 'trigram_transforms' for line-change analysis, 'wuxing_chain' for full 5-phase cycles.",
            serde_json::json!({"type":"object","properties":{"mode":{"type":"string","enum":["explore","trigram_transforms","wuxing_chain"],"description":"Dynamics mode to explore"},"trigram":{"type":"string","description":"Trigram name: kun, gen, kan, xun, zhen, li, dui, qian"},"phase":{"type":"string","enum":["wood","fire","earth","metal","water"]}},"required":[]})),
        tool("validate_encoding",
            "Validate that an encoding pair produces the expected semantic relationship. Given two multivectors and an expected role label, returns whether the classification matches. Use to iteratively improve encoding quality.",
            serde_json::json!({"type":"object","properties":{"a":{"oneOf":[mv8_schema(""),{"type":"string"}]},"b":{"oneOf":[mv8_schema(""),{"type":"string"}]},"expected_relation":{"type":"string","description":"Expected role: generative, causal, transmissive, constraining, influential, clarifying, balancing, receptive"}},"required":["a","b","expected_relation"]})),
        tool("encoding_benchmark",
            "Run 5 test-case encoding quality benchmark.",
            serde_json::json!({"type":"object","properties":{},"required":[]})),
        tool("store_open",
            "Open a concept store file (JSON-based). Creates if not exists. All store_* tools operate on this store.",
            serde_json::json!({"type":"object","properties":{"path":{"type":"string","description":"Path to JSON store file"}},"required":["path"]})),
        tool("store_concept",
            "Store a concept with its encoding (8 float coefficients) in the open store. Use after encoding via text_to_multivector or llm_encode.",
            serde_json::json!({"type":"object","properties":{"name":{"type":"string","description":"Concept name"},"text":{"type":"string","description":"Original concept text"},"encoding":{"type":"array","items":{"type":"number"},"minItems":8,"maxItems":8}},"required":["name","encoding"]})),
        tool("store_llm_concept",
            "Store a concept with LLM-provided coefficients. Auto-normalizes to unit norm and returns dominant role + description. Preferred over store_concept for LLM-assisted encoding.",
            serde_json::json!({"type":"object","properties":{"name":{"type":"string"},"text":{"type":"string","description":"Original concept text"},"coefficients":{"type":"array","items":{"type":"number"},"minItems":8,"maxItems":8}},"required":["name","coefficients"]})),
        tool("store_query_similar",
            "Query the open store for concepts similar to a query multivector. Returns top-k results with similarity scores.",
            serde_json::json!({"type":"object","properties":{"query":{"oneOf":[mv8_schema(""),{"type":"string"}]},"top_k":{"type":"integer","default":10}},"required":["query"]})),
        tool("store_get_concept",
            "Retrieve a stored concept by ID.",
            serde_json::json!({"type":"object","properties":{"id":{"type":"integer"}},"required":["id"]})),
        tool("store_list_concepts",
            "List all concepts in the open store with IDs and names.",
            serde_json::json!({"type":"object","properties":{},"required":[]})),
        tool("store_add_relation",
            "Classify and store the relationship between two stored concepts by ID.",
            serde_json::json!({"type":"object","properties":{"from_id":{"type":"integer"},"to_id":{"type":"integer"}},"required":["from_id","to_id"]})),
        tool("store_export",
            "Export the entire concept graph as JSON (nodes + edges).",
            serde_json::json!({"type":"object","properties":{},"required":[]})),
        tool("store_close",
            "Close the current store and save to disk.",
            serde_json::json!({"type":"object","properties":{},"required":[]})),
        tool("batch_process",
            "Execute multiple semantic operations in one call. Array of {name, arguments} objects.",
            serde_json::json!({"type":"object","properties":{"operations":{"type":"array","items":{"type":"object","properties":{"name":{"type":"string"},"arguments":{"type":"object"}}}}},"required":["operations"]})),
        tool("ideate_seed",
            "Encode a problem as a seed multivector for creative ideation. Use this to start the hexagram-based brainstorming process.",
            serde_json::json!({"type":"object","properties":{"name":{"type":"string","description":"Name of the problem/concept"},"coefficients":{"type":"array","description":"8 encoding coefficients from LLM","items":{"type":"number"},"minItems":8,"maxItems":8}},"required":["name","coefficients"]})),
        tool("ideate_step",
            "Step to a specific hexagram from a seed multivector. Returns a shifted perspective — a different way of looking at the problem.",
            serde_json::json!({"type":"object","properties":{"seed":mv8_schema("Seed multivector coefficients"),"hexagram":{"type":"integer","description":"Hexagram number (1-64)","minimum":1,"maximum":64}},"required":["seed","hexagram"]})),
        tool("ideate_explore",
            "Explore the top-N most divergent hexagram perspectives from a seed. Returns ranked list of shifted perspectives sorted by distance.",
            serde_json::json!({"type":"object","properties":{"seed":mv8_schema("Seed multivector coefficients"),"top_n":{"type":"integer","description":"Number of perspectives to return (1-64)","minimum":1,"maximum":64}},"required":["seed","top_n"]})),
        tool("ideate_blend",
            "Blend two concept multivectors via geometric product. Returns the emergent hexagram interpretation — what arises when these concepts combine.",
            serde_json::json!({"type":"object","properties":{"a":mv8_schema("First concept coefficients"),"b":mv8_schema("Second concept coefficients")},"required":["a","b"]})),
    ]
}

fn tool(name: &str, description: &str, input_schema: serde_json::Value) -> serde_json::Value {
    serde_json::json!({"name":name,"description":description,"inputSchema":input_schema})
}
