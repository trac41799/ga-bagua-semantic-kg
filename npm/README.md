# GA-Bagua Semantic KG — MCP Server

LLM semantic memory layer using Geometric Algebra and the I-Ching Bagua taxonomy.

## Quick Start

```bash
npx ga-semantics-mcp
```

Or install globally:

```bash
npm install -g ga-semantics-mcp
```

Then configure your LLM client (Claude Desktop, Cursor, OpenCode, etc.):

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```

## What it does

- **Encode any concept** into an 8-number vector using the LLM's semantic understanding
- **Classify relationships** between concepts using WuXing cycle dynamics (generating, controlling, clarifying, etc.)
- **Compute similarity** with role-weighted metrics
- **Solve analogies** (A:B :: C:?) deterministically
- **Persist knowledge graphs** as JSON files with query and export capabilities
- **29 MCP tools** for LLM-driven semantic reasoning

## Documentation

- [System Guide](https://github.com/YOUR_ORG/ga-bagua-semantic-kg/blob/main/docs/SYSTEM_GUIDE.md) — complete technical reference
- [Delivery Guide](https://github.com/YOUR_ORG/ga-bagua-semantic-kg/blob/main/docs/DELIVERY.md) — configuration for every LLM client
- [Encoding Skill](https://github.com/YOUR_ORG/ga-bagua-semantic-kg/blob/main/docs/skills/bagua-encoder/SKILL.md) — the LLM encoding protocol

## License

MIT OR Apache-2.0
