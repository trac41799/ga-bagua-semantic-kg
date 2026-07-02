use anyhow::{Context, Result};
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::time::Instant;

pub struct McpClient {
    process: Child,
    stdin: std::process::ChildStdin,
    reader: BufReader<std::process::ChildStdout>,
}

impl McpClient {
    /// Spawn ga-semantics-mcp binary and initialize the MCP session.
    /// Searches for the binary in: target/debug/, target/release/, and PATH.
    pub fn spawn() -> Result<Self> {
        let binary = Self::find_binary()?;

        let mut process = Command::new(&binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .with_context(|| format!("Failed to spawn MCP server binary: {}", binary))?;

        let stdin = process.stdin.take().context("Failed to capture stdin")?;
        let stdout = process.stdout.take().context("Failed to capture stdout")?;
        let reader = BufReader::new(stdout);

        let mut client = Self {
            process,
            stdin,
            reader,
        };

        client.initialize()?;
        Ok(client)
    }

    fn find_binary() -> Result<String> {
        let candidates = vec![
            "target/debug/ga-semantics-mcp.exe",
            "target/release/ga-semantics-mcp.exe",
            "target/debug/ga-semantics-mcp",
            "target/release/ga-semantics-mcp",
            "ga-semantics-mcp",
        ];

        for candidate in &candidates {
            if std::path::Path::new(candidate).exists() {
                return Ok(candidate.to_string());
            }
        }

        Err(anyhow::anyhow!(
            "Could not find ga-semantics-mcp binary. Build it first: cargo build -p ga-semantics-mcp"
        ))
    }

    fn initialize(&mut self) -> Result<()> {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "ga-benchmarks",
                    "version": "0.1.0"
                }
            }
        });

        let response = self.send_request(&request)?;

        if response.get("error").is_some() {
            anyhow::bail!("MCP initialize failed: {:?}", response["error"]);
        }

        Ok(())
    }

    /// Send a JSON-RPC request and wait for the response.
    fn send_request(&mut self, request: &Value) -> Result<Value> {
        let request_str = serde_json::to_string(request)?;
        self.stdin.write_all(request_str.as_bytes())?;
        self.stdin.write_all(b"\n")?;
        self.stdin.flush()?;

        let mut line = String::new();
        self.reader.read_line(&mut line)?;

        if line.is_empty() {
            anyhow::bail!("MCP server returned empty response");
        }

        let response: Value = serde_json::from_str(&line)?;
        Ok(response)
    }

    /// Call an MCP tool by name with arguments. Returns the result text.
    pub fn call_tool(&mut self, name: &str, arguments: Value) -> Result<(Value, u64)> {
        let start = Instant::now();

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": name,
                "arguments": arguments
            }
        });

        let response = self.send_request(&request)?;
        let latency = start.elapsed().as_micros() as u64;

        if let Some(error) = response.get("error") {
            anyhow::bail!(
                "Tool '{}' error: {}",
                name,
                error["message"].as_str().unwrap_or("unknown")
            );
        }

        let result = response["result"].clone();

        Ok((result, latency))
    }

    /// List all available tools.
    pub fn list_tools(&mut self) -> Result<Value> {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list",
            "params": {}
        });

        let response = self.send_request(&request)?;

        if let Some(error) = response.get("error") {
            anyhow::bail!("tools/list failed: {}", error);
        }

        Ok(response["result"].clone())
    }

    /// Convenience: encode a concept via llm_encode.
    pub fn llm_encode(&mut self, name: &str, coefficients: &[f64; 8]) -> Result<(Value, u64)> {
        self.call_tool(
            "llm_encode",
            serde_json::json!({
                "name": name,
                "coefficients": coefficients
            }),
        )
    }

    /// Convenience: store a concept with coefficients.
    pub fn store_llm_concept(
        &mut self,
        name: &str,
        coefficients: &[f64; 8],
        text: &str,
    ) -> Result<(Value, u64)> {
        self.call_tool(
            "store_llm_concept",
            serde_json::json!({
                "name": name,
                "coefficients": coefficients,
                "text": text
            }),
        )
    }

    /// Convenience: query similar concepts.
    pub fn store_query_similar(
        &mut self,
        query_coeffs: &[f64; 8],
        top_k: usize,
    ) -> Result<(Value, u64)> {
        self.call_tool(
            "store_query_similar",
            serde_json::json!({
                "query": query_coeffs,
                "top_k": top_k
            }),
        )
    }

    /// Convenience: classify relation between two concepts.
    pub fn classify_relation(
        &mut self,
        a_coeffs: &[f64; 8],
        b_coeffs: &[f64; 8],
    ) -> Result<(Value, u64)> {
        self.call_tool(
            "classify_relation",
            serde_json::json!({
                "a": a_coeffs,
                "b": b_coeffs
            }),
        )
    }

    /// Convenience: detect contradiction.
    pub fn detect_contradiction(
        &mut self,
        a_coeffs: &[f64; 8],
        b_coeffs: &[f64; 8],
        threshold: f64,
    ) -> Result<(Value, u64)> {
        self.call_tool(
            "detect_contradiction",
            serde_json::json!({
                "a": a_coeffs,
                "b": b_coeffs,
                "threshold": threshold
            }),
        )
    }

    /// Convenience: compute semantic similarity.
    pub fn semantic_similarity(
        &mut self,
        a_coeffs: &[f64; 8],
        b_coeffs: &[f64; 8],
    ) -> Result<(Value, u64)> {
        self.call_tool(
            "semantic_similarity",
            serde_json::json!({
                "a": a_coeffs,
                "b": b_coeffs
            }),
        )
    }

    /// Convenience: store list concepts.
    pub fn store_list_concepts(&mut self) -> Result<(Value, u64)> {
        self.call_tool("store_list_concepts", serde_json::json!({}))
    }

    /// Convenience: store export graph.
    pub fn store_export(&mut self) -> Result<(Value, u64)> {
        self.call_tool("store_export", serde_json::json!({}))
    }

    /// Convenience: validate encoding pair.
    pub fn validate_encoding(
        &mut self,
        a_coeffs: &[f64; 8],
        b_coeffs: &[f64; 8],
        expected_relation: &str,
    ) -> Result<(Value, u64)> {
        self.call_tool(
            "validate_encoding",
            serde_json::json!({
                "a": a_coeffs,
                "b": b_coeffs,
                "expected_relation": expected_relation
            }),
        )
    }

    /// Convenience: multivector describe.
    pub fn multivector_describe(&mut self, coeffs: &[f64; 8]) -> Result<(Value, u64)> {
        self.call_tool(
            "multivector_describe",
            serde_json::json!({
                "multivector": coeffs
            }),
        )
    }

    /// Convenience: open store.
    pub fn store_open(&mut self, path: &str) -> Result<(Value, u64)> {
        self.call_tool(
            "store_open",
            serde_json::json!({
                "path": path
            }),
        )
    }

    /// Convenience: close store.
    pub fn store_close(&mut self) -> Result<(Value, u64)> {
        self.call_tool("store_close", serde_json::json!({}))
    }
}

impl Drop for McpClient {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

/// Parse tool result text content from MCP response.
pub fn parse_tool_text(result: &Value) -> String {
    if let Some(content) = result.get("content") {
        if let Some(arr) = content.as_array() {
            let texts: Vec<&str> = arr
                .iter()
                .filter_map(|c| c["text"].as_str())
                .collect();
            return texts.join("\n");
        }
    }
    result.to_string()
}
