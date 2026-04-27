# Known Limitations and Design Debt

## Monitor Handling

`hyprmt` currently assumes a simple two-display setup with one internal and one
external monitor supplied by the user.

The tool validates that both monitor names exist in the current Hyprland
session, but it does not try to infer which connected display should be treated
as internal or external.

## `monitorv2` Only

The implementation only targets `monitorv2`. It does not manage older monitor
syntax or attempt to translate between Hyprland configuration styles.

## Reload-Based Switching

Single-display modes currently use a reload-first workflow because the direct
re-enable path for `monitorv2[NAME]:disabled 0` does not restore resolution
reliably.

That means the current behavior is practical but not ideal if you want a pure
live-reconfiguration path without reloading Hyprland.

## Overlay Dependency

The `--overlay` flag depends on `fuzzel` being installed and available on `PATH`.

If `fuzzel` is missing, the command fails before any display change is applied.
