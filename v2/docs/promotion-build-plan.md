# Promotion Validation and Product Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use the dev-test multi-agent loop. Each task starts with one failing behavior test, adds the smallest implementation, runs the focused test, then runs the affected suite and an independent verification pass.

**Goal:** Build the evidence-backed product candidates without overstating their validation status: Yarrow becomes an auditable technical beta with a ready-to-run product-validation study, coverage and state-diff become fail-closed bounded components, iching-tools becomes installable and MCP-contract verified, and portfolio QA distinguishes execution evidence from claim promotion.

**Architecture:** Keep deterministic math and product validation separate from LLM protocol wrappers. Use small public contracts at each boundary: Yarrow study records and release manifests, role-heading validation, strict state-diff records, shared provider configuration, and MCP tool/result validators. No local test or simulator may create external-user evidence or upgrade a model-dependent claim.

**Tech Stack:** Python 3.10+, stdlib-first packages, pytest, setuptools, JSON/JSONL artifacts, JSON-RPC stdio, official Python MCP SDK in test dependencies, GitHub Actions.

---

## Workstreams

| Workstream | Spec | TDD | Product outcome | Evidence that remains pending |
|---|---|---|---|---|
| Yarrow technical beta | `docs/specs/yarrow-product-validation-spec.md` | `docs/tdd/yarrow-product-validation-tdd.md` | Reproducible reference parity, preregistered study contract, release manifest, clean install | External users, task-time, usability, adoption |
| Coverage guardrail | `docs/specs/coverage-guardrail-spec.md` | `docs/tdd/coverage-guardrail-tdd.md` | Fail-closed output contract for the bounded checklist tool | New real-model benchmark after protocol change |
| State-diff measurement | `docs/specs/statediff-measurement-spec.md` | `docs/tdd/statediff-measurement-tdd.md` | Strict parser and non-leaking objective measurement | Second-model replication and human comprehension study |
| MCP distribution | `docs/specs/iching-tools-distribution-spec.md` | `docs/tdd/iching-tools-distribution-tdd.md` | Clean wheel, provider routing, six-tool contract, official SDK verification | LLM quality claims and market fit |
| Promotion evidence QA | `docs/specs/promotion-evidence-gates-spec.md` | `docs/tdd/promotion-evidence-gates-tdd.md` | Claim matrix with `PASS`, `FAIL`, `PENDING`, and `NOT_RUN` | Any human or external evidence not yet collected |

## Execution Order

### Task 1: Freeze the specs and TDDs

- [ ] Review the five SDDs and five TDDs for scope, file ownership, acceptance bars, and evidence status.
- [ ] Do not edit product code before the relevant failing test is identified.
- [ ] Record protocol identifiers and fixture hashes before running any real-model benchmark.

### Task 2: Build independent product slices in parallel

- [ ] Dispatch one agent for Yarrow files only.
- [ ] Dispatch one agent for coverage files and its tests only.
- [ ] Dispatch one agent for state-diff/POC-10 files and its tests only.
- [ ] Dispatch one agent for iching-tools provider/MCP/distribution files only.
- [ ] Keep promotion-QA changes separate until the product slices expose their final statuses.

### Task 3: Run focused RED/GREEN verification

- [ ] Each agent runs its new focused tests before and after implementation.
- [ ] Each agent reports exact commands, counts, changed files, and any evidence that remains pending.
- [ ] No agent may report a simulated or cached result as a real-model or external-user result.

### Task 4: Integrate promotion evidence QA

- [ ] Build the claim matrix from explicit evidence records, not from the presence of `PASS` or `FAIL` strings in arbitrary markdown.
- [ ] Add a test proving that POC-15 R-08 is represented as `FAIL`/model-dependent, not product-green.
- [ ] Add a test proving that Yarrow with no external records is `PENDING` or `NOT_RUN`, never `PRODUCT_VALIDATED`.

### Task 5: Independent factual verification

- [ ] Run Yarrow tests, deterministic reference benchmark, clean-build checks, and study-contract tests.
- [ ] Run the iching-tools full suite, build a fresh wheel outside the checked-in `dist/`, install it into a clean environment, and run the official MCP SDK test against the installed entry point.
- [ ] Run the POC-10 clean-question protocol in simulator/replay mode and verify that the question contains no planted before/after values.
- [ ] Run the promotion report and compare it with the thesis; expected failures remain visible.

## Required Verification Commands

```powershell
# Yarrow
python -m pytest tests/ -q
python -m benchmarks.run --cases benchmarks/cases.json --check
python tools/study_contract.py validate-preregistration docs/plans/product_validation/preregistration.v1.json
python tools/study_analyze.py --preregistration docs/plans/product_validation/preregistration.v1.json --records tests/fixtures/study/rehearsal.jsonl --output $env:TEMP\yarrow-study.json
python tools/release_check.py --mode technical --no-write

# iching-tools
python -m pytest coverage/tests reframe/tests statediff/tests mcp/tests cl3calc/tests xai/tests rotor/tests tests bench/tests -q
python -m build --wheel --sdist --outdir .tmp-iching-dist
python -m pytest tests/test_distribution.py -q
python -m pytest mcp/tests/test_debug.py -q

# portfolio claim status
python v2/qa/promotion_report.py --output v2/qa/promotion-report.md
python -m pytest v2/qa/tests -q
```

## Definition of Done

- All five SDDs and TDDs exist and identify exact files, public interfaces, tests, and acceptance bars.
- The deterministic products build and pass their mechanical tests.
- Coverage and state-diff invalid outputs fail closed.
- The installed iching-tools wheel contains the MCP implementation and can be consumed by the official SDK in simulator mode.
- Promotion status is honest: Yarrow is `TECHNICAL_BETA` or `PENDING_PRODUCT_VALIDATION`; coverage is bounded and conditional; state-diff remains single-model/pending; reframe remains model-dependent; failed POCs remain failed.
- No external-user, human, or real-model evidence is invented by local tests.
