"""T-03.2 Naming: LLM naming protocol, JSON validation, countable failures (AC-03.3)."""

import pytest

import moves
import naming
import statements


def test_03_2_1_name_position_returns_text(sim_engine):
    """T-03.2.1: name_position returns non-empty text for all 8 positions."""
    naming.set_engine(sim_engine)
    stmt = statements.STATEMENTS[0]["text"]
    for move_name, state in moves.all_positions(moves.START_STATE):
        name = naming.name_position(stmt, move_name, state)
        assert isinstance(name, str)
        assert name.strip()


def test_03_2_1_free_form_reframes_returns_eight(sim_engine):
    """Arm A protocol: one call returns exactly 8 reframes as a JSON array."""
    naming.set_engine(sim_engine)
    reframes = naming.free_form_reframes(statements.STATEMENTS[0]["text"])
    assert len(reframes) == 8
    assert all(isinstance(r, str) and r.strip() for r in reframes)


def test_03_2_2_name_position_failures_countable(garbage_engine):
    """T-03.2.2: bad LLM output is recorded (counted), not retried, returns ''."""
    naming.set_engine(garbage_engine)
    name = naming.name_position("a statement", "flip0", (0, +1))
    assert name == ""
    assert garbage_engine.failures["name_position"] == 1
    name2 = naming.name_position("another statement", "complement", (0, +1))
    assert name2 == ""
    assert garbage_engine.failures["name_position"] == 2


def test_03_2_2_free_form_failures_countable(garbage_engine):
    """Free-form protocol failure: [] returned, counted, not retried."""
    naming.set_engine(garbage_engine)
    assert naming.free_form_reframes("a statement") == []
    assert garbage_engine.failures["free_form"] == 1


def test_03_2_unconfigured_engine_raises():
    with pytest.raises(RuntimeError):
        naming.name_position("s", "flip0", (0, +1))
