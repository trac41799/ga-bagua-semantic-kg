# TDD - State-Diff Protocol and Measurement Repair

**Strategy:** parser contract first, then metric regression, then clean comprehension-question construction; old POC-10 artifacts are never overwritten.

## Task 1 - Strict parser

**Files:** `statediff/iching_statediff/__init__.py`, `statediff/tests/test_statediff.py`.

RED tests:

```python
def test_parse_three_non_empty_lines(): ...
```

Run:

```powershell
python -m pytest statediff/tests/test_statediff.py -q
```

GREEN implementation: make `summarize()` call `parse_aspects()` and `validate_aspects()` before returning.

## Task 2 - Same-record metric

**Files:** `bench/common.py`, `bench/bench_statediff.py`, `bench/tests/test_measurement.py`.

RED tests:

```python
def test_same_row_pair_counts(): ...
```

Run:

```powershell
python -m pytest bench/tests/test_measurement.py -q
```

GREEN implementation must return:

```json
{
  "method": "same_record_value_pair_v1",
  "matched": [true, false],
  "covered": 1,
  "total": 2,
  "coverage": 0.5
}
```

## Task 3 - Clean comprehension questions

**Files:** `poc-10-communication-measurement/measure.py`, `poc-10-communication-measurement/tests/test_poc10.py`, create `poc-10-communication-measurement/tests/test_question_leakage.py`.

RED tests:

```python
def test_new_question_does_not_contain_before_text(): ...
```

Run:

```powershell
python -m pytest poc-10-communication-measurement/tests -q
```

GREEN implementation should accept a generated summary and aspect names, not raw before/after source text, for the new protocol.

## Task 4 - Artifact and documentation separation

Modify the benchmark writer so repaired outputs use a new filename or protocol field and never overwrite the historical POC-10 output. Add method, source hash, protocol hash, and model status to the artifact.

## Definition of Done

- Strict parser and same-record metric tests pass.
- Leakage tests pass.
- Historical POC-10 evidence remains untouched and labeled historical.
- New state-diff product claim remains pending until a fresh real-model run and second-model replication.
