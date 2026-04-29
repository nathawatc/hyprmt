use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, ValueEnum};
use hyprland::ctl;
use hyprland::data::Monitors;
use hyprland::keyword::Keyword;
use hyprland::prelude::{HyprData, HyprDataVec};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(long)]
    internal: String,
    #[arg(long)]
    external: String,

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
            other => bail!("unsupported overlay selection: {other}"),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.mode {
        #[cfg(feature = "brightstay")]
        Mode::Brightstay => {
            brightstay::run();
            Ok(())
        }
        Mode::Overlay => {
            let selected = prompt_mode()?;
            match selected {
                Mode::Extended => enable_extended(),
                Mode::Internal => apply_single_output_mode(&cli.internal, &cli.external),
                Mode::External => apply_single_output_mode(&cli.external, &cli.internal),
                #[cfg(feature = "brightstay")]
                Mode::Brightstay => {
                    brightstay::run();
                    Ok(())
                }
                Mode::Overlay => unreachable!(),
            }
        }
        Mode::Extended => enable_extended(),
        Mode::Internal => apply_single_output_mode(&cli.internal, &cli.external),
        Mode::External => apply_single_output_mode(&cli.external, &cli.internal),
    }
}

#[cfg(feature = "brightstay")]
fn prompt_mode() -> Result<Mode> {
    let mut child = Command::new("fuzzel")
        .args(["--dmenu", "--prompt", "Display: "])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("failed to launch fuzzel")?;

    {
        let mut stdin = child.stdin.take().context("failed to open fuzzel stdin")?;
        stdin
            .write_all(b"Internal\nExternal\nExtended\nBrightStay\n")
            .context("failed to write choices to fuzzel")?;
    }

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

#[cfg(not(feature = "brightstay"))]
fn prompt_mode() -> Result<Mode> {
    let mut child = Command::new("fuzzel")
        .args(["--dmenu", "--prompt", "Display: "])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("failed to launch fuzzel")?;

    {
        let mut stdin = child.stdin.take().context("failed to open fuzzel stdin")?;
        stdin
            .write_all(b"Internal\nExternal\nExtended\n")
            .context("failed to write choices to fuzzel")?;
    }

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
    ensure_monitor_present(enabled)?;
    ensure_monitor_present(disabled)?;
    Keyword::set(format!("monitorv2[{disabled}]:disabled"), 1).map_err(map_hypr_error)?;
    Ok(())
}

fn ensure_monitor_present(name: &str) -> Result<()> {
    let monitor_names = Monitors::get()
        .map_err(map_hypr_error)?
        .to_vec()
        .into_iter()
        .map(|monitor| monitor.name)
        .collect::<Vec<_>>();

    if monitor_names.iter().any(|monitor| monitor == name) {
        return Ok(());
    }

    Err(anyhow!(
        "monitor {name:?} is not available in Hyprland: {}",
        monitor_names.join(", ")
    ))
}

fn map_hypr_error(error: hyprland::error::HyprError) -> anyhow::Error {
    anyhow!(error.to_string())
}

#[cfg(feature = "brightstay")]
mod brightstay;
