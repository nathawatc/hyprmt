# Code Style and Conventions

## General Rules

- Keep the code ASCII-only unless a non-ASCII character is required for a specific reason.
- Keep functions small and direct.
- Prefer explicit error propagation with `anyhow::Result`.
- Avoid inline code comments unless a section is genuinely non-obvious.

## Rust Style

- Run `cargo fmt` before committing changes.
- Keep `cargo clippy --all-targets --all-features -- -D warnings` clean.
- Use strong types for user-facing concepts such as the mode selection.

## Hyprland Integration

- Use the `hyprland` crate for IPC when possible
- `src/ipc.rs` is the only manual IPC path, used for `eval` because the crate does not expose it yet
- Keep monitor and output names aligned with the actual Hyprland schema
