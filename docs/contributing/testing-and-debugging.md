# Testing and Debugging

## Checks to Run

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build
```

## Manual Testing

Use a known pair of monitor names and test all three modes:

```bash
hyprmt --internal eDP-1 --external HDMI-A-1 internal
hyprmt --internal eDP-1 --external HDMI-A-1 external
hyprmt --internal eDP-1 --external HDMI-A-1 extended
```

Then repeat with `--overlay` to confirm that `fuzzel` returns the expected mode.

## Debugging Tips

- If the command fails with a monitor lookup error, verify the exact Hyprland monitor names.
- If `--overlay` fails immediately, check that `fuzzel` is installed and in `PATH`.
- If a mode change does not behave as expected, confirm that Hyprland accepted the reload and the `monitor` keyword write.
