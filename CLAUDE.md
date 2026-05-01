# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build

```bash
cargo build              # debug
cargo build --release    # optimized binary
```

## Run

```bash
cargo run -- -i eDP-1 -e HDMI-A-1              # default: extended mode
cargo run -- -i eDP-1 -e HDMI-A-1 --overlay    # fuzzel picker
```

## Architecture

`hyprmt` is a single-binary Rust CLI. It communicates with Hyprland via the `hyprland` crate's IPC
(not `hyprctl`):

- `hyprland::ctl::reload::call()` — config reload
- `hyprland::data::Monitors::get()` — monitor validation
- `hyprland::keyword::Keyword::set()` — `monitorv2[NAME]:disabled` toggle

## Source

- `src/main.rs` — CLI parsing, mode dispatch, Hyprland IPC
- `src/brightstay.rs` — DDC/EDID brightness fix for L22i-40 monitor; gated behind `brightstay` feature (not compiled by default) <!-- rumdl-disable-line MD013 -->

## Code Style

- Run `cargo fmt` before committing
- Keep `cargo clippy --all-targets --all-features -- -D warnings` clean
- Strong types for user-facing concepts (e.g., `Mode` enum)

## Agent Skills

### Issue Tracker

Forgejo issues on codeberg.org/nathawat/hyprmt. See `docs/agents/issue-tracker.md`.

### Triage Labels

`Priority/Low` (needs-triage), `Status/Need More Info` (needs-info), `Priority/Medium`
(ready-for-agent), `Priority/Critical` (ready-for-human), `Reviewed/Won't Fix` (wontfix). See
`docs/agents/triage-labels.md`.

### Domain Docs

Single-context: `CONTEXT.md` at repo root, `docs/adr/` for architectural decisions. See `docs/agents/domain.md`.

## Key Constraints

- Targets `monitorv2` only (no older monitor syntax)
- Single-display modes use reload-first workflow (`monitorv2[NAME]:disabled 1`)
- `--overlay` requires `fuzzel` on PATH
