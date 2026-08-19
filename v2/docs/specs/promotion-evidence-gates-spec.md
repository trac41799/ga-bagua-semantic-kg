# SDD - Promotion Evidence Status and Gate Integrity

**Status:** Required QA specification  
**Purpose:** prevent execution gates, evidence-presence checks, and claim verdicts from being represented as the same status.

## 1. Status Model

Every claim record uses:

```text
PASS
FAIL
PENDING
NOT_RUN
INCONCLUSIVE
MODEL_DEPENDENT
SUPERSEDED
```

`PASS` means the claim met its pre-registered bar. A test command exiting zero is not a claim `PASS`.

## 2. Claim Matrix

Create `v2/qa/claims.json` or an equivalent typed source with one row per POC/path/product claim. Each row includes:

```text
claim_id
name
claim_status
execution_status
evidence_status
replication_status
product_status
primary_metric
bar
observed
artifact
protocol_id
model_scope
next_gate
```

Required initial statuses:

- POC-02/Yarrow correctness: `PASS`, product `PENDING`;
- POC-05 coverage generation: `PASS`, replicated `PASS`, product `PENDING`;
- POC-08 reframe: `MODEL_DEPENDENT`;
- POC-13 evaluation: `PASS` for the frozen pilot, product `PENDING`;
- POC-06/state-diff: `INCONCLUSIVE`/`PENDING` pending clean protocol and replication;
- POC-01, POC-03, POC-04, POC-09, A, B, D: `FAIL`;
- POC-10: `INCONCLUSIVE`, product `PENDING` until the clean no-leakage rerun exists; retain the calibration failure and historical objective finding;
- POC-11, POC-12, and POC-14: deterministic claim `PASS`, execution/evidence `PASS`, product `PENDING` pending host/product fit;
- ICHING-TOOLS: package/MCP/SDK mechanics `PASS`, product `PENDING` pending host/product fit;
- POC-15: `PASS` for R-05 replication and `FAIL` for R-08 replication.

## 3. Promotion Rules

Product status can be `PROMOTE_BETA` only when:

- claim status is `PASS` or explicitly bounded;
- evidence is reproducible;
- replication scope is stated;
- host-product fit exists;
- no open safety/contract defect exists.

Product status cannot be `PROMOTE_PRODUCT` when:

- external/user gate is missing;
- the claim is model-dependent but positioned as model-general;
- required wheel/MCP distribution checks are missing;
- artifacts conflict or are overwritten by simulated runs.

## 4. Report

Create `v2/qa/promotion_report.py` and render `v2/qa/promotion-report.md` with:

- an executive decision;
- the claim matrix;
- blockers;
- evidence artifacts and hashes;
- separate execution-gate summary;
- separate replication and product status summaries;
- status-transition validation and hard failure on missing artifacts;
- no conversion of expected experimental `FAIL` into product-green `PASS`.

## 5. Acceptance

| ID | Acceptance |
|---|---|
| G1 | R-08 appears model-dependent/failed, not claim-green |
| G2 | Yarrow with no external rows remains pending |
| G3 | Failed POCs remain failed while their tests may still be green |
| G4 | Report distinguishes mechanics from claim results |
| G5 | Report is deterministic from the claim source |
