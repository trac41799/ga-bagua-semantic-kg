<div align="center">

[**English**](README.md) · [**中文**](README.zh.md) · [**Tiếng Việt**](README.vi.md) · [**한국어**](README.ko.md) · [**日本語**](README.ja.md)

</div>

---

<p align="center">
  <img src="docs/img/logo-bw.png" alt="GA-Bagua Đồ thị Tri thức Ngữ nghĩa" width="600">
</p>

<p align="center">
  <strong>Bộ nhớ ngữ nghĩa cho LLM — 8 chiều, 64 trạng thái quẻ, không cần huấn luyện.</strong><br>
  <a href="https://crates.io/crates/ga-semantics-core"><img src="https://img.shields.io/crates/v/ga-semantics-core?label=core" alt="Crates.io"></a>
  <a href="https://crates.io/crates/ga-semantics-mcp"><img src="https://img.shields.io/crates/v/ga-semantics-mcp?label=mcp" alt="Crates.io"></a>
  <a href="https://crates.io/crates/ga-semantics-cli"><img src="https://img.shields.io/crates/v/ga-semantics-cli?label=cli" alt="Crates.io"></a>
  <a href="https://www.npmjs.com/package/ga-semantics-mcp"><img src="https://img.shields.io/npm/v/ga-semantics-mcp?color=red" alt="npm"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue" alt="License"></a>
</p>

---

Mỗi khái niệm trở thành **8 con số**. Mỗi quan hệ là một **bước trong chu kỳ Ngũ Hành**.
Không cần huấn luyện. Không có cơ sở dữ liệu vector. Không gọi LLM lặp lại.
Suy luận hoàn tất trong **34 nano giây** đến **320 micro giây** với sai số tích lũy bằng không.

```
 Tên khái niệm
      │
      ▼
 LLM (đọc SKILL.md, ~200 token, một lần)
      │
      ▼
 [☷ 0.04, ☳ -0.09, ☵ -0.51, ☶ 0.68, ☲ 0.21, ☴ -0.26, ☱ 0.17, ☰ -0.34]
      │
      ├── tương tự?   →  dominant_similarity()
      ├── quan hệ?    →  classify_hexagram()       (tra chu kỳ Ngũ Hành)
      └── loại suy?   →  analogy()                  (dự đoán hướng chu kỳ)
```

| 木 Sinh Hỏa | 火 Sinh Thổ | 土 Sinh Kim | 金 Sinh Thủy | 水 Sinh Mộc |
|:---:|:---:|:---:|:---:|:---:|
| Quan hệ 100% | Vai trò 100% | P@K 73.3% | MRR 0.878 | Loại suy 80% |

---

## Cài đặt

```bash
# Cách 1 — npm (khuyến nghị)
npm install -g ga-semantics-mcp     # MCP server + CLI + skill

# Cách 2 — Cargo
cargo install ga-semantics-mcp      # MCP server
cargo install ga-semantics-cli      # CLI tool

# Cách 3 — tải binary
# Chọn nền tảng của bạn tại: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
```

Mỗi file nén chứa `ga-semantics-mcp` (server), `ga-semantics` (CLI) và `bagua-encoder-skill.md`.

---

## LLM Tự cài đặt

<details>
<summary><b>☰ Claude Desktop</b></summary>

```
Cài đặt MCP server + CLI GA-Bagua Đồ thị Tri thức Ngữ nghĩa cho Claude Desktop.

Bước 1 — Phát hiện hệ điều hành, tải xuống:
  https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
  (chọn .zip cho Windows hoặc .tar.gz cho macOS/Linux)

Bước 2 — Giải nén vào ~/.local/bin/ (macOS/Linux) hoặc %USERPROFILE%\.local\bin\ (Windows).
  Trên macOS/Linux: chmod +x cả hai file nhị phân.

Bước 3 — Cấu hình Claude Desktop MCP. Đọc hoặc tạo:
  Windows: %APPDATA%\Claude\claude_desktop_config.json
  macOS:   ~/Library/Application Support/Claude/claude_desktop_config.json
  Linux:   ~/.config/Claude/claude_desktop_config.json
  Thêm: { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<đường-dẫn>/ga-semantics-mcp" } } }

Bước 4 — Cài đặt skill: tải xuống
  https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
  Lưu vào %APPDATA%\Claude\skills\bagua-encoder\SKILL.md (hoặc ~/.config/Claude/skills/...)

Bước 5 — Khởi động lại Claude Desktop. Kiểm tra: "Mã hóa Rate Limiter bằng llm_encode
  với hệ số [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34]"
  Kết quả mong đợi: dominant_role = "constraining".
```
</details>

<details>
<summary><b>☲ OpenCode</b></summary>

```
Cài đặt GA-Bagua Đồ thị Tri thức Ngữ nghĩa cho OpenCode CLI. Tải từ releases, giải nén cả hai
file nhị phân. Cấu hình ~/.config/opencode/opencode.json:
  { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<đường-dẫn>/ga-semantics-mcp" } } }
Cài đặt skill vào ~/.config/opencode/skills/bagua-encoder/SKILL.md.
Khởi động lại và kiểm tra với llm_encode.
```
</details>

<details>
<summary><b>☵ Cursor</b></summary>

```
Cài đặt GA-Bagua Đồ thị Tri thức Ngữ nghĩa cho Cursor. Tải từ releases, giải nén cả hai file nhị phân.
Cấu hình ~/.cursor/mcp.json:
  { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<đường-dẫn>/ga-semantics-mcp" } } }
Cài đặt skill vào ~/.cursor/skills/bagua-encoder/SKILL.md.
Trong Composer agent, kiểm tra với llm_encode.
```
</details>

<details>
<summary><b>☳ Claude Code CLI</b></summary>

```
Cài đặt GA-Bagua Đồ thị Tri thức Ngữ nghĩa cho Claude Code. Tải từ releases, giải nén cả hai
file nhị phân. Cấu hình ~/.claude/mcp.json. Lưu skill vào ~/.claude/skills/bagua-encoder/.
Kiểm tra: echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | <đường-dẫn>/ga-semantics-mcp
```
</details>

<details>
<summary><b>☴ Continue.dev / ☱ Cline / ☰ Windsurf / khác</b></summary>

```
Tải từ https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
Giải nén, đặt file nhị phân vào PATH, cấu hình MCP settings của client.
Cài đặt skill từ docs/skills/bagua-encoder/SKILL.md
Xem docs/DELIVERY.md để biết hướng dẫn chi tiết cho từng client.
```
</details>

---

## CLI Sử dụng

```bash
# Mã hóa khái niệm
ga-semantics encode 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20
ga-semantics encode -j 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20 --json

# Phân loại quan hệ
ga-semantics classify \
  "[0.04,-0.09,-0.51,0.68,0.21,-0.26,0.17,-0.34]" \
  "[0.25,0.15,-0.10,0.55,0.40,0.05,0.30,0.20]"

# Tính độ tương tự
ga-semantics sim "[0.15,0.25,0.81,...]" "[0.30,0.10,0.60,...]"

# Giải loại suy
ga-semantics analogy  "[A]" "[B]" "[C]"

# Khám phá Bát Quái
ga-semantics trigram qian --transforms
ga-semantics hexagram "[A]" "[B]"
ga-semantics wuxing water --cycle controlling

# Đồ thị tri thức
ga-semantics store add "Auth System" 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20
ga-semantics store query "[0.05,-0.05,-0.45,0.70,0.15,-0.20,0.10,-0.30]"
ga-semantics store list
ga-semantics store export

# Điểm chuẩn
ga-semantics bench timing
ga-semantics bench semantic
```

`--json` cho đầu ra máy đọc được, `--csv` cho bảng tính, `--quiet` chỉ hiển thị giá trị.

---

## Bảng tra mã hóa nhanh

```
8 vai trò theo thứ tự:
[tiếp nhận, nhân quả, truyền dẫn, ràng buộc, làm rõ, ảnh hưởng, cân bằng, sáng tạo]

Thang đo:  >0.5 mạnh  |  0.2–0.5 trung bình  |  0.05–0.2 nhẹ
          -0.05–0.05 không liên quan  |  <-0.05 đối nghịch  |  <-0.5 đối nghịch mạnh

Chuẩn hóa về độ dài đơn vị. Chỉ xuất ra một mảng JSON gồm 8 số thực.
```

Xem **[SKILL.md](docs/skills/bagua-encoder/SKILL.md)** để biết giao thức mã hóa đầy đủ.

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

## Kiến trúc

```
┌─────────────────────────────────────────┐
│  Layer 4: MCP + CLI + Python            │
│  Layer 3: tương tự, loại suy, kho      │
│  Layer 2: Cl(3) đa vectơ                │
│  Layer 1: llm_encode, mô tả vai trò    │
│  Layer 0: bát quái, ngũ hành, 64 quẻ   │
└─────────────────────────────────────────┘
```

**8 blade × 8 vai trò × 5 hành × 64 quẻ** — một đại số ngữ nghĩa
khép kín hoàn chỉnh với phân loại quan hệ xác định thông qua chu kỳ
tương sinh/tương khắc của Ngũ Hành, thay vì các phép biến đổi đại số dễ sai sót.

</td>
<td width="400">
  <img src="docs/img/architecture.png" alt="Kiến trúc Hệ thống" width="400">
</td>
</tr>
</table>

<br>

<p align="center">
  <img src="docs/img/ga-bagua-encoding.jpg" alt="Quy trình Mã hóa" width="700">
</p>

<br>

<p align="center">
  <img src="docs/img/ga-bagua-wuxing.jpg" alt="Chu kỳ Ngũ Hành" width="500">
</p>

---

## Tài liệu

| Tài liệu | Mục đích |
|----------|---------|
| **[Hướng dẫn Hệ thống](docs/SYSTEM_GUIDE.md)** | Tham khảo đầy đủ: toán học, phân loại, thao tác, API, điểm chuẩn |
| **[Hướng dẫn Triển khai](docs/DELIVERY.md)** | Cấu hình từng client, xử lý sự cố, phân phối |
| **[Kỹ năng Mã hóa](docs/skills/bagua-encoder/SKILL.md)** | Giao thức LLM — 8 vai trò, thang điểm, ví dụ |
| **[Lộ trình Chiến lược](docs/engineering/strategy-to-excellence.md)** | Lộ trình cải tiến 7 tầng |
| **[Báo cáo Điểm chuẩn](docs/engineering/semantic-accuracy-benchmark.md)** | Báo cáo độ chính xác trung thực |

## Giấy phép

MIT OR Apache-2.0
