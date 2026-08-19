"""POC-07 LLM client (naming rater): real + SimulatedLLM (tests only)."""

import json
import os
import time
import urllib.error
import urllib.request


class LLMClient:
    def __init__(self, base_url, api_key, model="deepseek-chat",
                 max_tokens=64, budget_tokens=60_000, retries=2):
        self.base_url, self.api_key, self.model = base_url, api_key, model
        self.max_tokens, self.budget_tokens, self.retries = max_tokens, budget_tokens, retries
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

    def chat(self, messages, max_tokens=None, temperature=0.0):
        body = json.dumps({"model": self.model, "messages": messages,
                           "max_tokens": max_tokens or self.max_tokens,
                           "temperature": temperature}).encode()
        req = urllib.request.Request(self.base_url, data=body, method="POST",
                                     headers={"Content-Type": "application/json",
                                              "Authorization": "Bearer " + self.api_key})
        last = None
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
                return text, u
            except (urllib.error.HTTPError, urllib.error.URLError) as e:
                last = e
                time.sleep(2 * (attempt + 1))
        raise RuntimeError(f"LLM call failed: {last}")


class SimulatedLLM:
    def __init__(self):
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

    def chat(self, messages, max_tokens=None, temperature=0.0):
        self.usage["total_tokens"] += 12
        self.usage["calls"] += 1
        return "3", {"total_tokens": 12}


def load_api_config():
    key = os.environ.get("DEEPSEEK_API_KEY") or os.environ.get("OPENROUTER_API_KEY")
    if key:
        return "https://api.deepseek.com/chat/completions", key, "deepseek-chat"
    env = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.abspath(__file__))))), ".env.local")
    if os.path.exists(env):
        for line in open(env, encoding="utf-8"):
            if line.startswith("DEEPSEEK_API_KEY="):
                key = line.strip().split("=", 1)[1].strip().strip('"').strip("'")
                return "https://api.deepseek.com/chat/completions", key, "deepseek-chat"
    return None
