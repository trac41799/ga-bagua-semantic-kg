"""Cosine retrieval + IR baselines (TF-IDF, BM25) + seeded random (Path D)."""

import math
import re
from collections import Counter

import numpy as np


def cosine_topk(encodings: np.ndarray, query_vec: np.ndarray, k: int):
    """Return list of (id, score) sorted descending. encodings: (N,8) row-normalized."""
    q = query_vec / (np.linalg.norm(query_vec) + 1e-12)
    scores = encodings @ q
    order = np.argsort(-scores)[:k]
    return [(int(i), float(scores[i])) for i in order]


def tokenize(text: str) -> list:
    return re.findall(r"[a-z0-9]+", text.lower())


class TFIDF:
    def __init__(self, docs):
        self.docs = [tokenize(d) for d in docs]
        n = len(self.docs)
        df = Counter()
        for toks in self.docs:
            df.update(set(toks))
        self.idf = {w: math.log((n + 1) / (c + 1)) + 1.0 for w, c in df.items()}
        self.vocab = {w: i for i, w in enumerate(self.idf)}

    def _vec(self, toks):
        v = np.zeros(len(self.vocab))
        tf = Counter(toks)
        n = max(len(toks), 1)
        for w, c in tf.items():
            if w in self.vocab:
                v[self.vocab[w]] = (c / n) * self.idf[w]
        return v

    def scores(self, query: str):
        qv = self._vec(tokenize(query))
        qn = np.linalg.norm(qv)
        if qn < 1e-12:
            return np.zeros(len(self.docs))
        out = []
        for i, toks in enumerate(self.docs):
            d = self._vec(toks)
            out.append(float(d @ qv / (np.linalg.norm(d) * qn + 1e-12)))
        return np.array(out)


class BM25:
    def __init__(self, docs, k1=1.5, b=0.75):
        self.docs = [tokenize(d) for d in docs]
        n = len(self.docs)
        self.avgdl = sum(len(t) for t in self.docs) / max(n, 1)
        self.k1 = k1
        self.b = b
        df = Counter()
        for toks in self.docs:
            df.update(set(toks))
        self.idf = {w: math.log((n - c + 0.5) / (c + 0.5) + 1.0) for w, c in df.items()}

    def scores(self, query: str):
        qt = tokenize(query)
        out = np.zeros(len(self.docs))
        for i, toks in enumerate(self.docs):
            dl = max(len(toks), 1)
            tf = Counter(toks)
            s = 0.0
            for w in qt:
                if w in tf and w in self.idf:
                    f = tf[w]
                    s += self.idf[w] * f * (self.k1 + 1) / (f + self.k1 * (1 - self.b + self.b * dl / self.avgdl))
            out[i] = s
        return out


def topk_from_scores(scores: np.ndarray, k: int):
    order = np.argsort(-scores)[:k]
    return [int(i) for i in order]


def random_topk(n: int, k: int, seed: int):
    rng = np.random.default_rng(seed)
    return rng.choice(n, size=min(k, n), replace=False).tolist()
