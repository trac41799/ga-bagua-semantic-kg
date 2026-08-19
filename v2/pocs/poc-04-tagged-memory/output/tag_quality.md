# Tag quality (LLM dominant role vs human, 30 items)

Single annotator (builder); human tags frozen at reference_tags.sha256.

| id | name | human dominant | LLM dominant | match |
|----|------|----------------|--------------|-------|
| 0 | Rate Limiter | constraining | constraining | yes |
| 1 | Message Queue | transmissive | transmissive | yes |
| 3 | Load Balancer | balancing | transmissive | no |
| 4 | Circuit Breaker | constraining | constraining | yes |
| 6 | Monitoring Dashboard | clarifying | clarifying | yes |
| 8 | Event Trigger | causal | causal | yes |
| 9 | Cache Layer | receptive | receptive | yes |
| 13 | Data Pipeline | generative | transmissive | no |
| 16 | Auth Service | constraining | clarifying | no |
| 36 | Chaos Injector | causal | causal | yes |
| 40 | Demand Forecast | clarifying | clarifying | yes |
| 42 | Pricing Engine | generative | generative | yes |
| 46 | Fraud Screen | constraining | clarifying | no |
| 50 | Marketing Budget Allocator | balancing | transmissive | no |
| 52 | Churn Predictor | clarifying | balancing | no |
| 56 | Demand Planner | balancing | balancing | yes |
| 59 | Loyalty Program | influential | influential | yes |
| 69 | Margin Guard | constraining | constraining | yes |
| 70 | Predator | constraining | constraining | yes |
| 71 | Photosynthesis | generative | generative | yes |
| 72 | Homeostasis | balancing | constraining | no |
| 74 | Pollinator | transmissive | generative | no |
| 75 | Seed Bank | receptive | receptive | yes |
| 76 | Mutation | generative | generative | yes |
| 95 | Regulator | constraining | constraining | yes |
| 97 | Auditor | clarifying | clarifying | yes |
| 99 | Citizen Assembly | receptive | constraining | no |
| 101 | Policy Think Tank | influential | influential | yes |
| 106 | Public Broadcaster | transmissive | transmissive | yes |
| 107 | Election Commission | clarifying | balancing | no |

**Tag quality: 20/30 = 66.7% (gate >= 80%)**
