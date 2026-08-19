# SDD - Yarrow Technical Beta and Product Validation

**Status:** Pre-registered implementation specification  
**Product:** `D:\TRANSFER DATA\Coding\OpenCode\yarrow-factorial`  
**Origin:** POC-02 factorial explorer  
**Purpose:** turn the technically validated POC-02 core into an auditable technical beta and prepare, but do not fabricate, the external product-validation gates.

## 1. Boundary

Yarrow may claim:

- exact `2^k` factorial contrast calculations for the supported range;
- deterministic Mobius interaction decomposition within the documented tolerance;
- local, dependency-free operation;
- Bagua as a mnemonic naming layer only.

Yarrow may not claim:

- statistical inference, prediction, or semantic meaning from Bagua;
- superiority over commercial DOE tools based only on parity with a local reference;
- external-user productivity or usability until eligible records exist;
- a geometric-product implementation path unless the shipped code actually contains and tests that path.

## 2. Requirements

### Y1 - Independent deterministic reference

Create `benchmarks/` with a fixed JSON case file and an independent stdlib reference implementation. The reference must not import `yarrow`, reuse Yarrow helper functions, generate random inputs at runtime, or include timing in the canonical result.

Required cases:

- `2^3` and `2^4` contrast signs;
- supported upper-bound `2^6` signs;
- fixed effects tables;
- documented `n=4` Mobius example;
- sparse and dense fixed `n=6` set functions.

Acceptance:

- contrast outputs match exactly;
- numeric outputs match within `1e-12`;
- two canonical runs are byte-identical;
- every case and output has a SHA-256 hash;
- the result says `PASS` for parity only, never superiority.

### Y2 - Frozen product-validation protocol

Create:

- `docs/plans/PRODUCT_VALIDATION_STUDY.md`;
- `docs/plans/product_validation/preregistration.v1.json`;
- `docs/plans/product_validation/session-record.schema.json`;
- `docs/plans/product_validation/result.schema.json`;
- fixed task materials under `docs/plans/product_validation/tasks/`.

The preregistration must freeze participant eligibility, task materials, two arms, counterbalancing, start/stop rules, success oracle, assistance recording, exclusion rules, privacy, and kill criteria before any external session.

Required product gate:

- at least five valid paired external task records;
- at least two ML/data participants and one DOE/experimentation participant;
- paired Yarrow/reference workflow;
- median time reduction `>= 30%` using `1 - median(yarrow_time/reference_time)`;
- no rehearsal or synthetic row can satisfy the gate.

UI usability is a separate gate: at least five unassisted completions and mean task ease `>= 4/5`.

### Y3 - Fail-closed study analysis

Create `tools/study_contract.py` and `tools/study_analyze.py` using only the standard library.

Allowed states:

```text
NOT_RUN
TEST_ONLY
ANALYZED
INCONCLUSIVE
PASS
FAIL
```

Rules:

- no external rows -> `NOT_RUN`, `pass: null`;
- rehearsal-only rows -> `TEST_ONLY`, `pass: null`;
- fewer than five valid paired external rows -> `INCONCLUSIVE`, `pass: null`;
- mismatched study, protocol, source, version, or baseline hashes -> reject;
- only frozen-protocol external records may produce `PASS` or `FAIL`.

### Y4 - Technical release provenance

Create `release/provenance.schema.json` and `tools/release_check.py`.

The manifest must include distribution name, import name, version, source commit, source tree hash, artifact hash, test result, benchmark hashes, preregistration hash, and external evidence status.

Technical checks may pass with external evidence `NOT_RUN`; product validation must remain blocked.

The release check must detect that the current directory is not an independent Git root and return `BLOCKED` rather than implying a reproducible release.

### Y5 - Warning-free release surface

Running `python -m yarrow.report design -k 3` and `python -m yarrow.report decompose ...` must exit zero with empty stderr. Preserve the public `effects_from_data` import without eagerly importing the CLI module during module execution.

### Y6 - Documentation synchronization

Update `README.md`, `docs/STATE.md`, `docs/VALIDATION.md`, and `docs/PRODUCT_PLAN.md` so that:

- technical correctness is marked complete;
- external product validation is marked pending/not run;
- the distribution name matches `pyproject.toml`;
- the geometric-product claim is narrowed or backed by an actual implementation;
- generated local rehearsals are not presented as user evidence.

## 3. Non-goals

- recruiting participants in this implementation cycle;
- publishing to PyPI without owner credentials;
- adding statistical inference or LLM design generation;
- rebuilding Yarrow inside the portfolio repository.

## 4. Acceptance Summary

| ID | Acceptance |
|---|---|
| Y1 | Independent deterministic parity benchmark passes |
| Y2 | Frozen study and schemas exist and validate |
| Y3 | Local rehearsal analysis cannot produce product `PASS` |
| Y4 | Release manifest is hash-based and fail-closed |
| Y5 | CLI success paths are warning-free |
| Y6 | Current docs state product validation is pending |
