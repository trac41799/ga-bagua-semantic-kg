# TDD - Promotion Evidence Status and Gate Integrity

**Strategy:** test the status model and report output directly; retain the existing execution gates as mechanical checks but stop using them as the promotion decision.

## Task 1 - Status schema

**Files:** create `v2/qa/claims.json`, create `v2/qa/promotion_report.py`, create `v2/qa/tests/test_promotion_report.py`.

RED tests:

```python
def test_failed_claim_is_not_product_pass(): ...
```

Run:

```powershell
python -m pytest v2/qa/tests/test_promotion_report.py -q
```

GREEN implementation: load explicit JSON rows, validate allowed statuses, and render deterministic markdown sorted by claim ID.

## Task 2 - Report evidence separation

Add report sections for execution status, claim status, replication, and product status. Add tests proving that a green test count does not alter a failed claim row.

Run:

```powershell
python v2/qa/promotion_report.py --output v2/qa/promotion-report.md
```

## Task 3 - Integration with current thesis

Update `v2/THESIS.md`, `v2/README.md`, and `v2/applications/README.md` only after the report exists. Add a test that stale claims such as unqualified POC-08 promotion are absent from the active front page.

## Definition of Done

- Promotion report is deterministic and generated from explicit claim rows.
- Expected experiment failures remain visible.
- Product recommendation is Yarrow technical beta, not fully market-validated product.
- No local test changes a claim to `PASS` merely because the test process exits zero.
