<div align="center">

[**English**](README.md) · [**中文**](README.zh.md) · [**Tiếng Việt**](README.vi.md) · [**한국어**](README.ko.md) · [**日本語**](README.ja.md)

</div>

---

<div align="center">

# GA-Bagua 의미 지식 그래프

**LLM 의미 기억 — 8차원, 64괘 상태, 훈련 불필요.**

[![Crates.io](https://img.shields.io/crates/v/ga-semantics-core?label=core)](https://crates.io/crates/ga-semantics-core)
[![Crates.io](https://img.shields.io/crates/v/ga-semantics-mcp?label=mcp)](https://crates.io/crates/ga-semantics-mcp)
[![Crates.io](https://img.shields.io/crates/v/ga-semantics-cli?label=cli)](https://crates.io/crates/ga-semantics-cli)
[![npm](https://img.shields.io/npm/v/ga-semantics-mcp?color=red)](https://www.npmjs.com/package/ga-semantics-mcp)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

</div>

```
                    ☰                        건 / 창조
                ☱       ☴                  태 / 균형      손 / 영향
                    ☲                        리 / 명료화
                ☳       ☶                  진 / 인과      간 / 제약
                    ☵                        감 / 전달
                    ☷                        곤 / 수용
```

---

모든 개념은 **8개의 숫자**가 됩니다. 모든 관계는 **오행 한 걸음**입니다.
훈련이 필요 없습니다. 벡터 데이터베이스가 필요 없습니다. 반복적인 LLM 호출이 필요 없습니다.
추론은 **34나노초**에서 **320마이크로초** 사이에 완료되며, 누적 오차는 0입니다.

```
 개념 이름
      │
      ▼
 LLM (SKILL.md 읽기, ~200토큰, 한 번)
      │
      ▼
 [☷ 0.04, ☳ -0.09, ☵ -0.51, ☶ 0.68, ☲ 0.21, ☴ -0.26, ☱ 0.17, ☰ -0.34]
      │
      ├── 유사?      →  dominant_similarity()
      ├── 관계?      →  classify_hexagram()       (오행 주기 조회)
      └── 유추?      →  analogy()                  (주기 방향 예측)
```

| 木 生火 | 火 生土 | 土 生金 | 金 生水 | 水 生木 |
|:---:|:---:|:---:|:---:|:---:|
| 관계 100% | 역할 100% | P@K 73.3% | MRR 0.878 | 유추 80% |

---

## 설치

```bash
# 방법 1 — npm (권장)
npm install -g ga-semantics-mcp     # MCP 서버 + CLI + 스킬 파일

# 방법 2 — Cargo
cargo install ga-semantics-mcp      # MCP 서버
cargo install ga-semantics-cli      # CLI 도구

# 방법 3 — 바이너리 다운로드
# 다음에서 플랫폼 선택: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
```

각 아카이브에는 `ga-semantics-mcp`(서버), `ga-semantics`(CLI) 및 `bagua-encoder-skill.md`가 포함되어 있습니다.

---

## LLM 자가 설치

<details>
<summary><b>☰ Claude Desktop</b></summary>

```
Claude Desktop용 GA-Bagua 의미 지식 그래프 MCP 서버 + CLI를 설치합니다.

1단계 — 운영체제 감지, 다운로드:
  https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
  (Windows는 .zip, macOS/Linux는 .tar.gz 선택)

2단계 — ~/.local/bin/(macOS/Linux) 또는 %USERPROFILE%\.local\bin\(Windows)에 압축 해제.
  macOS/Linux: 두 바이너리에 chmod +x 실행.

3단계 — Claude Desktop MCP 구성. 읽거나 생성:
  Windows: %APPDATA%\Claude\claude_desktop_config.json
  macOS:   ~/Library/Application Support/Claude/claude_desktop_config.json
  Linux:   ~/.config/Claude/claude_desktop_config.json
  추가: { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<경로>/ga-semantics-mcp" } } }

4단계 — 스킬 파일 설치: 다운로드
  https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
  %APPDATA%\Claude\skills\bagua-encoder\SKILL.md (또는 ~/.config/Claude/skills/...)에 저장

5단계 — Claude Desktop 재시작. 확인: "llm_encode로 Rate Limiter 인코딩,
  계수 [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34]"
  예상 결과: dominant_role = "constraining".
```
</details>

<details>
<summary><b>☲ OpenCode</b></summary>

```
OpenCode CLI용 GA-Bagua 의미 지식 그래프 설치. 릴리스에서 다운로드, 두 바이너리
압축 해제. ~/.config/opencode/opencode.json 구성:
  { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<경로>/ga-semantics-mcp" } } }
스킬 파일을 ~/.config/opencode/skills/bagua-encoder/SKILL.md에 설치.
재시작 후 llm_encode로 확인.
```
</details>

<details>
<summary><b>☵ Cursor</b></summary>

```
Cursor용 GA-Bagua 의미 지식 그래프 설치. 릴리스에서 다운로드, 두 바이너리 압축 해제.
~/.cursor/mcp.json 구성:
  { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<경로>/ga-semantics-mcp" } } }
스킬 파일을 ~/.cursor/skills/bagua-encoder/SKILL.md에 설치.
Composer 에이전트에서 llm_encode로 확인.
```
</details>

<details>
<summary><b>☳ Claude Code CLI</b></summary>

```
Claude Code용 GA-Bagua 의미 지식 그래프 설치. 릴리스에서 다운로드, 두 바이너리
압축 해제. ~/.claude/mcp.json 구성. 스킬을 ~/.claude/skills/bagua-encoder/에 저장.
확인: echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | <경로>/ga-semantics-mcp
```
</details>

<details>
<summary><b>☴ Continue.dev / ☱ Cline / ☰ Windsurf / 기타</b></summary>

```
https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest에서 다운로드
압축 해제, 바이너리를 PATH에 추가, 클라이언트 MCP 설정 구성.
docs/skills/bagua-encoder/SKILL.md에서 스킬 파일 설치
자세한 클라이언트별 지침은 docs/DELIVERY.md 참조.
```
</details>

---

## CLI 사용법

```bash
# 개념 인코딩
ga-semantics encode 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20
ga-semantics encode -j 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20 --json

# 관계 분류
ga-semantics classify \
  "[0.04,-0.09,-0.51,0.68,0.21,-0.26,0.17,-0.34]" \
  "[0.25,0.15,-0.10,0.55,0.40,0.05,0.30,0.20]"

# 유사도 계산
ga-semantics sim "[0.15,0.25,0.81,...]" "[0.30,0.10,0.60,...]"

# 유추 풀이
ga-semantics analogy  "[A]" "[B]" "[C]"

# 팔괘 탐험
ga-semantics trigram qian --transforms
ga-semantics hexagram "[A]" "[B]"
ga-semantics wuxing water --cycle controlling

# 지식 그래프
ga-semantics store add "Auth System" 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20
ga-semantics store query "[0.05,-0.05,-0.45,0.70,0.15,-0.20,0.10,-0.30]"
ga-semantics store list
ga-semantics store export

# 벤치마크
ga-semantics bench timing
ga-semantics bench semantic
```

`--json`은 기계 판독 가능 출력, `--csv`는 스프레드시트, `--quiet`는 값만 출력.

---

## 인코딩 빠른 참조표

```
순서대로 8가지 역할:
[수용, 인과, 전달, 제약, 명료화, 영향, 균형, 창조]

척도:  >0.5 강함  |  0.2–0.5 중간  |  0.05–0.2 약간
       -0.05–0.05 관련 없음  |  <-0.05 반대  |  <-0.5 강한 반대

단위 길이로 정규화. 8개 부동소수점 JSON 배열만 출력.
```

전체 인코딩 프로토콜은 **[SKILL.md](docs/skills/bagua-encoder/SKILL.md)** 참조.

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

## 아키텍처

```
┌─────────────────────────────────────────────────────────┐
│  4층 ── MCP 서버 (29개 도구) + CLI + Python             │
│  3층 ── semantics.rs — 유사도, 유추, 저장소             │
│  2층 ── Cl(3) 다중벡터 엔진 — 기하 곱                   │
│  1층 ── encoding.rs — llm_encode, 역할 설명             │
│  0층 ── bagua.rs — 팔괘, 오행, 64괘                    │
└─────────────────────────────────────────────────────────┘
```

**8개 블레이드 × 8개 역할 × 5개 행 × 64개 괘** — 오행의 상생/상극
주기를 통한 결정론적 관계 분류를 갖춘 완전한 폐쇄형 의미 대수학.
오류가 발생하기 쉬운 대수 변환이 아닙니다.

---

## 문서

| 문서 | 목적 |
|----------|---------|
| **[시스템 가이드](docs/SYSTEM_GUIDE.md)** | 전체 참조: 수학, 분류법, 연산, API, 벤치마크 |
| **[배포 가이드](docs/DELIVERY.md)** | 클라이언트별 설정, 문제 해결, 배포 |
| **[인코딩 스킬](docs/skills/bagua-encoder/SKILL.md)** | LLM 프로토콜 — 8가지 역할, 평가 기준, 예제 |
| **[전략 로드맵](docs/engineering/strategy-to-excellence.md)** | 7계층 개선 로드맵 |
| **[벤치마크 보고서](docs/engineering/semantic-accuracy-benchmark.md)** | 정직한 정확도 보고서 |

## 라이선스

MIT OR Apache-2.0
