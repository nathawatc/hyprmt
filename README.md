# HyprMonitor Toggle

A small Hyprland monitor toggle utility inspired by the Windows + P display picker.
Designed for `monitorv2`-based setups and uses the `hyprland` crate for IPC.

## Features

- Toggle between internal-only, external-only, and extended display modes
- Pick the mode through a `fuzzel` overlay
- Uses Hyprland IPC instead of shelling out to `hyprctl` for the main logic

## Requirements

- Hyprland
- `fuzzel` if you want the overlay picker
- Your monitor names, for example `eDP-1` and `HDMI-A-1`

## Build

```bash
cargo build --release
```

### Usage

```bash
hyprmt --internal eDP-1 --external HDMI-A-1
```

The default mode is `extended`.

#### Modes

- `extended`: reload Hyprland configuration
- `internal`: enable the internal display and disable the external display
- `external`: enable the external display and disable the internal display

### Overlay picker

```bash
hyprmt --internal eDP-1 --external HDMI-A-1 --overlay
```

This opens `fuzzel` in dmenu mode and lets you choose `Internal`, `External`, or
`Extended`.

### Hyprland Binds

See [Hyprland Bind Guide](docs/getting-started/hyprland-bind-guide.md) for ready-to-use `hyprland.conf` examples.

> [!NOTE]
>
> - This utility only targets `monitorv2`
> - The current workflow reloads Hyprland before applying the single-display modes
> - `monitorv2[NAME]:disabled 0` is intentionally avoided because it does not restore the correct resolution reliably
