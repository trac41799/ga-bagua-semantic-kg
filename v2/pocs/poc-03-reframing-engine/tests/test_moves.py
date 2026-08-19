"""T-03.1 Moves: the 8 exact cube operators over trigram states (AC-03.1, AC-03.2).

State = (blade_index, sign). The 8 positions of the cube are the 8 vertices of
the 3-bit cube: origin + 3 single-line flips + 3 double-line flips + the
complement (antipode, Hodge dual .e123).
"""

import pytest

import moves


@pytest.mark.parametrize("blade", range(8))
@pytest.mark.parametrize("sign", [+1, -1])
def test_03_1_1_all_positions_eight_distinct_states(blade, sign):
    """T-03.1.1: exactly 8 DISTINCT states for every trigram state."""
    pos = moves.all_positions((blade, sign))
    assert len(pos) == 8
    states = [s for _, s in pos]
    assert len(set(states)) == 8


def test_03_1_1_all_positions_ordered():
    """T-03.1.1: ordered (origin, 3 flips, 3 double-flips, 1 complement)."""
    names = [m for m, _ in moves.all_positions(moves.START_STATE)]
    assert names == ["origin", "flip0", "flip1", "flip2",
                     "double_flip01", "double_flip02", "double_flip12", "complement"]


@pytest.mark.parametrize("blade", range(8))
def test_03_1_2_flip_bit_level(blade):
    """T-03.1.2: 24 bit-level flip cases (8 blades x 3 lines), sign unchanged."""
    for line in (0, 1, 2):
        b, s = moves.flip((blade, +1), line)
        assert b == blade ^ (1 << line)
        assert s == +1
        b2, s2 = moves.flip((blade, -1), line)
        assert b2 == blade ^ (1 << line)
        assert s2 == -1


@pytest.mark.parametrize("state,expected", [
    ((2, +1), (5, -1)),   # Kan -> Li   (dual of e2 is -e13)
    ((4, +1), (3, +1)),   # Gen -> Dui  (dual of e3 is +e12)
    ((1, +1), (6, +1)),   # Zhen -> Xun (dual of e1 is +e23)
    ((0, +1), (7, +1)),   # Kun -> Qian (dual of 1 is +e123)
])
def test_03_1_3_complement_hodge_dual_natural_convention(state, expected):
    """T-03.1.3: complement = Hodge dual (.e123), natural convention, exact."""
    assert moves.complement(state) == expected


def test_03_1_3_complement_blade_mapping_is_bitwise_not():
    """All 8 blades: complement index = bitwise NOT of the 3-bit mask."""
    for blade in range(8):
        b, _ = moves.complement((blade, +1))
        assert b == blade ^ 0b111


def test_03_1_3_complement_sign_pattern_exact():
    """Dual signs derived from the geometric products (bit 1 of the index)."""
    for blade in range(8):
        b, s = moves.complement((blade, +1))
        expected_sign = +1 if (blade & 0b010) == 0 else -1
        assert s == expected_sign, f"blade {blade}"
        # complement is a sign-reversing involution; complement^4 = identity
        assert moves.complement((b, s)) == (blade, -1)
        assert moves.complement(moves.complement(moves.complement(moves.complement(
            (blade, +1))))) == (blade, +1)


@pytest.mark.parametrize("blade", range(8))
@pytest.mark.parametrize("sign", [+1, -1])
@pytest.mark.parametrize("pair", [(0, 1), (0, 2), (1, 2)])
def test_03_1_4_double_flip_is_two_flips_composed(blade, sign, pair):
    """T-03.1.4: double flip = composition of two flips (both orders)."""
    state = (blade, sign)
    a, b = pair
    assert moves.double_flip(state, a, b) == moves.flip(moves.flip(state, a), b)
    assert moves.double_flip(state, a, b) == moves.flip(moves.flip(state, b), a)


@pytest.mark.parametrize("blade", range(8))
@pytest.mark.parametrize("sign", [+1, -1])
def test_03_1_5_no_move_returns_identity_state(blade, sign):
    """T-03.1.5: the 8 positions are all distinct; no move returns the input."""
    state = (blade, sign)
    pos = moves.all_positions(state)
    for move_name, s in pos[1:]:
        assert s != state, f"move {move_name} returned the identity state"
    assert len({s for _, s in pos}) == 8


def test_03_1_5_degenerate_output_raises(monkeypatch):
    """Distinctness guard: a move returning the identity state must raise."""
    monkeypatch.setattr(moves, "flip", lambda state, line: state)
    with pytest.raises(ValueError):
        moves.all_positions((0, +1))


@pytest.mark.parametrize("bad_state", [(8, +1), (0, 2), (0, 0), (3, 5), (-1, +1)])
def test_03_1_invalid_states_raise(bad_state):
    with pytest.raises(ValueError):
        moves.flip(bad_state, 0)
    with pytest.raises(ValueError):
        moves.complement(bad_state)
    with pytest.raises(ValueError):
        moves.all_positions(bad_state)


@pytest.mark.parametrize("bad_line", [5, -1, 0.5, 3])
def test_03_1_invalid_lines_raise(bad_line):
    with pytest.raises(ValueError):
        moves.flip((0, +1), bad_line)
    with pytest.raises(ValueError):
        moves.double_flip((0, +1), bad_line, 1)


def test_03_1_double_flip_same_line_raises():
    with pytest.raises(ValueError):
        moves.double_flip((0, +1), 1, 1)


def test_03_1_start_state_is_kun():
    """Protocol choice: the neutral/empty trigram Kun is the cube's start vertex."""
    assert moves.START_STATE == (0, +1)
    assert moves.TRIGRAMS[moves.START_STATE[0]] == "Kun"


def test_03_1_describe_state_matches_repo_labels():
    assert moves.describe_state((5, +1)) == "Li (blade -e31, grade 2, sign +1)"
    assert moves.describe_state((0, -1)) == "Kun (blade 1, grade 0, sign -1)"
