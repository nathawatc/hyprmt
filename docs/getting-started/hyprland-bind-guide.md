# Hyprland Bind Guide

This guide shows how to wire `hyprmt` into your Hyprland config.

## Prerequisites

Make sure the binary is available on your `PATH`, or use the full path to the
executable in the examples below.

You also need the exact Hyprland monitor names for your setup. Common examples
are `eDP-1` for the laptop panel and `HDMI-A-1` for an external display.

## Windows + P Style Picker

If you want a Windows + P style picker, bind a key to `--overlay`:

```ini
bind = $mainMod, P, exec, hyprmt --internal eDP-1 --external HDMI-A-1 --overlay
```

This opens `fuzzel`, lets you choose `Internal`, `External`, or `Extended`, and
then applies the selected mode.

## Direct Mode Binds

If you prefer dedicated hotkeys instead of a picker, bind each mode directly:

```ini
bind = $mainMod, I, exec, hyprmt --internal eDP-1 --external HDMI-A-1 internal
bind = $mainMod, E, exec, hyprmt --internal eDP-1 --external HDMI-A-1 external
bind = $mainMod, M, exec, hyprmt --internal eDP-1 --external HDMI-A-1 extended
```

## Using a Full Path

If `hyprmt` is not on `PATH`, call it by absolute path:

```ini
bind = $mainMod, P, exec, /home/nchalapinyo/.cargo/bin/hyprmt --internal eDP-1
--external HDMI-A-1 --overlay
```

Adjust the path to match where you installed the binary.

## Example `hyprland.conf`

```ini
$mainMod = SUPER

bind = $mainMod, P, exec, hyprmt --internal eDP-1 --external HDMI-A-1 --overlay
bind = $mainMod, I, exec, hyprmt --internal eDP-1 --external HDMI-A-1 internal
bind = $mainMod, E, exec, hyprmt --internal eDP-1 --external HDMI-A-1 external
bind = $mainMod, M, exec, hyprmt --internal eDP-1 --external HDMI-A-1 extended
```

## Notes

- `hyprmt` uses the `monitor` keyword API (Hyprland 0.55.0+).
- The tool reloads Hyprland before applying single-display modes.
- If a bind does nothing, verify the monitor names first with `hyprctl monitors`.
