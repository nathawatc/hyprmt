use anyhow::{Context, Result, bail};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

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
