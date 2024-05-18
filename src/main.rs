mod cmd;
mod pacman;
mod yay;
mod file;

use std::{env, fs};
use once_cell::unsync::Lazy;

const HOME_DIR: Lazy<String> = Lazy::new(|| {
    env::var("HOME").unwrap_or(String::from("~"))
});

const CONFIG_DIR: Lazy<String> = Lazy::new(|| {
    env::var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", *HOME_DIR))
});

const USER: Lazy<String> = Lazy::new(|| {
    cmd::read("whoami", &[])
        .lines()
        .next()
        .unwrap()
        .trim()
        .to_string()
});

fn main() {
    if USER.as_str() == "root" {
        eprintln!("Don't run this as root.");

        return;
    }

    let mut dev_only = false;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--dev-only" => dev_only = true,
            _ => panic!("Unknown arg: {arg}")
        }
    }

    setup(dev_only);
}

fn setup(dev_only: bool) {
    // Validate in order to avoid prompting later
    cmd::run("sudo", &["--validate"]);

    // DESKTOP
    install_hyprland_desktop();

    // CORE
    install_alacritty();
    pacman::install("btop");
    pacman::install("file-roller");
    install_thunar();
    pacman::install("ffmpegthumbnailer");
    pacman::install("keepassxc");
    pacman::install("ristretto");
    install_starship();

    // DEVELOPMENT
    install_sublime();
    pacman::install("gitui");
    pacman::install("rustup");
    pacman::install("rust-analyzer");

    // FONTS
    yay::install("ttf-profont-nerd");
    install_nerd_fonts(&["ShareTechMono", "SourceCodePro", "ZedMono"]);

    // THEMING UTILITIES
    yay::install("qt5ct");
    yay::install("qt6ct");
    yay::install("kvantum");

    if dev_only {
        return;
    }

    // PROGRAMS ONLY NEEDED FOR PERSONAL USE
    pacman::install("mpv");
    pacman::install("qbittorrent");
    pacman::install("steam");
    yay::install("vesktop");
    yay::install("musikcube");
    yay::install("mullvad-vpn-bin");
}

fn install_hyprland_desktop() {
    // Color scheme: https://coolors.co/232f2e-293635-aca695-d9ddde-ff8000-70d900-ff4c57-00dbd7-ff64a2
    // Kora icons: https://www.gnome-look.org/p/1256209
    // Orchis: https://store.kde.org/p/1458909

    pacman::install("hyprland");
    install_greetd();
    pacman::install("mako");
    pacman::install("grim");
    pacman::install("slurp");
    pacman::install("wl-clipboard");
    pacman::install("fuzzel");
    pacman::install("swaybg");
    yay::install("adw-gtk3");

    file::copy(
        ".config/fuzzel/fuzzel.ini",
        format!("{}/fuzzel/fuzzel.ini", &*CONFIG_DIR)
    );

    file::copy(
        ".config/mako/config",
        format!("{}/mako/config", &*CONFIG_DIR)
    );

    file::copy(
        ".config/hypr/gamemode.sh",
        format!("{}/hypr/gamemode.sh", &*CONFIG_DIR)
    );

    file::copy(
        ".config/hypr/wallpaper.png",
        format!("{}/hypr/wallpaper.png", &*CONFIG_DIR)
    );

    file::copy(
        ".config/hypr/hyprland.conf",
        format!("{}/hypr/hyprland.conf", &*CONFIG_DIR)
    );
}

fn install_greetd() {
    const CONFIG: &str =
r#"[terminal]
# The VT to run the greeter on. Can be "next", "current" or a number
# designating the VT.
vt = 1

# The default session, also known as the greeter.
[default_session]

# `agreety` is the bundled agetty/login-lookalike. You can replace `/bin/sh`
# with whatever you want started, such as `sway`.
command = "agreety --cmd /bin/bash"

# The user to run the command as. The privileges this user must have depends
# on the greeter. A graphical greeter may for example require the user to be
# in the `video` group.
user = "{USER}"

[initial_session]
command = "Hyprland"
user = "{USER}"
"#;

    pacman::install("greetd");

    let config = CONFIG.replace("{USER}", USER.as_str());

    let mut file = file::current_dir();
    file.push("config.toml");

    file::write(file.as_path(), config.as_bytes());

    let from = file.as_path().to_str().unwrap();
    cmd::run_privileged("mv", &[from, "/etc/greetd/config.toml"])
}

fn install_alacritty() {
    pacman::install("alacritty");

    file::copy(
        ".config/alacritty/alacritty.toml",
        format!("{}/alacritty/alacritty.toml", &*CONFIG_DIR)
    );
}

fn install_sublime() {
    yay::install("sublime-text-4");

    file::copy(
        ".config/sublime-text/Packages/User/Package Control.sublime-settings",
        format!("{}/sublime-text/Packages/User/Package Control.sublime-settings", &*CONFIG_DIR)
    );

    file::copy(
        ".config/sublime-text/Packages/User/Preferences.sublime-settings",
        format!("{}/sublime-text/Packages/User/Preferences.sublime-settings", &*CONFIG_DIR)
    );
}

fn install_thunar() {
    pacman::install("thunar");
    pacman::install("gvfs"); // Android filesystem support
    pacman::install("gvfs-mtp"); // Android filesystem support
    pacman::install("thunar-archive-plugin");
    pacman::install("thunar-volman"); // Removable device support
}

fn install_starship() {
    pacman::install("starship");

    // https://starship.rs/guide/#step-2-set-up-your-shell-to-use-starship
    const CMD: &str = "eval \"$(starship init bash)\"";

    let path = format!("{}/.bashrc", &*HOME_DIR);

    match fs::read_to_string(&path) {
        Ok(content) => {
            if content.contains(CMD) {
                println!("Skipping setting up starship prompt, already initialized.");
            } else  {
                file::append(path, CMD.as_bytes());
            }
        },
        Err(err) => eprintln!("Error opening {path}: {err}"),
    }
}

/// Downloads files from: https://github.com/ryanoasis/nerd-fonts/releases/latest
/// `names` must correspond to the filenames.
fn install_nerd_fonts(names: &[&'static str]) {
    let dest_dir = format!("{}/.local/share/fonts", &*HOME_DIR);

    for name in names {
        let req = ehttp::Request::get(format!(
            "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/{name}.tar.xz"
        ));

        println!("Downloading font: {}", req.url);

        match ehttp::fetch_blocking(&req) {
            Ok(resp) if resp.ok => {
                let mut archive = file::current_dir();
                archive.push(format!("{name}.tar.xz"));

                file::write(archive.as_path(), &resp.bytes);

                let from = archive.as_path().to_str().unwrap();
                cmd::run("tar", &["-xf", from, "-C", &dest_dir]);

                file::remove(archive.as_path());

                println!("Installed font {name} to {}", dest_dir);
            }
            Ok(resp) => eprintln!("Failed to download {}: {}", req.url, resp.status_text),
            Err(err) => eprintln!("Failed to download {}: {err}", req.url)
        }
    }
}
