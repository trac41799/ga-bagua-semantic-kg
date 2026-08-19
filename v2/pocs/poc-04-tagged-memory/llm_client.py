"""LLM client (real: DeepSeek/OpenAI-compatible) + SimulatedLLM.

Adapted from Path D's llm_client.py: budget 200_000 tokens, cache dir
`data/cache/`. SimulatedLLM is a deterministic stand-in used ONLY in tests and
the `--sim` smoke mode; it never feeds reported claim results.
"""

import hashlib
import json
import os
import re
import time
import urllib.error
import urllib.request

import numpy as np

from tags import TAG_PROMPT


class BudgetError(RuntimeError):
    pass


class LLMClient:
    def __init__(self, base_url, api_key, model="deepseek-chat",
                 max_tokens=256, budget_tokens=200_000, retries=2,
                 cache_dir="data/cache"):
        self.base_url = base_url
        self.api_key = api_key
        self.model = model
        self.max_tokens = max_tokens
        self.budget_tokens = budget_tokens
        self.retries = retries
        self.cache_dir = cache_dir
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0,
                      "total_tokens": 0, "calls": 0}

    def chat(self, messages, max_tokens=None, temperature=0.0) -> tuple:
        """Return (text, usage). Raises BudgetError when the budget cap is exceeded."""
        body = json.dumps({"model": self.model, "messages": messages,
                           "max_tokens": max_tokens or self.max_tokens,
                           "temperature": temperature}).encode()
        req = urllib.request.Request(self.base_url, data=body, method="POST",
                                     headers={"Content-Type": "application/json",
                                              "Authorization": "Bearer " + self.api_key})
        last_err = None
        for attempt in range(self.retries + 1):
            try:
                with urllib.request.urlopen(req, timeout=120) as r:
                    data = json.loads(r.read())
                text = data["choices"][0]["message"]["content"]
                u = data.get("usage", {"prompt_tokens": 0, "completion_tokens": 0,
                                       "total_tokens": 0})
                for k in self.usage:
                    if k != "calls":
                        self.usage[k] += u.get(k, 0)
                self.usage["calls"] += 1
                if self.usage["total_tokens"] > self.budget_tokens:
                    raise BudgetError(f"budget exceeded: {self.usage['total_tokens']}")
                return text, u
            except urllib.error.HTTPError as e:
                last_err = e
                time.sleep(2 * (attempt + 1))
            except urllib.error.URLError as e:
                last_err = e
                time.sleep(2 * (attempt + 1))
        raise RuntimeError(f"LLM call failed after retries: {last_err}")

    def tag(self, description: str, run_id: int = 0) -> tuple:
        """Return (raw text, usage) for role tagging. Temperature 0."""
        return self.chat(TAG_PROMPT(description), max_tokens=256, temperature=0.0)


class SimulatedLLM:
    """Deterministic stand-in — UNIT TESTS / --sim SMOKE ONLY (never reported results).

    Tags are a seeded keyword heuristic (role vocabularies scored against the
    description) plus small seeded noise. The seed mixes in run_id so the two
    stability runs differ slightly, exercising the consistency metric.
    """

    KEYWORDS = {
        "receptive": ["store", "cache", "archive", "preserv", "capture", "record",
                      "log", "seed", "bank", "collect", "gather", "buffer",
                      "repository", "mirror", "replay", "hold"],
        "causal": ["trigger", "initiate", "start", "introduce", "inject", "fire",
                   "launch", "apply", "rotate", "schedul", "respond", "cause",
                   "invoke", "activate", "toggle", "execut"],
        "transmissive": ["relay", "deliver", "move", "route", "distribut", "ship",
                         "carry", "transport", "communicat", "transmit", "dispatch",
                         "broadcast", "migrat", "flow", "deliver", "stream"],
        "constraining": ["restrict", "limit", "bound", "enforce", "guard", "block",
                         "stop", "prevent", "deny", "control", "protect", "quota",
                         "threshold", "regulat", "cull", "screen", "check", "border",
                         "policy", "standard", "compliance", "rule", "law",
                         "constrain", "curb", "deter", "prosecut", "inspect",
                         "assess", "approval", "fair", "veto", "judg", "limit",
                         "strict"],
        "clarifying": ["monitor", "dashboard", "visualiz", "observe", "analy",
                       "measure", "count", "track", "verif", "audit", "resolve",
                       "map", "flag", "predict", "forecast", "rank", "score",
                       "probe", "survey", "census", "statistics", "review",
                       "scrutin", "contract", "test", "identify", "reveal",
                       "metric", "trace", "sla", "uptime"],
        "influential": ["loyalty", "retain", "advocacy", "shape", "influenc",
                        "persuade", "nudge", "agenda", "keystone", "brand",
                        "campaign", "loyalty"],
        "balancing": ["balanc", "equaliz", "distribut", "equilibri", "fair",
                      "smooth", "compensat", "mediate", "reconcile", "adjust",
                      "partition", "allocat", "budget", "optimiz", "homeostasis",
                      "revis", "rollback", "restore", "revert", "bid", "procure",
                      "prioritiz", "retrie"],
        "generative": ["convert", "transform", "produce", "generate", "create",
                       "compute", "calculate", "build", "synthesize", "grow",
                       "bloom", "surge", "photosynthesis", "mutat", "breed",
                       "reproduce", "fund", "financ", "manufacture", "develop",
                       "design", "select", "plan", "genet"],
    }

    def __init__(self):
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0,
                      "total_tokens": 0, "calls": 0}

    def _tick(self, text):
        p = len(text) // 4 + 10
        c = 8
        self.usage["prompt_tokens"] += p
        self.usage["completion_tokens"] += c
        self.usage["total_tokens"] += p + c
        self.usage["calls"] += 1
        return {"prompt_tokens": p, "completion_tokens": c, "total_tokens": p + c}

    def tag(self, description: str, run_id: int = 0) -> tuple:
        from tags import ROLES
        text = description.lower()
        scores = {}
        for role, words in self.KEYWORDS.items():
            hits = sum(1 for w in words if w in text)
            scores[role] = float(hits)
        rng = np.random.default_rng(abs(hash(f"{description}|{run_id}")) % (2**32))
        for role in ROLES:
            scores[role] += float(rng.uniform(-0.15, 0.15))
        best = max(ROLES, key=lambda r: scores[r])
        out = {role: round(float(np.clip(rng.uniform(-0.3, 0.3), -1.0, 1.0)), 2)
               for role in ROLES}
        out[best] = round(float(np.clip(scores[best] + 0.6, 0.0, 1.0)), 2)
        if out[best] <= max(v for r, v in out.items() if r != best):
            out[best] = round(max(v for r, v in out.items() if r != best) + 0.05, 2)
            out[best] = min(out[best], 1.0)
        payload = json.dumps(out)
        return payload, self._tick(payload)


def load_api_config():
    """API key from env var, else repo-root .env.local. Returns (base_url, key, model) or None."""
    key = os.environ.get("DEEPSEEK_API_KEY") or os.environ.get("OPENROUTER_API_KEY")
    base = "https://api.deepseek.com/chat/completions"
    model = "deepseek-chat"
    if key:
        return base, key, model
    env_path = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.abspath(__file__))))), ".env.local")
    if os.path.exists(env_path):
        for line in open(env_path, encoding="utf-8"):
            line = line.strip()
            if line.startswith("DEEPSEEK_API_KEY="):
                key = line.split("=", 1)[1].strip().strip('"').strip("'")
                return base, key, model
    return None


def cache_key(description, run_id=0):
    """Deterministic cache key for a tagging call."""
    return hashlib.sha256(f"{run_id}|{description}".encode("utf-8")).hexdigest()
