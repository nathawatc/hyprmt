# AGENTS.md

**Generated:** 2026-05-11
**Commit:** 8cc591e
**Branch:** main

## OVERVIEW

`hyprmt` — Single-binary Rust CLI for Hyprland monitor toggle (Windows+P style).
Communicates via `hyprland` crate IPC (not `hyprctl`).

## STRUCTURE

```text
hyprmt/
├── src/                   # Rust source (binary crate, no lib)
│   ├── main.rs            # CLI parsing, mode dispatch, Hyprland IPC
│   └── brightstay.rs      # DDC/EDID brightness fix (feature-gated)
├── docs/                  # Project documentation
│   ├── agents/            # Agent skills config (Forgejo issue tracker, triage, domain)
│   ├── architecture/      # Design notes, known limitations
│   ├── contributing/      # Code style, testing, AI-assisted contributions
│   └── getting-started/   # Build guide, Hyprland bind guide, project layout
├── arch/                  # Arch Linux PKGBUILDs (normal + brightstay variants)
├── .forgejo/              # Forgejo CI workflows (lint, publish to Codeberg registry)
├── .rumdl.toml            # Markdown lint config
└── Cargo.toml             # edition 2024, MSRV 1.85, feature: brightstay
```

## WHERE TO LOOK

| Task                     | Location                        | Notes                          |
| ------------------------ | ------------------------------- | ------------------------------ |
| Mode logic, CLI parsing  | `src/main.rs`                   | `Mode` enum, `main()` dispatch |
| Brightness fix (L22i-40) | `src/brightstay.rs`             | Feature-gated (`brightstay`)   |
| Architecture             | `docs/architecture/overview.md` |                                |
| Forgejo issue ops        | `docs/agents/issue-tracker.md`  | Uses `fj` CLI                  |
| Triage labels            | `docs/agents/triage-labels.md`  | Role→label mapping             |
| CI config                | `.forgejo/workflows/`           | Lint + publish                 |

## CONVENTIONS

- **edition 2024** + **MSRV 1.85** — pinned in `Cargo.toml`
- **No lib.rs** — pure binary crate, no library target
- **No workspace** — single crate at root
- **`brightstay` feature** gates DDC/i2c/serde deps (`default-features = false` disables it)
- **No tests** — no `tests/` dir, no `#[test]`, no dev-dependencies
- **`cargo fmt` + `clippy -D warnings`** must pass before commit

## FORBIDDEN

- Using `hyprctl` for main logic — use `hyprland` crate IPC
- `monitorv2` dispatcher — use `Keyword::set("monitor", ...)` (Hyprland 0.55.0+)

## KEY CONSTRAINTS

- Uses `Keyword::set("monitor", "...")` (Hyprland 0.55.0+ `monitor` keyword syntax)
- Single-display modes use reload-first workflow (`monitor "NAME, disable"`)
- `--overlay` requires `fuzzel` on PATH

## COMMANDS

```bash
cargo build                    # debug
cargo build --release          # optimized (LTO, strip, abort)
cargo fmt --check              # formatting
cargo clippy --all-targets --all-features -- -D warnings  # lint
cargo run -- -i eDP-1 -e HDMI-A-1                         # default: extended
cargo run -- -i eDP-1 -e HDMI-A-1 --overlay                # fuzzel picker
```

## AGENT SKILLS

### Issue Tracker

Forgejo issues on codeberg.org/nathawat/hyprmt. See `docs/agents/issue-tracker.md`.

### Triage Labels

`Priority/Low` (needs-triage), `Status/Need More Info` (needs-info), `Priority/Medium`
(ready-for-agent), `Priority/Critical` (ready-for-human), `Reviewed/Won't Fix` (wontfix).
See `docs/agents/triage-labels.md`.

### Domain Docs

Single-context: `CONTEXT.md` at repo root, `docs/adr/` for architectural decisions.
See `docs/agents/domain.md`.

## NOTES

- **CONTEXT.md** does not exist at root (referred in docs but deferred creation per agent conventions)
- **docs/adr/** does not exist (no ADRs yet)
- **Release**: tag `v*.*.*` triggers `publish.yml` → Codeberg cargo registry
- **Forgejo** (not GitHub) — CI in `.forgejo/workflows/`
- **Arch packages**: `arch/normal/` and `arch/brightstay/` with separate PKGBUILDs
