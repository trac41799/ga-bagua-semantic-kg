"""
LLM Client — calls DeepSeek / OpenRouter API with real token tracking.
"""
import os
import json
import time
import requests

class LlmClient:
    """Simple LLM client for DeepSeek or OpenRouter."""

    def __init__(self, provider: str = "deepseek", model: str = "deepseek-chat"):
        self.provider = provider
        self.model = model
        self.api_key = ""
        self.base_url = ""

        if provider == "deepseek":
            self.api_key = os.environ.get("DEEPSEEK_API_KEY", "")
            self.base_url = "https://api.deepseek.com/v1/chat/completions"
        elif provider == "openrouter":
            self.api_key = os.environ.get("OPENROUTER_API_KEY", "")
            self.base_url = "https://openrouter.ai/api/v1/chat/completions"
            if "deepseek" in model and "/" not in model:
                self.model = f"deepseek/{model}"
        else:
            raise ValueError(f"Unknown provider: {provider}")

        if not self.api_key:
            raise RuntimeError(f"No API key found for {provider}. Set {provider.upper()}_API_KEY env var.")

    def chat(self, system_prompt: str, user_message: str,
             max_tokens: int = 2048, temperature: float = 0.0) -> dict:
        """Send a single chat message and return response with usage."""
        headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json",
        }

        if self.provider == "openrouter":
            headers["HTTP-Referer"] = "https://github.com/trac41799/ga-bagua-semantic-kg"
            headers["X-Title"] = "GA-Bagua NIAH Benchmark"

        payload = {
            "model": self.model,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_message},
            ],
            "max_tokens": max_tokens,
            "temperature": temperature,
        }

        start = time.time()
        resp = requests.post(self.base_url, headers=headers, json=payload, timeout=300)
        latency = time.time() - start

        if resp.status_code != 200:
            return {
                "answer": "",
                "error": f"API error {resp.status_code}: {resp.text[:500]}",
                "prompt_tokens": 0,
                "completion_tokens": 0,
                "total_tokens": 0,
                "latency_sec": latency,
            }

        data = resp.json()

        if "error" in data:
            return {
                "answer": "",
                "error": str(data["error"]),
                "prompt_tokens": 0,
                "completion_tokens": 0,
                "total_tokens": 0,
                "latency_sec": latency,
            }

        usage = data.get("usage", {})
        choice = data.get("choices", [{}])[0]
        message = choice.get("message", {})
        answer = message.get("content", "")

        return {
            "answer": answer,
            "prompt_tokens": usage.get("prompt_tokens", 0),
            "completion_tokens": usage.get("completion_tokens", 0),
            "total_tokens": usage.get("total_tokens", 0),
            "latency_sec": latency,
        }
