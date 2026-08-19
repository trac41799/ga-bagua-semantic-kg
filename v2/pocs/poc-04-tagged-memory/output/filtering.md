# Filtering (role filter vs ground truth, 10 queries)

Retrieved = items whose dominant role equals the query role. Precision = relevant/retrieved.

| qid | role | retrieved | precision | recall |
|-----|------|-----------|-----------|--------|
| 0 | constraining | 20 | 0.75 | 0.54 |
| 1 | transmissive | 19 | 0.42 | 0.42 |
| 2 | receptive | 12 | 0.50 | 0.75 |
| 3 | causal | 14 | 0.50 | 0.70 |
| 4 | clarifying | 26 | 0.54 | 0.70 |
| 5 | generative | 13 | 0.54 | 0.50 |
| 6 | influential | 5 | 0.60 | 1.00 |
| 7 | balancing | 11 | 0.36 | 0.24 |
| 8 | constraining | 20 | 0.20 | 0.57 |
| 9 | transmissive | 19 | 0.21 | 0.50 |

**Mean precision: 0.46 (gate >= 0.50)**
