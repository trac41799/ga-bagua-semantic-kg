# TDD Plan — Path C (GA×Bagua Teaching / Visualization Tool)

**Strategy:** Math core as pure JS functions exposed on `window.BaguaTool`, tested via `node --check` + a Node test runner executed through `tests/run_js_tests.js`; structural checks via Python. All local, no network. Every AC maps to ≥1 test case.

## Test inventory (AC → cases)

### T-C1 Math core (JS, `tests/math_core.test.js` — run with node)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| C1.1 | product(e1,e2) | == {blade:'e12', sign:+1} | C3 |
| C1.2 | product(e2,e1) | == {blade:'e12', sign:-1} | C3 |
| C1.3 | product(e1,e1) | == {blade:'1', sign:+1} | C3 |
| C1.4 | product(e12,e12) | == {blade:'1', sign:-1} | C3 |
| C1.5 | product(e123,e123) | == {blade:'1', sign:-1} | C3 |
| C1.6 | product table 8×8 | all 64 entries match verified v1 PROD_TABLE | C3 |
| C1.7 | flipLine each trigram × 3 lines | bit XOR semantics; 24 cases | C4 |
| C1.8 | hexagram names | spot-check ≥8 known hexagrams (e.g., Kun over Kun = 地地坤 #2) | C5 |
| C1.9 | rotor π in e12 on e1 | → −e1 | C6 |
| C1.10 | rotor π/2 in e12 on e1 | → e2 | C6 |
| C1.11 | rotor sandwich unit norm | |a'| == |a| | C6 |

### T-C2 Structure (Python, `tests/test_tool.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| C2.1 | single file | exactly one .html; no external files referenced | C1 |
| C2.2 | zero network | no `http://`, `https://`, `<link href>`, `src=` external, no CDN | C1 |
| C2.3 | HTML parses | `html.parser` no errors | C8 |
| C2.4 | JS syntax | extract inline `<script>` → `node --check` exit 0 | C8 |
| C2.5 | DOM IDs | gallery, blade-view, line-flip, product-table, hexagram-stack, rotor-demo, quiz, glossary all present | C9 |
| C2.6 | 8 trigram panels | each has 3 line elements with correct yin/yang class + blade label text | C2 |
| C2.7 | quiz | 5 questions, scoring function present; all-correct → 5/5 (via BaguaTool.quizScore) | C7 |
| C2.8 | BaguaTool exposed | `window.BaguaTool` with product/flipLine/hexagram/rotorApply | C3-C6 |

### T-C3 Docs & protocol (`tests/test_docs.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| C3.1 | README | contains open instructions (file://) + human-gate protocol + template path | C10 |
| C3.2 | lesson plan | exists, 20-min guided session, pre/post protocol | C10 |
| C3.3 | human gate template | `output/human-gate-report.md` template with pre-registered thresholds | C10 |

## Red-green-refactor order

1. `math_core.test.js` (pure JS math) → implement math core in JS module (`assets/math_core.js` extracted for test, then inlined into index.html with a build-free sync script `scripts/inline_math.py` that keeps single-file invariant — OR simpler: math core lives inside index.html and tests load it via `node -e` reading the html and eval'ing the script — spec mandates single file, so the test harness extracts the `<script>` content from index.html and evaluates it in node with a `window` shim).
2. `test_tool.py` structure checks → build `index.html` DOM skeleton
3. Wire math core into DOM interactions (gallery, product table, line flip)
4. Rotor demo + hexagram stack + quiz
5. Docs + human-gate template

## Test harness note (single-file constraint)

`tests/run_js_tests.js` extracts the inline `<script>` from `index.html` (regex between `<script>` tags), creates a `window` shim (`global.window = {}`), evaluates the script, then runs the C1.x assertions against `window.BaguaTool`. This keeps the "one file, zero build" invariant while remaining testable.

## Definition of done

- `node tests/run_js_tests.js` → all C1.x green (≥ 15 cases)
- `python -m pytest tests/test_tool.py tests/test_docs.py -q` → green
- `python tests/verify_offline.py` → zero-network + single-file checks green
- README: open instructions, lesson plan link, human-gate thresholds (pre-registered), explicit "no semantic claims" statement
