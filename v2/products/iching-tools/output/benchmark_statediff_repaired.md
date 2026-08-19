# B3 — state_diff repaired measurement benchmark

- method: `same_record_value_pair_v1`
- protocol: `state_diff.measurement.same_record_value_pair_v1`
- source hash: `f580b9962b412b5101a7aa2c85f3baa32bad1f249aa254804cb71d5beb400963`
- protocol hash: `6f7637c577ec6aacc587f6750165c1f82fe431825d216a7e2810bfad2aab47f7`
- model: `deepseek-v4-flash`
- model status: `single_model`
- evidence status: **PENDING: fresh real-model rerun and second-model replication required**

## Per-transition compliance + same-record matches

| tid | domain | compliance | matched | coverage | defects |
|---|---|---|---|---|---|
| 1 | system | PASS | [true, true, true] | 1.0000 | - |
| 2 | system | PASS | [true, true, true] | 1.0000 | - |
| 3 | system | PASS | [true, true, true] | 1.0000 | - |
| 4 | system | PASS | [true, true, true] | 1.0000 | - |
| 5 | system | PASS | [true, true, true] | 1.0000 | - |
| 6 | business | PASS | [true, true, true] | 1.0000 | - |
| 7 | business | PASS | [true, true, true] | 1.0000 | - |
| 8 | business | PASS | [true, true, true] | 1.0000 | - |
| 9 | business | PASS | [true, true, true] | 1.0000 | - |
| 10 | business | PASS | [true, true, true] | 1.0000 | - |
| 11 | biology | PASS | [true, true, true] | 1.0000 | - |
| 12 | biology | PASS | [true, true, true] | 1.0000 | - |
| 13 | biology | FAIL | [false, false, false] | 0.0000 | ProtocolError: line does not match 'aspect: before -> after': 'fish count: ' |
| 14 | biology | PASS | [true, true, true] | 1.0000 | - |
| 15 | biology | PASS | [true, true, true] | 1.0000 | - |
| 16 | governance | PASS | [true, true, true] | 1.0000 | - |
| 17 | governance | FAIL | [false, false, false] | 0.0000 | ProtocolError: expected exactly 3 aspect lines, got 2 |
| 18 | governance | PASS | [true, true, true] | 1.0000 | - |
| 19 | governance | PASS | [true, true, true] | 1.0000 | - |
| 20 | governance | PASS | [true, true, true] | 1.0000 | - |

**Mean coverage: 0.9000** | compliant 18/20

## Verdicts

| metric | value | bar | verdict |
|---|---|---|---|
| statediff repaired compliance | 18/20 | 20/20 (zero defects) | FAIL |
| statediff repaired coverage | 0.9000 | >= 0.95 | FAIL |
| statediff repaired defects | 2 | 0 | FAIL |

## Defects

- tid 13 (biology): ProtocolError: line does not match 'aspect: before -> after': 'fish count: '
- tid 17 (governance): ProtocolError: expected exactly 3 aspect lines, got 2
