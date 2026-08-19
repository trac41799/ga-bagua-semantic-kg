"""Regression tests for the repaired state-diff measurement protocol."""

from bench import bench_statediff
from bench.common import (STATE_DIFF_MEASUREMENT_PROTOCOL,
                          measure_planted_delta_coverage)


def test_same_row_pair_counts():
    aspects = [
        {"aspect": "first", "before": "A", "after": "B"},
        {"aspect": "second", "before": "C", "after": "D"},
        {"aspect": "third", "before": "E", "after": "F"},
    ]
    planted = [("not the label", "A", "B"), ("another label", "C", "F")]

    assert measure_planted_delta_coverage(aspects, planted) == {
        "method": "same_record_value_pair_v1",
        "matched": [True, False],
        "covered": 1,
        "total": 2,
        "coverage": 0.5,
    }
    assert STATE_DIFF_MEASUREMENT_PROTOCOL == (
        "state_diff.measurement.same_record_value_pair_v1"
    )


def test_repaired_artifact_records_protocol_hashes_and_matches(tmp_path):
    source = [(1, "system", "before", "after", [("x", "1", "2")])]
    metadata = bench_statediff.artifact_metadata(source)
    report = tmp_path / "repaired.md"

    bench_statediff.write_repaired_report(
        str(report),
        [{
            "tid": 1,
            "domain": "system",
            "ok": True,
            "coverage": 0.5,
            "matched": [True, False],
            "defects": [],
            "source": "cache",
        }],
        metadata,
    )

    text = report.read_text(encoding="utf-8")
    assert metadata["method"] in text
    assert metadata["source_hash"] in text
    assert metadata["protocol_hash"] in text
    assert metadata["model_status"] in text
    assert "PENDING" in text
    assert "[true, false]" in text
    assert bench_statediff.CACHE.endswith(".cache_statediff_repaired.json")
    assert bench_statediff.REPORT.endswith("benchmark_statediff_repaired.md")
