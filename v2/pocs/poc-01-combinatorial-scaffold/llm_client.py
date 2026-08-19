"""POC-01 LLM client: real (DeepSeek, cached, budget-capped) + SimulatedLLM (tests only)."""

import json
import os
import time
import urllib.error
import urllib.request

from protocol import OPS  # noqa: F401 (used by SimulatedLLM to emit valid plans)


class BudgetError(RuntimeError):
    pass


class LLMClient:
    def __init__(self, base_url, api_key, model="deepseek-chat",
                 max_tokens=128, budget_tokens=300_000, retries=2):
        self.base_url = base_url
        self.api_key = api_key
        self.model = model
        self.max_tokens = max_tokens
        self.budget_tokens = budget_tokens
        self.retries = retries
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

    def chat(self, messages, max_tokens=None, temperature=0.0):
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
                u = data.get("usage", {})
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


class SimulatedLLM:
    """Deterministic stand-in — UNIT TESTS ONLY (L4). Emits a valid-but-wrong plan."""

    def __init__(self):
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

    def _tick(self, text):
        p = len(text) // 4 + 10
        c = 8
        for k in ("prompt_tokens", "completion_tokens", "total_tokens"):
            self.usage[k] += p if k != "completion_tokens" else c
        self.usage["total_tokens"] += p + c
        self.usage["calls"] += 1
        return {"prompt_tokens": p, "completion_tokens": c, "total_tokens": p + c}

    def chat(self, messages, max_tokens=None, temperature=0.0):
        if "op array" in messages[-1]["content"]:
            return '[{"op":"complement","state":"kun"}]', self._tick("plan")
        if "Calculator result" in messages[-1]["content"]:
            return "0", self._tick("interp")
        return "0", self._tick("answer")


def load_api_config():
    key = os.environ.get("DEEPSEEK_API_KEY") or os.environ.get("OPENROUTER_API_KEY")
    if key:
        return "https://api.deepseek.com/chat/completions", key, "deepseek-chat"
    env = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.abspath(__file__))))), ".env.local")
    if os.path.exists(env):
        for line in open(env, encoding="utf-8"):
            line = line.strip()
            if line.startswith("DEEPSEEK_API_KEY="):
                key = line.split("=", 1)[1].strip().strip('"').strip("'")
                return "https://api.deepseek.com/chat/completions", key, "deepseek-chat"
    return None
