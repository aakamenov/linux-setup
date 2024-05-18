use std::collections::HashSet;
use once_cell::unsync::Lazy;

use crate::cmd;

const INSTALLED: Lazy<HashSet<String>> = Lazy::new(|| {
    let mut result = HashSet::new();

    for line in cmd::read("yay", &["-Q"]).lines() {
        let name = line.split(" ").next().expect("Invalid yay -Q output.");
        result.insert(name.to_string());
    }

    result
});

#[inline]
pub fn is_installed(package: &'static str) -> bool {
    INSTALLED.contains(package)
}

pub fn install(package: &'static str) {
    if is_installed(package) {
        println!("Skipping {package}, already installed.");

        return;
    }

    println!("Installing {package}...");
    cmd::run("yay", &["-S", "--noconfirm", "--removemake", package]);
}
