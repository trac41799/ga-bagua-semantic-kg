import re
from pathlib import Path

import pytest

from v2.qa.promotion_report import load_claims, render_report, validate_claims


def test_failed_claim_is_not_product_pass():
    claim = {
        "claim_id": "TEST-FAIL",
        "name": "failed test claim",
        "claim_status": "FAIL",
        "execution_status": "PASS",
        "evidence_status": "PASS",
        "replication_status": "NOT_RUN",
        "product_status": "PASS",
        "primary_metric": "metric",
        "bar": "must pass",
        "observed": "failed",
        "artifact": "v2/qa/tests/test_promotion_report.py",
        "protocol_id": "test-v1",
        "model_scope": "test",
        "next_gate": "do not promote",
    }

    with pytest.raises(ValueError, match="product_status"):
        validate_claims([claim])


def test_explicit_matrix_covers_pocs_paths_and_yarrow():
    claims = load_claims()
    by_id = {claim["claim_id"]: claim for claim in claims}

    assert {f"POC-{number:02d}" for number in range(1, 16)} <= by_id.keys()
    assert {"A", "B", "C", "D", "YARROW"} <= by_id.keys()

    assert by_id["POC-08"]["claim_status"] == "MODEL_DEPENDENT"
    assert by_id["POC-15-R05"]["claim_status"] == "PASS"
    assert by_id["POC-15-R05"]["replication_status"] == "PASS"
    assert by_id["POC-15-R08"]["claim_status"] == "FAIL"
    assert by_id["POC-15-R08"]["replication_status"] == "FAIL"
    assert by_id["POC-15-R08"]["model_scope"] == "model-dependent"
    assert by_id["YARROW"]["product_status"] == "PENDING"


def test_deterministic_products_and_suite_have_mechanical_evidence_without_product_promotion():
    claims = load_claims()
    by_id = {claim["claim_id"]: claim for claim in claims}

    for claim_id in ("POC-11", "POC-12", "POC-14"):
        row = by_id[claim_id]
        assert row["claim_status"] == "PASS"
        assert row["execution_status"] == "PASS"
        assert row["evidence_status"] == "PASS"
        assert row["product_status"] == "PENDING"
        assert "implementation and execution evidence not present" not in row["observed"]
        assert row["related_artifacts"]
        assert "pytest" in row["observed"]
        assert "build" in row["observed"]

    suite = by_id["ICHING-TOOLS"]
    assert suite["claim_status"] == "PASS"
    assert suite["execution_status"] == "PASS"
    assert suite["evidence_status"] == "PASS"
    assert suite["product_status"] == "PENDING"
    assert all(term in suite["observed"] for term in ("package", "MCP", "SDK"))
    assert "host/product-fit" in suite["next_gate"]


def test_poc10_waits_for_clean_no_leakage_rerun_but_keeps_calibration_finding():
    poc10 = {claim["claim_id"]: claim for claim in load_claims()}["POC-10"]

    assert poc10["claim_status"] == "INCONCLUSIVE"
    assert poc10["product_status"] == "PENDING"
    assert poc10["replication_status"] == "PENDING"
    assert "calibration" in poc10["observed"].lower()
    assert "no-leakage" in poc10["next_gate"]


def test_missing_artifacts_fail_report_generation():
    claim = {
        "claim_id": "TEST-MISSING",
        "name": "missing artifact claim",
        "claim_status": "PENDING",
        "execution_status": "NOT_RUN",
        "evidence_status": "PENDING",
        "replication_status": "NOT_RUN",
        "product_status": "PENDING",
        "primary_metric": "metric",
        "bar": "pending",
        "observed": "pending",
        "artifact": "v2/qa/does-not-exist.md",
        "protocol_id": "test-v1",
        "model_scope": "test",
        "next_gate": "collect evidence",
    }

    with pytest.raises(ValueError, match="missing artifact"):
        render_report([claim], repo_root=Path.cwd())


def test_status_transitions_cannot_turn_mechanics_into_claim_or_product_pass():
    base = {
        "claim_id": "TEST-TRANSITION",
        "name": "transition claim",
        "claim_status": "PASS",
        "execution_status": "NOT_RUN",
        "evidence_status": "PASS",
        "replication_status": "NOT_RUN",
        "product_status": "PENDING",
        "primary_metric": "metric",
        "bar": "pass",
        "observed": "pass",
        "artifact": "v2/qa/tests/test_promotion_report.py",
        "protocol_id": "test-v1",
        "model_scope": "test",
        "next_gate": "none",
    }

    with pytest.raises(ValueError, match="execution_status"):
        validate_claims([base])

    base["execution_status"] = "PASS"
    base["claim_status"] = "INCONCLUSIVE"
    base["product_status"] = "PROMOTE_BETA"
    with pytest.raises(ValueError, match="product_status"):
        validate_claims([base])

    base["claim_status"] = "PASS"
    with pytest.raises(ValueError, match="replication_status"):
        validate_claims([base])


def test_report_is_deterministic_and_separates_execution_from_claim_status():
    claims = load_claims()
    first = render_report(claims, repo_root=Path.cwd())
    second = render_report(claims, repo_root=Path.cwd())

    assert first == second
    assert "## Execution-Gate Summary" in first
    assert "## Replication Status Summary" in first
    assert "## Product Status Summary" in first
    assert "| POC-01 | Combinatorial reasoning scaffold | FAIL | PASS | PASS | NOT_RUN | FAIL |" in first
    assert "R-08 reframe replication" in first
    assert "MODEL_DEPENDENT" in first
    assert "Yarrow product validation and adoption" in first
    assert "external-user records: 0" in first
    assert "PRODUCT_VALIDATED" not in first
    assert "sha256" in first
    assert "Claim source `v2/qa/claims.json`:" in first


def test_active_portfolio_docs_do_not_promote_unqualified_poc08():
    root = Path(__file__).resolve().parents[3]
    documents = [
        root / "v2" / "THESIS.md",
        root / "v2" / "README.md",
        root / "v2" / "applications" / "README.md",
        root / "v2" / "qa" / "gate-report.md",
        root / "v2" / "products" / "PRODUCTION_ASSESSMENT.md",
        root / "v2" / "products" / "iching-tools" / "README.md",
        root / "v2" / "products" / "iching-tools" / "skills" / "bagua-reframe" / "SKILL.md",
    ]
    active_text = "\n".join(document.read_text(encoding="utf-8") for document in documents)

    assert not re.search(r"POC-08[^\n]*(?:\bPASS\b|\bSUPPORTED\b)", active_text, re.IGNORECASE)
    assert "MODEL_DEPENDENT" in active_text or "model-dependent" in active_text
    assert "PENDING" in active_text
