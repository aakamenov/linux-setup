use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf}
};
use once_cell::unsync::Lazy;

#[inline]
pub fn append(path: impl AsRef<Path>, contents: &[u8]) {
    dir_create_if_not_exists(path.as_ref());

    match OpenOptions::new().create(true).append(true).open(&path) {
        Ok(mut file) => if let Err(err) = file.write_all(contents) {
            panic!("Error appending to file {}: {err}", path.as_ref().to_string_lossy())
        }
        Err(err) => panic!("Error opening file {}: {err}", path.as_ref().to_string_lossy())
    }
}

#[inline]
pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    dir_create_if_not_exists(to.as_ref());

    if let Err(err) = fs::copy(&from, &to) {
        panic!(
            "Error copying {} to {}: {err}",
            from.as_ref().to_string_lossy(),
            to.as_ref().to_string_lossy()
        )
    }
}

#[inline]
pub fn write(path: impl AsRef<Path>, contents: &[u8]) {
    dir_create_if_not_exists(path.as_ref());

    match OpenOptions::new().create(true).write(true).open(&path) {
        Ok(mut file) => if let Err(err) = file.write_all(contents) {
            panic!("Error writing to file {}: {err}", path.as_ref().to_string_lossy())
        }
        Err(err) => panic!("Error opening file {}: {err}", path.as_ref().to_string_lossy())
    }
}

pub fn remove(path: impl AsRef<Path>) {
    if !path.as_ref().exists() {
        return;
    }

    if let Err(err) = fs::remove_file(&path) {
        panic!("Error removing file {}: {err}", path.as_ref().to_string_lossy());
    }
}

pub fn current_dir() -> PathBuf {
    const CURRENT_DIR: Lazy<PathBuf> = Lazy::new(|| {
        let mut current_dir = env::current_dir().expect("couldn't get current directory");
        // Push trailing /
        current_dir.push("");

        current_dir
    });

    CURRENT_DIR.clone()
}

#[inline]
fn dir_create_if_not_exists(path: &Path) {
    if !path.exists() {
        if let Some(dir) = path.parent() {
            if let Err(err) = fs::create_dir_all(&dir) {
                panic!("Failed to create directory {}: {err}", dir.to_string_lossy());
            }
        }
    }
}
