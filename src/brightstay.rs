use ddc::Ddc;
use ddc::Edid;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

const TARGET_PRODUCT_CODE: u16 = 26542;
const VCP_BRIGHTNESS_CODE: ddc::FeatureCode = 0x14;
const VCP_BRIGHTNESS_VALUE: u16 = 11;
const I2C_DEV_PATH: &str = "/dev";

#[derive(Debug, Serialize, Deserialize)]
struct CacheEntry {
    bus: u8,
    product_code: u16,
}

impl CacheEntry {
    fn new(bus: u8, product_code: u16) -> Self {
        Self { bus, product_code }
    }
}

fn get_cache_path() -> Option<PathBuf> {
    ProjectDirs::from("org", "lenovo", "fix-l22i-40")
        .map(|dirs| dirs.cache_dir().join("monitor.cache"))
}

fn read_cache() -> Option<CacheEntry> {
    let path = get_cache_path()?;
    let content = fs::read_to_string(&path).ok()?;
    let mut iter = content.split(':');
    let bus = iter.next()?.parse().ok()?;
    let product_code = iter.next()?.parse().ok()?;
    Some(CacheEntry::new(bus, product_code))
}

fn write_cache(entry: &CacheEntry) -> io::Result<()> {
    let path = get_cache_path()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "cache directory unavailable"))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, format!("{}:{}", entry.bus, entry.product_code))?;
    Ok(())
}

fn edid_check(edid: &[u8; 128]) -> Option<u16> {
    let manufacturer_id = (edid[8] as u16) << 8 | (edid[9] as u16);
    if manufacturer_id == 0 {
        return None;
    }
    let product_code = (edid[10] as u16) | ((edid[11] as u16) << 8);
    if product_code != TARGET_PRODUCT_CODE {
        return None;
    }
    Some(product_code)
}

fn fast_path(bus: u8) -> Option<bool> {
    let device_path = format!("{}/i2c-{}", I2C_DEV_PATH, bus);
    let mut ddc = ddc_i2c::from_i2c_device(&device_path).ok()?;
    let mut edid = [0u8; 128];
    ddc.read_edid(0, &mut edid).ok()?;
    if edid_check(&edid).is_none() {
        return Some(false);
    }
    ddc.set_vcp_feature(VCP_BRIGHTNESS_CODE, VCP_BRIGHTNESS_VALUE)
        .ok()?;
    Some(true)
}

fn slow_path() -> Option<CacheEntry> {
    let sysfs_path = PathBuf::from("/sys/class/i2c-dev");
    let entries = fs::read_dir(sysfs_path).ok()?;
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if !name.starts_with("i2c-") {
            continue;
        }
        let bus: u8 = name[4..].parse().ok()?;
        let device_path = format!("{}/{}", I2C_DEV_PATH, name);
        let mut ddc = ddc_i2c::from_i2c_device(&device_path).ok()?;
        let mut edid = [0u8; 128];
        if ddc.read_edid(0, &mut edid).is_err() {
            continue;
        }
        let product_code = match edid_check(&edid) {
            Some(pc) => pc,
            None => continue,
        };
        ddc.set_vcp_feature(VCP_BRIGHTNESS_CODE, VCP_BRIGHTNESS_VALUE)
            .ok()?;
        return Some(CacheEntry::new(bus, product_code));
    }
    None
}

pub fn run() {
    if let Some(cached) = read_cache() {
        if fast_path(cached.bus).is_some() {
            process::exit(0);
        }
    }

    if let Some(entry) = slow_path() {
        let _ = write_cache(&entry);
        process::exit(0);
    }

    process::exit(0);
}
