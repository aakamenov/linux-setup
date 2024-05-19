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

    println!("Appending to {}", path.as_ref().to_string_lossy());

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

    println!(
        "Copying {} to {}",
        from.as_ref().to_string_lossy(),
        to.as_ref().to_string_lossy()
    );

    if let Err(err) = fs::copy(&from, &to) {
        panic!(
            "Error copying {} to {}: {err}",
            from.as_ref().to_string_lossy(),
            to.as_ref().to_string_lossy()
        );
    }
}

#[inline]
pub fn write(path: impl AsRef<Path>, contents: &[u8]) {
    dir_create_if_not_exists(path.as_ref());

    println!("Writing to {}", path.as_ref().to_string_lossy());

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

    println!("Removing {}", path.as_ref().to_string_lossy());

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
pub fn dir_create_if_not_exists(path: impl AsRef<Path>) {
    let path = path.as_ref();

    if path.exists() {
        return;
    }

    if path.extension().is_some() {
        if let Some(parent) = path.parent() {
            if let Err(err) = fs::create_dir_all(&parent) {
                panic!("Failed to create directory {}: {err}", parent.to_string_lossy());
            }
        }
    } else if let Err(err) = fs::create_dir_all(&path) {
        panic!("Failed to create directory {}: {err}", path.to_string_lossy());
    }
}
