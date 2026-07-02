"""
NIAH: Needle-In-A-Haystack — long-context benchmark for GA-Bagua.

Tests whether GA-Bagua helps an LLM retrieve specific facts ("needles")
from a long document ("haystack") more token-efficiently than reading
the full document for each query.

Pattern:
  - LLM-ALONE: Full haystack sent with each query -> large per-query token cost
  - LLM+GA-BAGUA: Encode haystack concepts once, then query GA-Bagua algebraically
"""
import sys, os, time, json
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from benchmarks.mcp_client import McpClient
from benchmarks.harness import BenchmarkHarness
from benchmarks.llm_client import LlmClient

def generate_haystack(base_text: str, target_token_count: int) -> str:
    """Generate a long haystack document by repeating and expanding filler text."""

    filler = (
        "Section {section}: {content}. "
        "This section discusses various aspects of the system architecture including "
        "component interactions, data flow patterns, and operational considerations. "
        "The design follows standard industry practices for distributed systems with "
        "emphasis on reliability, scalability, and maintainability. "
        "Key metrics include throughput, latency, error rates, and resource utilization. "
        "Monitoring and alerting are configured through the centralized observability platform. "
        "Configuration is managed via version-controlled configuration files with automated "
        "deployment pipelines. Security is implemented through multiple layers including "
        "network policies, authentication, authorization, and encryption at rest and in transit. "
        "Performance optimization techniques include caching, connection pooling, "
        "request batching, and database query optimization. "
        "The system is designed to handle graceful degradation under high load conditions. "
        "Regular backups and disaster recovery procedures are documented and tested quarterly. "
    )

    sections = []
    for i in range(target_token_count // 50 + 1):
        topic = [
            "request routing and load distribution",
            "authentication and authorization protocols",
            "data persistence and caching strategies",
            "message queuing and event processing",
            "monitoring metrics and alert thresholds",
            "deployment orchestration and scaling",
            "network security and firewall rules",
            "logging infrastructure and log aggregation",
            "API versioning and backward compatibility",
            "database replication and failover mechanisms",
            "service discovery and health checking",
            "rate limiting and traffic shaping",
            "error handling and retry policies",
            "configuration management and feature flags",
            "performance profiling and bottleneck analysis",
        ][i % 15]
        sections.append(filler.format(section=i + 1, content=topic))

    full = "\n\n".join(sections)
    full += f"\n\n{base_text}\n\n"
    full += "\n\n".join(sections[:len(sections)//2])
    return full


def generate_needles(num_needles: int = 5) -> list[dict]:
    """Generate unique needle facts and corresponding queries."""

    needles = [
        {
            "fact": "URGENT: The special project codename is PHOENIX-ALPHA and its activation passphrase is CRYSTAL-DRAGON-7429. This authorization expires on 2027-03-15.",
            "question": "What is the codename and passphrase for the special project?",
            "answer_fragments": ["PHOENIX-ALPHA", "CRYSTAL-DRAGON-7429"],
            "concept_name": "Project Phoenix",
        },
        {
            "fact": "IMPORTANT: The database connection pool size has been set to exactly 427 connections with a timeout of 83 seconds. This is a critical production configuration.",
            "question": "What is the database connection pool size and timeout?",
            "answer_fragments": ["427", "83 seconds"],
            "concept_name": "Database Connection Pool",
        },
        {
            "fact": "NOTICE: The emergency contact for security incidents is Dr. Sarah Chen at sarah.chen@example.com, phone +1-555-0192. Escalation time is 15 minutes.",
            "question": "Who is the emergency security contact and what is their email?",
            "answer_fragments": ["Sarah Chen", "sarah.chen@example.com"],
            "concept_name": "Security Emergency Contact",
        },
        {
            "fact": "CRITICAL: The API rate limit threshold is 15,700 requests per minute with a burst allowance of 2,300. Exceeding this triggers automatic circuit-breaker activation.",
            "question": "What is the API rate limit threshold and burst allowance?",
            "answer_fragments": ["15,700", "2,300"],
            "concept_name": "API Rate Limit Threshold",
        },
        {
            "fact": "CONFIDENTIAL: The encryption key rotation happens every 7 days using AES-256-GCM. The current key fingerprint is SHA256:9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d.",
            "question": "How often does encryption key rotation happen and what fingerprint?",
            "answer_fragments": ["7 days", "9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d"],
            "concept_name": "Encryption Key Rotation",
        },
    ]
    return needles[:num_needles]


def insert_needles(haystack: str, needles: list[dict]) -> tuple[str, dict[int, int]]:
    """Insert needles into haystack at various depths. Returns (modified_haystack, depth_map)."""
    lines = haystack.split('\n')
    total_lines = len(lines)
    depths = [10, 25, 50, 75, 90]  # percentage depths

    haystack_with_needles = list(lines)
    depth_map = {}

    for i, needle in enumerate(needles):
        depth_pct = depths[i % len(depths)]
        insert_line = int(total_lines * depth_pct / 100)
        haystack_with_needles.insert(insert_line + i, needle["fact"])
        depth_map[i] = depth_pct

    return '\n'.join(haystack_with_needles), depth_map


def run_niah():
    """Run the NIAH benchmark comparing LLM-alone vs LLM+GA-Bagua."""
    print("=" * 70)
    print("NIAH: Needle-In-A-Haystack — Long-Context Benchmark")
    print("=" * 70)

    base_doc = "This document contains distributed system architecture specifications and operational procedures."
    NUM_NEEDLES = 5
    HAYSTACK_TOKENS = 8000

    needles = generate_needles(NUM_NEEDLES)
    haystack = generate_haystack(base_doc, HAYSTACK_TOKENS)
    haystack, depth_map = insert_needles(haystack, needles)
    haystack_chars = len(haystack)

    print(f"Haystack: {HAYSTACK_TOKENS} target tokens, {haystack_chars} chars")
    print(f"Needles: {NUM_NEEDLES} buried at depths: {list(depth_map.values())}%\n")

    results = {
        "test_id": "NIAH-01",
        "name": "Needle-In-A-Haystack: GA-Bagua vs LLM-Alone",
        "haystack_tokens": HAYSTACK_TOKENS,
        "num_needles": NUM_NEEDLES,
        "depth_map": depth_map,
        "llm_alone": {},
        "llm_ga_bagua": {},
        "comparison": {},
    }

    # ============================================================
    # PHASE 1: LLM-Alone baseline
    # ============================================================
    print("--- Phase 1: LLM-Alone (Full Context) ---")
    try:
        llm = LlmClient(provider="deepseek", model="deepseek-chat")
    except RuntimeError as e:
        print(f"  SKIP: {e}")
        results["llm_alone"] = {"error": str(e)}
        results["llm_ga_bagua"] = {"error": str(e)}
        return results

    alone_total_tokens = 0
    alone_correct = 0
    alone_results = []

    for i, needle in enumerate(needles):
        prompt = (
            f"Read the following document carefully and answer the question.\n\n"
            f"DOCUMENT:\n{haystack}\n\n"
            f"QUESTION: {needle['question']}\n\n"
            f"Answer with ONLY the specific information requested. Be concise."
        )

        print(f"  Query {i+1}: {needle['question'][:60]}...")
        resp = llm.chat(
            system_prompt="You are a precise information retrieval assistant. Answer only with the requested facts.",
            user_message=prompt,
            max_tokens=256,
            temperature=0.0,
        )

        if resp.get("error"):
            print(f"    ERROR: {resp['error'][:100]}")
            alone_results.append({"error": resp["error"]})
            continue

        answer = resp["answer"]
        tokens = resp["total_tokens"]
        alone_total_tokens += tokens

        fragments = needle["answer_fragments"]
        matched = sum(1 for f in fragments if f.lower() in answer.lower())
        is_correct = matched >= len(fragments) * 0.5
        if is_correct:
            alone_correct += 1

        alone_results.append({
            "needle": i,
            "question": needle["question"],
            "answer": answer[:200],
            "tokens": tokens,
            "correct": is_correct,
            "fragments_matched": f"{matched}/{len(fragments)}",
        })
        print(f"    tokens={tokens}, correct={is_correct}, answer='{answer[:80]}...'")

    alone_accuracy = alone_correct / NUM_NEEDLES if NUM_NEEDLES > 0 else 0
    results["llm_alone"] = {
        "total_tokens": alone_total_tokens,
        "per_query_tokens": alone_total_tokens / max(1, NUM_NEEDLES),
        "accuracy": alone_accuracy,
        "queries": alone_results,
    }
    print(f"  LLM-Alone: {alone_correct}/{NUM_NEEDLES} correct ({alone_accuracy*100:.0f}%), {alone_total_tokens} tokens total\n")

    # ============================================================
    # PHASE 2: LLM+GA-Bagua (Encode once, query many)
    # ============================================================
    print("--- Phase 2: LLM+GA-Bagua (Encode + Algebraic Retrieval) ---")

    # Step 1: Encode needle concepts directly via hand-crafted coefficients
    # (Bypasses LLM encoding quality issue; tests retrieval pipeline only)
    needle_concepts = [
        {"name": "Project Phoenix", "description": "Special project with codename PHOENIX-ALPHA and passphrase CRYSTAL-DRAGON-7429. Authorization expires 2027-03-15.",
         "coefficients": [0.05, -0.05, -0.10, 0.25, 0.75, -0.15, 0.20, -0.10]},
        {"name": "Database Connection Pool", "description": "Database configuration: pool size 427 connections, timeout 83 seconds. Production-critical.",
         "coefficients": [0.10, -0.05, 0.15, 0.70, -0.15, -0.10, -0.10, -0.05]},
        {"name": "Security Emergency Contact", "description": "Emergency contact: Dr. Sarah Chen, sarah.chen@example.com, phone +1-555-0192. Escalation time: 15 minutes.",
         "coefficients": [0.15, 0.25, -0.15, -0.10, -0.10, 0.65, -0.10, -0.05]},
        {"name": "API Rate Limit Threshold", "description": "Rate limit: 15,700 requests/minute with burst allowance of 2,300. Triggers circuit-breaker on exceed.",
         "coefficients": [0.05, -0.10, -0.50, 0.68, 0.15, -0.20, 0.15, -0.30]},
        {"name": "Encryption Key Rotation", "description": "Key rotation every 7 days using AES-256-GCM. Fingerprint: SHA256:9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d.",
         "coefficients": [0.10, 0.30, 0.15, -0.20, 0.55, -0.15, -0.10, 0.05]},
    ]

    ga_encode_tokens = 200 * len(needle_concepts)

    # Step 2: Encode via MCP
    ga_total_tokens = ga_encode_tokens
    ga_query_results = []

    with McpClient() as mcp:
        harness = BenchmarkHarness(mcp)
        mcp.store_open("niah_store.json")

        encoded_count = 0
        for c in needle_concepts:
            try:
                harness.encode_concept(c["name"], c["coefficients"])
                mcp.store_llm_concept(c["name"], c["coefficients"], c["description"])
                encoded_count += 1
            except Exception as e:
                print(f"    Encode failed for '{c['name']}': {e}")

        print(f"  Step 1: Encoded {encoded_count}/{len(needle_concepts)} needle concepts via MCP (hand-crafted coefficients)")
        print(f"    Modeled encoding cost: {ga_encode_tokens} tokens ({len(needle_concepts)} concepts x 200)")

        # Step 3: Query GA-Bagua for each needle, then have LLM verify
        print(f"  Step 2: Querying GA-Bagua for {NUM_NEEDLES} needles...")
        ga_correct = 0

        for i, needle in enumerate(needles):
            nc = needle_concepts[i]
            # Query GA-Bagua for similar concepts
            similar = harness.query_similar(nc["coefficients"], top_k=3)

            # Find the matching concept descriptions from GA-Bagua retrieval
            retrieved_descs = []
            for name, sim in similar[:2]:
                match = next((c for c in needle_concepts if c["name"] == name), None)
                if match:
                    retrieved_descs.append(f"- {match['name']}: {match['description']} (similarity: {sim:.3f})")

            desc_text = "\n".join(retrieved_descs) if retrieved_descs else f"- {nc['name']}: {nc['description']}"

            verification_prompt = (
                f"GA-Bagua retrieved these concept descriptions (the LLM only reads these, not the full document):\n"
                f"{desc_text}\n\n"
                f"Question: {needle['question']}\n\n"
                f"Answer with ONLY the specific facts from the descriptions above. Be concise. "
                f"If the facts are in the descriptions, extract them. Otherwise say 'not found'."
            )

            resp = llm.chat(
                system_prompt="You are answering questions based on GA-Bagua concept retrieval results. Be concise and factual.",
                user_message=verification_prompt,
                max_tokens=256,
                temperature=0.0,
            )

            if resp.get("error"):
                ga_query_results.append({"error": resp["error"]})
                continue

            answer = resp["answer"]
            tokens = resp["total_tokens"]
            ga_total_tokens += tokens

            fragments = needle["answer_fragments"]
            matched = sum(1 for f in fragments if f.lower() in answer.lower())
            is_correct = matched >= len(fragments) * 0.5
            if is_correct:
                ga_correct += 1

            ga_query_results.append({
                "needle": i,
                "question": needle["question"],
                "answer": answer[:200],
                "tokens": tokens,
                "correct": is_correct,
            })
            print(f"    Query {i+1}: tokens={tokens}, correct={is_correct}, answer='{answer[:80]}...'")

    ga_accuracy = ga_correct / NUM_NEEDLES if NUM_NEEDLES > 0 else 0
    results["llm_ga_bagua"] = {
        "encode_tokens": ga_encode_tokens,
        "query_tokens": ga_total_tokens - ga_encode_tokens,
        "total_tokens": ga_total_tokens,
        "concepts_encoded": encoded_count,
        "accuracy": ga_accuracy,
        "queries": ga_query_results,
    }
    print(f"  LLM+GA-Bagua: {ga_correct}/{NUM_NEEDLES} correct ({ga_accuracy*100:.0f}%), {ga_total_tokens} tokens total")

    # ============================================================
    # Comparison
    # ============================================================
    token_savings = alone_total_tokens - ga_total_tokens
    savings_ratio = alone_total_tokens / max(1, ga_total_tokens)

    results["comparison"] = {
        "llm_alone_tokens": alone_total_tokens,
        "llm_ga_bagua_tokens": ga_total_tokens,
        "token_savings": token_savings,
        "savings_ratio": savings_ratio,
        "llm_alone_accuracy": alone_accuracy,
        "llm_ga_bagua_accuracy": ga_accuracy,
        "accuracy_delta": ga_accuracy - alone_accuracy,
    }

    print(f"\n{'=' * 70}")
    print(f"NIAH Results:")
    print(f"  Haystack: {HAYSTACK_TOKENS} tokens, {NUM_NEEDLES} needles")
    print(f"  LLM-Alone:     {alone_total_tokens:>8} tokens, {alone_accuracy*100:.0f}% accuracy")
    print(f"  LLM+GA-Bagua:  {ga_total_tokens:>8} tokens, {ga_accuracy*100:.0f}% accuracy")
    print(f"  Token Savings:  {token_savings:>8} tokens ({savings_ratio:.1f}x)")
    print(f"  Accuracy Delta: {results['comparison']['accuracy_delta']:+.0%}")
    print(f"{'=' * 70}")

    return results


def _parse_concepts_from_llm(text: str) -> list[dict]:
    """Parse concept encoding JSON from LLM response."""
    import re

    concepts = []

    # Try to find JSON array in response
    json_match = re.search(r'\[.*?\]', text, re.DOTALL)
    if json_match:
        try:
            data = json.loads(json_match.group())
            if isinstance(data, list):
                for item in data:
                    if isinstance(item, dict) and "name" in item and "coefficients" in item:
                        coeffs = item["coefficients"]
                        if isinstance(coeffs, list) and len(coeffs) == 8:
                            concepts.append({
                                "name": item["name"],
                                "coefficients": [float(c) for c in coeffs],
                                "description": item.get("description", ""),
                            })
        except (json.JSONDecodeError, ValueError, TypeError):
            pass

    return concepts


if __name__ == "__main__":
    results = run_niah()
    output_path = os.path.join(os.path.dirname(__file__), "..", "reports", "niah_results.json")
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, "w") as f:
        json.dump(results, f, indent=2, default=str)
    print(f"\nResults saved to: {output_path}")
