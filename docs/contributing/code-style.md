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

- Use the `hyprland` crate for IPC instead of spawning `hyprctl` unless there is a clear gap in the crate API.
- Keep monitor and keyword names aligned with the actual Hyprland schema.
