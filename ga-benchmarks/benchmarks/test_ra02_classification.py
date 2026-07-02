"""RA-02: Relation Classification vs. Ground Truth."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from benchmarks.mcp_client import McpClient
from benchmarks.harness import BenchmarkHarness
from benchmarks.test_data import SMOKE_TEST_CONCEPTS, RELATION_PAIRS

def run_ra02():
    print("=" * 60)
    print("RA-02: Relation Classification Accuracy")
    print("=" * 60)

    ac_results = []
    warnings = []

    with McpClient() as mcp:
        harness = BenchmarkHarness(mcp)
        concepts = SMOKE_TEST_CONCEPTS
        harness.store_and_encode(concepts)

        concept_map = {c["name"].lower(): c for c in concepts}

        correct = 0
        high_conf_correct = 0
        high_conf_total = 0
        total = len(RELATION_PAIRS)
        label_counts = {}

        per_result = []
        for pair in RELATION_PAIRS:
            a_name = pair["concept_a"]
            b_name = pair["concept_b"]
            expected = pair["expected_relation"]

            a = concept_map.get(a_name.lower())
            b = concept_map.get(b_name.lower())
            if not a or not b:
                warnings.append(f"Concept not found: {a_name} or {b_name}")
                continue

            result = harness.classify_pair(a["suggested_coefficients"], b["suggested_coefficients"])
            predicted = result.get("relation_type", "unknown")
            confidence = result.get("confidence", 0.0)

            is_correct = predicted.lower() == expected.lower()
            if is_correct:
                correct += 1
                if isinstance(confidence, (int, float)) and confidence > 0.7:
                    high_conf_correct += 1

            if isinstance(confidence, (int, float)) and confidence > 0.7:
                high_conf_total += 1

            label_counts[expected] = label_counts.get(expected, 0) + (1 if is_correct else 0)
            label_counts[f"_{predicted}_predicted"] = label_counts.get(f"_{predicted}_predicted", 0) + 1

            per_result.append({
                "pair": f"{a_name} -> {b_name}",
                "expected": expected,
                "predicted": predicted,
                "confidence": confidence,
                "correct": is_correct,
            })
            print(f"  {a_name} -> {b_name}: expected={expected}, predicted={predicted}, conf={confidence} {'OK' if is_correct else 'MISMATCH'}")

        accuracy = correct / max(1, total)

        # AC1: Accuracy >= 25% (2x random)
        ac1_passed = accuracy >= 0.25
        ac_results.append({
            "ac_id": "RA-02-AC1",
            "criterion": "Classification accuracy >= 25% (2x random baseline of 12.5%)",
            "actual": f"{accuracy*100:.1f}% ({correct}/{total})",
            "expected": ">= 25%",
            "passed": ac1_passed,
        })
        print(f"\n  RA-02-AC1: Accuracy {accuracy*100:.1f}% — {'PASS' if ac1_passed else 'FAIL'}")

        # AC2: High-confidence accuracy >= 60%
        if high_conf_total > 0:
            hc_acc = high_conf_correct / high_conf_total
            ac2_passed = hc_acc >= 0.60
        else:
            hc_acc = 0.0
            ac2_passed = False
            warnings.append("No high-confidence predictions for AC2")

        ac_results.append({
            "ac_id": "RA-02-AC2",
            "criterion": "High-confidence (>0.7) accuracy >= 60%",
            "actual": f"{hc_acc*100:.1f}% ({high_conf_correct}/{high_conf_total})",
            "expected": ">= 60%",
            "passed": ac2_passed,
        })
        print(f"  RA-02-AC2: High-conf accuracy {hc_acc*100:.1f}% — {'PASS' if ac2_passed else 'FAIL'}")

        # AC3: At least 3 of 8 relation types predicted
        unique_predicted = len([k for k in label_counts if k.endswith("_predicted")])
        ac3_passed = unique_predicted >= 3
        ac_results.append({
            "ac_id": "RA-02-AC3",
            "criterion": "At least 3 relationship types predicted",
            "actual": str(unique_predicted),
            "expected": ">= 3",
            "passed": ac3_passed,
        })
        print(f"  RA-02-AC3: {unique_predicted} unique types predicted — {'PASS' if ac3_passed else 'FAIL'}")

    all_pass = all(a["passed"] for a in ac_results)
    status = "PASS" if all_pass else "FAIL"
    print(f"\nRA-02 Status: {status} ({sum(1 for a in ac_results if a['passed'])}/{len(ac_results)} ACs)")

    return {
        "test_id": "RA-02",
        "name": "Relation Classification Accuracy",
        "status": status,
        "ac_results": ac_results,
        "warnings": warnings,
        "token_summary": "N/A (algebraic ops are 0-token)",
        "accuracy_summary": f"{accuracy*100:.1f}%",
        "key_metric": f"{accuracy*100:.1f}% accuracy ({correct}/{total})",
        "summary": (
            f"Classified {total} relation pairs. Accuracy: {accuracy*100:.1f}% ({correct}/{total}). "
            f"High-confidence (>0.7) accuracy: {hc_acc*100:.1f}% ({high_conf_correct}/{high_conf_total}). "
            f"Unique types predicted: {unique_predicted}."
        ),
    }

if __name__ == "__main__":
    import json
    result = run_ra02()
    print(json.dumps(result, indent=2, default=str))
