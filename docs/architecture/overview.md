# Architecture Overview

`hyprmt` is a single-binary Rust CLI that talks to Hyprland over IPC through the
`hyprland` crate.

The binary has three responsibilities:

1. Read the user-supplied monitor names and selected mode.
2. Optionally prompt the user through `fuzzel`.
3. Apply the requested display state through Hyprland IPC.

## Runtime Flow

The current flow is straightforward:

1. Parse `--internal`, `--external`, `--overlay`, and the optional mode argument.
2. If `--overlay` is present, open `fuzzel` in dmenu mode and read the selected mode.
3. For `Extended`, call Hyprland reload.
4. For `Internal` or `External`, reload first and then disable the opposite monitor through `Keyword::set("monitor", "NAME, disable")`.

This design intentionally avoids enabling a disabled monitor with `disabled 0`,
because that path does not reliably restore the correct resolution.

## IPC Usage

The project uses the following Hyprland crate entry points:

- `hyprland::ctl::reload::call()` for config reloads
- `hyprland::data::Monitors::get()` to validate that the requested monitors exist
- `hyprland::keyword::Keyword::set()` to write the `monitor "NAME, disable"` keyword

## Mode Model

The supported display modes are:

- `Extended`: keep both monitors active
- `Internal`: keep only the internal panel active
- `External`: keep only the external monitor active

The default mode is `Extended`.
