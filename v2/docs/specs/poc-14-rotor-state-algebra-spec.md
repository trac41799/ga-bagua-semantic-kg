# SDD — POC-14: Rotor Transition Algebra MCP Tool

**Status:** Pre-registered | **Purpose:** exact, composable, invertible state-transition algebra for agents (Tier-3 candidate): rotors as transitions, composition = rotor product, inverse = reverse, distance = norm of difference. The algebra is the validated Cl(3) family; semantics are assigned by the LLM, never derived.

## 1. Pre-registration
| Item | Commitment |
|------|------------|
| Tool contract | `rotor_transition(ops)` — ops: JSON list of {op, args}; ops: `compose(r1, r2)`, `invert(r)`, `apply(r, blade)` (sandwich a' = R a R̃), `distance(r1, r2)`; rotors as [s, b12, b23, b31] (unit-normalized); blades by name |
| Correctness | Composition associativity, invert round-trip, apply exactness (π/2 in e12 maps e1→e2, π maps e1→−e1), unit-norm preservation — all exact (≤1e-12), deterministic, no LLM |
| Composition closure | 100 random rotor chains (depth ≤ 10) stay unit-norm and round-trip inverse exactly |
| Kill | Any exactness failure → dies |

## 2. Interfaces
`iching_rotor/__init__.py`: `compose(r1,r2)`, `invert(r)`, `apply(r, blade_name)`, `distance(r1,r2)`, strict JSON op validation. MCP tool registration.

## 3. ACs (TDD `poc-14-rotor-state-algebra-tdd.md`)
- R1 rotor math exactness (compose/invert/apply/distance hand cases)
- R2 100-chain closure + unit-norm + inverse round-trip (≤1e-12)
- R3 strict op validation
- R4 MCP tools/list + tools/call (sim)
- R5 MCP-SDK client call
- R6 tests green; zero LLM calls
