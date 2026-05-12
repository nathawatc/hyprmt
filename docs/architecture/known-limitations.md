# Known Limitations and Design Debt

## Monitor Handling

`hyprmt` currently assumes a simple two-display setup with one internal and one
external monitor supplied by the user.

The tool validates that both monitor names exist in the current Hyprland
session, but it does not try to infer which connected display should be treated
as internal or external.

## Hyprland 0.55.0+ Only

The implementation uses the Lua `hl.monitor` config API introduced in Hyprland 0.55.0.
It does not support the removed `monitorv2` dispatcher or older monitor syntax.

## Hyprland IPC Eval Gap

The `hyprland` crate does not expose `eval` yet, so `hyprmt` sends `/eval` directly
to the Hyprland socket as a workaround.

## Reload-Based Switching

Single-display modes currently use a reload-first workflow because the direct
re-enable path for `monitor "NAME, [config]"` requires manual parameters not inherited from config.

That means the current behavior is practical but not ideal if you want a pure
live-reconfiguration path without reloading Hyprland.

## Overlay Dependency

The `--overlay` flag depends on `fuzzel` being installed and available on `PATH`.

If `fuzzel` is missing, the command fails before any display change is applied.
