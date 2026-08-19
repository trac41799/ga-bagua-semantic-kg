"""Stability metric: dominant-role consistency across re-tagging runs."""

from tags import dominant_role


def consistency(tag_sets):
    """Fraction of items whose dominant role matches across all runs.

    tag_sets: list of runs; each run is a list of per-item tag dicts
    (same item order across runs). Hand case: 8/10 matches -> 0.8.
    """
    runs = [list(r) for r in tag_sets]
    if len(runs) < 2:
        raise ValueError("consistency needs >= 2 runs")
    n = len(runs[0])
    if any(len(r) != n for r in runs):
        raise ValueError("runs have unequal length")
    if n == 0:
        return 0.0
    matches = sum(
        1 for i in range(n)
        if len({dominant_role(r[i]) for r in runs}) == 1
    )
    return matches / n
