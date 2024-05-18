use crate::{yay, cmd};

pub fn install(package: &'static str) {
    if yay::is_installed(package) {
        println!("Skipping {package}, already installed.");

        return;
    }

    println!("Installing {package}...");
    cmd::run_privileged("pacman", &["-S", "--noconfirm", package]);
}
