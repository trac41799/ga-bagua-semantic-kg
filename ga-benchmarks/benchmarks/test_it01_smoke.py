"""IT-01: End-to-End MCP Pipeline Smoke Test."""
import sys
import os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from benchmarks.mcp_client import McpClient
from benchmarks.harness import BenchmarkHarness
from benchmarks.test_data import SMOKE_TEST_CONCEPTS

def run_it01():
    """Verify the full pipeline: encode concepts -> store -> query -> verify."""
    print("=" * 60)
    print("IT-01: End-to-End MCP Pipeline Smoke Test")
    print("=" * 60)

    ac_results = []
    warnings = []

    with McpClient() as mcp:
        harness = BenchmarkHarness(mcp)
        concepts = SMOKE_TEST_CONCEPTS
        records = harness.store_and_encode(concepts)

        # AC1: LLM successfully encodes at least 4 concepts
        ac1_passed = len(records) >= 4
        ac_results.append({
            "ac_id": "IT-01-AC1",
            "criterion": f"Encode at least 4 concepts",
            "actual": str(len(records)),
            "expected": ">= 4",
            "passed": ac1_passed,
        })
        print(f"  IT-01-AC1: Encoded {len(records)} concepts — {'PASS' if ac1_passed else 'FAIL'}")

        # AC2: store_query_similar works
        try:
            results = harness.query_similar(concepts[0]["suggested_coefficients"], top_k=5)
            ac2_passed = len(results) > 0
        except Exception as e:
            results = []
            ac2_passed = False
            warnings.append(f"store_query_similar failed: {e}")

        ac_results.append({
            "ac_id": "IT-01-AC2",
            "criterion": "store_query_similar returns results",
            "actual": f"{len(results)} results",
            "expected": "> 0",
            "passed": ac2_passed,
        })
        print(f"  IT-01-AC2: Query returned {len(results)} results — {'PASS' if ac2_passed else 'FAIL'}")

        # AC3: Operations complete within reasonable time
        max_latency_us = max(lat for _, lat in harness.call_log) if harness.call_log else 0
        ac3_passed = max_latency_us < 5_000_000  # 5 seconds
        ac_results.append({
            "ac_id": "IT-01-AC3",
            "criterion": "All GA-Bagua calls within 5s",
            "actual": f"{max_latency_us / 1000:.1f}ms max",
            "expected": "< 5000ms",
            "passed": ac3_passed,
        })
        print(f"  IT-01-AC3: Max latency {max_latency_us / 1000:.1f}ms — {'PASS' if ac3_passed else 'FAIL'}")

        # AC4: classify_relation works
        try:
            rel = harness.classify_pair(
                concepts[0]["suggested_coefficients"],
                concepts[1]["suggested_coefficients"],
            )
            relation_type = rel.get("relation_type", "unknown")
            ac4_passed = len(relation_type) > 0 and relation_type != "unknown"
        except Exception as e:
            relation_type = "error"
            ac4_passed = False
            warnings.append(f"classify_relation failed: {e}")

        ac_results.append({
            "ac_id": "IT-01-AC4",
            "criterion": "classify_relation returns valid type",
            "actual": f"'{relation_type}'",
            "expected": "non-empty, non-'unknown'",
            "passed": ac4_passed,
        })
        print(f"  IT-01-AC4: Relation type '{relation_type}' — {'PASS' if ac4_passed else 'FAIL'}")

        # AC5: Encoding produces valid sharpness
        avg_sharp = harness.avg_sharpness()
        ac5_passed = avg_sharp > 0.15
        ac_results.append({
            "ac_id": "IT-01-AC5",
            "criterion": "Average encoding sharpness > 0.15",
            "actual": f"{avg_sharp:.4f}",
            "expected": "> 0.15",
            "passed": ac5_passed,
        })
        print(f"  IT-01-AC5: Avg sharpness {avg_sharp:.4f} — {'PASS' if ac5_passed else 'FAIL'}")

        # AC6: Phase distribution covers multiple phases
        phase_dist = harness.phase_distribution()
        ac6_passed = len(phase_dist) >= 3
        ac_results.append({
            "ac_id": "IT-01-AC6",
            "criterion": "Concepts span at least 3 WuXing phases",
            "actual": f"{len(phase_dist)} phases: {phase_dist}",
            "expected": ">= 3",
            "passed": ac6_passed,
        })
        print(f"  IT-01-AC6: {len(phase_dist)} phases — {'PASS' if ac6_passed else 'FAIL'}")
        print(f"  Phase distribution: {dict(phase_dist)}")

        # AC7: detect_contradiction works
        try:
            is_contra = harness.detect_contradiction(
                concepts[0]["suggested_coefficients"],
                concepts[1]["suggested_coefficients"],
            )
            ac7_passed = isinstance(is_contra, bool)
        except Exception as e:
            is_contra = "error"
            ac7_passed = False
            warnings.append(f"detect_contradiction failed: {e}")

        ac_results.append({
            "ac_id": "IT-01-AC7",
            "criterion": "detect_contradiction returns boolean",
            "actual": str(is_contra),
            "expected": "True or False",
            "passed": ac7_passed,
        })
        print(f"  IT-01-AC7: Contradiction: {is_contra} — {'PASS' if ac7_passed else 'FAIL'}")

    all_pass = all(a["passed"] for a in ac_results)
    status = "PASS" if all_pass else "FAIL"
    print(f"\nIT-01 Status: {status} ({sum(1 for a in ac_results if a['passed'])}/{len(ac_results)} ACs)")

    return {
        "test_id": "IT-01",
        "name": "End-to-End MCP Pipeline Smoke Test",
        "status": status,
        "ac_results": ac_results,
        "warnings": warnings,
        "token_summary": "N/A (smoke test)",
        "accuracy_summary": "N/A (smoke test)",
        "key_metric": f"{len(records)} concepts encoded, {len(phase_dist)} WuXing phases",
        "summary": f"Encoded {len(records)} concepts across {len(phase_dist)} WuXing phases. "
                   f"Avg sharpness: {harness.avg_sharpness():.4f}. "
                   f"GA-Bagua latency: {max_latency_us/1000:.1f}ms max.",
    }

if __name__ == "__main__":
    result = run_it01()
    import json
    print(json.dumps(result, indent=2, default=str))
