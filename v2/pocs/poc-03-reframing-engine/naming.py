"""LLM naming layer for POC-03 (spec interface).

name_position(statement, move_name, state) -> str   -- one LLM call per position
free_form_reframes(statement) -> list[str]          -- Arm A: one call, 8 reframes

The engine (RealEngine or SimulatedLLM) is injected via set_engine(); run_all.py
sets it, tests set it. Failures are recorded on the engine (counted, not
retried) and surface as "" / [] from these functions.
"""

from llm_client import RealEngine

import moves

_engine = None


def set_engine(engine):
    """Set (or clear, with None) the LLM engine used by the naming layer."""
    global _engine
    _engine = engine


def _require_engine():
    if _engine is None:
        raise RuntimeError("naming engine not configured: call set_engine() first")
    return _engine


def name_position(statement: str, move_name: str, state) -> str:
    """Name one cube position: LLM call with the exact move + state.

    Returns the named reframe (non-empty string) or "" on protocol failure
    (failure recorded on the engine, never retried)."""
    eng = _require_engine()
    state_desc = f"{moves.MOVE_DESCRIPTIONS[move_name]} | state: {moves.describe_state(state)}"
    return eng.name_position(statement, move_name, state_desc)


def free_form_reframes(statement: str) -> list:
    """Arm A: one LLM call returning exactly 8 free-form reframes (JSON array).

    Returns [] on protocol failure (recorded, not retried)."""
    eng = _require_engine()
    return eng.free_form_reframes(statement)
