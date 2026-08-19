"""LLM client (real: DeepSeek/OpenAI-compatible) + SimulatedLLM (unit tests only)."""

import json
import os
import time
import urllib.error
import urllib.request

import numpy as np

from rubric import encode_prompt, parse_encoding, parse_id_list


class BudgetError(RuntimeError):
    pass


class LLMClient:
    def __init__(self, base_url, api_key, model="deepseek-chat",
                 max_tokens=64, budget_tokens=250_000, retries=2):
        self.base_url = base_url
        self.api_key = api_key
        self.model = model
        self.max_tokens = max_tokens
        self.budget_tokens = budget_tokens
        self.retries = retries
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

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

    # ---- task-specific calls ----

    def encode_concept(self, description: str) -> tuple:
        """Return (np.ndarray[8] normalized, usage)."""
        text, usage = self.chat(encode_prompt(description), max_tokens=128)
        return parse_encoding(text), usage

    def verify_candidates(self, query: str, candidates) -> tuple:
        """Ask the LLM which candidate concept ids are relevant to the query.

        candidates: list of (id, name). Returns (list of chosen ids, usage).
        """
        listing = "\n".join(f"{cid}: {name}" for cid, name in candidates)
        messages = [
            {"role": "system", "content": "Select which of the listed concepts are relevant to the query. "
                                          "Output ONLY a JSON array of the selected ids (numbers), no prose."},
            {"role": "user", "content": f"Query: {query}\nConcepts:\n{listing}\nOutput the JSON array of ids."},
        ]
        text, usage = self.chat(messages, max_tokens=64)
        try:
            ids = parse_id_list(text)
            return [int(x) for x in ids if isinstance(x, (int, float))], usage
        except Exception:
            return [], usage  # verifier failure counts as empty selection (honest)

    def full_context(self, query: str, descriptions) -> tuple:
        """Baseline: answer the query with ALL descriptions in context. Returns (text, usage)."""
        listing = "\n".join(descriptions)
        messages = [
            {"role": "system", "content": "Answer the query using ONLY the listed concepts. "
                                          "Name every relevant concept exactly as written."},
            {"role": "user", "content": f"Query: {query}\nConcepts:\n{listing}"},
        ]
        return self.chat(messages, max_tokens=256)


class SimulatedLLM:
    """Deterministic stand-in — UNIT TESTS ONLY (lesson L4). Never used for reported results."""

    def __init__(self):
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

    def _tick(self, text):
        p = len(text) // 4 + 10
        c = 8
        self.usage["prompt_tokens"] += p
        self.usage["completion_tokens"] += c
        self.usage["total_tokens"] += p + c
        self.usage["calls"] += 1
        return {"prompt_tokens": p, "completion_tokens": c, "total_tokens": p + c}

    def encode_concept(self, description: str) -> tuple:
        rng = np.random.default_rng(abs(hash(description)) % (2**32))
        v = rng.normal(size=8)
        v /= np.linalg.norm(v)
        text = json.dumps(v.tolist())
        return v, self._tick(text)

    def verify_candidates(self, query: str, candidates) -> tuple:
        qwords = set(query.lower().split())
        chosen = [cid for cid, name in candidates
                  if any(w in name.lower() for w in qwords if len(w) > 4)]
        return chosen, self._tick(json.dumps(chosen))

    def full_context(self, query: str, descriptions) -> tuple:
        text = " ".join(descriptions[:1])
        return text, self._tick(text)


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
