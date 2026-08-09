# AGENTS.md

**Generated:** 2026-08-09
**Branch:** main

## OVERVIEW

`hyprmt` is a single-binary Rust CLI for Windows+P-style Hyprland monitor switching. It uses the
`hyprland` crate for reload and monitor-data IPC, plus manual Unix-socket `eval` IPC for
`hl.monitor(...)` because the crate does not expose `eval`.

The optional `brightstay` feature adds DDC/i2c handling for the Lenovo L22i-40 brightness workaround.

## STRUCTURE

```text
hyprmt/
├── src/
│   ├── main.rs                         # Clap CLI, mode dispatch, monitor operations, fuzzel picker
│   ├── ipc.rs                          # Manual Hyprland /eval socket client
│   └── brightstay.rs                   # Feature-gated DDC/EDID brightness workaround
├── arch/
│   ├── assets/                         # systemd, sysusers, tmpfiles, udev, kernel-module assets
│   ├── hyprmt/                         # AUR git submodule from .gitmodules
│   └── hyprmt-brightstay/              # Arch PKGBUILD, .SRCINFO, install script
├── .forgejo/workflows/lint.yml         # Formatting and Clippy CI
├── .gitmodules                         # AUR submodule declaration
├── .rumdl.toml                         # Markdown lint configuration
├── README.md                           # Project documentation and usage
├── Cargo.toml                          # Package, dependencies, feature, profiles
├── Cargo.lock                           # Locked dependency graph
└── AGENTS.md                           # Repository guidance
```

## WHERE TO LOOK

| Task                          | Location                      | Notes                                                  |
| ----------------------------- | ----------------------------- | ------------------------------------------------------ |
| CLI parsing and mode dispatch | `src/main.rs`                 | `Cli`, `Mode`, `main()`                                |
| Conditional monitor arguments | `src/main.rs`                 | Clap relations on `internal` and `external`            |
| Hyprland eval IPC             | `src/ipc.rs`                  | Socket discovery, `/eval` writes, debug response check |
| BrightStay hardware logic     | `src/brightstay.rs`           | Feature-gated DDC, EDID, cache, and tests              |
| User-facing usage             | `README.md`                   | Build, modes, overlay, BrightStay notes                |
| Base Arch package             | `arch/hyprmt/`                | AUR submodule; parent repository tracks gitlink only   |
| BrightStay Arch package       | `arch/hyprmt-brightstay/`     | `PKGBUILD`, `.SRCINFO`, `hyprmt.install`               |
| Shared Arch assets            | `arch/assets/`                | systemd, sysusers, tmpfiles, udev, modules-load        |
| CI checks                     | `.forgejo/workflows/lint.yml` | Stable toolchain, `fmt`, all-target/all-feature Clippy |

## CONVENTIONS

- Rust edition 2024; `rust-version = "1.92"` is the minimum supported toolchain.
- No workspace and no `lib.rs`; this is a binary crate.
- `brightstay` is optional and enables `ddc`, `ddc-i2c`, and `i2c-linux`.
- `Mode::Extended` is the default mode.
- `--internal`/`-i` and `--external`/`-e` are required for default, internal, external,
  extended, and overlay modes. They are optional only for direct `brightstay` mode.
- Overlay mode still requires both monitor names, even if picker later selects BrightStay.
- `overlay` launches `fuzzel` with dmenu input; fuzzel must be on `PATH`.
- Single-display operations validate monitor presence, reload Hyprland, validate the disabled
  monitor, then send the `hl.monitor(...)` disable command.
- Unit tests live beside implementation in `src/main.rs`, `src/ipc.rs`, and feature-gated
  `src/brightstay.rs`. There is no `tests/` directory or dev-dependency section.
- Match existing Rust style. Run formatting and Clippy before committing.

## FORBIDDEN

- Do not shell out to `hyprctl` for main monitor logic; use `hyprland` crate IPC.
- Do not use the deprecated `monitorv2` dispatcher; use `eval "hl.monitor({...})"` on Hyprland
  0.55.0+.
- Do not make BrightStay runtime create `/var/cache/fix-l22i-40` or call `chown`.
- Do not treat generated Arch package artifacts as source files.

## KEY CONSTRAINTS

- Hyprland target version is 0.55.0+ because code calls the Lua `hl.monitor` config API.
- `src/ipc.rs` resolves the socket from `XDG_RUNTIME_DIR` and
  `HYPRLAND_INSTANCE_SIGNATURE`, then connects to `.socket.sock` and writes `/eval` plus Lua.
- Debug builds read and require an `ok` response from Hyprland; release builds only write the
  command.
- Monitor names are interpolated into Lua string literals with escaping for backslashes, quotes,
  and control characters.
- BrightStay targets EDID product code `26542`, sets DDC VCP code `0x14` to value `11`, and
  uses `/dev/i2c-*` plus `/sys/class/i2c-dev` for detection.
- BrightStay cache file is `/var/cache/fix-l22i-40/monitor.cache`, formatted as `bus:product_code`.
- Cache directory ownership and mode come from `arch/assets/tmpfiles.d/fix-l22i-40.conf`:
  `root:i2c`, mode `2775`.
- BrightStay reads cache first, falls back to hardware scanning, and fails open when cache or
  hardware detection fails.
- Cache writes use a create-new temporary file, permissions `0664`, `fsync`, rename, and parent
  directory `fsync`; readers must tolerate atomic replacement.
- The BrightStay package installs systemd units, sysusers, tmpfiles, udev, and `i2c-dev` module
  assets. Its install script reminds users to join group `i2c` and enable units manually.

## COMMANDS

```bash
cargo build
cargo build --release
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test
cargo test --all-features
cargo run -- -i eDP-1 -e HDMI-A-1
cargo run -- -i eDP-1 -e HDMI-A-1 internal
cargo run -- -i eDP-1 -e HDMI-A-1 overlay
cargo run --features brightstay -- brightstay
```

`cargo run --features brightstay -- brightstay` needs suitable Hyprland, DDC, i2c, and device
permissions to perform hardware work.

## PACKAGING

- `arch/hyprmt/` is an AUR git submodule declared in `.gitmodules`; do not edit its contents from
  the parent checkout unless intentionally working in the submodule.
- `arch/hyprmt-brightstay/PKGBUILD` builds with `--features brightstay`, runs feature-enabled
  tests, installs `/usr/bin/hyprmt`, and installs README plus shared service/device assets.
- `arch/hyprmt-brightstay/hyprmt.install` contains post-install, upgrade, and removal notices.
- Keep `.SRCINFO` synchronized with its `PKGBUILD` when changing Arch package metadata.

## CI

Forgejo workflow `.forgejo/workflows/lint.yml` runs on pull requests targeting `main` or `master`,
plus manual dispatch. It installs stable Rust with `rustfmt` and `clippy`, then runs:

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
```

CI does not publish crates or run the test suite.

## NOTES

- Repository: `https://codeberg.org/nathawat/hyprmt`.
- README is canonical project documentation after removal of `docs/`.
- Release profile uses LTO, one codegen unit, stripped symbols, and `panic = "abort"`.
- Cargo dependency patches point to the `v0.3.x` branches of the DDC/i2c Git repositories.
