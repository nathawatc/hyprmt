use anyhow::{Context, Result, bail};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::Duration;

pub fn eval_monitor_disable(output: &str) -> Result<()> {
    let lua = format!(
        "hl.monitor({{output={}, disabled=true}})",
        format_lua_string(output)
    );
    eval(&lua)
}

pub fn eval(lua: &str) -> Result<()> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").context("XDG_RUNTIME_DIR not set")?;
    let instance = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .context("HYPRLAND_INSTANCE_SIGNATURE not set")?;
    let socket_path: PathBuf = PathBuf::from(runtime_dir)
        .join("hypr")
        .join(instance)
        .join(".socket.sock");

    let mut stream = UnixStream::connect(&socket_path).with_context(|| {
        format!(
            "failed to connect to Hyprland socket: {}",
            socket_path.display()
        )
    })?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .context("failed to set Hyprland socket read timeout")?;

    let command = format!("/eval {}", lua);
    stream
        .write_all(command.as_bytes())
        .context("failed to write eval command to Hyprland socket")?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .context("failed to read Hyprland eval response")?;

    if response.trim() != "ok" {
        bail!("hypr eval failed: {}", response.trim());
    }
    Ok(())
}

fn format_lua_string(value: &str) -> String {
    let mut formatted = String::with_capacity(value.len() + 2);
    formatted.push('"');
    for ch in value.chars() {
        match ch {
            '\\' => formatted.push_str("\\\\"),
            '"' => formatted.push_str("\\\""),
            '\n' => formatted.push_str("\\n"),
            '\r' => formatted.push_str("\\r"),
            '\t' => formatted.push_str("\\t"),
            c => formatted.push(c),
        }
    }
    formatted.push('"');
    formatted
}
