use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, ValueEnum};
use hyprland::ctl;
use hyprland::data::Monitors;
use hyprland::prelude::HyprData;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(
        short = 'i',
        long,
        required_unless_present("mode"),
        required_if_eq_any([
            ("mode", "internal"),
            ("mode", "external"),
            ("mode", "extended"),
            ("mode", "overlay"),
        ])
    )]
    internal: Option<String>,
    #[arg(
        short = 'e',
        long,
        required_unless_present("mode"),
        required_if_eq_any([
            ("mode", "internal"),
            ("mode", "external"),
            ("mode", "extended"),
            ("mode", "overlay"),
        ])
    )]
    external: Option<String>,

    #[arg(value_enum, default_value_t = Mode::Extended)]
    mode: Mode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum Mode {
    Internal,
    External,
    Extended,
    Overlay,
    #[cfg(feature = "brightstay")]
    #[value(name = "brightstay")]
    Brightstay,
}

impl Mode {
    fn from_overlay(value: &str) -> Result<Self> {
        match value.trim() {
            "Internal" => Ok(Self::Internal),
            "External" => Ok(Self::External),
            "Extended" => Ok(Self::Extended),
            #[cfg(feature = "brightstay")]
            "BrightStay" => Ok(Self::Brightstay),
            _ => bail!("unsupported overlay selection: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cli, Mode};
    use clap::Parser;

    #[test]
    fn overlay_selection_accepts_supported_modes() {
        let cases = [
            ("Internal", Mode::Internal),
            ("External", Mode::External),
            ("Extended", Mode::Extended),
            #[cfg(feature = "brightstay")]
            ("BrightStay", Mode::Brightstay),
        ];

        for (selection, expected) in cases {
            assert_eq!(Mode::from_overlay(selection).unwrap(), expected);
        }
    }

    #[test]
    fn overlay_selection_trims_surrounding_whitespace() {
        assert_eq!(
            Mode::from_overlay("\n Extended \t").unwrap(),
            Mode::Extended
        );
    }

    #[test]
    fn overlay_selection_rejects_invalid_input() {
        assert!(Mode::from_overlay("Mirror").is_err());
    }

    #[test]
    fn overlay_selection_rejects_overlay_mode() {
        assert!(Mode::from_overlay("Overlay").is_err());
    }

    #[test]
    fn monitor_arguments_are_required_for_default_mode() {
        let error = Cli::try_parse_from(["hyprmt"]).unwrap_err();

        assert_eq!(
            error.kind(),
            clap::error::ErrorKind::MissingRequiredArgument
        );
    }

    #[test]
    fn monitor_arguments_are_required_for_non_brightstay_modes() {
        for mode in ["internal", "external", "extended", "overlay"] {
            let error = Cli::try_parse_from(["hyprmt", mode]).unwrap_err();

            assert_eq!(
                error.kind(),
                clap::error::ErrorKind::MissingRequiredArgument
            );
        }
    }

    #[cfg(feature = "brightstay")]
    #[test]
    fn brightstay_does_not_require_monitor_arguments() {
        let cli = Cli::try_parse_from(["hyprmt", "brightstay"]).unwrap();

        assert_eq!(cli.mode, Mode::Brightstay);
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    #[cfg(feature = "brightstay")]
    if cli.mode == Mode::Brightstay {
        brightstay::run();
        return Ok(());
    }

    let internal = cli
        .internal
        .as_deref()
        .expect("internal monitor required outside brightstay mode");
    let external = cli
        .external
        .as_deref()
        .expect("external monitor required outside brightstay mode");

    match cli.mode {
        Mode::Overlay => {
            let selected = prompt_mode()?;
            match selected {
                Mode::Extended => enable_extended(),
                Mode::Internal => apply_single_output_mode(internal, external),
                Mode::External => apply_single_output_mode(external, internal),
                #[cfg(feature = "brightstay")]
                Mode::Brightstay => {
                    brightstay::run();
                    Ok(())
                }
                Mode::Overlay => unreachable!(),
            }
        }
        Mode::Extended => enable_extended(),
        Mode::Internal => apply_single_output_mode(internal, external),
        Mode::External => apply_single_output_mode(external, internal),
        #[cfg(feature = "brightstay")]
        Mode::Brightstay => unreachable!(),
    }
}

#[cfg(feature = "brightstay")]
const PROMPT_OPTIONS: &[u8] = b"Internal\nExternal\nExtended\nBrightStay\n";

#[cfg(not(feature = "brightstay"))]
const PROMPT_OPTIONS: &[u8] = b"Internal\nExternal\nExtended\n";

fn prompt_mode() -> Result<Mode> {
    let mut child = Command::new("fuzzel")
        .args(["--dmenu", "--prompt", "Display: "])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("failed to launch fuzzel")?;

    child
        .stdin
        .take()
        .context("failed to open fuzzel stdin")?
        .write_all(PROMPT_OPTIONS)
        .context("failed to write choices to fuzzel")?;

    let output = child
        .wait_with_output()
        .context("failed to wait for fuzzel")?;

    if !output.status.success() {
        if output.status.code() == Some(1) {
            std::process::exit(0);
        }
        bail!("fuzzel exited with status {}", output.status);
    }

    let selection =
        String::from_utf8(output.stdout).context("fuzzel returned invalid UTF-8 output")?;

    if selection.trim().is_empty() {
        std::process::exit(0);
    }

    Mode::from_overlay(&selection)
}

fn enable_extended() -> Result<()> {
    ctl::reload::call().map_err(map_hypr_error)?;
    Ok(())
}

fn apply_single_output_mode(enabled: &str, disabled: &str) -> Result<()> {
    ensure_monitor_present(enabled)?;
    ctl::reload::call().map_err(map_hypr_error)?;
    ensure_monitor_present(disabled)?;
    ipc::eval_monitor_disable(disabled)?;
    Ok(())
}

#[inline]
fn ensure_monitor_present(name: &str) -> Result<()> {
    let monitors = Monitors::get().map_err(map_hypr_error)?;
    if monitors.iter().any(|m| m.name == name) {
        return Ok(());
    }
    let monitor_names: Vec<_> = monitors.iter().map(|m| m.name.clone()).collect();
    Err(anyhow!(
        "monitor {name:?} is not available in Hyprland: {}",
        monitor_names.join(", ")
    ))
}

#[inline]
fn map_hypr_error(error: hyprland::error::HyprError) -> anyhow::Error {
    anyhow!("{error}")
}

mod ipc;

#[cfg(feature = "brightstay")]
mod brightstay;
