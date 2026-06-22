# AGENTS.md

**Generated:** 2026-06-22
**Branch:** main

## OVERVIEW

`hyprmt` — Single-binary Rust CLI for Hyprland monitor toggle (Windows+P style). Communicates via
`hyprland` crate IPC, plus manual IPC `eval` for `hl.monitor` because the crate does not expose
`eval` yet.

## STRUCTURE

```text
hyprmt/
├── src/                   # Rust source (binary crate, no lib)
│   ├── main.rs            # CLI parsing, mode dispatch, Hyprland IPC
│   ├── ipc.rs             # Manual IPC eval helper (crate lacks eval)
│   └── brightstay.rs      # DDC/EDID brightness fix (feature-gated)
├── docs/                  # Project documentation
│   ├── agents/            # Agent skills config (Forgejo issue tracker, triage, domain)
│   ├── architecture/      # Design notes, known limitations
│   ├── contributing/      # Code style, testing, AI-assisted contributions
│   └── getting-started/   # Build guide, Hyprland bind guide, project layout
├── arch/                  # Arch packaging: PKGBUILDs, .SRCINFO, shared package assets
├── .forgejo/              # Forgejo CI workflows (lint only)
├── .rumdl.toml            # Markdown lint config
└── Cargo.toml             # edition 2024, MSRV 1.85, feature: brightstay
```

## WHERE TO LOOK

| Task                     | Location                        | Notes                                           |
| ------------------------ | ------------------------------- | ----------------------------------------------- |
| Mode logic, CLI parsing  | `src/main.rs`                   | `Mode` enum, `main()` dispatch                  |
| Brightness fix (L22i-40) | `src/brightstay.rs`             | Feature-gated (`brightstay`)                    |
| Architecture             | `docs/architecture/overview.md` |                                                 |
| Forgejo issue ops        | `docs/agents/issue-tracker.md`  | Uses `fj` CLI                                   |
| Triage labels            | `docs/agents/triage-labels.md`  | Role→label mapping                              |
| CI config                | `.forgejo/workflows/`           | Lint only (publish removed)                     |
| Arch packages            | `arch/hyprmt*/`                 | PKGBUILD, .SRCINFO, install scriptlets          |
| Arch shared assets       | `arch/assets/`                  | sysusers, udev, modules-load, systemd, tmpfiles |

## CONVENTIONS

- **edition 2024** + **MSRV 1.85** — pinned in `Cargo.toml`
- **No lib.rs** — pure binary crate, no library target
- **No workspace** — single crate at root
- **`brightstay` feature** gates DDC/i2c deps (`default-features = false` disables it)
- **Focused unit tests exist in `src/brightstay.rs`** for cache parsing/fallback/atomic writes
- **No integration tests** — no `tests/` dir, no dev-dependencies
- **`cargo fmt` + `clippy -D warnings`** must pass before commit

## FORBIDDEN

- Using `hyprctl` for main logic — use `hyprland` crate IPC
- `monitorv2` dispatcher — use IPC `eval "hl.monitor({...})"` (Hyprland 0.55.0+)

## KEY CONSTRAINTS

- Uses IPC `eval "hl.monitor({output='NAME', disabled=true})"` to disable monitors (Hyprland 0.55.0+)
- Manual IPC eval in `src/ipc.rs` because the `hyprland` crate does not expose `eval` yet
- Single-display modes use reload-first workflow (`hl.monitor(...)` via eval)
- `--overlay` requires `fuzzel` on PATH
- Brightstay monitor cache is shared hardware state at `/var/cache/fix-l22i-40/monitor.cache`
- Brightstay cache directory is owned by systemd-tmpfiles via `arch/assets/tmpfiles.d/fix-l22i-40.conf`
- Runtime must not create cache directory or call `chown`; cache failures fail open to hardware detection
- Cache writes use temp file + fsync + rename; readers tolerate atomic replacement

## COMMANDS

```bash
cargo build                    # debug
cargo build --release          # optimized (LTO, strip, abort)
cargo fmt --check              # formatting
cargo clippy --all-targets --all-features -- -D warnings  # lint
cargo test --all-features                                  # tests
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
- **Release**: tag `v*.*.*` triggers lint workflow only
- **Forgejo** (not GitHub) — CI in `.forgejo/workflows/`
- **Arch packages**: `arch/hyprmt/` and `arch/hyprmt-brightstay/` with separate PKGBUILDs
- **Arch package assets**: shared assets live under `arch/assets/`; package install scriptlets
  live beside each PKGBUILD as `hyprmt.install`
- **Arch install scripts**: PKGBUILDs use `install=hyprmt.install`; do not install scriptlets
  as payload files
- **Arch tmpfiles**: brightstay installs `/usr/lib/tmpfiles.d/fix-l22i-40.conf` from `arch/assets/tmpfiles.d/`
- **No publish** — Codeberg cargo registry publishing disabled
