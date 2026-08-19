"""Cl(3) geometric algebra core — ported EXACTLY from v1 ga-semantics-core (verified correct).

Blade order: 0=1, 1=e1, 2=e2, 3=e3, 4=e12, 5=e23, 6=e31, 7=e123
"""

import numpy as np

# PROD_TABLE[i][j] = (result_index, sign) — verified against v1 multivector.rs
PROD_TABLE = [
    [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1)],
    [(1, 1), (0, 1), (4, 1), (6, -1), (2, 1), (7, 1), (3, -1), (5, 1)],
    [(2, 1), (4, -1), (0, 1), (5, 1), (1, -1), (3, 1), (7, 1), (6, 1)],
    [(3, 1), (6, 1), (5, -1), (0, 1), (7, 1), (2, -1), (1, 1), (4, 1)],
    [(4, 1), (2, -1), (1, 1), (7, 1), (0, -1), (6, -1), (5, 1), (3, -1)],
    [(5, 1), (7, 1), (3, -1), (2, 1), (6, 1), (0, -1), (4, -1), (1, -1)],
    [(6, 1), (3, 1), (7, 1), (1, -1), (5, -1), (4, 1), (0, -1), (2, -1)],
    [(7, 1), (5, 1), (6, 1), (4, 1), (3, -1), (1, -1), (2, -1), (0, -1)],
]

BLADE_NAMES = ["1", "e1", "e2", "e3", "e12", "e23", "e31", "e123"]


class Multivector:
    def __init__(self, coeffs):
        self.c = np.asarray(coeffs, dtype=float)

    def __getitem__(self, i):
        return self.c[i]

    def __mul__(self, other):
        if isinstance(other, Multivector):
            return self.geo_product(other)
        return Multivector(self.c * other)

    def __rmul__(self, other):
        return Multivector(self.c * other)

    def reverse(self):
        r = self.c.copy()
        r[4:] *= -1
        return Multivector(r)

    def norm(self):
        return float(np.linalg.norm(self.c))

    def normalized(self):
        n = self.norm()
        return Multivector(self.c / n) if n > 1e-12 else Multivector([1.0, 0, 0, 0, 0, 0, 0, 0])

    def geo_product(self, other):
        a = self.c
        b = other.c
        result = np.zeros(8)
        for i in range(8):
            if a[i] == 0.0:
                continue
            row = PROD_TABLE[i]
            for j in range(8):
                if b[j] == 0.0:
                    continue
                k, s = row[j]
                result[k] += a[i] * b[j] * s
        return Multivector(result)

    def inner_product(self, other):
        return float(self.geo_product(other.reverse()).c[0])

    def dominant_blade(self):
        return int(np.argmax(np.abs(self.c)))


def grade_spectrum(a, b):
    """Normalized grade magnitudes of the geometric product a*b: [scalar, vector, bivector, trivector]."""
    gp = a.geo_product(b)
    g0 = float(np.linalg.norm(gp.c[[0]]))
    g1 = float(np.linalg.norm(gp.c[[1, 2, 3]]))
    g2 = float(np.linalg.norm(gp.c[[4, 5, 6]]))
    g3 = float(np.linalg.norm(gp.c[[7]]))
    total = g0 + g1 + g2 + g3
    if total < 1e-12:
        return np.zeros(4)
    return np.array([g0, g1, g2, g3]) / total
