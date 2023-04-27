use std::path::{Path, PathBuf};

use eyre::{Result, WrapErr};

pub fn read_to_string<P: AsRef<Path>>(path: &P) -> Result<String> {
    std::fs::read_to_string(&path).wrap_err_with(|| format!("Unable to read from {}", absolutepath(&path)))
}

pub fn absolutepath<P: AsRef<Path>>(path: &P) -> String {
    std::fs::canonicalize(path).unwrap_or_else(|_| {
        let mut buf = PathBuf::new();
        buf.push(std::env::current_dir().unwrap_or(PathBuf::default()));
        buf.push(path);
        buf
    }).display().to_string()
}
