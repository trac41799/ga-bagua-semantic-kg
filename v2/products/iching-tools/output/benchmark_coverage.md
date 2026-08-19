# iching-tools validation benchmark

| metric | value | bar | verdict |
|---|---|---|---|
| coverage_delta (original - audited mean missing) | +3.33 | >= 1.0 (validated +1.15, tol +/-0.30) | PASS |
| coverage_original_missing | 3.33 | POC-05 Arm-A baseline |  |
| coverage_audited_missing | 0.00 | <= original - 1.0 |  |
| defects | 20 | 0 required | FAIL |

## B1 coverage_audit detail (real LLM, temperature 0, cached)

- tasks: 6/20 | original mean missing: 3.33 | audited mean missing: 0.00 | delta: +3.33 (validated +1.15) | verdict: **FAIL**
- client: `LLMClient(api_key=None, model={llm.model!r}, base_url=None)` (production constructor; cache shim adds chat() for the rater)
- cache: `bench/.cache_coverage_deepseek-v4-flash.json` (key = purpose|tid|input-hash)

| id | domain | original missing | audited missing | original bits | audited bits |
|---|---|---|---|---|---|
| 1 | product | - | - | - | - |
| 2 | product | - | - | - | - |
| 3 | product | - | - | - | - |
| 4 | product | - | - | - | - |
| 5 | product | - | - | - | - |
| 6 | incident | - | - | - | - |
| 7 | incident | 0 | 0 | 111111 | 111111 |
| 8 | incident | 4 | 0 | 000110 | 111111 |
| 9 | incident | - | - | - | - |
| 10 | incident | - | - | - | - |
| 11 | policy | 3 | 0 | 100110 | 111111 |
| 12 | policy | 4 | 0 | 000011 | 111111 |
| 13 | policy | - | - | - | - |
| 14 | policy | - | - | - | - |
| 15 | policy | - | - | - | - |
| 16 | research | 4 | 0 | 000011 | 111111 |
| 17 | research | 5 | 0 | 000010 | 111111 |
| 18 | research | - | - | - | - |
| 19 | research | - | - | - | - |
| 20 | research | - | - | - | - |

## Defects

- tid 1 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 2 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 3 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 4 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 4 [rater_parse]: audited: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 5 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 5 [rater_parse]: audited: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 6 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 9 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 10 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 10 [rater_parse]: audited: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 13 [rater_parse]: audited: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 14 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 15 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 18 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 18 [rater_parse]: audited: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 19 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 19 [rater_parse]: audited: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 20 [rater_parse]: original: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')
- tid 20 [rater_parse]: audited: JSONDecodeError('Expecting property name enclosed in double quotes: line 1 column 2 (char 1)')

## Harness note

- `common.poc05_draft_plans()` key-collides `N_audit_A` over `N_A` (both end with `_A`); drafts loaded here with the POC's exact-key `<tid>_A` semantics from the same frozen cache file (see run_all.py).
