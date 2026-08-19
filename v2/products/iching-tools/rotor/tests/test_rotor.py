"""POC-14 tests: rotor state algebra.

Covers TDD R1 (hand cases), R2 (100 seeded random chains), R3 (strict
validation), R6 (import audit, README zero-LLM statement). Deterministic;
no network access, no LLM calls.
"""

import math
import os
import random

import pytest

import iching_rotor as ir
from iching_rotor import apply, compose, distance, evaluate, invert, rotor

I = ir.IDENTITY
PI = math.pi


def _rand_rotor(rng):
    """Uniform-ish random unit rotor (any unit even multivector is a rotor)."""
    while True:
        v = [rng.gauss(0.0, 1.0) for _ in range(4)]
        n = math.sqrt(sum(x * x for x in v))
        if n > 1e-12:
            return tuple(x / n for x in v)


def _norm(r):
    return math.sqrt(sum(x * x for x in r))


# ---------------------------------------------------------------- R1: hand cases

class TestR1HandCases:
    def test_apply_pi_over_2_e12_maps_e1_to_e2(self):
        assert apply(rotor(PI / 2, "e12"), "e1") == "e2"

    def test_apply_pi_over_2_e12_maps_e2_to_neg_e1(self):
        assert apply(rotor(PI / 2, "e12"), "e2") == "-e1"

    def test_apply_pi_e12_maps_e1_to_neg_e1(self):
        assert apply(rotor(PI, "e12"), "e1") == "-e1"

    def test_apply_pi_e12_maps_e2_to_neg_e2(self):
        assert apply(rotor(PI, "e12"), "e2") == "-e2"

    def test_apply_pi_over_2_e23(self):
        assert apply(rotor(PI / 2, "e23"), "e2") == "e3"

    def test_apply_pi_over_2_e31(self):
        assert apply(rotor(PI / 2, "e31"), "e3") == "e1"

    def test_apply_identity_blade(self):
        assert apply(I, "e1") == "e1"
        assert apply(I, "e123") == "e123"
        assert apply(I, "1") == "1"

    def test_apply_scalar_blade_invariant(self):
        for th in (PI / 3, PI / 2, PI):
            assert apply(rotor(th, "e23"), "1") == "1"

    def test_apply_rotor_roundtrip(self):
        r = rotor(PI / 4, "e31")
        assert apply(invert(r), apply(r, "e2")) == "e2"

    def test_invert_roundtrip_left(self):
        r = rotor(1.1, "e12")
        back = compose(invert(r), r)
        assert _norm((back[0] - 1.0, back[1], back[2], back[3])) <= 1e-12

    def test_invert_roundtrip_right(self):
        r = rotor(1.1, "e12")
        back = compose(r, invert(r))
        assert _norm((back[0] - 1.0, back[1], back[2], back[3])) <= 1e-12

    def test_invert_is_reverse(self):
        r = (0.3, 0.4, 0.5, math.sqrt(1.0 - 0.09 - 0.16 - 0.25))
        assert invert(r) == (r[0], -r[1], -r[2], -r[3])

    def test_distance_identity_is_zero(self):
        r = rotor(0.7, "e23")
        assert distance(r, r) == 0.0

    def test_distance_identity_vs_identity_is_zero(self):
        assert distance(I, I) == 0.0

    def test_distance_distinct_rotors_positive(self):
        assert distance(I, rotor(0.3, "e12")) > 0.0

    def test_distance_triangle_property(self):
        a, b, c = rotor(0.4, "e12"), rotor(0.9, "e23"), rotor(1.5, "e31")
        assert distance(a, c) <= distance(a, b) + distance(b, c) + 1e-12

    def test_compose_renormalizes(self):
        a, b = rotor(1.2, "e12"), rotor(0.8, "e23")
        assert abs(_norm(compose(a, b)) - 1.0) <= 1e-12

    def test_compose_hand_value(self):
        # two pi/2 e12 rotations = pi rotation: e1 -> -e1
        half = rotor(PI / 2, "e12")
        assert apply(compose(half, half), "e1") == "-e1"


# ---------------------------------------------------------------- R2: random chains

class TestR2RandomChains:
    def test_100_chains_unit_norm_inverse_roundtrip(self):
        rng = random.Random(42)
        for _ in range(100):
            depth = rng.randint(1, 10)
            r = _rand_rotor(rng)
            for _ in range(depth):
                r = compose(r, _rand_rotor(rng))
                assert abs(_norm(r) - 1.0) <= 1e-12, "unit norm lost in chain"
            back = compose(r, invert(r))
            assert _norm((back[0] - 1.0, back[1], back[2], back[3])) <= 1e-12
            back2 = compose(invert(r), r)
            assert _norm((back2[0] - 1.0, back2[1], back2[2], back2[3])) <= 1e-12

    def test_100_chains_evaluate_pipeline(self):
        rng = random.Random(7)
        for _ in range(100):
            depth = rng.randint(1, 10)
            r = _rand_rotor(rng)
            ops = []
            for _ in range(depth):
                step = _rand_rotor(rng)
                ops.append({"op": "compose", "r1": list(r), "r2": list(step)})
                r = compose(r, step)
            ops.append({"op": "invert", "r": list(r)})
            ops.append({"op": "distance", "r1": list(r), "r2": list(r)})
            res = evaluate(ops)
            assert len(res) == depth + 2
            inv = res[depth]
            assert _norm((inv[0] - r[0], inv[1] + r[1], inv[2] + r[2], inv[3] + r[3])) <= 1e-12
            assert res[depth + 1] == 0.0

    def test_100_random_triples_associative(self):
        rng = random.Random(123)
        for _ in range(100):
            a, b, c = _rand_rotor(rng), _rand_rotor(rng), _rand_rotor(rng)
            lhs = compose(compose(a, b), c)
            rhs = compose(a, compose(b, c))
            assert _norm(tuple(lhs[k] - rhs[k] for k in range(4))) <= 1e-12

    def test_apply_roundtrip_for_axis_aligned_rotors(self):
        # rotations by +/-pi/2 and pi in any plane map canonical blades to
        # canonical blades, so the name round-trip is exact for all blades
        rng = random.Random(5)
        for _ in range(100):
            plane = rng.choice(["e12", "e23", "e31"])
            theta = rng.choice([-PI / 2, PI / 2, PI])
            r = rotor(theta, plane)
            for blade in ir.BLADE_NAMES:
                assert apply(invert(r), apply(r, blade)) == blade

    def test_apply_exact_or_error_mixed_result_raises(self):
        # a general rotation maps e1 to a mixed vector (not a canonical
        # blade); apply must refuse to name it rather than lie
        with pytest.raises(ValueError):
            apply(rotor(0.3, "e12"), "e1")

    def test_apply_random_rotor_blade_invariants(self):
        # e123 (pseudoscalar, center of Cl(3)) and the scalar 1 are fixed
        # by every rotor, so apply answers exactly for any rotor
        rng = random.Random(9)
        for _ in range(100):
            r = _rand_rotor(rng)
            assert apply(r, "e123") == "e123"
            assert apply(r, "1") == "1"


# ---------------------------------------------------------------- R3: strict validation

class TestR3StrictValidation:
    def test_unknown_op_raises(self):
        with pytest.raises(ValueError):
            evaluate([{"op": "frobnicate", "r": list(I)}])

    def test_missing_op_key_raises(self):
        with pytest.raises(ValueError):
            evaluate([{"r": list(I)}])

    def test_non_dict_op_raises(self):
        with pytest.raises(ValueError):
            evaluate([42])

    def test_ops_not_a_list_raises(self):
        with pytest.raises(ValueError):
            evaluate({"op": "invert", "r": list(I)})

    def test_non_unit_rotor_raises(self):
        with pytest.raises(ValueError):
            evaluate([{"op": "invert", "r": [1.0, 0.5, 0.0, 0.0]}])

    def test_wrong_length_raises(self):
        with pytest.raises(ValueError):
            evaluate([{"op": "invert", "r": [1.0, 0.0, 0.0]}])
        with pytest.raises(ValueError):
            evaluate([{"op": "invert", "r": [1.0, 0.0, 0.0, 0.0, 0.0]}])

    def test_non_numeric_component_raises(self):
        with pytest.raises(ValueError):
            evaluate([{"op": "invert", "r": ["a", 0.0, 0.0, 0.0]}])
        with pytest.raises(ValueError):
            evaluate([{"op": "invert", "r": [True, 0.0, 0.0, 0.0]}])

    def test_non_finite_component_raises(self):
        with pytest.raises(ValueError):
            evaluate([{"op": "invert", "r": [float("nan"), 0.0, 0.0, 0.0]}])
        with pytest.raises(ValueError):
            evaluate([{"op": "invert", "r": [float("inf"), 0.0, 0.0, 0.0]}])

    def test_missing_arg_raises(self):
        with pytest.raises(ValueError):
            evaluate([{"op": "compose", "r1": list(I)}])
        with pytest.raises(ValueError):
            evaluate([{"op": "distance", "r1": list(I), "r2": list(I), "blade": "e1"}])

    def test_unknown_blade_raises(self):
        with pytest.raises(ValueError):
            evaluate([{"op": "apply", "r": list(I), "blade": "e4"}])

    def test_no_partial_execution(self):
        # valid op first, invalid op second: the whole call must raise,
        # i.e. nothing is executed before validation completes.
        bad = [{"op": "invert", "r": list(I)},
               {"op": "compose", "r1": [1.0, 0.5, 0.0, 0.0], "r2": list(I)}]
        with pytest.raises(ValueError):
            evaluate(bad)
        with pytest.raises(ValueError):
            evaluate([{"op": "invert", "r": list(I)}, {"op": "nope", "r": list(I)}])

    def test_no_operation_executes_before_full_validation(self, monkeypatch):
        calls = []
        real_invert = ir.invert

        def spy_invert(r):
            calls.append(r)
            return real_invert(r)

        monkeypatch.setattr(ir, "invert", spy_invert)
        ops = [{"op": "invert", "r": list(I)},
               {"op": "compose", "r1": [1.0, 0.5, 0.0, 0.0], "r2": list(I)}]

        with pytest.raises(ValueError):
            evaluate(ops)

        assert calls == []

    def test_empty_ops_returns_empty(self):
        assert evaluate([]) == []

    def test_unknown_plane_raises(self):
        with pytest.raises(ValueError):
            rotor(1.0, "e13")


# ---------------------------------------------------------------- R6: import audit / zero-LLM

class TestR6ZeroLLM:
    FORBIDDEN = [
        "openai", "anthropic", "cohere", "google.generativeai", "ollama",
        "langchain", "llama", "transformers", "torch", "requests",
        "httpx", "urllib", "aiohttp", "socket", "http.client",
    ]

    def test_no_forbidden_imports_in_package_sources(self):
        here = os.path.dirname(os.path.abspath(__file__))
        pkg_dir = os.path.join(os.path.dirname(here), "iching_rotor")
        files = sorted(os.listdir(pkg_dir))
        assert files, "package directory must contain source files"
        for name in files:
            if not name.endswith(".py"):
                continue
            with open(os.path.join(pkg_dir, name), encoding="utf-8") as fh:
                src = fh.read()
            for bad in self.FORBIDDEN:
                assert bad not in src, "%s must not import/mention %s" % (name, bad)

    def test_cl3_core_copy_present_and_importable(self):
        here = os.path.dirname(os.path.abspath(__file__))
        core = os.path.join(os.path.dirname(here), "cl3.py")
        assert os.path.isfile(core), "verified Cl(3) core cl3.py must be copied into rotor/"
        with open(core, encoding="utf-8") as fh:
            assert "PROD_TABLE" in fh.read()

    def test_prod_table_matches_ground_truth(self):
        from cl3 import PROD_TABLE as GROUND_TRUTH
        assert ir.PROD_TABLE == GROUND_TRUTH

    def test_readme_contains_zero_llm_statement(self):
        here = os.path.dirname(os.path.abspath(__file__))
        readme = os.path.join(os.path.dirname(here), "README.md")
        with open(readme, encoding="utf-8") as fh:
            text = fh.read()
        assert "zero-LLM" in text or "zero LLM" in text
