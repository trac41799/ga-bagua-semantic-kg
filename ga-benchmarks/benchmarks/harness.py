"""
Benchmark harness — orchestrates test runs against GA-Bagua MCP server.
"""
import time
from typing import Optional
from .mcp_client import McpClient
from .evaluator import score_answer

class EncodingRecord:
    def __init__(self, name, coefficients, dominant_role, dominant_trigram,
                 wuxing_phase, sharpness, norm=1.0):
        self.concept_name = name
        self.coefficients = list(coefficients)
        self.dominant_role = dominant_role
        self.dominant_trigram = dominant_trigram
        self.wuxing_phase = wuxing_phase
        self.sharpness = sharpness
        self.norm = norm

class QueryResult:
    def __init__(self, query_id, query_text, expected_concepts, retrieved,
                 accuracy, tokens, latency_ms, ga_bagua_calls):
        self.query_id = query_id
        self.query_text = query_text
        self.expected_concepts = expected_concepts
        self.retrieved_concepts = retrieved
        self.accuracy_score = accuracy
        self.tokens_consumed = tokens
        self.latency_ms = latency_ms
        self.ga_bagua_calls = ga_bagua_calls

class SessionResult:
    def __init__(self, test_id, name, config, encoding_tokens, encoding_count,
                 queries, total_tokens, accuracy, sharpness_avg):
        self.test_id = test_id
        self.name = name
        self.configuration = config
        self.encoding_tokens = encoding_tokens
        self.encoding_concepts = encoding_count
        self.query_results = queries
        self.total_tokens = total_tokens
        self.accuracy = accuracy
        self.encoding_sharpness_avg = sharpness_avg

class TokenEfficiency:
    def __init__(self, ga_total, baseline_total, savings, ratio, break_even,
                 encoding_tokens, query_tokens, encoding_pct):
        self.total_ga_bagua = ga_total
        self.total_baseline = baseline_total
        self.savings = savings
        self.savings_ratio = ratio
        self.break_even = break_even
        self.encoding_tokens = encoding_tokens
        self.query_tokens = query_tokens
        self.encoding_percentage = encoding_pct

class RetrievalMetrics:
    def __init__(self, hits_at_1, precision_at_5, recall_at_10, mrr, num_retrieved, same_role_found):
        self.hits_at_1 = hits_at_1
        self.precision_at_5 = precision_at_5
        self.recall_at_10 = recall_at_10
        self.mrr = mrr
        self.num_retrieved = num_retrieved
        self.same_role_found = same_role_found

class BenchmarkHarness:
    """Runs GA-Bagua benchmarks via MCP."""

    def __init__(self, mcp: McpClient):
        self.mcp = mcp
        self.encoding_records: list[EncodingRecord] = []
        self.call_log = []

    def encode_concept(self, name: str, coefficients: list[float]) -> EncodingRecord:
        result, latency = self.mcp.llm_encode(name, coefficients)

        dominant_role = str(result.get("dominant_role", ""))
        dominant_trigram = str(result.get("bagua_trigram", ""))
        wuxing_phase = str(result.get("wuxing_phase", ""))

        sharpness = result.get("sharpness", 0.0)
        if not isinstance(sharpness, (int, float)) or sharpness == 0.0:
            coeffs_norm = result.get("normalized_coefficients", coefficients)
            if coeffs_norm and len(coeffs_norm) == 8:
                abs_vals = sorted([abs(c) for c in coeffs_norm], reverse=True)
                if len(abs_vals) >= 2 and abs_vals[0] > 0:
                    sharpness = (abs_vals[0] - abs_vals[1]) / abs_vals[0] if abs_vals[0] != 0 else 0.0

        record = EncodingRecord(
            name=name,
            coefficients=coefficients,
            dominant_role=dominant_role,
            dominant_trigram=dominant_trigram,
            wuxing_phase=wuxing_phase,
            sharpness=sharpness,
        )
        self.encoding_records.append(record)
        self.call_log.append(("llm_encode", latency * 1_000_000))
        return record

    def store_and_encode(self, annotations: list) -> list[EncodingRecord]:
        self.mcp.store_open("benchmark_store.json")
        records = []
        for a in annotations:
            record = self.encode_concept(a["name"], a["suggested_coefficients"])
            self.mcp.store_llm_concept(a["name"], a["suggested_coefficients"], a["description"])
            records.append(record)
        return records

    def open_store(self, path: str = "benchmark_store.json"):
        self.mcp.store_open(path)

    def query_similar(self, coeffs: list[float], top_k: int = 10) -> list[tuple[str, float]]:
        result, latency = self.mcp.store_query_similar(coeffs, top_k)
        self.call_log.append(("store_query_similar", latency * 1_000_000))
        concepts = result.get("results", result.get("similar_concepts", []))
        return [(c.get("name", ""), c.get("similarity", 0.0)) for c in concepts]

    def classify_pair(self, a: list[float], b: list[float]) -> dict:
        result, latency = self.mcp.classify_relation(a, b)
        self.call_log.append(("classify_relation", latency * 1_000_000))
        return result

    def detect_contradiction(self, a: list[float], b: list[float]) -> bool:
        result, latency = self.mcp.detect_contradiction(a, b)
        self.call_log.append(("detect_contradiction", latency * 1_000_000))
        return result.get("is_contradiction", False)

    def avg_sharpness(self) -> float:
        if not self.encoding_records:
            return 0.0
        return sum(r.sharpness for r in self.encoding_records) / len(self.encoding_records)

    def phase_distribution(self) -> dict[str, int]:
        dist = {}
        for r in self.encoding_records:
            dist[r.wuxing_phase] = dist.get(r.wuxing_phase, 0) + 1
        return dist

    def all_encodings(self) -> list[dict]:
        return [{"name": r.concept_name, "coefficients": r.coefficients,
                 "dominant_role": r.dominant_role, "wuxing_phase": r.wuxing_phase}
                for r in self.encoding_records]


def compute_token_efficiency(encoding_tokens: int, query_tokens: int,
                              baseline_per_query: int, num_queries: int) -> TokenEfficiency:
    total_ga = encoding_tokens + query_tokens
    total_baseline = baseline_per_query * num_queries
    savings = total_baseline - total_ga
    ratio = total_baseline / total_ga if total_ga > 0 else 0

    break_even = None
    if baseline_per_query > 0 and num_queries > 0:
        query_saving = baseline_per_query - (query_tokens // num_queries)
        if query_saving > 0:
            be = int((encoding_tokens / query_saving) + 0.5)
            if be == 0:
                be = 1
            break_even = be

    encoding_pct = (encoding_tokens / total_ga * 100) if total_ga > 0 else 0

    return TokenEfficiency(
        ga_total=total_ga, baseline_total=total_baseline,
        savings=savings, ratio=ratio, break_even=break_even,
        encoding_tokens=encoding_tokens, query_tokens=query_tokens,
        encoding_pct=encoding_pct,
    )


def compute_retrieval_metrics(retrieved: list[tuple[str, float]],
                               expected: list[str]) -> RetrievalMetrics:
    k = len(retrieved)
    expected_lower = [e.lower() for e in expected]

    hits_1 = 1.0 if k > 0 and retrieved[0][0].lower() in expected_lower else 0.0

    top5 = min(5, k)
    p5 = sum(1 for r in retrieved[:top5] if r[0].lower() in expected_lower) / max(1, top5)

    top10 = min(10, k)
    r10 = sum(1 for r in retrieved[:top10] if r[0].lower() in expected_lower) / max(1, len(expected))

    mrr = 0.0
    for i, (name, _) in enumerate(retrieved):
        if name.lower() in expected_lower:
            mrr = 1.0 / (i + 1)
            break

    return RetrievalMetrics(hits_at_1=hits_1, precision_at_5=p5,
                             recall_at_10=r10, mrr=mrr,
                             num_retrieved=k, same_role_found=0)
