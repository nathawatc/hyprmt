use ddc::Ddc;
use ddc::Edid;
use std::fs::{self, File, OpenOptions, Permissions};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process;

const TARGET_PRODUCT_CODE: u16 = 26542;
const VCP_BRIGHTNESS_CODE: ddc::FeatureCode = 0x14;
const VCP_BRIGHTNESS_VALUE: u16 = 11;
const I2C_DEV_PATH: &str = "/dev";
macro_rules! cache_dir_path {
    () => {
        "/var/cache/fix-l22i-40"
    };
}

const CACHE_DIR: &str = cache_dir_path!();
const CACHE_FILE: &str = concat!(cache_dir_path!(), "/monitor.cache");
const CACHE_TEMP_PREFIX: &str = ".monitor.cache";
const CACHE_FILE_MODE: u32 = 0o664;

#[derive(Debug)]
struct CacheEntry {
    bus: u8,
    product_code: u16,
}

impl CacheEntry {
    fn new(bus: u8, product_code: u16) -> Self {
        Self { bus, product_code }
    }
}

fn cache_dir() -> &'static Path {
    Path::new(CACHE_DIR)
}

fn cache_file() -> &'static Path {
    Path::new(CACHE_FILE)
}

fn read_cache() -> Option<CacheEntry> {
    read_cache_from(cache_dir(), cache_file())
}

fn read_cache_from(cache_dir: &Path, cache_file: &Path) -> Option<CacheEntry> {
    if !cache_dir.is_dir() || !cache_file.is_file() {
        return None;
    }

    let content = fs::read_to_string(cache_file).ok()?;
    parse_cache_entry(&content)
}

fn parse_cache_entry(content: &str) -> Option<CacheEntry> {
    let mut iter = content.trim_end().split(':');
    let bus = iter.next()?.parse().ok()?;
    let product_code = iter.next()?.parse().ok()?;
    if iter.next().is_some() || product_code != TARGET_PRODUCT_CODE {
        return None;
    }

    Some(CacheEntry::new(bus, product_code))
}

fn write_cache(entry: &CacheEntry) -> io::Result<()> {
    write_cache_to(entry, cache_dir(), cache_file())
}

fn write_cache_to(entry: &CacheEntry, cache_dir: &Path, cache_file: &Path) -> io::Result<()> {
    write_cache_to_with(entry, cache_dir, cache_file, write_cache_atomic)
}

fn write_cache_to_with(
    entry: &CacheEntry,
    cache_dir: &Path,
    cache_file: &Path,
    atomic_write: impl FnOnce(&CacheEntry, &Path, &Path) -> io::Result<()>,
) -> io::Result<()> {
    if !cache_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "cache directory unavailable",
        ));
    }

    let temp_path = cache_dir.join(format!(
        "{CACHE_TEMP_PREFIX}.{}.{}",
        process::id(),
        unique_temp_suffix()
    ));
    let write_result = atomic_write(entry, &temp_path, cache_file);
    if write_result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }
    write_result
}

fn write_cache_atomic(entry: &CacheEntry, temp_path: &Path, cache_file: &Path) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temp_path)?;
    file.set_permissions(Permissions::from_mode(CACHE_FILE_MODE))?;
    writeln!(file, "{}:{}", entry.bus, entry.product_code)?;
    file.sync_all()?;
    drop(file);

    fs::rename(temp_path, cache_file)?;
    File::open(cache_file.parent().unwrap_or_else(|| Path::new("/")))?.sync_all()?;
    Ok(())
}

fn unique_temp_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0)
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
    let mut ddc = ddc_i2c::open_linux_device(&device_path).ok()?;
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
    let entries = fs::read_dir(Path::new("/sys/class/i2c-dev")).ok()?;
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if !name.starts_with("i2c-") {
            continue;
        }
        let bus: u8 = name[4..].parse().ok()?;
        let device_path = format!("{}/{}", I2C_DEV_PATH, name);
        let mut ddc = ddc_i2c::open_linux_device(&device_path).ok()?;
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

fn fix_brightness(
    read_cache: impl FnOnce() -> Option<CacheEntry>,
    fast_path: impl FnOnce(u8) -> Option<bool>,
    slow_path: impl FnOnce() -> Option<CacheEntry>,
    write_cache: impl FnOnce(&CacheEntry) -> io::Result<()>,
) {
    if let Some(cached) = read_cache() {
        if fast_path(cached.bus) == Some(true) {
            return;
        }
    }

    if let Some(entry) = slow_path() {
        let _ = write_cache(&entry);
    }
}

pub fn run() {
    fix_brightness(read_cache, fast_path, slow_path, write_cache);
    process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::path::PathBuf;

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "hyprmt-brightstay-test-{}-{}",
                process::id(),
                unique_temp_suffix()
            ));
            fs::create_dir(&path).expect("create test cache dir");
            Self { path }
        }

        fn cache_file(&self) -> PathBuf {
            self.path.join("monitor.cache")
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn assert_cached_fast_failure_falls_back(fast_result: Option<bool>) {
        let slow_called = Cell::new(false);
        let write_called = Cell::new(false);

        fix_brightness(
            || Some(CacheEntry::new(7, TARGET_PRODUCT_CODE)),
            |bus| {
                assert_eq!(bus, 7);
                fast_result
            },
            || {
                slow_called.set(true);
                Some(CacheEntry::new(3, TARGET_PRODUCT_CODE))
            },
            |entry| {
                write_called.set(true);
                assert_eq!(entry.bus, 3);
                assert_eq!(entry.product_code, TARGET_PRODUCT_CODE);
                Ok(())
            },
        );

        assert!(slow_called.get());
        assert!(write_called.get());
    }

    fn temp_file_count(dir: &Path) -> usize {
        fs::read_dir(dir)
            .expect("read test cache dir")
            .flatten()
            .filter(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with(CACHE_TEMP_PREFIX)
            })
            .count()
    }

    #[test]
    fn fast_path_none_falls_back_to_slow_path() {
        assert_cached_fast_failure_falls_back(None);
    }

    #[test]
    fn fast_path_false_falls_back_to_slow_path() {
        assert_cached_fast_failure_falls_back(Some(false));
    }

    #[test]
    fn malformed_cache_input_is_ignored() {
        for content in ["", "abc", "1", "1:2:3", "1:26542x", "1:0", "999:26542"] {
            assert!(parse_cache_entry(content).is_none(), "accepted {content:?}");
        }
    }

    #[test]
    fn malformed_cache_file_is_ignored() {
        let dir = TestDir::new();
        let cache_file = dir.cache_file();
        fs::write(&cache_file, "not-a-cache").expect("write malformed cache");

        assert!(read_cache_from(&dir.path, &cache_file).is_none());
    }

    #[test]
    fn cache_write_atomically_replaces_existing_cache() {
        let dir = TestDir::new();
        let cache_file = dir.cache_file();
        fs::write(&cache_file, "old-invalid-cache").expect("write old cache");

        write_cache_to(
            &CacheEntry::new(9, TARGET_PRODUCT_CODE),
            &dir.path,
            &cache_file,
        )
        .expect("write cache");

        assert_eq!(
            fs::read_to_string(&cache_file).expect("read cache"),
            format!("9:{TARGET_PRODUCT_CODE}\n")
        );
        assert_eq!(
            read_cache_from(&dir.path, &cache_file)
                .expect("parse replaced cache")
                .bus,
            9
        );
        assert_eq!(
            fs::metadata(&cache_file)
                .expect("cache metadata")
                .permissions()
                .mode()
                & 0o777,
            CACHE_FILE_MODE
        );
        assert_eq!(temp_file_count(&dir.path), 0);
    }

    #[test]
    fn temp_file_is_removed_after_write_failure() {
        let dir = TestDir::new();
        let cache_file = dir.cache_file();
        let result = write_cache_to_with(
            &CacheEntry::new(9, TARGET_PRODUCT_CODE),
            &dir.path,
            &cache_file,
            |_entry, temp_path, _cache_file| {
                File::create(temp_path)?;
                Err(io::Error::new(io::ErrorKind::PermissionDenied, "forced"))
            },
        );

        assert!(result.is_err());
        assert!(!cache_file.exists());
        assert_eq!(temp_file_count(&dir.path), 0);
    }
}
