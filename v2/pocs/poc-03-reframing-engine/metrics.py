"""Metrics for POC-03 (spec interface).

diversity(statements) -> float
    Mean pairwise cosine DISTANCE on rubric-encoded 8-dim vectors
    (1 - cosine; identical vectors -> 0, orthogonal -> 1). Statements are
    encoded through the injected engine (one LLM call per statement, counted
    in the ledger); vectors may also be passed directly for hand-computed
    cases. Sets with fewer than 2 usable vectors contribute 0.0.

coherence(statement, reframes) -> float
    LLM judge, 1-5, one call per set. Returns the engine's score (clamped to
    [1, 5]); 0.0 on judge protocol failure (recorded on the engine, not
    retried). Deterministic on cached responses.
"""

import numpy as np

from llm_client import RealEngine

_engine = None


def set_engine(engine):
    """Set (or clear, with None) the LLM engine used by the metrics layer."""
    global _engine
    _engine = engine


def _require_engine():
    if _engine is None:
        raise RuntimeError("metrics engine not configured: call set_engine() first")
    return _engine


def diversity(statements, vectors=None) -> float:
    """Mean pairwise cosine distance on rubric-encoded vectors (1 - cosine)."""
    if vectors is None:
        eng = _require_engine()
        vectors = [v for v in (eng.encode(s) for s in statements) if v is not None]
    else:
        vectors = [v for v in vectors if v is not None]
    n = len(vectors)
    if n < 2:
        return 0.0
    total = 0.0
    count = 0
    for i in range(n):
        vi = np.asarray(vectors[i], dtype=float)
        for j in range(i + 1, n):
            vj = np.asarray(vectors[j], dtype=float)
            ni, nj = np.linalg.norm(vi), np.linalg.norm(vj)
            if ni < 1e-12 or nj < 1e-12:
                continue
            cos = float(np.dot(vi, vj) / (ni * nj))
            total += 1.0 - cos
            count += 1
    return total / count if count else 0.0


def coherence(statement: str, reframes) -> float:
    """LLM-judged coherence of a reframe set, 1-5 (0.0 on judge failure)."""
    eng = _require_engine()
    score = float(eng.judge(statement, reframes))
    if score == 0.0:
        return 0.0
    return float(min(5.0, max(1.0, score)))
