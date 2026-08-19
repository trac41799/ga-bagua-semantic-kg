# Promotion Evidence Report

## Executive Decision

- **Recommendation:** `TECHNICAL_BETA` only; do not promote the portfolio as a fully market-validated product.
- **Yarrow:** deterministic internal correctness evidence is present, but `product_status` remains `PENDING` because external-user and task-time records are absent.
- **R-08:** the replication is `FAIL` and the reframe claim is `MODEL_DEPENDENT`; it is not claim-green or model-general.
- **Status discipline:** execution-gate results are mechanical evidence and do not rewrite claim or product status.

## Status Transition Rules

- A claim `PASS` requires both `execution_status` and `evidence_status` to be `PASS`.
- Execution `PASS` never changes `claim_status`; deterministic product rows remain `PENDING` until host/product-fit evidence exists.
- A green product status is rejected for `FAIL`, `INCONCLUSIVE`, `MODEL_DEPENDENT`, `PENDING`, `NOT_RUN`, or `SUPERSEDED` claims.
- Any green product status, including `PROMOTE_BETA` and `PROMOTE_PRODUCT`, requires replication `PASS`; this report does not infer host fit from tests.

## Claim Matrix

| Claim ID | Name | Claim | Execution | Evidence | Replication | Product | Primary metric | Bar | Observed | Artifact | Protocol | Model scope | Next gate |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| A | Path A semantic index | FAIL | PASS | PASS | NOT_RUN | FAIL | probe against the strongest baseline | primary probe must beat the registered baseline wall | v2 probe 0.431 vs best secondary baseline 0.725; archived as failed despite a weaker-baseline delta | archive/experimentation/fails/README.md | PATH-A-v2 | synthetic mini-KG and CI evidence; no external validity | do not re-propose without a new preregistration and changed evidence |
| B | Path B learned-rotor KG embedding | FAIL | PASS | PASS | NOT_RUN | FAIL | embedding ranking versus baseline | rotor model must beat the registered baseline | corrected v2 TransE 0.557 > GeomE 0.472 > RotatE 0.439; WN18RR external run unavailable | archive/experimentation/fails/README.md | PATH-B-v2 | synthetic mini-KG; WN18RR not executed | archived; require a new preregistration to reopen |
| C | Path C teaching and visualization tool | PENDING | PASS | PENDING | NOT_RUN | PENDING | human pre/post learning and helpfulness gates | >=60% learners improve and >=70% rate >=4/5 | offline and math tests green; human-gate report is still a template with no sessions | v2/probes/path-c/output/human-gate-report.md | PATH-C-v1 | offline single-file tool; human learners not observed | run and document the preregistered human session |
| D | Path D compact semantic retrieval product claim | FAIL | PASS | PASS | NOT_RUN | FAIL | retrieval and pipeline quality | D1 R@10 >= 60%; D2 >=95% recall and break-even <=10; D3 >=10pp over lexical baselines | D1 0.370 FAIL; D2 0.331 vs 0.702 and break-even 23 FAIL; D3 +10.6pp/+11.4pp PASS | v2/probes/path-d/output/gate_summary.md | PATH-D-v1 | deepseek-chat, temperature 0, single annotator | retain D3 as a mechanism finding; do not promote the compact-index product claim |
| ICHING-TOOLS | iching-tools package, MCP, and SDK distribution contract | PASS | PASS | PASS | NOT_RUN | PENDING | package, MCP, and official SDK contract | package v0.2.0 installs from a fresh wheel; six MCP tools list and call through the official SDK; host/product fit remains open | iching-tools package v0.2.0, unified CLI, six-tool MCP surface, official SDK source/clean-wheel calls, and three SKILL.md files are implemented; official SDK `test_debug.py` passes 1 test, clean-wheel/distribution `test_distribution.py` passes 5 tests, suite-v2 passes 11 tests, and `python -m build --wheel --sdist` succeeds as mechanical evidence; no host/product-fit or external-user evidence | v2/products/iching-tools/tests/test_distribution.py | ICHING-TOOLS-v0.2.0 | deterministic package/MCP/SDK checks; LLM quality and host fit out of scope | collect host/product-fit evidence and an owned agent workflow before any product promotion |
| POC-01 | Combinatorial reasoning scaffold | FAIL | PASS | PASS | NOT_RUN | FAIL | scaffold accuracy delta | >= +20pp and p < 0.05 | scaffold 0.120 vs alone 0.060; delta +6.0pp; McNemar p=0.5078 | v2/pocs/poc-01-combinatorial-scaffold/output/verdict.md | POC-01-v1 | real LLM protocol; deterministic calculator | new preregistration with strict JSON input/output enforcement |
| POC-02 | Factorial interaction explorer correctness | PASS | PASS | PASS | NOT_RUN | PENDING | exact contrast and Mobius agreement | all registered contrast signs exact; Mobius error <= 1e-9 | 2^3 7/7, 2^4 15/15, Mobius max error 3.91e-14, names 19/19 | v2/pocs/poc-02-factorial-explorer/output/verdict.md | POC-02-v1 | deterministic; no LLM | Yarrow host-product validation; keep inference and semantic claims out of scope |
| POC-03 | Dialectical reframing engine proxy | FAIL | PASS | PASS | NOT_RUN | FAIL | diversity delta and coherence | >= +0.15 diversity delta and coherence >= 3.5 | diversity delta +0.0819; coherence 2.80 | v2/pocs/poc-03-reframing-engine/output/verdict.md | POC-03-v1 | deepseek-chat; LLM naming and rating | new naming protocol and preregistered margin; do not revive the old claim |
| POC-04 | Interpretable-tag agent memory | FAIL | PASS | PASS | NOT_RUN | FAIL | tag quality and filtering precision | quality >= 80% and precision >= 0.50 | quality 66.7%; precision 0.46; stability 83.3%; non-interference true | v2/pocs/poc-04-tagged-memory/output/gate_summary.md | POC-04-v1 | deepseek-chat; real mode; dense retrieval out of scope | new tag vocabulary or learned axes under a new preregistration |
| POC-05 | Coverage checklist generation | PASS | PASS | PASS | PASS | PENDING | missing-aspect reduction delta | >= +1.0 missing aspects (Arm A minus Arm B) | original +1.15; POC-15 replication +1.80 on 20 frozen tasks | v2/pocs/poc-05-coverage-rubric/output/verdict.md | POC-05-v1 plus POC-15-v1/R-05 | deepseek-chat and openai/gpt-4o-mini replication | host as a bounded evaluation capability and collect product-fit evidence |
| POC-06 | State-diff comprehension claim | INCONCLUSIVE | PASS | PASS | PENDING | PENDING | rater comprehension delta and planted-delta coverage | rater delta >= +0.5; automated coverage >= 0.85 | rater delta +0.10 fails; automated coverage A 0.85/B 1.00; rater instrument later failed calibration | v2/pocs/poc-06-state-diff/output/verdict.md | POC-06-v1 | single LLM rater; objective parser separate | clean non-leaking protocol, second-model replication, and human comprehension study |
| POC-07 | Interaction XAI math and naming claim | FAIL | PASS | PASS | NOT_RUN | FAIL | planted interaction recovery and naming clarity delta | math error <= 1e-9 and naming delta >= +0.5 | math error 2.22e-16 PASS; naming clarity delta +0.00 FAIL | v2/pocs/poc-07-interaction-xai/output/verdict.md | POC-07-v1 | deterministic math plus LLM naming | promote only the separated math capability after a new product contract |
| POC-08 | Few-shot reframing v2 | MODEL_DEPENDENT | PASS | PASS | FAIL | PENDING | reframe diversity and coherence | diversity >= 0.403 and coherence >= 3.5 on replication | deepseek baseline 0.453/3.75; gpt-4o-mini replication 0.296/3.10 | v2/pocs/poc-15-replication/output/replication_reframe.md | POC-08-v2 plus POC-15-v1/R-08 | model-dependent | model-specific preregistration; never present as model-general |
| POC-09 | Situation labeling and framing | FAIL | PASS | PASS | NOT_RUN | FAIL | comprehension and trust delta | delta >= +0.5 | Arm A 4.80 vs Arm B 4.80; delta +0.00; framing compliance 20/20 | v2/pocs/poc-09-situation-labeling/output/verdict.md | POC-09-v1 | single LLM rater; subjective proxy | do not re-promote without calibrated objective or human evidence |
| POC-10 | Objective communication measurement finding, not a product capability | INCONCLUSIVE | PASS | PASS | PENDING | PENDING | objective comprehension QA delta | clean no-leakage rerun must reproduce objective QA delta >= +0.10 before promotion | Historical objective QA: Arm A 0.900 vs Arm B 1.000, delta +0.100; rater calibration failed; conveyance delta -0.078; clean no-leakage rerun result is not yet available | v2/pocs/poc-10-communication-measurement/output/verdict.md | POC-10-v1 | objective QA plus unfit LLM rater | run and archive the clean no-leakage rerun, then replicate with a second reader model and more transitions; no product promotion |
| POC-11 | Cl3 calculator MCP contract | PASS | PASS | PASS | NOT_RUN | PENDING | exact operation and MCP contract parity | all registered operations exact; 100 seeded sequences match reference | Implemented deterministic `iching_cl3calc` package; `python -m pytest v2/products/iching-tools/cl3calc/tests/test_cl3calc.py -q` passes 1141 tests covering 24 flips, 8 complements, 64 products, strict no-partial validation, and 100 seeded reference-parity sequences; official SDK `test_debug.py` passes 1 test, fresh distribution `test_distribution.py` passes 5 tests, and `python -m build --wheel --sdist` succeeds; no LLM calls | v2/products/iching-tools/cl3calc/tests/test_cl3calc.py | POC-11-v1 | deterministic tool; no LLM | conditional host/product-fit evidence for an agent use case; do not infer LLM reasoning quality from the deterministic contract |
| POC-12 | Interaction-spectrum MCP contract | PASS | PASS | PASS | NOT_RUN | PENDING | planted interaction recovery and MCP contract | 3/3 planted subsets, error <= 1e-9, zero false positives | Implemented deterministic `iching_xai` package; `python -m pytest v2/products/iching-tools/xai/tests/test_xai.py -q` passes 21 tests verifying the three planted subsets recover with error <= 1e-9 and zero false positives plus strict input validation; official SDK `test_debug.py` passes 1 test, fresh distribution `test_distribution.py` passes 5 tests, and `python -m build --wheel --sdist` succeeds; no LLM calls | v2/products/iching-tools/xai/tests/test_xai.py | POC-12-v1 | deterministic tool; no LLM | conditional host/product-fit evidence for bounded interaction analysis; keep naming and semantic claims out of scope |
| POC-13 | Coverage rubric as output-quality evaluation | PASS | PASS | PASS | NOT_RUN | PENDING | bad-output detection and false alarms | detect >= 8/12 bad and rubric >= plain + 3; false alarms <= 2/12 | rubric detects 8/12 vs plain 3/12; false alarms 0/12; frozen cached pilot | v2/pocs/poc-13-coverage-eval/output/verdict.md | POC-13-v1 | deepseek-chat; cached frozen pilot | replicate and establish host-product fit for a bounded guardrail |
| POC-14 | Rotor transition algebra MCP contract | PASS | PASS | PASS | NOT_RUN | PENDING | rotor exactness, closure, and MCP contract | all hand cases and 100 random chains exact within 1e-12 | Implemented deterministic `iching_rotor` package; `python -m pytest v2/products/iching-tools/rotor/tests/test_rotor.py -q` passes 42 tests covering hand cases, composition/inverse/apply/distance, strict validation, and 100 random-chain closure/inverse/associativity checks; official SDK `test_debug.py` passes 1 test, fresh distribution `test_distribution.py` passes 5 tests, and `python -m build --wheel --sdist` succeeds; no LLM calls | v2/products/iching-tools/rotor/tests/test_rotor.py | POC-14-v1 | deterministic tool; no LLM | conditional host/product-fit evidence for a bounded state-transition use case; no semantic interpretation claim |
| POC-15 | Cross-model replication workstream | MODEL_DEPENDENT | PASS | PASS | INCONCLUSIVE | NOT_RUN | R-05 and R-08 replication against frozen bars | each registered claim must pass on the second model | R-05 PASS; R-08 FAIL; replication does not establish model-general promotion | v2/pocs/poc-15-replication/output/replication_reframe.md | POC-15-v1 | deepseek-chat and openai/gpt-4o-mini | keep subclaim verdicts separate; preregister any model-specific follow-up |
| POC-15-R05 | R-05 coverage replication | PASS | PASS | PASS | PASS | NOT_RUN | coverage missing-aspect delta | >= +1.0 | +1.80 on the frozen second-model run | v2/pocs/poc-15-replication/output/replication_coverage.md | POC-15-v1/R-05 | openai/gpt-4o-mini | bind only to a bounded host use case; no standalone product promotion |
| POC-15-R08 | R-08 reframe replication | FAIL | PASS | PASS | FAIL | NOT_RUN | reframe diversity and coherence | diversity >= 0.403 and coherence >= 3.5 | gpt-4o-mini diversity 0.296 and coherence 3.10; both fail | v2/pocs/poc-15-replication/output/replication_reframe.md | POC-15-v1/R-08 | model-dependent | model-specific follow-up only; do not claim model-general success |
| YARROW | Yarrow product validation and adoption | PENDING | PASS | PASS | NOT_RUN | PENDING | external users, task-time reduction, and adoption | >=3 external users, task-time reduction >=30%, and preregistered product exit evidence | 16/16 internal verification tests documented; external-user records: 0; task-time records: 0 | v2/products/yarrow-factorial/REPO_HANDOFF.md | YARROW-PHASE-1-v1 | deterministic local package; no external records | run the preregistered external-user and task-time study; keep recommendation at technical beta |

## Blockers

- `A`: claim `FAIL`, product `FAIL`; next gate: do not re-propose without a new preregistration and changed evidence.
- `B`: claim `FAIL`, product `FAIL`; next gate: archived; require a new preregistration to reopen.
- `C`: claim `PENDING`, product `PENDING`; next gate: run and document the preregistered human session.
- `D`: claim `FAIL`, product `FAIL`; next gate: retain D3 as a mechanism finding; do not promote the compact-index product claim.
- `ICHING-TOOLS`: claim `PASS`, product `PENDING`; next gate: collect host/product-fit evidence and an owned agent workflow before any product promotion.
- `POC-01`: claim `FAIL`, product `FAIL`; next gate: new preregistration with strict JSON input/output enforcement.
- `POC-02`: claim `PASS`, product `PENDING`; next gate: Yarrow host-product validation; keep inference and semantic claims out of scope.
- `POC-03`: claim `FAIL`, product `FAIL`; next gate: new naming protocol and preregistered margin; do not revive the old claim.
- `POC-04`: claim `FAIL`, product `FAIL`; next gate: new tag vocabulary or learned axes under a new preregistration.
- `POC-05`: claim `PASS`, product `PENDING`; next gate: host as a bounded evaluation capability and collect product-fit evidence.
- `POC-06`: claim `INCONCLUSIVE`, product `PENDING`; next gate: clean non-leaking protocol, second-model replication, and human comprehension study.
- `POC-07`: claim `FAIL`, product `FAIL`; next gate: promote only the separated math capability after a new product contract.
- `POC-08`: claim `MODEL_DEPENDENT`, product `PENDING`; next gate: model-specific preregistration; never present as model-general.
- `POC-09`: claim `FAIL`, product `FAIL`; next gate: do not re-promote without calibrated objective or human evidence.
- `POC-10`: claim `INCONCLUSIVE`, product `PENDING`; next gate: run and archive the clean no-leakage rerun, then replicate with a second reader model and more transitions; no product promotion.
- `POC-11`: claim `PASS`, product `PENDING`; next gate: conditional host/product-fit evidence for an agent use case; do not infer LLM reasoning quality from the deterministic contract.
- `POC-12`: claim `PASS`, product `PENDING`; next gate: conditional host/product-fit evidence for bounded interaction analysis; keep naming and semantic claims out of scope.
- `POC-13`: claim `PASS`, product `PENDING`; next gate: replicate and establish host-product fit for a bounded guardrail.
- `POC-14`: claim `PASS`, product `PENDING`; next gate: conditional host/product-fit evidence for a bounded state-transition use case; no semantic interpretation claim.
- `POC-15`: claim `MODEL_DEPENDENT`, product `NOT_RUN`; next gate: keep subclaim verdicts separate; preregister any model-specific follow-up.
- `POC-15-R05`: claim `PASS`, product `NOT_RUN`; next gate: bind only to a bounded host use case; no standalone product promotion.
- `POC-15-R08`: claim `FAIL`, product `NOT_RUN`; next gate: model-specific follow-up only; do not claim model-general success.
- `YARROW`: claim `PENDING`, product `PENDING`; next gate: run the preregistered external-user and task-time study; keep recommendation at technical beta.

## Evidence Artifacts and Hashes

- Claim source `v2/qa/claims.json`: `3203da626c46995fc552eae96b321e063df24a14cd6c55493f2da091e25b30f7`

| Claim ID | Artifact | sha256 |
|---|---|---|
| `A` | `archive/experimentation/fails/README.md` | `8c4982997a0c904286e31fb3ad8fd9bc0dae35fcc89899e9006852b7117db73c` |
| `B` | `archive/experimentation/fails/README.md` | `8c4982997a0c904286e31fb3ad8fd9bc0dae35fcc89899e9006852b7117db73c` |
| `C` | `v2/probes/path-c/output/human-gate-report.md` | `220e9a8c97f7c4030181820655ab324de6dca5f91899a2c93267a09b136c44fb` |
| `D` | `v2/probes/path-d/output/gate_summary.md` | `64190aeac7b15494a0d8339e52fab93579f3930cefb6e3a06900864f68457809` |
| `ICHING-TOOLS` | `v2/products/iching-tools/tests/test_distribution.py` | `b5a68ef490611ddc1c78c118d5db5d66eacfe830b6a2438e5e0893ae5e564d01` |
| `ICHING-TOOLS` | `v2/products/iching-tools/pyproject.toml` | `c963bf9283235200d0075ba6d53df8fe737907c9259db02acbe5807c88d6ee96` |
| `ICHING-TOOLS` | `v2/products/iching-tools/README.md` | `f29f856226ab29bec4e3006aff3d1d756b8b9449241cd7cccabad61004ae2583` |
| `ICHING-TOOLS` | `v2/products/iching-tools/AGENT_INTEGRATION.md` | `6afe404d89a7ef26ef318ab4e4fda701d7f0848bfa82b9553f1d6403f9661852` |
| `ICHING-TOOLS` | `v2/products/iching-tools/tests/test_suite_v2.py` | `375975ffb446e2b5a0ebdb3a5ed768838b8cb165cc0f8c20dcc0b52fa8083245` |
| `ICHING-TOOLS` | `v2/products/iching-tools/mcp/tests/test_mcp.py` | `0547a4bbf277f4abfb9542d3235f9050678c5a6eed9cacd5239be589e6202243` |
| `ICHING-TOOLS` | `v2/products/iching-tools/mcp/tests/test_debug.py` | `86b87d63e34b6f8927c61a19110ec868695bf760d9c1b7cf6cd3ce9934af20d8` |
| `ICHING-TOOLS` | `v2/products/iching-tools/mcp/tests/sdk_probe.py` | `f273c6df2875885ad68fde89ad04148599a3dddeaddd87f32a96033867c07884` |
| `ICHING-TOOLS` | `v2/products/iching-tools/.github/workflows/ci.yml` | `3868afc4be75d7a01a120678c0a9d5a51366a016372dcfe68fe6630459c4d630` |
| `ICHING-TOOLS` | `v2/products/iching-tools/skills/coverage-audit/SKILL.md` | `f2e6f7cb837a9c300a102e830d32e30153fc568e3bc6f36f0661e0e6f4cb1ba9` |
| `ICHING-TOOLS` | `v2/products/iching-tools/skills/bagua-reframe/SKILL.md` | `2488930666b352f2fa59a2f55da72f6b1826980d938d0efea586161b8d4193c0` |
| `ICHING-TOOLS` | `v2/products/iching-tools/skills/state-diff/SKILL.md` | `e57b0367e652c395b32ff91e7a59f829f1bca36b4e94981b2c4702d95956f353` |
| `ICHING-TOOLS` | `v2/products/iching-tools/dist/iching_tools-0.2.0-py3-none-any.whl` | `66726a58fa573ecc355ad592ebd5c8d6d5f049970efd8ac47d86cdb15939bdf2` |
| `ICHING-TOOLS` | `v2/products/iching-tools/dist/iching_tools-0.2.0.tar.gz` | `738b96f97a2bcb013625b07ae1efa1a89d3952b692b6627c5ba977b0223a2496` |
| `POC-01` | `v2/pocs/poc-01-combinatorial-scaffold/output/verdict.md` | `86ce489973bcf22a79ed3c8a88996d6f95e7b1139ffb6c55e327fb1948d485e6` |
| `POC-02` | `v2/pocs/poc-02-factorial-explorer/output/verdict.md` | `d346745902b060123ba54756392d457560f14109102fce6d60af6ed109067156` |
| `POC-03` | `v2/pocs/poc-03-reframing-engine/output/verdict.md` | `d18317b53238545037a9c713b391f52793a524ae9ee33d2a74bd131cceb53f0f` |
| `POC-04` | `v2/pocs/poc-04-tagged-memory/output/gate_summary.md` | `59ab229e5b5fe09ed625a182828d8089e2ec4014ae88d249bc0abb830c229a4e` |
| `POC-05` | `v2/pocs/poc-05-coverage-rubric/output/verdict.md` | `3e720e8b95e568fa51bf97c9174f22e6f10a23dad5613458d58c6473912c59dd` |
| `POC-06` | `v2/pocs/poc-06-state-diff/output/verdict.md` | `6a0f34919672e5c010fb4d0b654c3768c2f1b7012910da62381a50e48551ff29` |
| `POC-07` | `v2/pocs/poc-07-interaction-xai/output/verdict.md` | `423544bbd349d585c025d1940e1f11ee9d08815f13824973c00e686758523b67` |
| `POC-08` | `v2/pocs/poc-15-replication/output/replication_reframe.md` | `0b3f12832b302eca686134acc0a5b13c9b68ef2299c5c71ac0026535c7883988` |
| `POC-09` | `v2/pocs/poc-09-situation-labeling/output/verdict.md` | `f4b7d6a6b76dfb19c3be6cc4cbc6398fcf66472379d3a7ff1480aa0126fdbfd9` |
| `POC-10` | `v2/pocs/poc-10-communication-measurement/output/verdict.md` | `608e51eceb06f4920c1ecb39234a120aa545de8ff404d8bfc6e10d9a89d1cd14` |
| `POC-10` | `v2/pocs/poc-10-communication-measurement/tests/test_question_leakage.py` | `73ed7b95635839f5aa2d12c65fe5387edfa4429dd0673513ecdcee8d51136ede` |
| `POC-10` | `v2/pocs/poc-10-communication-measurement/measure.py` | `b826eedfc7541e5bbe09b176a5731fd5df9e621d7b2de37e3236c95575da70df` |
| `POC-10` | `v2/pocs/poc-10-communication-measurement/run_all.py` | `4a784c830922a7abf26a0f50bebdf4ef55f54479eb121500111554e8d142b8ac` |
| `POC-10` | `v2/docs/specs/statediff-measurement-spec.md` | `5489c708869fcc0becb2961e2ad7243c00f84eba5a81e71bee8680ad117964b1` |
| `POC-10` | `v2/docs/tdd/statediff-measurement-tdd.md` | `1946c97e8fe744a0dade18f6e6858cc8371067dbd287a04e030b6914befb83ff` |
| `POC-11` | `v2/products/iching-tools/cl3calc/tests/test_cl3calc.py` | `97f711e8e9e853d86af55c5b6f49d3c73ed59ebf0708d3bd326b37250b446f4e` |
| `POC-11` | `v2/products/iching-tools/cl3calc/iching_cl3calc/__init__.py` | `91694244217968d7f6f3178e2ef76a27d256e87c7339a4f7ac3a289c788a5831` |
| `POC-11` | `v2/products/iching-tools/cl3calc/reference/calculator.py` | `750b02b96b204978c675e9afffc12f74ca808b542d7caff33d03f6ce6002bf38` |
| `POC-11` | `v2/products/iching-tools/cl3calc/reference/cl3.py` | `829df5fdc8b62471d972fac495c293a77ae642c46696adf497937d3b87dfc4d4` |
| `POC-11` | `v2/products/iching-tools/cl3calc/README.md` | `623e4050f977b1302a0dc44423b8c538b64122e38895b3402dfd6a2bc20bbd98` |
| `POC-11` | `v2/products/iching-tools/mcp/tests/test_mcp.py` | `0547a4bbf277f4abfb9542d3235f9050678c5a6eed9cacd5239be589e6202243` |
| `POC-11` | `v2/products/iching-tools/mcp/tests/test_debug.py` | `86b87d63e34b6f8927c61a19110ec868695bf760d9c1b7cf6cd3ce9934af20d8` |
| `POC-11` | `v2/products/iching-tools/mcp/tests/sdk_probe.py` | `f273c6df2875885ad68fde89ad04148599a3dddeaddd87f32a96033867c07884` |
| `POC-11` | `v2/products/iching-tools/tests/test_distribution.py` | `b5a68ef490611ddc1c78c118d5db5d66eacfe830b6a2438e5e0893ae5e564d01` |
| `POC-11` | `v2/products/iching-tools/pyproject.toml` | `c963bf9283235200d0075ba6d53df8fe737907c9259db02acbe5807c88d6ee96` |
| `POC-11` | `v2/products/iching-tools/.github/workflows/ci.yml` | `3868afc4be75d7a01a120678c0a9d5a51366a016372dcfe68fe6630459c4d630` |
| `POC-11` | `v2/products/iching-tools/dist/iching_tools-0.2.0-py3-none-any.whl` | `66726a58fa573ecc355ad592ebd5c8d6d5f049970efd8ac47d86cdb15939bdf2` |
| `POC-11` | `v2/products/iching-tools/dist/iching_tools-0.2.0.tar.gz` | `738b96f97a2bcb013625b07ae1efa1a89d3952b692b6627c5ba977b0223a2496` |
| `POC-12` | `v2/products/iching-tools/xai/tests/test_xai.py` | `0253ce94ac99b30e4af1ae8b6a2a95fb8497df2909488f1afc008ba817d81573` |
| `POC-12` | `v2/products/iching-tools/xai/iching_xai/__init__.py` | `9cb6186babcb4b1c57cf7a4ecf64dcce2c7274edf5aad038ad5a39b8186588ab` |
| `POC-12` | `v2/products/iching-tools/xai/README.md` | `2ff3b57587f8ea53ad06e7a7d54f3854d770848bc9cbde9b74ac5e2a3dc1dd5d` |
| `POC-12` | `v2/products/iching-tools/mcp/tests/test_mcp.py` | `0547a4bbf277f4abfb9542d3235f9050678c5a6eed9cacd5239be589e6202243` |
| `POC-12` | `v2/products/iching-tools/mcp/tests/test_debug.py` | `86b87d63e34b6f8927c61a19110ec868695bf760d9c1b7cf6cd3ce9934af20d8` |
| `POC-12` | `v2/products/iching-tools/mcp/tests/sdk_probe.py` | `f273c6df2875885ad68fde89ad04148599a3dddeaddd87f32a96033867c07884` |
| `POC-12` | `v2/products/iching-tools/tests/test_distribution.py` | `b5a68ef490611ddc1c78c118d5db5d66eacfe830b6a2438e5e0893ae5e564d01` |
| `POC-12` | `v2/products/iching-tools/pyproject.toml` | `c963bf9283235200d0075ba6d53df8fe737907c9259db02acbe5807c88d6ee96` |
| `POC-12` | `v2/products/iching-tools/.github/workflows/ci.yml` | `3868afc4be75d7a01a120678c0a9d5a51366a016372dcfe68fe6630459c4d630` |
| `POC-12` | `v2/products/iching-tools/dist/iching_tools-0.2.0-py3-none-any.whl` | `66726a58fa573ecc355ad592ebd5c8d6d5f049970efd8ac47d86cdb15939bdf2` |
| `POC-12` | `v2/products/iching-tools/dist/iching_tools-0.2.0.tar.gz` | `738b96f97a2bcb013625b07ae1efa1a89d3952b692b6627c5ba977b0223a2496` |
| `POC-13` | `v2/pocs/poc-13-coverage-eval/output/verdict.md` | `d33418d6084f60afafc8e8c5102d8093d7e95c8b8595e3a80f968aa81d30a0c1` |
| `POC-14` | `v2/products/iching-tools/rotor/tests/test_rotor.py` | `20c4310ee3bf39b311f8f66bd0a5e252da2499080c5f73aa3d21eae032356698` |
| `POC-14` | `v2/products/iching-tools/rotor/iching_rotor/__init__.py` | `32f29b948dd9c8d39715731cde720f2be82b1e8794da662367f6fa3ff6755e54` |
| `POC-14` | `v2/products/iching-tools/rotor/cl3.py` | `829df5fdc8b62471d972fac495c293a77ae642c46696adf497937d3b87dfc4d4` |
| `POC-14` | `v2/products/iching-tools/rotor/README.md` | `433394fa24ee64476b5e2b3444c8e3d5a9ba4d91056a71dad9be5125ae6b8e14` |
| `POC-14` | `v2/products/iching-tools/mcp/tests/test_mcp.py` | `0547a4bbf277f4abfb9542d3235f9050678c5a6eed9cacd5239be589e6202243` |
| `POC-14` | `v2/products/iching-tools/mcp/tests/test_debug.py` | `86b87d63e34b6f8927c61a19110ec868695bf760d9c1b7cf6cd3ce9934af20d8` |
| `POC-14` | `v2/products/iching-tools/mcp/tests/sdk_probe.py` | `f273c6df2875885ad68fde89ad04148599a3dddeaddd87f32a96033867c07884` |
| `POC-14` | `v2/products/iching-tools/tests/test_distribution.py` | `b5a68ef490611ddc1c78c118d5db5d66eacfe830b6a2438e5e0893ae5e564d01` |
| `POC-14` | `v2/products/iching-tools/pyproject.toml` | `c963bf9283235200d0075ba6d53df8fe737907c9259db02acbe5807c88d6ee96` |
| `POC-14` | `v2/products/iching-tools/.github/workflows/ci.yml` | `3868afc4be75d7a01a120678c0a9d5a51366a016372dcfe68fe6630459c4d630` |
| `POC-14` | `v2/products/iching-tools/dist/iching_tools-0.2.0-py3-none-any.whl` | `66726a58fa573ecc355ad592ebd5c8d6d5f049970efd8ac47d86cdb15939bdf2` |
| `POC-14` | `v2/products/iching-tools/dist/iching_tools-0.2.0.tar.gz` | `738b96f97a2bcb013625b07ae1efa1a89d3952b692b6627c5ba977b0223a2496` |
| `POC-15` | `v2/pocs/poc-15-replication/output/replication_reframe.md` | `0b3f12832b302eca686134acc0a5b13c9b68ef2299c5c71ac0026535c7883988` |
| `POC-15-R05` | `v2/pocs/poc-15-replication/output/replication_coverage.md` | `675a9207a75b7bf0136bb8495af575f7275e3c88a9a4812100a21a4c73c579df` |
| `POC-15-R08` | `v2/pocs/poc-15-replication/output/replication_reframe.md` | `0b3f12832b302eca686134acc0a5b13c9b68ef2299c5c71ac0026535c7883988` |
| `YARROW` | `v2/products/yarrow-factorial/REPO_HANDOFF.md` | `0d484a431bf13eb8df9d8872e9d5fd3db9c265b025d3e1ab3d795d8910f79dc2` |

## Execution-Gate Summary

Execution status is reported separately from claim status. A green test command is not a claim PASS.

| Execution status | Count | Claim IDs |
|---|---:|---|
| PASS | 23 | `A`, `B`, `C`, `D`, `ICHING-TOOLS`, `POC-01`, `POC-02`, `POC-03`, `POC-04`, `POC-05`, `POC-06`, `POC-07`, `POC-08`, `POC-09`, `POC-10`, `POC-11`, `POC-12`, `POC-13`, `POC-14`, `POC-15`, `POC-15-R05`, `POC-15-R08`, `YARROW` |

## Claim-Status Summary

| Claim status | Count | Claim IDs |
|---|---:|---|
| FAIL | 9 | `A`, `B`, `D`, `POC-01`, `POC-03`, `POC-04`, `POC-07`, `POC-09`, `POC-15-R08` |
| INCONCLUSIVE | 2 | `POC-06`, `POC-10` |
| MODEL_DEPENDENT | 2 | `POC-08`, `POC-15` |
| PASS | 8 | `ICHING-TOOLS`, `POC-02`, `POC-05`, `POC-11`, `POC-12`, `POC-13`, `POC-14`, `POC-15-R05` |
| PENDING | 2 | `C`, `YARROW` |

## Replication Status Summary

| Replication status | Count | Claim IDs |
|---|---:|---|
| FAIL | 2 | `POC-08`, `POC-15-R08` |
| INCONCLUSIVE | 1 | `POC-15` |
| NOT_RUN | 16 | `A`, `B`, `C`, `D`, `ICHING-TOOLS`, `POC-01`, `POC-02`, `POC-03`, `POC-04`, `POC-07`, `POC-09`, `POC-11`, `POC-12`, `POC-13`, `POC-14`, `YARROW` |
| PASS | 2 | `POC-05`, `POC-15-R05` |
| PENDING | 2 | `POC-06`, `POC-10` |

## Product Status Summary

| Product status | Count | Claim IDs |
|---|---:|---|
| FAIL | 8 | `A`, `B`, `D`, `POC-01`, `POC-03`, `POC-04`, `POC-07`, `POC-09` |
| NOT_RUN | 3 | `POC-15`, `POC-15-R05`, `POC-15-R08` |
| PENDING | 12 | `C`, `ICHING-TOOLS`, `POC-02`, `POC-05`, `POC-06`, `POC-08`, `POC-10`, `POC-11`, `POC-12`, `POC-13`, `POC-14`, `YARROW` |
