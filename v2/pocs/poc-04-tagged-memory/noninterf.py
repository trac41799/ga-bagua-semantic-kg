"""Non-interference: the tag layer must NEVER change retrieval ranking.

Ranking invariance is byte-level: the ranked id list with tags attached must
equal the ranked id list without tags. `cosine_retrieval` is a TF-IDF cosine
stand-in over (name, description) text only — retrieval quality is NOT the
claim here; ranking invariance is. The 8 roles are tags, never the embedding.
"""

import re

import numpy as np


def _tokenize(text):
    return [t for t in re.split(r"[^a-z0-9]+", text.lower()) if t]


def _tfidf_matrix(docs):
    vocab = {}
    dfs = []
    for doc in docs:
        tokens = _tokenize(doc)
        uniq = set(tokens)
        dfs.append(uniq)
        for t in uniq:
            vocab.setdefault(t, len(vocab))
    n = len(docs)
    idf = {t: 1.0 + np.log((1.0 + n) / (1.0 + sum(1 for d in dfs if t in d)))
           for t in vocab}
    mat = np.zeros((n, len(vocab)))
    for i, doc in enumerate(docs):
        for t in _tokenize(doc):
            if t in vocab:
                mat[i, vocab[t]] += idf[t]
    return mat, vocab


def cosine_retrieval(query, items):
    """Rank items by TF-IDF cosine similarity to the query.

    items: list of (id, name, description, tags_or_none) — the tags field is
    deliberately never read. Returns ranked list of ids (ties -> id asc).
    """
    docs = [f"{name} {description}" for _cid, name, description, _tags in items]
    mat, vocab = _tfidf_matrix(docs)
    qvec = np.zeros(len(vocab))
    for t in _tokenize(query):
        if t in vocab:
            qvec[vocab[t]] += 1.0
    qn = np.linalg.norm(qvec)
    if qn == 0.0:
        return sorted(_cid for _cid, *_ in items)
    sims = mat.dot(qvec) / (np.linalg.norm(mat, axis=1) * qn + 1e-12)
    ids = [_cid for _cid, *_ in items]
    return [cid for _, cid in sorted(zip(sims, ids), key=lambda p: (-p[0], p[1]))]


def rankings_identical(retrieval_fn, with_tags, without_tags):
    """True iff retrieval ranking is byte-identical with vs without tag fields.

    retrieval_fn: callable(items) -> ranked list of ids.
    """
    r1 = list(retrieval_fn(with_tags))
    r2 = list(retrieval_fn(without_tags))
    return r1 == r2
