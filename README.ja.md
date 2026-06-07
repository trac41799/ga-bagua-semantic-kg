<div align="center">

[**English**](README.md) · [**中文**](README.zh.md) · [**Tiếng Việt**](README.vi.md) · [**한국어**](README.ko.md) · [**日本語**](README.ja.md)

</div>

---

<div align="center">

```
                    ☰                        乾 / 生成
                ☱       ☴                 兌 / 均衡     巽 / 浸透
                    ☲                        離 / 明確化
                ☳       ☶                 震 / 因果     艮 / 制約
                    ☵                        坎 / 伝達
                    ☷                        坤 / 受容
```

# GA-Bagua セマンティック知識グラフ

**LLMの意味記憶 — 8次元、64卦状態、学習不要。**

[![Crates.io](https://img.shields.io/crates/v/ga-semantics-core?label=core)](https://crates.io/crates/ga-semantics-core)
[![Crates.io](https://img.shields.io/crates/v/ga-semantics-mcp?label=mcp)](https://crates.io/crates/ga-semantics-mcp)
[![Crates.io](https://img.shields.io/crates/v/ga-semantics-cli?label=cli)](https://crates.io/crates/ga-semantics-cli)
[![npm](https://img.shields.io/npm/v/ga-semantics-mcp?color=red)](https://www.npmjs.com/package/ga-semantics-mcp)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

</div>

---

あらゆる概念が **8つの数値** になります。あらゆる関係は **五行の一歩** です。
学習は不要。ベクトルデータベースは不要。LLMの繰り返し呼び出しは不要。
推論は **34ナノ秒** から **320マイクロ秒** で完了し、累積誤差はゼロです。

```
 概念名
      │
      ▼
 LLM（SKILL.mdを読む、〜200トークン、一回限り）
      │
      ▼
 [☷ 0.04, ☳ -0.09, ☵ -0.51, ☶ 0.68, ☲ 0.21, ☴ -0.26, ☱ 0.17, ☰ -0.34]
      │
      ├── 類似?     →  dominant_similarity()
      ├── 関係?     →  classify_hexagram()       （五行周期ルックアップ）
      └── 類推?     →  analogy()                  （周期方向予測）
```

| 木 火を生む | 火 土を生む | 土 金を生む | 金 水を生む | 水 木を生む |
|:---:|:---:|:---:|:---:|:---:|
| 関係 100% | 役割 100% | P@K 73.3% | MRR 0.878 | 類推 80% |

---

## インストール

```bash
# 方法 1 — npm（推奨）
npm install -g ga-semantics-mcp     # MCPサーバー + CLI + スキル

# 方法 2 — Cargo
cargo install ga-semantics-mcp      # MCPサーバー
cargo install ga-semantics-cli      # CLIツール

# 方法 3 — バイナリをダウンロード
# 以下からプラットフォームを選択：https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
```

各アーカイブには `ga-semantics-mcp`（サーバー）、`ga-semantics`（CLI）、および `bagua-encoder-skill.md` が含まれています。

---

## LLM自己インストール

<details>
<summary><b>☰ Claude Desktop</b></summary>

```
Claude Desktop用にGA-Baguaセマンティック知識グラフのMCPサーバー+CLIをインストールします。

ステップ 1 — OSを検出し、ダウンロード：
  https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
  （Windowsは.zip、macOS/Linuxは.tar.gzを選択）

ステップ 2 — ~/.local/bin/（macOS/Linux）または %USERPROFILE%\.local\bin\（Windows）に展開。
  macOS/Linux：両方のバイナリに chmod +x を実行。

ステップ 3 — Claude Desktop MCPを設定。以下を読み取りまたは作成：
  Windows: %APPDATA%\Claude\claude_desktop_config.json
  macOS:   ~/Library/Application Support/Claude/claude_desktop_config.json
  Linux:   ~/.config/Claude/claude_desktop_config.json
  追加：{ "mcpServers": { "ga-bagua-semantic-kg": { "command": "<パス>/ga-semantics-mcp" } } }

ステップ 4 — スキルをインストール：ダウンロード
  https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
  %APPDATA%\Claude\skills\bagua-encoder\SKILL.md（または~/.config/Claude/skills/...）に保存

ステップ 5 — Claude Desktopを再起動。確認：「llm_encodeを使用してRate Limiterをエンコード、
  係数 [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34]」
  期待結果：dominant_role = "constraining"。
```
</details>

<details>
<summary><b>☲ OpenCode</b></summary>

```
OpenCode CLI用にGA-Baguaセマンティック知識グラフをインストール。リリースからダウンロードし、両方の
バイナリを展開。~/.config/opencode/opencode.jsonを設定：
  { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<パス>/ga-semantics-mcp" } } }
スキルを~/.config/opencode/skills/bagua-encoder/SKILL.mdにインストール。
再起動してllm_encodeで確認。
```
</details>

<details>
<summary><b>☵ Cursor</b></summary>

```
Cursor用にGA-Baguaセマンティック知識グラフをインストール。リリースからダウンロード、両方のバイナリを展開。
~/.cursor/mcp.jsonを設定：
  { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<パス>/ga-semantics-mcp" } } }
スキルを~/.cursor/skills/bagua-encoder/SKILL.mdにインストール。
Composerエージェントでllm_encodeを使用して確認。
```
</details>

<details>
<summary><b>☳ Claude Code CLI</b></summary>

```
Claude Code用にGA-Baguaセマンティック知識グラフをインストール。リリースからダウンロード、両方の
バイナリを展開。~/.claude/mcp.jsonを設定。スキルを~/.claude/skills/bagua-encoder/に保存。
確認：echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | <パス>/ga-semantics-mcp
```
</details>

<details>
<summary><b>☴ Continue.dev / ☱ Cline / ☰ Windsurf / その他</b></summary>

```
https://github.com/trac41799/ga-bagua-semantic-kg/releases/latestからダウンロード
展開し、バイナリをPATHに配置、クライアントのMCP設定を構成。
docs/skills/bagua-encoder/SKILL.mdからスキルをインストール
詳細なクライアント別手順はdocs/DELIVERY.mdを参照。
```
</details>

---

## CLIの使用法

```bash
# 概念をエンコード
ga-semantics encode 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20
ga-semantics encode -j 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20 --json

# 関係を分類
ga-semantics classify \
  "[0.04,-0.09,-0.51,0.68,0.21,-0.26,0.17,-0.34]" \
  "[0.25,0.15,-0.10,0.55,0.40,0.05,0.30,0.20]"

# 類似度を計算
ga-semantics sim "[0.15,0.25,0.81,...]" "[0.30,0.10,0.60,...]"

# 類推を解く
ga-semantics analogy  "[A]" "[B]" "[C]"

# 八卦を探検
ga-semantics trigram qian --transforms
ga-semantics hexagram "[A]" "[B]"
ga-semantics wuxing water --cycle controlling

# 知識グラフ
ga-semantics store add "Auth System" 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20
ga-semantics store query "[0.05,-0.05,-0.45,0.70,0.15,-0.20,0.10,-0.30]"
ga-semantics store list
ga-semantics store export

# ベンチマーク
ga-semantics bench timing
ga-semantics bench semantic
```

`--json`は機械可読出力、`--csv`はスプレッドシート、`--quiet`は値のみ出力。

---

## エンコード早見表

```
8つの役割（順序）：
[受容、因果、伝達、制約、明確化、浸透、均衡、生成]

尺度：  >0.5 強い  |  0.2–0.5 中程度  |  0.05–0.2 弱い
       -0.05–0.05 無関係  |  <-0.05 反対  |  <-0.5 強い反対

単位長に正規化。8つの浮動小数点数のJSON配列のみを出力。
```

完全なエンコードプロトコルは **[SKILL.md](docs/skills/bagua-encoder/SKILL.md)** を参照。

---

## Rust API

```rust
use ga_semantics_core::prelude::*;

let mv = llm_encode(&[0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20]);
let desc = multivector_describe(&mv);
let (rel, conf) = RelationType::from_pair(&a, &b);
let sim = dominant_similarity(&a, &b);
let d = analogy(&a, &b, &c);
```

```toml
[dependencies]
ga-semantics-core = { version = "0.1", features = ["store"] }
```

---

## アーキテクチャ

```
┌─────────────────────────────────────────────────────────┐
│  第4層 ── MCPサーバー（29ツール）+ CLI + Python         │
│  第3層 ── semantics.rs — 類似度、類推、ストア           │
│  第2層 ── Cl(3) 多重ベクトルエンジン — 幾何積           │
│  第1層 ── encoding.rs — llm_encode、役割説明            │
│  第0層 ── bagua.rs — 八卦、五行、64卦                  │
└─────────────────────────────────────────────────────────┘
```

**8ブレード × 8役割 × 5行 × 64卦** — 五行の相生・相克サイクルによる
決定論的な関係分類を備えた、完全な閉形式の意味代数。
エラーが発生しやすい代数変換ではありません。

---

## ドキュメント

| ドキュメント | 目的 |
|----------|---------|
| **[システムガイド](docs/SYSTEM_GUIDE.md)** | 完全リファレンス：数学、分類法、操作、API、ベンチマーク |
| **[導入ガイド](docs/DELIVERY.md)** | クライアント別設定、トラブルシューティング、配布 |
| **[エンコードスキル](docs/skills/bagua-encoder/SKILL.md)** | LLMプロトコル — 8つの役割、評価基準、例 |
| **[戦略ロードマップ](docs/engineering/strategy-to-excellence.md)** | 7層の改善ロードマップ |
| **[ベンチマークレポート](docs/engineering/semantic-accuracy-benchmark.md)** | 率直な精度レポート |

## ライセンス

MIT OR Apache-2.0
