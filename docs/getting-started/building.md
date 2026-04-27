# Building from Source

## Prerequisites

- Rust toolchain with Cargo
- A working Hyprland session for runtime testing
- `fuzzel` if you want to use the overlay picker

## Build

```bash
cargo build
```

For a release binary:

```bash
cargo build --release
```

## Run

```bash
cargo run -- --internal eDP-1 --external HDMI-A-1
```

To use the overlay picker:

```bash
cargo run -- --internal eDP-1 --external HDMI-A-1 --overlay
```

## Installation

If you want to install the binary locally:

```bash
cargo install --path .
```
