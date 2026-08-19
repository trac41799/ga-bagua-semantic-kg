"""T-04.3 Runner: five reports, verdict rows, SimulatedLLM exit 0, ledger."""

import csv
import os

import run_all
from run_all import REPORT_NAMES, main


def run_twice(tmp_path):
    out = tmp_path / "out"
    cache = tmp_path / "cache"
    code1 = main(["--sim", "--out-dir", str(out), "--cache-dir", str(cache)])
    assert code1 == 0
    out2 = tmp_path / "out2"
    code2 = main(["--sim", "--out-dir", str(out2), "--cache-dir", str(cache)])
    assert code2 == 0
    return out, out2, cache


def test_0431_all_five_reports_render(tmp_path):
    out, _out2, _cache = run_twice(tmp_path)
    for name in REPORT_NAMES:
        assert (out / name).exists(), name
        assert (out / name).stat().st_size > 0


def test_0432_verdict_rows_present(tmp_path):
    out, _out2, _cache = run_twice(tmp_path)
    text = (out / "gate_summary.md").read_text(encoding="utf-8")
    for claim in ["Tag quality", "Tag stability", "Filtering precision",
                  "Non-interference"]:
        assert claim in text
    assert "Verdict" in text
    for token in ("PASS", "FAIL"):
        assert token in text
    assert "Overall gate" in text


def test_0433_simulated_llm_runner_exit_zero(tmp_path):
    out = tmp_path / "out"
    cache = tmp_path / "cache"
    assert main(["--sim", "--out-dir", str(out), "--cache-dir", str(cache)]) == 0


def test_0434_ledger_rows_appended_and_cached_rerun_identical(tmp_path):
    out, out2, cache = run_twice(tmp_path)
    ledger = out / "claims_ledger.csv"
    ledger2 = out2 / "claims_ledger.csv"
    rows1 = list(csv.reader(open(ledger, encoding="utf-8")))
    rows2 = list(csv.reader(open(ledger2, encoding="utf-8")))
    assert rows1[0][0] == "run_timestamp"
    assert len(rows1) == 1 + 5
    assert len(rows2) == 1 + 5
    for name in REPORT_NAMES:
        assert (out / name).read_bytes() == (out2 / name).read_bytes(), name
    cache_files = [f for f in os.listdir(cache) if f.endswith(".json")]
    assert len(cache_files) >= 30 + 120


def test_0434_ledger_same_file_appends(tmp_path):
    out = tmp_path / "out"
    cache = tmp_path / "cache"
    main(["--sim", "--out-dir", str(out), "--cache-dir", str(cache)])
    main(["--sim", "--out-dir", str(out), "--cache-dir", str(cache)])
    rows = list(csv.reader(open(out / "claims_ledger.csv", encoding="utf-8")))
    assert len(rows) == 1 + 10


def test_offline_mode_fails_on_cache_miss(tmp_path):
    out = tmp_path / "out"
    cache = tmp_path / "cache"
    assert main(["--offline", "--out-dir", str(out),
                 "--cache-dir", str(cache)]) == 1


def test_offline_mode_succeeds_with_warm_cache(tmp_path):
    out = tmp_path / "out"
    cache = tmp_path / "cache"
    main(["--sim", "--out-dir", str(out), "--cache-dir", str(cache)])
    out2 = tmp_path / "out2"
    assert main(["--offline", "--out-dir", str(out2),
                 "--cache-dir", str(cache)]) == 0
    for name in REPORT_NAMES:
        assert (out2 / name).exists()
