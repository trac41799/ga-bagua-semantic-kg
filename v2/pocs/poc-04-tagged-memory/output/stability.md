# Tag stability (dominant role across 2 LLM runs, temperature 0)

| id | name | run1 dominant | run2 dominant | match |
|----|------|---------------|---------------|-------|
| 0 | Rate Limiter | constraining | constraining | yes |
| 1 | Message Queue | transmissive | transmissive | yes |
| 3 | Load Balancer | transmissive | balancing | no |
| 4 | Circuit Breaker | constraining | constraining | yes |
| 6 | Monitoring Dashboard | clarifying | clarifying | yes |
| 8 | Event Trigger | causal | transmissive | no |
| 9 | Cache Layer | receptive | receptive | yes |
| 13 | Data Pipeline | transmissive | generative | no |
| 16 | Auth Service | clarifying | clarifying | yes |
| 36 | Chaos Injector | causal | causal | yes |
| 40 | Demand Forecast | clarifying | clarifying | yes |
| 42 | Pricing Engine | generative | generative | yes |
| 46 | Fraud Screen | clarifying | clarifying | yes |
| 50 | Marketing Budget Allocator | transmissive | transmissive | yes |
| 52 | Churn Predictor | balancing | clarifying | no |
| 56 | Demand Planner | balancing | balancing | yes |
| 59 | Loyalty Program | influential | influential | yes |
| 69 | Margin Guard | constraining | constraining | yes |
| 70 | Predator | constraining | constraining | yes |
| 71 | Photosynthesis | generative | generative | yes |
| 72 | Homeostasis | constraining | constraining | yes |
| 74 | Pollinator | generative | generative | yes |
| 75 | Seed Bank | receptive | receptive | yes |
| 76 | Mutation | generative | causal | no |
| 95 | Regulator | constraining | constraining | yes |
| 97 | Auditor | clarifying | clarifying | yes |
| 99 | Citizen Assembly | constraining | constraining | yes |
| 101 | Policy Think Tank | influential | influential | yes |
| 106 | Public Broadcaster | transmissive | transmissive | yes |
| 107 | Election Commission | balancing | balancing | yes |

**Stability: 83.3% (gate >= 80%)**
