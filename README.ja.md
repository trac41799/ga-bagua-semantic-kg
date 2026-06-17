<div align="center">

[**English**](README.md) · [**中文**](README.zh.md) · [**Tiếng Việt**](README.vi.md) · [**한국어**](README.ko.md) · [**日本語**](README.ja.md)

</div>

---

<p align="center">
  <img src="docs/img/logo-bw.png" alt="GA-Bagua セマンティック知識グラフ" width="600">
</p>

<p align="center">
  <strong>LLMの意味記憶 — 8次元、64卦状態、学習不要。</strong><br>
  <a href="https://crates.io/crates/ga-semantics-core"><img src="https://img.shields.io/crates/v/ga-semantics-core?label=core" alt="Crates.io"></a>
  <a href="https://www.npmjs.com/package/ga-semantics-mcp"><img src="https://img.shields.io/npm/v/ga-semantics-mcp?color=red" alt="npm"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue" alt="License"></a>
</p>

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

## 機能

| 機能 | 方法 | パフォーマンス |
|-----------|-----|:----------:|
| **同役割検索** | クエリと同じ八卦の役割を持つ概念を検索 | 42% P@1 (same domain), 100% R@10 |
| **相補的発見** | 任意の概念の対極を見つける（GA-Bagua固有） | 正確な卦レベルマッチング |
| **五行パス探索** | 相生/相克チェーンに沿ったマルチホップ探索 | 500ns/ホップ |
| **マルチホップ合成** | ローター代数による100推論ステップの合成 | 200µs、ゼロドリフト |
| **エンコード安定性** | 同じ概念 → 毎回同じラベル | 99.8%（±5%ノイズ下） |
| **概念進化** | 一つの側面が変化したときの概念の行方を予測 | 決定論的変爻変換 |
| **関係分類** | LLM検証のための方向性ヒント | 45–52% テスト精度 |
| **ストレージ密度** | 概念あたり64バイト。100万概念 = 64 MB | BERTより48倍高密度 |
| **ゼロクエリコスト** | エンコード後の全操作は純粋な代数 | 0トークン、500ns/操作 |
| **シャープネスゲート** | ランダムノイズは信頼度0.0 | 93.5%のランダムペアが遮断 |
| **文書アライメント** | 関係分類による文書横断的クレームマッチング | Precision@5 ≥ 70% |
| **ポリシー一貫性** | 文書内/文書間の矛盾条項を検出 | F1 ≥ 0.67 |
| **議論分析** | 循環論法、非 sequitur、矛盾した議論を検出 | F1 ≥ 0.89 |
| **チーム互換性** | 五行ベースの性格マッチングとチーム編成 | Compatible > identical pairs |
| **学習パス** | 五行順序のカリキュラムシーケンスを生成 | 正しいフェーズ順序 |
| **創造的発想** | ローターによる64卦視点探索 | 3+ trigram coverage |

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

## 仕組み

### エンコード（LLM、一回限り）
```
概念説明 → SKILL.mdプロトコル → LLMが8つの係数を出力 → llm_encode() → 64バイト多重ベクトル
トークンコスト：概念あたり約200トークン（一回限り）
```

### 検索（代数的、ゼロトークン）
```
"制約する概念を検索" → WuXingIndexが土相バケットをスキャン → dominant_similarityでランク付け → top-Kを返す
レイテンシ：500ns/クエリ。トークン：0。
```

### パイプラインパターン（LLM + GA-Bagua）
```
1. GA-Baguaがtop-K候補を提示（0トークン）
2. LLMが各候補を元の説明と照合（各15トークン）
3. LLMが結果を推論し、発見を提示（50トークン）
クエリあたり合計：約150トークン vs 全説明読込の約4,000トークン
```

## 八卦の8つの役割

| 役割 | 卦 | 五行 | Blade | 説明 |
|------|---------|--------|-------|-------------|
| 生成 | Qian ☰ | Metal | e123 | 新しいパターンを創造、開始する |
| 受容 | Kun ☷ | Earth | scalar | 受け入れ、従う、接地する |
| 因果 | Zhen ☳ | Wood | e1 | 引き金となり、連鎖反応を開始する |
| 伝達 | Kan ☵ | Water | e2 | 導き、流れ、伝達する |
| 制約 | Gen ☶ | Earth | e3 | 制限し、境界を定め、抑制する |
| 影響 | Xun ☴ | Wood | e23 | 浸透し、徐々に影響を与える |
| 明確化 | Li ☲ | Fire | e12 | 明らかにし、照らし出す |
| 均衡 | Dui ☱ | Metal | e31 | 反映し、均衡させ、映し出す |

## 現在のベンチマーク

| 指標 | 値 | 備考 |
|--------|:-----:|-------|
| Same-role P@1 (same domain) | 42% | dominant_similarity；ボトルネックはエンコードの識別性 |
| Same-role R@10 (same domain) | 100% | 全ての同役割ピアがtop-10に出現 |
| 関係分類（テスト） | 45–52% | from_pair_multi（ホールドアウトペア） |
| エンコード安定性 | 99.8% | ±5%ノイズ下で支配的役割が保持される |
| マルチホップ（100ホップ） | 200µs、ゼロドリフト | GA-Bagua固有 |
| トークン節約（200クエリ） | 219x | $101.00 → $0.46/セッション |
| ランダムペア遮断 | 93.5% → 0.0 conf | シャープネス閾値0.25 |
| 全8ラベル予測 | はい | from_pair_multiが全て同時にスコア |
| ストレージ | 64バイト/概念 | 100万概念 = 64 MB |
| クエリレイテンシ | 500ns | 代数的、API不要、GPU不要 |

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

<table>
<tr>
<td valign="top">

## アーキテクチャ

```
┌─────────────────────────────────────────┐
│  Layer 4: MCP + CLI + Python            │
│  Layer 3: 類似度、類推、ストア          │
│  Layer 2: Cl(3) 多重ベクトルエンジン    │
│  Layer 1: llm_encode、役割説明          │
│  Layer 0: 八卦、五行、64卦              │
└─────────────────────────────────────────┘
```

**8ブレード × 8役割 × 5行 × 64卦** — 五行の相生・相克サイクルによる
決定論的な関係分類を備えた、完全な閉形式の意味代数。
エラーが発生しやすい代数変換ではありません。

</td>
<td width="400">
  <img src="docs/img/architecture.png" alt="システムアーキテクチャ" width="400">
</td>
</tr>
</table>

<br>

<p align="center">
  <img src="docs/img/ga-bagua-encoding.jpg" alt="エンコードパイプライン" width="700">
</p>

<br>

<p align="center">
  <img src="docs/img/ga-bagua-wuxing.jpg" alt="五行のサイクル" width="500">
</p>

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
