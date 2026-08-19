# SDD - State-Diff Protocol and Measurement Repair

**Status:** Superseding implementation specification  
**Product:** `v2/products/iching-tools/statediff` and POC-10 measurement harness  
**Evidence boundary:** current B3 compliance is mechanical and DeepSeek-only; the prior POC-10 QA protocol leaks planted ground truth into the question.

## 1. Strict Production Parser

Expose:

```python
def parse_aspects(text: str) -> list[dict[str, str]]: ...
```

Require exactly three non-empty lines matching `aspect: before -> after`, exactly three non-empty fields, and unique aspect labels under case-folding. Reject extra prose, empty values, duplicate labels, and non-list/non-dict results.

## 2. Measurement Protocol

Introduce protocol identifier:

```text
state_diff.measurement.same_record_value_pair_v1
```

The metric counts a planted `(before, after)` pair only when both values occur in the same parsed aspect row. A value found in one row and its counterpart found in another row is not a match.

```python
def measure_planted_delta_coverage(aspects, planted) -> dict[str, object]:
    # method, matched, covered, total, coverage
```

The metric is value-pair retention, not semantic label accuracy.

## 3. No-Ground-Truth-Leakage QA

Supersede the POC-10 comprehension question protocol with:

```text
state_diff.comprehension.no_ground_truth_in_question_v1
```

The reader sees only the generated summary and an aspect prompt. The question must not include the raw `before` string, raw `after` string, or planted values. The evaluator compares the answer against the hidden planted values.

The existing `qa_questions(before, after, planted)` protocol is retained only as historical evidence and must not be used for a new product claim.

## 4. Evidence Rules

- Mechanical parser tests may pass locally.
- A changed measurement method requires new cached artifacts and a new claims-ledger row.
- The new method does not inherit POC-10's objective result automatically.
- No LLM 1-5 rater is added; POC-10 calibration showed it is unfit without a new calibration pass.
- State-diff remains `single_model` until a second-model artifact passes the same protocol.

## 5. Acceptance

| ID | Acceptance |
|---|---|
| S1 | Strict parser rejects empty fields and duplicate labels |
| S2 | Same-record metric rejects cross-row value mixing |
| S3 | New QA question contains no hidden ground-truth values |
| S4 | Repaired benchmark reports method and per-record matches |
| S5 | Documentation marks new evidence as pending until rerun |
