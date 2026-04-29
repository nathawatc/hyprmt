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

## Usage

- Short: `hyprmt -i eDP-1 -e HDMI-A-1`
- Long: `hyprmt --internal eDP-1 --external HDMI-A-1`

The default mode is `extended`.

### Modes

| Mode       | Description                                                  |
| ---------- | ------------------------------------------------------------ |
| `extended` | Reload Hyprland configuration                                |
| `internal` | Enable the internal display and disable the external display |
| `external` | Enable the external display and disable the internal display |

### Overlay Picker

- Short: `hyprmt -i eDP-1 -e HDMI-A-1 --overlay`
- Long: `hyprmt --internal eDP-1 --external HDMI-A-1 --overlay`

This opens `fuzzel` and lets you choose **Internal**, **External**, or **Extended**.

## BrightStay

> [!IMPORTANT]
> Not compiled by default, for personal use only.

Workaround for a firmware issue on the L22i-40 monitor. When the monitor wakes
from sleep in custom user color mode, it resets its brightness to 100%. This tool
re-applies the color preset (VCP code 0x14) to restore the correct brightness and
RGB Range settings manually.

## Hyprland Binds

See [Hyprland Bind Guide](docs/getting-started/hyprland-bind-guide.md) for ready-to-use `hyprland.conf` examples.

## Design Notes

> [!NOTE]
>
> - This utility only targets `monitorv2`
> - The current workflow reloads Hyprland before applying the single-display modes
> - `monitorv2[NAME]:disabled 0` is intentionally avoided on v0.54.3 it not restore resolution reliably <!-- rumdl-disable-line MD013 -->
