# Arm comparison -- POC-03 reframing engine

- run_id: `20260808T154020Z`
- engine: REAL (deepseek-chat, cached in D:\TRANSFER DATA\Coding\OpenCode\ga-bagua-semantic-kg\v2\pocs\poc-03-reframing-engine\data\cache)
- statements: 20 frozen, 4 domains x 5 (freeze marker ok: True)
- protocol: Arm A = 1 free-form call (8 reframes); Arm B = 8 exact cube moves, 1 naming call per position

## Overall

| arm | sets | mean diversity | mean coherence | naming failures | encode failures | judge failures | tokens |
|-----|------|---------------|----------------|-----------------|-----------------|----------------|--------|
| A | 20 | 0.2781 | 4.3000 | 1 | 1 | 0 | 51873 |
| B | 20 | 0.3601 | 2.8000 | 0 | 0 | 0 | 77883 |

## Per domain

| domain | arm | sets | mean diversity | mean coherence |
|--------|-----|------|---------------|----------------|
| product | A | 5 | 0.2392 | 4.8000 |
| product | B | 5 | 0.3860 | 2.4000 |
| policy | A | 5 | 0.4268 | 3.8000 |
| policy | B | 5 | 0.3113 | 3.4000 |
| science | A | 5 | 0.2382 | 4.8000 |
| science | B | 5 | 0.3544 | 2.6000 |
| design | A | 5 | 0.2083 | 3.8000 |
| design | B | 5 | 0.3885 | 2.8000 |
