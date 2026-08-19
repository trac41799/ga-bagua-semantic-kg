"""Ten filter queries: query -> target role + ground-truth item ids.

Authored by the same single annotator (relational reasoning over the 120
descriptions, using the ROLE_GLOSS semantics in tags.py), BEFORE any LLM run.
Ground truth = every corpus item whose description is centrally about the
target role's function. Q8/Q9 reuse roles with narrower ground truths to test
role-level discrimination (same retrieved set, different ground truth).

Query text is deliberately written from the role's function words, so a
retrieval system answering them (in a later phase) would need the same
semantic lens the tags encode.
"""

FILTER_QUERIES = [
    {"id": 0, "role": "constraining",
     "query": "Which components restrict, limit, or enforce bounds on access, usage, or behavior?",
     "ground_truth": [0, 4, 10, 16, 22, 24, 38, 45, 46, 49, 62, 69, 70, 73, 80,
                      83, 86, 92, 93, 95, 103, 109, 110, 112, 113, 114, 115, 117]},
    {"id": 1, "role": "transmissive",
     "query": "Which components move, relay, or deliver things between parties or locations?",
     "ground_truth": [1, 11, 21, 25, 30, 33, 48, 51, 58, 61, 74, 79, 82, 88,
                      89, 91, 98, 106, 108]},
    {"id": 2, "role": "receptive",
     "query": "Which components absorb, contain, or store inputs for later use?",
     "ground_truth": [2, 7, 9, 12, 23, 75, 99, 111]},
    {"id": 3, "role": "causal",
     "query": "Which components trigger or initiate change or action?",
     "ground_truth": [5, 8, 14, 18, 27, 28, 36, 39, 41, 100]},
    {"id": 4, "role": "clarifying",
     "query": "Which components reveal, measure, or verify the state of things?",
     "ground_truth": [6, 15, 17, 19, 20, 26, 32, 34, 35, 40, 43, 44, 47, 52,
                      55, 64, 67, 104, 107, 118]},
    {"id": 5, "role": "generative",
     "query": "Which components transform inputs into new outputs or produce new things?",
     "ground_truth": [13, 31, 42, 53, 54, 63, 65, 71, 76, 77, 78, 84, 85, 102]},
    {"id": 6, "role": "influential",
     "query": "Which components shape behavior or outcomes through influence rather than direct control?",
     "ground_truth": [59, 81, 101]},
    {"id": 7, "role": "balancing",
     "query": "Which components equalize, distribute, or maintain equilibrium?",
     "ground_truth": [3, 29, 37, 50, 56, 57, 60, 66, 68, 72, 87, 90, 94, 96,
                      105, 116, 119]},
    {"id": 8, "role": "constraining",
     "query": "Which software components enforce access or usage limits?",
     "ground_truth": [0, 4, 10, 16, 22, 24, 38]},
    {"id": 9, "role": "transmissive",
     "query": "Which biological or governance bodies relay signals, goods, or information between parties?",
     "ground_truth": [74, 79, 82, 88, 89, 91, 98, 106]},
]
