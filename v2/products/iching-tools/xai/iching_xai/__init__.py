"""POC-12: interaction-spectrum XAI — Walsh-Hadamard / contrast transform.

Deterministic pure-algebra interaction recovery (the POC-07 math, reused
exactly: c_S = mean over design runs r of sign(S, r) * y_r). Stdlib only,
no model calls, no network.

Exposes:
    interaction_spectrum(points, values) -> {subset_mask: coefficient}
    identify(spectrum, tol=1e-6) -> [subset_mask]

Per SDD v2/docs/specs/poc-12-interaction-xai-mcp-spec.md.
"""

from __future__ import annotations

import itertools
import numbers

__all__ = ["XAIValidationError", "interaction_spectrum", "identify"]


class XAIValidationError(ValueError):
    """Typed error raised on any invalid input to the spectrum functions."""


def _is_scalar(v):
    """Numeric scalar check: accepts int/float (incl. numpy scalars); rejects bool."""
    return isinstance(v, numbers.Real) and not isinstance(v, bool)


def _validate_points(points):
    if not isinstance(points, (list, tuple)) or len(points) == 0:
        raise XAIValidationError("points must be a non-empty list of +-1 vectors")
    length = None
    for i, vec in enumerate(points):
        if not isinstance(vec, (list, tuple)) or len(vec) == 0:
            raise XAIValidationError(
                f"points[{i}] must be a non-empty vector of +-1 entries")
        if length is None:
            length = len(vec)
        elif len(vec) != length:
            raise XAIValidationError("all point vectors must have the same length")
        for j, v in enumerate(vec):
            if not _is_scalar(v) or (float(v) != 1.0 and float(v) != -1.0):
                raise XAIValidationError(
                    f"points[{i}][{j}] must be exactly +-1, got {v!r}")
    return length


def _validate_values(values, n_points):
    if not isinstance(values, (list, tuple)):
        raise XAIValidationError("values must be a list of numeric responses")
    if len(values) == 0:
        raise XAIValidationError("values must be non-empty")
    if len(values) != n_points:
        raise XAIValidationError(
            f"len(values)={len(values)} must equal len(points)={n_points}")
    out = []
    for i, y in enumerate(values):
        if not _is_scalar(y):
            raise XAIValidationError(f"values[{i}] must be numeric, got {y!r}")
        out.append(float(y))
    return out


def interaction_spectrum(points, values):
    """Interaction coefficients via the Walsh-Hadamard / contrast transform.

    points: list of +-1 vectors (a 2^k design), values: responses, one per
    point. Returns {subset_mask: coefficient} where
    c_S = mean over runs r of sign(S, r) * y_r and
    sign(S, r) = prod(r[i] for i in S); subset_mask = sum(1 << i for i in S).
    For a polynomial response on the full 2^k design this recovers the
    planted coefficients exactly.
    """
    length = _validate_points(points)
    ys = _validate_values(values, len(points))
    runs = [tuple(float(v) for v in vec) for vec in points]
    n_runs = len(runs)
    out = {}
    for size in range(length + 1):
        for S in itertools.combinations(range(length), size):
            mask = 0
            for i in S:
                mask |= 1 << i
            total = 0.0
            for r, y in zip(runs, ys):
                sign = 1.0
                for i in S:
                    sign *= r[i]
                total += sign * y
            out[mask] = total / n_runs
    return out


def identify(spectrum, tol=1e-6):
    """Subset masks whose |coefficient| > tol, sorted ascending."""
    if not isinstance(spectrum, dict):
        raise XAIValidationError(
            "spectrum must be a dict {subset_mask: coefficient}")
    if not _is_scalar(tol) or float(tol) < 0.0:
        raise XAIValidationError("tol must be a non-negative number")
    masks = []
    for m, c in spectrum.items():
        if (not isinstance(m, numbers.Integral) or isinstance(m, bool) or m < 0):
            raise XAIValidationError(
                f"spectrum keys must be non-negative subset masks, got {m!r}")
        if not _is_scalar(c):
            raise XAIValidationError(
                f"spectrum values must be numeric, got {c!r}")
        if abs(float(c)) > float(tol):
            masks.append(int(m))
    masks.sort()
    return masks
