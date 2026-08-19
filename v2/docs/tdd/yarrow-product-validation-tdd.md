# TDD - Yarrow Technical Beta and Product Validation

**Strategy:** one public behavior per RED/GREEN cycle; stdlib reference and study records are independent from Yarrow implementation; no local fixture may count as external evidence.

## Task 1 - Study contract

**Files:**

- Create `tools/__init__.py`.
- Create `tools/study_contract.py`.
- Create `tests/fixtures/study/rehearsal.jsonl`.
- Create `tests/test_study_contract.py`.

RED tests:

```python
def test_rehearsal_rows_never_count_as_external(): ...
```

Run:

```powershell
python -m pytest tests/test_study_contract.py -q
```

GREEN contract functions:

```python
load_preregistration(path) -> dict
validate_preregistration(data) -> None
load_records(path) -> list[dict]
validate_records(preregistration, records) -> None
```

The rehearsal fixture must use `session_mode: "rehearsal"`. It must be accepted for schema testing but rejected as product evidence.

## Task 2 - Independent reference benchmark

**Files:**

- Create `benchmarks/__init__.py`.
- Create `benchmarks/cases.json`.
- Create `benchmarks/reference_impl.py`.
- Create `benchmarks/run.py`.
- Create `tests/test_benchmarks.py`.

RED tests:

```python
def test_reference_does_not_import_yarrow(): ...
```

Run:

```powershell
python -m pytest tests/test_benchmarks.py -q
```

GREEN runner:

```powershell
python -m benchmarks.run --cases benchmarks/cases.json --check
```

The command must be deterministic and must not call a network or use a random seed.

## Task 3 - Study analysis

**Files:**

- Create `tools/study_analyze.py`.
- Create `tests/test_study_analysis.py`.

RED tests:

```python
def test_no_external_records_returns_not_run(): ...
```

Run:

```powershell
python -m pytest tests/test_study_contract.py tests/test_study_analysis.py -q
```

GREEN command:

```powershell
python tools/study_analyze.py --preregistration docs/plans/product_validation/preregistration.v1.json --records tests/fixtures/study/rehearsal.jsonl --output $env:TEMP\yarrow-study.json
```

Expected semantic result: `status=NOT_RUN` or `TEST_ONLY`, `pass=null`.

## Task 4 - Release provenance and CLI warning

**Files:**

- Modify `yarrow/__init__.py`.
- Create `release/provenance.schema.json`.
- Create `tools/release_check.py`.
- Create `tests/test_release_provenance.py`.
- Extend `tests/test_cli.py`.

RED tests:

```python
def test_design_success_has_empty_stderr(): ...
```

Run:

```powershell
python -m pytest tests/test_cli.py tests/test_release_provenance.py -q
```

GREEN commands:

```powershell
python tools/release_check.py --mode technical --no-write
python -m pytest tests/ -q
```

The current workspace should report a clear technical release blocker for missing standalone Git provenance, not silently pass a product release gate.

## Task 5 - Documentation and CI

**Files:**

- Modify `README.md`, `docs/STATE.md`, `docs/VALIDATION.md`, `docs/PRODUCT_PLAN.md`.
- Modify `.github/workflows/ci.yml`.

Required CI commands:

```text
python -m pytest tests/ -q
python -m benchmarks.run --cases benchmarks/cases.json --check
python tools/release_check.py --mode technical --no-write
```

CI must not create participant records or treat rehearsal fixtures as product evidence.

## Definition of Done

- Full Yarrow test suite passes.
- Reference benchmark passes independently.
- Rehearsal analysis cannot produce product `PASS`.
- CLI success paths have no warning on stderr.
- Release provenance is explicit and fail-closed.
- Documentation states `product validation: NOT RUN` until eligible external records exist.
