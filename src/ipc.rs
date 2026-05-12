use anyhow::{Context, Result, bail};
use std::io::{IoSlice, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

#[cfg(debug_assertions)]
use std::io::Read;
#[cfg(debug_assertions)]
use std::time::Duration;

pub fn eval_monitor_disable(output: &str) -> Result<()> {
    let lua = format!(
        "hl.monitor({{output={}, disabled=true}})",
        format_lua_string(output)
    );
    eval(&lua)
}

pub fn eval(lua: &str) -> Result<()> {
    let socket_path = socket_path()?;
    let mut stream = UnixStream::connect(&socket_path).with_context(|| {
        format!(
            "failed to connect to Hyprland socket: {}",
            socket_path.display()
        )
    })?;

    write_eval(&mut stream, lua)?;

    #[cfg(debug_assertions)]
    read_ok(&mut stream)?;

    Ok(())
}

fn socket_path() -> Result<PathBuf> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").context("XDG_RUNTIME_DIR not set")?;
    let instance = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .context("HYPRLAND_INSTANCE_SIGNATURE not set")?;
    Ok(PathBuf::from(runtime_dir)
        .join("hypr")
        .join(instance)
        .join(".socket.sock"))
}

fn write_eval(stream: &mut UnixStream, lua: &str) -> Result<()> {
    let mut prefix: &[u8] = b"/eval ";
    let mut payload = lua.as_bytes();
    let mut remaining = prefix.len() + payload.len();

    while remaining > 0 {
        let bufs = [IoSlice::new(prefix), IoSlice::new(payload)];
        let n = stream
            .write_vectored(&bufs)
            .context("failed to write eval command to Hyprland socket")?;
        if n == 0 {
            bail!("hypr eval failed: socket write returned 0 bytes");
        }
        if n >= prefix.len() {
            let consumed = n - prefix.len();
            prefix = &[];
            payload = &payload[consumed..];
        } else {
            prefix = &prefix[n..];
        }
        remaining -= n;
    }
    Ok(())
}

#[cfg(debug_assertions)]
fn read_ok(stream: &mut UnixStream) -> Result<()> {
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .context("failed to set Hyprland socket read timeout")?;

    let mut buf = [0u8; 256];
    let n = stream
        .read(&mut buf)
        .context("failed to read Hyprland eval response")?;
    if n == 0 {
        bail!("hypr eval failed: empty response");
    }
    let response =
        std::str::from_utf8(&buf[..n]).context("hypr eval returned non-UTF8 response")?;

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
