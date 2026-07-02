#!/usr/bin/env python3
"""
GA-Bagua LLM Integration Benchmark Suite — Run All Tests.

Usage:
    python run_all.py
    python run_all.py --test IT-01
    python run_all.py --test TE-01,RA-01
"""
import sys
import os
import json
import argparse

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from benchmarks.reporter import Reporter

TEST_CONFIG = {
    "IT-01": ("test_it01_smoke", "run_it01"),
    "TE-01": ("test_te01_efficiency", "run_te01"),
    "RA-01": ("test_ra01_retrieval", "run_ra01"),
    "RA-02": ("test_ra02_classification", "run_ra02"),
    "XD-01": ("test_xd01_crossdoc", "run_xd01"),
    "CB-01": ("test_cb01_competitive", "run_cb01"),
    "NIAH-01": ("test_niah", "run_niah"),
    "COMPETE": ("test_competitive", "run_competitive"),
}

def run_test(test_id: str) -> dict:
    config = TEST_CONFIG.get(test_id)
    if not config:
        return {"test_id": test_id, "status": "SKIP", "error": f"Unknown test: {test_id}"}

    module_name, func_name = config

    print(f"\n{'=' * 60}")
    print(f"  Running: {test_id}")
    print(f"{'=' * 60}")

    try:
        module = __import__(f"benchmarks.{module_name}", fromlist=[func_name])
        func = getattr(module, func_name)
        result = func()
        return result
    except Exception as e:
        import traceback
        traceback.print_exc()
        return {
            "test_id": test_id,
            "status": "ERROR",
            "error": str(e),
            "traceback": traceback.format_exc(),
        }

def main():
    parser = argparse.ArgumentParser(description="GA-Bagua LLM Integration Benchmarks")
    parser.add_argument("--test", "-t", help="Comma-separated test IDs to run (default: all)")
    parser.add_argument("--output", "-o", default="reports/benchmark_report.md",
                        help="Output report path (default: reports/benchmark_report.md)")
    parser.add_argument("--json-output", "-j", default="reports/benchmark_results.json",
                        help="JSON results path (default: reports/benchmark_results.json)")
    args = parser.parse_args()

    if args.test:
        test_ids = [t.strip() for t in args.test.split(",")]
    else:
        test_ids = list(TEST_CONFIG.keys())

    print("GA-Bagua LLM Integration Benchmark Suite")
    print(f"Tests to run: {', '.join(test_ids)}")
    print(f"Output: {args.output}")

    results = []
    all_pass = True

    for test_id in test_ids:
        result = run_test(test_id)
        results.append(result)

        status = result.get("status", "ERROR")
        if status == "FAIL" or status == "ERROR":
            all_pass = False

        acs = result.get("ac_results", [])
        passed = sum(1 for a in acs if a.get("passed"))
        total = len(acs)
        print(f"  {test_id}: {status} ({passed}/{total} ACs passed)")

    overall = "PASS" if all_pass else "PARTIAL" if any(r.get("status") == "PASS" for r in results) else "FAIL"
    total_ac_pass = sum(
        sum(1 for a in r.get("ac_results", []) if a.get("passed"))
        for r in results
    )
    total_ac = sum(len(r.get("ac_results", [])) for r in results)

    print(f"\n{'=' * 60}")
    print(f"  OVERALL: {overall} — {total_ac_pass}/{total_ac} ACs passed")
    print(f"{'=' * 60}")

    Reporter.generate_aggregate(results, os.path.join(os.path.dirname(__file__), args.output))

    json_path = os.path.join(os.path.dirname(__file__), args.json_output)
    os.makedirs(os.path.dirname(json_path), exist_ok=True)
    with open(json_path, "w") as f:
        json.dump({
            "overall_status": overall,
            "total_ac_passed": total_ac_pass,
            "total_ac_total": total_ac,
            "test_reports": results,
        }, f, indent=2, default=str)

    print(f"\nReport written to: {args.output}")
    print(f"JSON results: {args.json_output}")

    return 0 if all_pass else 1

if __name__ == "__main__":
    sys.exit(main())
