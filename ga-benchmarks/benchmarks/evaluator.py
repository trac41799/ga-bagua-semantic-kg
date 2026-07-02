"""Answer evaluation — scores LLM answers against expected concepts and fragments."""

def score_answer(answer: str, expected_concepts: list[str],
                  expected_fragments: list[str]) -> float:
    """Score an answer in [0.0, 1.0]."""
    answer_lower = answer.lower()

    concept_score = 0.5
    if expected_concepts:
        found = sum(1 for c in expected_concepts if c.lower() in answer_lower)
        concept_score = found / len(expected_concepts)

    fragment_score = 0.5
    if expected_fragments:
        found = sum(1 for f in expected_fragments if f.lower() in answer_lower)
        fragment_score = found / len(expected_fragments)

    return 0.6 * concept_score + 0.4 * fragment_score


def concepts_found(answer: str, concepts: list[str]) -> list[str]:
    """Return which expected concepts appear in the answer."""
    answer_lower = answer.lower()
    return [c for c in concepts if c.lower() in answer_lower]
