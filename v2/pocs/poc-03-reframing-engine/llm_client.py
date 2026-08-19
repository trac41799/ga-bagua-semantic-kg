"""LLM client for POC-03: real OpenAI-compatible client + cache + SimulatedLLM.

Adapted from v2/probes/path-d/llm_client.py (same pattern). Differences for
POC-03:

  * budget cap 300_000 tokens,
  * deterministic response cache under data/cache/ (key = sha256 of the full
    request: model + messages + max_tokens + temperature),
  * task-level protocol with strict JSON validation: naming a position,
    free-form reframes (Arm A), rubric encoding, coherence judging. Protocol
    failures are COUNTED (self.failures) and never retried -- the returned
    value is "" / [] / None / 0.0 (honest, per the pre-registration).

SimulatedLLM is a deterministic stand-in for unit tests ONLY (never used for
reported results). It implements the same task methods; hashing is sha256-based
so behaviour is identical across processes and runs.
"""

import hashlib
import json
import os
import re
import time
import urllib.error
import urllib.request
from pathlib import Path

import numpy as np

from rubric import encode_prompt, parse_encoding

BUDGET_TOKENS = 300_000
DEFAULT_CACHE_DIR = Path(__file__).resolve().parent / "data" / "cache"

NAME_SYSTEM_PROMPT = (
    "You are the naming layer of a dialectical reframing engine. Given a statement and an "
    "exact cube move applied to a trigram state, NAME the resulting position: a short reframe "
    "of the statement as seen from that position (2-10 words, a phrase -- not a full sentence, "
    "not a restatement). The move is exact; your only task is naming.\n"
    "Output ONLY JSON of the form {\"name\": \"<the reframe>\"} -- no prose, no markdown."
)

FREE_FORM_SYSTEM_PROMPT = (
    "Given a statement, produce exactly 8 ALTERNATIVE framings: genuinely different "
    "perspectives, each a short phrase (2-10 words), distinct from the original statement and "
    "from each other.\n"
    "Output ONLY a JSON array of exactly 8 strings -- no prose, no markdown."
)

JUDGE_SYSTEM_PROMPT = (
    "You are a coherence judge for sets of reframes. Rate how coherent the set is as a set: "
    "1 = incoherent, unrelated fragments; 5 = highly coherent, clearly related while "
    "distinct.\n"
    "Output ONLY JSON of the form {\"score\": <number 1-5>} -- no prose, no markdown."
)


class BudgetError(RuntimeError):
    pass


class CacheMissError(RuntimeError):
    pass


class LLMClient:
    """Real client (DeepSeek/OpenAI-compatible chat completions)."""

    def __init__(self, base_url, api_key, model="deepseek-chat",
                 max_tokens=64, budget_tokens=BUDGET_TOKENS, retries=2):
        self.base_url = base_url
        self.api_key = api_key
        self.model = model
        self.max_tokens = max_tokens
        self.budget_tokens = budget_tokens
        self.retries = retries
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

    def chat(self, messages, max_tokens=None, temperature=0.0):
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


class CachedClient:
    """Deterministic cache in front of any chat backend.

    Key = sha256 of model + messages + max_tokens + temperature. Cache hits
    return the stored text without touching the backend and without consuming
    budget. In offline mode a cache miss raises CacheMissError.
    """

    def __init__(self, backend, cache_dir=DEFAULT_CACHE_DIR, offline=False, model="deepseek-chat"):
        self.backend = backend
        self.cache_dir = Path(cache_dir)
        self.offline = offline
        self.model = model
        self.cache_hits = 0
        self.cache_misses = 0

    def _key(self, messages, max_tokens, temperature):
        material = json.dumps({"model": self.model, "messages": messages,
                               "max_tokens": max_tokens, "temperature": temperature},
                              sort_keys=True)
        return hashlib.sha256(material.encode("utf-8")).hexdigest()

    def chat(self, messages, max_tokens=None, temperature=0.0):
        key = self._key(messages, max_tokens, temperature)
        path = self.cache_dir / f"{key}.json"
        if path.exists():
            self.cache_hits += 1
            data = json.loads(path.read_text(encoding="utf-8"))
            return data["text"], data["usage"]
        if self.offline:
            self.cache_misses += 1
            raise CacheMissError(f"offline mode: no cache entry for {key}")
        if self.backend is None:
            raise RuntimeError("no LLM backend configured")
        text, usage = self.backend.chat(messages, max_tokens=max_tokens, temperature=temperature)
        self.cache_dir.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps({"key": key, "text": text, "usage": usage},
                                   ensure_ascii=False, indent=1), encoding="utf-8")
        return text, usage


class RealEngine:
    """Task-level protocol over a chat backend: strict JSON validation + failure counting.

    Every protocol failure increments self.failures[<task>] and is NOT retried;
    the honest fallback value is returned instead ("" / [] / None / 0.0).
    """

    def __init__(self, backend, budget_tokens=BUDGET_TOKENS):
        self.backend = backend
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}
        self.failures = {"name_position": 0, "free_form": 0, "encode": 0, "judge": 0}

    @property
    def cache_hits(self):
        return getattr(self.backend, "cache_hits", 0)

    def _chat(self, messages, max_tokens):
        text, usage = self.backend.chat(messages, max_tokens=max_tokens, temperature=0.0)
        for k in ("prompt_tokens", "completion_tokens", "total_tokens"):
            self.usage[k] += usage.get(k, 0)
        self.usage["calls"] += 1
        return text

    # ---- tasks ----

    def name_position(self, statement, move_name, state_desc):
        text = self._chat([
            {"role": "system", "content": NAME_SYSTEM_PROMPT},
            {"role": "user", "content":
             f"Statement: {statement}\nMove: {move_name} -- {state_desc}\nOutput the JSON."},
        ], max_tokens=64)
        try:
            obj = _load_json(text)
            if not isinstance(obj, dict) or not isinstance(obj.get("name"), str) \
                    or not obj["name"].strip():
                raise ValueError("expected {\"name\": <non-empty string>}")
            return obj["name"].strip()
        except Exception:
            self.failures["name_position"] += 1
            return ""

    def free_form_reframes(self, statement):
        text = self._chat([
            {"role": "system", "content": FREE_FORM_SYSTEM_PROMPT},
            {"role": "user", "content": f"Statement: {statement}\nOutput the JSON array."},
        ], max_tokens=512)
        try:
            obj = _load_json(text)
            if not isinstance(obj, list) or len(obj) != 8 \
                    or not all(isinstance(x, str) and x.strip() for x in obj):
                raise ValueError("expected a JSON array of exactly 8 non-empty strings")
            return [x.strip() for x in obj]
        except Exception:
            self.failures["free_form"] += 1
            return []

    def encode(self, text):
        raw = self._chat(encode_prompt(text), max_tokens=128)
        try:
            return parse_encoding(raw)
        except ValueError:
            self.failures["encode"] += 1
            return None

    def judge(self, statement, reframes):
        body = "\n".join(f"{i + 1}. {r}" for i, r in enumerate(reframes))
        text = self._chat([
            {"role": "system", "content": JUDGE_SYSTEM_PROMPT},
            {"role": "user", "content":
             f"Original statement: {statement}\nReframes:\n{body}\nOutput the JSON."},
        ], max_tokens=16)
        try:
            obj = _load_json(text)
            score = obj.get("score") if isinstance(obj, dict) else obj
            s = float(score)
            if not (1.0 <= s <= 5.0):
                raise ValueError(f"score out of range: {s}")
            return float(s)
        except Exception:
            self.failures["judge"] += 1
            return 0.0


class SimulatedLLM:
    """Deterministic stand-in -- UNIT TESTS ONLY (never used for reported results).

    Implements the same task methods as RealEngine. All randomness is seeded
    from sha256 digests, so outputs are identical across runs and processes.
    """

    def __init__(self):
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}
        self.failures = {"name_position": 0, "free_form": 0, "encode": 0, "judge": 0}
        self.cache_hits = 0

    def _seed(self, *parts):
        h = hashlib.sha256("|".join(parts).encode("utf-8")).hexdigest()
        return int(h[:16], 16)

    def _tick(self, text):
        p = len(text) // 4 + 10
        c = 8
        self.usage["prompt_tokens"] += p
        self.usage["completion_tokens"] += c
        self.usage["total_tokens"] += p + c
        self.usage["calls"] += 1
        return {"prompt_tokens": p, "completion_tokens": c, "total_tokens": p + c}

    def name_position(self, statement, move_name, state_desc):
        text = f"{move_name.upper()} -- {state_desc.split(' (')[0]}: {statement[:48]}"
        self._tick(text)
        return text

    def free_form_reframes(self, statement):
        reframes = [f"free-framing {i}: {statement[:48]}" for i in range(8)]
        self._tick(json.dumps(reframes))
        return reframes

    def encode(self, text):
        rng = np.random.default_rng(self._seed("encode", text))
        v = rng.normal(size=8)
        v /= np.linalg.norm(v)
        self._tick(json.dumps(v.tolist()))
        return v

    def judge(self, statement, reframes):
        score = 3.0 + self._seed("judge", statement, str(len(reframes))) % 3
        self._tick(json.dumps({"score": score}))
        return float(score)


def _load_json(text):
    """Strict JSON load after stripping optional markdown code fences."""
    t = re.sub(r"^```(json)?\s*", "", text.strip())
    t = re.sub(r"\s*```$", "", t)
    if not t:
        raise ValueError("empty LLM response")
    try:
        return json.loads(t)
    except json.JSONDecodeError as e:
        raise ValueError(f"not valid JSON: {e}") from e


def load_api_config():
    """API key from env var, else repo-root .env.local. Returns (base_url, key, model) or None."""
    key = os.environ.get("DEEPSEEK_API_KEY") or os.environ.get("OPENROUTER_API_KEY")
    base = "https://api.deepseek.com/chat/completions"
    model = "deepseek-chat"
    if key:
        return base, key, model
    env_path = Path(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(
        Path(__file__).resolve()))))) / ".env.local"
    if env_path.exists():
        for line in open(env_path, encoding="utf-8"):
            line = line.strip()
            if line.startswith("DEEPSEEK_API_KEY="):
                key = line.split("=", 1)[1].strip().strip('"').strip("'")
                return base, key, model
    return None
