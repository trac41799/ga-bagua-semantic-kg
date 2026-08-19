"""Filtering metrics: role-filter precision/recall vs ground truth.

Retrieval by tag = items whose DOMINANT role equals the query role. The tag
layer filters, it never re-ranks (see noninterf.py).
"""

from tags import dominant_role


def _retrieved_ids(query_role, items_with_roles):
    return [cid for cid, tags in items_with_roles
            if dominant_role(tags) == query_role]


def filter_precision(query_role, items_with_roles, ground_truth_ids):
    """Fraction of role-retrieved items that are in ground truth.

    items_with_roles: list of (concept_id, tags dict). Empty retrieval -> 0.0.
    Hand case: 3 relevant of 5 retrieved -> 0.6.
    """
    retrieved = _retrieved_ids(query_role, items_with_roles)
    if not retrieved:
        return 0.0
    hits = len(set(retrieved) & set(ground_truth_ids))
    return hits / len(retrieved)


def filter_recall(query_role, items_with_roles, ground_truth_ids):
    """Fraction of ground truth items that were role-retrieved.

    Empty ground truth -> 0.0. Hand case: 3 of 4 relevant -> 0.75.
    """
    retrieved = _retrieved_ids(query_role, items_with_roles)
    gt = set(ground_truth_ids)
    if not gt:
        return 0.0
    return len(set(retrieved) & gt) / len(gt)
