use crate::types::{GaBaguaToolCall, LlmUsage};
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use std::time::Instant;

pub struct LlmClient {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl LlmClient {
    pub fn new(model: &str, provider: &str, api_key_env: &str) -> Result<Self> {
        let api_key = env::var(api_key_env)
            .with_context(|| format!("Environment variable {} not set", api_key_env))?;

        let base_url = match provider {
            "openrouter" => "https://openrouter.ai/api/v1/chat/completions",
            "openai" => "https://api.openai.com/v1/chat/completions",
            "anthropic" => "https://api.anthropic.com/v1/messages",
            _ => anyhow::bail!("Unknown provider: {}", provider),
        };

        Ok(Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()?,
            api_key,
            model: model.to_string(),
            base_url: base_url.to_string(),
        })
    }

    /// Send a chat completion request and return the text response with usage stats.
    /// `tools` are JSON schemas for function-calling (MCP tool definitions).
    pub fn chat(
        &self,
        system_prompt: &str,
        user_messages: &[(&str, &str)],
        tools: &[Value],
        max_tokens: u32,
        temperature: f64,
    ) -> Result<(String, LlmUsage, Vec<GaBaguaToolCall>)> {
        let mut messages: Vec<Value> = Vec::new();

        if !system_prompt.is_empty() {
            messages.push(serde_json::json!({
                "role": "system",
                "content": system_prompt
            }));
        }

        for (role, content) in user_messages {
            let role_str = match *role {
                "assistant" => "assistant",
                "tool" => "tool",
                _ => "user",
            };
            messages.push(serde_json::json!({
                "role": role_str,
                "content": content
            }));
        }

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "max_tokens": max_tokens,
            "temperature": temperature,
        });

        if !tools.is_empty() {
            body["tools"] = serde_json::json!(tools);
            body["tool_choice"] = serde_json::json!("auto");
        }

        let start = Instant::now();
        let response = self
            .client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/trac41799/ga-bagua-semantic-kg")
            .header("X-Title", "GA-Bagua Benchmark Runner")
            .json(&body)
            .send()?;

        let json: Value = response.json()?;

        if let Some(error) = json.get("error") {
            anyhow::bail!("LLM API error: {}", error);
        }

        let choice = &json["choices"][0];
        let message = &choice["message"];

        let content = message["content"].as_str().unwrap_or("").to_string();

        let usage = LlmUsage {
            prompt_tokens: json["usage"]["prompt_tokens"].as_u64().unwrap_or(0),
            completion_tokens: json["usage"]["completion_tokens"].as_u64().unwrap_or(0),
            total_tokens: json["usage"]["total_tokens"].as_u64().unwrap_or(0),
        };

        let mut tool_calls = Vec::new();
        if let Some(tc_array) = message["tool_calls"].as_array() {
            for tc in tc_array {
                let name = tc["function"]["name"].as_str().unwrap_or("").to_string();
                let args_str = tc["function"]["arguments"].as_str().unwrap_or("{}");
                let arguments: Value = serde_json::from_str(args_str).unwrap_or(Value::Null);
                tool_calls.push(GaBaguaToolCall {
                    tool_name: name,
                    arguments,
                    result: None,
                    latency_us: 0,
                });
            }
        }

        let _elapsed = start.elapsed();

        Ok((content, usage, tool_calls))
    }

    /// Simple chat without tools.
    pub fn chat_simple(
        &self,
        system_prompt: &str,
        user_message: &str,
        max_tokens: u32,
    ) -> Result<(String, LlmUsage)> {
        let (content, usage, _) = self.chat(
            system_prompt,
            &[("user", user_message)],
            &[],
            max_tokens,
            0.0,
        )?;
        Ok((content, usage))
    }

    /// Count tokens in text (approximate using 1 token ≈ 4 chars).
    pub fn estimate_tokens(text: &str) -> u64 {
        (text.len() as f64 / 4.0).ceil() as u64
    }
}
