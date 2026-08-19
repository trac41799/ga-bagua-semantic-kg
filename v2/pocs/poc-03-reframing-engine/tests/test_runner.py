"""T-03.4 Runner: arm comparison, verdict, SimulatedLLM run, ledger + determinism (AC-03.6, AC-03.7)."""

from collections import Counter

import run_all
import statements


def _run(run_args):
    return run_all.run(run_args)


def test_03_4_1_arm_comparison_renders(run_args):
    """T-03.4.1: Arm A vs Arm B rows with diversity + coherence."""
    assert _run(run_args) == 0
    import pathlib
    md = pathlib.Path(run_args.output_dir) / "arm_comparison.md"
    text = md.read_text(encoding="utf-8")
    assert "| A |" in text and "| B |" in text
    assert "diversity" in text.lower() and "coherence" in text.lower()
    assert "Per domain" in text
    for domain in statements.DOMAINS:
        assert domain in text


def test_03_4_2_verdict_rows(run_args):
    """T-03.4.2: verdict rows -- proxy claim PASS/FAIL vs +0.15 and >= 3.5."""
    assert _run(run_args) == 0
    import pathlib
    text = (pathlib.Path(run_args.output_dir) / "verdict.md").read_text(encoding="utf-8")
    assert "+0.15" in text and "3.5" in text
    assert "PASS" in text or "FAIL" in text
    assert "mean diversity" in text and "mean coherence" in text


def test_03_4_3_sim_runner_exit_zero_reports_exist(run_args):
    """T-03.4.3: SimulatedLLM runner exits 0 and writes all reports."""
    assert _run(run_args) == 0
    import pathlib
    out = pathlib.Path(run_args.output_dir)
    assert (out / "arm_comparison.md").exists()
    assert (out / "verdict.md").exists()
    assert (out / "claims_ledger.csv").exists()


def test_03_4_4_ledger_appended_and_runs_deterministic(run_args):
    """T-03.4.4: ledger rows appended per run; re-runs render byte-identical reports."""
    import pathlib
    out = pathlib.Path(run_args.output_dir)

    run_args.run_id = "RUN-1"
    assert _run(run_args) == 0
    comp1 = (out / "arm_comparison.md").read_text(encoding="utf-8")
    verd1 = (out / "verdict.md").read_text(encoding="utf-8")
    lines1 = (out / "claims_ledger.csv").read_text(encoding="utf-8").splitlines()

    # identical re-run (same run id) -> identical reports, ledger appended
    assert _run(run_args) == 0
    comp2 = (out / "arm_comparison.md").read_text(encoding="utf-8")
    verd2 = (out / "verdict.md").read_text(encoding="utf-8")
    lines2 = (out / "claims_ledger.csv").read_text(encoding="utf-8").splitlines()
    assert comp1 == comp2
    assert verd1 == verd2
    assert len(lines2) == len(lines1) + len(lines1) - 1  # header + 2x data rows

    # distinct run id -> rows appended with both ids present
    run_args.run_id = "RUN-2"
    assert _run(run_args) == 0
    lines3 = (out / "claims_ledger.csv").read_text(encoding="utf-8").splitlines()
    assert len(lines3) == len(lines2) + len(lines1) - 1
    assert "RUN-1" in lines3[1] and "RUN-2" in lines3[-1]


def test_03_4_ledger_has_40_rows_per_run(run_args):
    """One ledger row per (statement, arm): 20 statements x 2 arms."""
    run_args.run_id = "LEDGER-CHECK"
    assert _run(run_args) == 0
    import csv
    import pathlib
    path = pathlib.Path(run_args.output_dir) / "claims_ledger.csv"
    with open(path, newline="", encoding="utf-8") as fh:
        rows = list(csv.DictReader(fh))
    assert len(rows) == 40
    arms = Counter(r["arm"] for r in rows)
    assert arms == {"A": 20, "B": 20}


def test_03_4_offline_without_cache_fails(run_args):
    """Offline mode with an empty cache must abort (exit 1), not fabricate data."""
    run_args.sim = False
    run_args.offline = True
    assert _run(run_args) == 1
    import pathlib
    assert not (pathlib.Path(run_args.output_dir) / "verdict.md").exists()


def test_03_4_freeze_mismatch_refuses_to_run(run_args, monkeypatch):
    """A tampered statement set must refuse to run (exit 2)."""
    monkeypatch.setattr(statements, "verify_frozen", lambda: False)
    assert _run(run_args) == 2
