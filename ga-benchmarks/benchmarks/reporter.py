"""
Report generation — outputs benchmark results as markdown.
"""
import os
import json
from datetime import datetime

class Reporter:
    @staticmethod
    def generate_aggregate(tests: list[dict], output_path: str):
        """Generate aggregate markdown report."""
        now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        total_pass = sum(1 for t in tests if t.get("ac_results"))
        total_ac_passed = sum(
            sum(1 for ac in t.get("ac_results", []) if ac.get("passed"))
            for t in tests
        )
        total_ac = sum(len(t.get("ac_results", [])) for t in tests)

        lines = []
        lines.append("# GA-Bagua LLM Integration Benchmark Report\n")
        lines.append(f"**Date:** {now}\n")
        lines.append(f"**Total Tests:** {len(tests)} | **ACs Passed:** {total_ac_passed}/{total_ac}\n")

        lines.append("\n## Scorecard\n")
        lines.append("| Test ID | Name | Status | Token Savings | Accuracy | Key Metric |\n")
        lines.append("|---------|------|--------|---------------|----------|------------|\n")

        for t in tests:
            name = t.get("name", t.get("test_id", "?"))
            status = t.get("status", "?")
            savings = t.get("token_summary", "N/A")
            acc = t.get("accuracy_summary", "N/A")
            key = t.get("key_metric", "")
            lines.append(f"| {t.get('test_id', '?')} | {name} | {status} | {savings} | {acc} | {key} |\n")

        lines.append("\n## Detailed Results\n\n")
        for t in tests:
            lines.append(f"### {t.get('test_id', '?')}: {t.get('name', '?')}\n")
            lines.append(f"**Status:** {t.get('status', '?')}\n")

            if t.get("summary"):
                lines.append(f"\n{t['summary']}\n")

            if t.get("ac_results"):
                lines.append("\n**Acceptance Criteria:**\n")
                lines.append("| AC | Pass |\n")
                lines.append("|----|------|\n")
                for ac in t["ac_results"]:
                    mark = "PASS" if ac.get("passed") else "FAIL"
                    lines.append(f"| {ac.get('ac_id', '?')}: {ac.get('criterion', '?')} | {mark} |\n")

            if t.get("warnings"):
                lines.append("\n**Warnings:**\n")
                for w in t["warnings"]:
                    lines.append(f"- {w}\n")

            lines.append("\n---\n\n")

        os.makedirs(os.path.dirname(output_path), exist_ok=True)
        with open(output_path, "w") as f:
            f.write("".join(lines))

        return "".join(lines)

    @staticmethod
    def save_json(data: dict, path: str):
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w") as f:
            json.dump(data, f, indent=2, default=str)
