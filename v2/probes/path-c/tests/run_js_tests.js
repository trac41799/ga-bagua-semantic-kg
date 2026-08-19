'use strict';

const fs = require('fs');
const path = require('path');

const ROOT = path.join(__dirname, '..');
const HTML_PATH = path.join(ROOT, 'index.html');

const html = fs.readFileSync(HTML_PATH, 'utf8');
const scriptMatch = html.match(/<script>([\s\S]*?)<\/script>/);
if (!scriptMatch) {
  console.error('FATAL: no inline <script> found in index.html');
  process.exit(1);
}

global.window = {};
eval(scriptMatch[1]);

const BT = window.BaguaTool;
if (!BT || typeof BT.product !== 'function' || typeof BT.flipLine !== 'function' ||
    typeof BT.hexagram !== 'function' || typeof BT.rotorApply !== 'function' ||
    typeof BT.quizScore !== 'function') {
  console.error('FATAL: window.BaguaTool not properly exposed');
  process.exit(1);
}

let pass = 0;
let fail = 0;
const failures = [];

function check(cond, label) {
  if (cond) {
    pass++;
  } else {
    fail++;
    failures.push(label);
    console.log('  FAIL: ' + label);
  }
}

function eq(a, b) {
  return JSON.stringify(a) === JSON.stringify(b);
}

function labelOf(mask) {
  const b = BT.bladeOf(mask);
  return (b.sign < 0 ? '-' : '') + b.blade;
}

console.log('T-C1: Math core tests (extracted inline script, window shim)');
console.log('-------------------------------------------------------------');

console.log('C1.1  product(e1,e2) == {blade:e12, sign:+1}');
check(eq(BT.product('e1', 'e2'), { blade: 'e12', sign: 1 }), 'C1.1 product(e1,e2)');

console.log('C1.2  product(e2,e1) == {blade:e12, sign:-1}');
check(eq(BT.product('e2', 'e1'), { blade: 'e12', sign: -1 }), 'C1.2 product(e2,e1)');

console.log('C1.3  product(e1,e1) == {blade:1, sign:+1}');
check(eq(BT.product('e1', 'e1'), { blade: '1', sign: 1 }), 'C1.3 product(e1,e1)');

console.log('C1.4  product(e12,e12) == {blade:1, sign:-1}');
check(eq(BT.product('e12', 'e12'), { blade: '1', sign: -1 }), 'C1.4 product(e12,e12)');

console.log('C1.5  product(e123,e123) == {blade:1, sign:-1}');
check(eq(BT.product('e123', 'e123'), { blade: '1', sign: -1 }), 'C1.5 product(e123,e123)');

console.log('C1.6  full 8x8 product table vs verified v1 PROD_TABLE (64 checks)');
const REF_TABLE = [
  [[0, 1], [1, 1], [2, 1], [3, 1], [4, 1], [5, 1], [6, 1], [7, 1]],
  [[1, 1], [0, 1], [4, 1], [6, -1], [2, 1], [7, 1], [3, -1], [5, 1]],
  [[2, 1], [4, -1], [0, 1], [5, 1], [1, -1], [3, 1], [7, 1], [6, 1]],
  [[3, 1], [6, 1], [5, -1], [0, 1], [7, 1], [2, -1], [1, 1], [4, 1]],
  [[4, 1], [2, -1], [1, 1], [7, 1], [0, -1], [6, -1], [5, 1], [3, -1]],
  [[5, 1], [7, 1], [3, -1], [2, 1], [6, 1], [0, -1], [4, -1], [1, -1]],
  [[6, 1], [3, 1], [7, 1], [1, -1], [5, -1], [4, 1], [0, -1], [2, -1]],
  [[7, 1], [5, 1], [6, 1], [4, 1], [3, -1], [1, -1], [2, -1], [0, -1]]
];
let c16ok = true;
const c16bad = [];
for (let i = 0; i < 8; i++) {
  for (let j = 0; j < 8; j++) {
    const got = BT.product(BT.BLADES[i], BT.BLADES[j]);
    const expected = { blade: BT.BLADES[REF_TABLE[i][j][0]], sign: REF_TABLE[i][j][1] };
    if (!eq(got, expected)) {
      c16ok = false;
      c16bad.push(BT.BLADES[i] + 'x' + BT.BLADES[j] + ' got ' + JSON.stringify(got) + ' want ' + JSON.stringify(expected));
    }
  }
}
check(c16ok, 'C1.6 all 64 entries');
c16bad.forEach(function (b) { console.log('    mismatch: ' + b); });

console.log('C1.7  flipLine: all 24 line flips (bit XOR) + 24 involution checks');
let c17ok = true;
for (let t = 0; t < 8; t++) {
  for (let li = 0; li < 3; li++) {
    if (BT.flipLine(t, li) !== (t ^ (1 << li))) {
      c17ok = false;
      console.log('    mismatch flipLine(' + t + ',' + li + ')');
    }
    if (BT.flipLine(BT.flipLine(t, li), li) !== t) {
      c17ok = false;
      console.log('    non-involutive flipLine(' + t + ',' + li + ')');
    }
  }
}
check(c17ok, 'C1.7 all 24 flipLine cases (24 XOR + 24 involution)');

console.log('C1.8  hexagram names: 10 spot-checks');
function hexCheck(u, l, name, num) {
  const h = BT.hexagram(u, l);
  check(h && h.name === name, 'C1.8 hexagram(' + u + ',' + l + ') = ' + name + ' got ' + (h && h.name));
  check(h && h.number === num, 'C1.8 hexagram number for ' + name + ' = ' + num + ' got ' + (h && h.number));
}
hexCheck(0, 0, '地地坤', 2);
hexCheck(7, 7, '乾乾', 1);
hexCheck(2, 5, '水火既济', 63);
hexCheck(5, 2, '火水未济', 64);
hexCheck(2, 1, '水雷屯', 3);
hexCheck(0, 7, '地天泰', 11);
hexCheck(7, 0, '天地否', 12);
hexCheck(7, 3, '天泽履', 10);
hexCheck(4, 5, '山火贲', 22);
hexCheck(1, 0, '雷地豫', 16);

console.log('C1.9  rotor theta=pi in e12 maps e1 -> -e1');
{
  const r = BT.rotorApply(Math.PI, 'e12', 'e1');
  check(r.blade === 'e1' && r.sign === -1, 'C1.9 pi/e12/e1 => -e1 (got ' + JSON.stringify(r) + ')');
}

console.log('C1.10  rotor theta=pi/2 in e12 maps e1 -> e2');
{
  const r = BT.rotorApply(Math.PI / 2, 'e12', 'e1');
  check(r.blade === 'e2' && r.sign === 1, 'C1.10 pi/2/e12/e1 => e2 (got ' + JSON.stringify(r) + ')');
}

console.log('C1.11  rotor sandwich preserves norm |a\'| == |a| (6 combos)');
{
  let ok = true;
  const combos = [
    [Math.PI, 'e12', 'e1'], [Math.PI / 2, 'e23', 'e2'], [Math.PI / 4, 'e31', 'e3'],
    [Math.PI / 3, 'e12', 'e3'], [1.2, 'e23', 'e1'], [Math.PI * 0.75, 'e31', 'e2']
  ];
  combos.forEach(function (c) {
    const r = BT.rotorApply(c[0], c[1], c[2]);
    if (Math.abs(r.norm - 1) > 1e-9) {
      ok = false;
      console.log('    norm mismatch: ' + JSON.stringify(c) + ' -> ' + r.norm);
    }
  });
  check(ok, 'C1.11 norm preserved (6 combos)');
}

console.log('EXTRA  bladeOf: grade == popcount(yang lines), all 8 trigrams');
{
  let ok = true;
  for (let t = 0; t < 8; t++) {
    const b = BT.bladeOf(t);
    if (b.grade !== BT.bladeOf(t).grade) { ok = false; }
  }
  const expectedGrades = [0, 1, 1, 2, 1, 2, 2, 3];
  for (let t = 0; t < 8; t++) {
    if (BT.bladeOf(t).grade !== expectedGrades[t]) {
      ok = false;
      console.log('    grade mismatch trigram ' + t + ' = ' + BT.bladeOf(t).grade);
    }
  }
  check(ok, 'EXTRA grades 0,1,1,2,1,2,2,3 for masks 0..7');
}

console.log('EXTRA  trigram -> blade mapping (with orientation sign)');
{
  const expected = [
    { blade: '1', sign: 1 }, { blade: 'e1', sign: 1 }, { blade: 'e2', sign: 1 },
    { blade: 'e12', sign: 1 }, { blade: 'e3', sign: 1 }, { blade: 'e31', sign: -1 },
    { blade: 'e23', sign: 1 }, { blade: 'e123', sign: 1 }
  ];
  let ok = true;
  for (let t = 0; t < 8; t++) {
    const b = BT.bladeOf(t);
    if (b.blade !== expected[t].blade || b.sign !== expected[t].sign) {
      ok = false;
      console.log('    blade mismatch trigram ' + t + ' = ' + JSON.stringify(b));
    }
  }
  check(ok, 'EXTRA blade map 1,e1,e2,e12,e3,-e31,e23,e123');
}

console.log('EXTRA  product spot-checks');
check(eq(BT.product('e1', 'e123'), { blade: 'e23', sign: 1 }), 'EXTRA e1*e123 = e23');
check(eq(BT.product('e123', 'e1'), { blade: 'e23', sign: 1 }), 'EXTRA e123*e1 = e23');
check(eq(BT.product('e2', 'e3'), { blade: 'e23', sign: 1 }), 'EXTRA e2*e3 = e23');
check(eq(BT.product('e3', 'e2'), { blade: 'e23', sign: -1 }), 'EXTRA e3*e2 = -e23');
check(eq(BT.product('e1', 'e3'), { blade: 'e31', sign: -1 }), 'EXTRA e1*e3 = -e31');

console.log('EXTRA  rotor extra cases');
{
  const r = BT.rotorApply(Math.PI, 'e12', 'e2');
  check(r.blade === 'e2' && r.sign === -1, 'EXTRA pi/e12/e2 => -e2');
  const r2 = BT.rotorApply(2 * Math.PI, 'e12', 'e1');
  check(r2.blade === 'e1' && r2.sign === 1, 'EXTRA 2pi/e12/e1 => e1');
  const r3 = BT.rotorApply(Math.PI / 2, 'e12', 'e3');
  check(r3.blade === 'e3' && r3.sign === 1, 'EXTRA pi/2/e12/e3 => e3 (perpendicular fixed)');
}

console.log('EXTRA  quizScore');
check(BT.quizScore([2, 1, 1, 0, 1]) === 5, 'EXTRA quizScore all-correct = 5/5');
check(BT.quizScore([0, 0, 0, 1, 0]) === 0, 'EXTRA quizScore all-wrong = 0/5');
check(BT.quizScore([2, 1, 1]) === 3, 'EXTRA quizScore partial = 3/5');
check(BT.quizScore([]) === 0, 'EXTRA quizScore empty = 0');

console.log('-------------------------------------------------------------');
console.log('RESULT: ' + pass + ' assertions passed, ' + fail + ' failed');
if (fail > 0) {
  console.log('FAILED CASES:');
  failures.forEach(function (f) { console.log('  - ' + f); });
  process.exit(1);
}
console.log('ALL T-C1 TESTS GREEN');
