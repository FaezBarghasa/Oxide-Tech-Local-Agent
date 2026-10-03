//! Native single-binary self-installation and environment bootstrapping.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(PathBuf::from)
}

pub fn get_oxide_home() -> PathBuf {
    if let Ok(home) = std::env::var("OXIDE_HOME") {
        PathBuf::from(home)
    } else if let Some(home) = home_dir() {
        home.join(".oxide")
    } else {
        PathBuf::from(".oxide")
    }
}

pub fn run_self_install(force: bool) -> Result<()> {
    println!(
        "\x1b[1;36m====================================================================\x1b[0m"
    );
    println!(
        "\x1b[1;36m   Oxide-Tech Local Agent — Native Standalone Self-Installer       \x1b[0m"
    );
    println!(
        "\x1b[1;36m====================================================================\x1b[0m\n"
    );

    let oxide_home = get_oxide_home();
    println!(
        "[1/4] Bootstrapping workspace runtime: {}",
        oxide_home.display()
    );
    fs::create_dir_all(&oxide_home)?;
    fs::create_dir_all(oxide_home.join("cache"))?;
    fs::create_dir_all(oxide_home.join("models"))?;
    fs::create_dir_all(oxide_home.join("project.db"))?;

    // Write default config.toml if missing
    let config_path = oxide_home.join("config.toml");
    if !config_path.exists() || force {
        if let Some(embedded_cfg) = crate::assets::SetupAssets::get("config.toml") {
            fs::write(&config_path, &embedded_cfg.data)?;
            println!(
                "      \x1b[1;32m[✓]\x1b[0m Embedded config.toml initialized -> {}",
                config_path.display()
            );
        }
    } else {
        println!(
            "      \x1b[1;33m[i]\x1b[0m Config already exists -> {}",
            config_path.display()
        );
    }

    // Binary relocation & Desktop Entry on Linux
    #[cfg(target_os = "linux")]
    {
        println!("[2/4] Relocating binary to user PATH...");
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(home) = home_dir() {
                let bin_dir = home.join(".local/bin");
                let _ = fs::create_dir_all(&bin_dir);
                let target_bin = bin_dir.join("oxide-tech-local-agent");

                if current_exe != target_bin {
                    fs::copy(&current_exe, &target_bin)
                        .context("Failed to copy executable to ~/.local/bin")?;
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let _ = fs::set_permissions(&target_bin, fs::Permissions::from_mode(0o755));
                    }
                    println!(
                        "      \x1b[1;32m[✓]\x1b[0m Installed binary -> {}",
                        target_bin.display()
                    );
                } else {
                    println!(
                        "      \x1b[1;32m[✓]\x1b[0m Binary already located at {}",
                        target_bin.display()
                    );
                }

                // Desktop entry
                println!("[3/4] Registering Desktop Application Entry...");
                let apps_dir = home.join(".local/share/applications");
                let _ = fs::create_dir_all(&apps_dir);
                let desktop_file = apps_dir.join("oxide-studio.desktop");

                let desktop_content = format!(
                    "[Desktop Entry]\n\
                     Type=Application\n\
                     Name=Oxide Studio\n\
                     GenericName=AI Agent & Hardware Workstation\n\
                     Comment=Local-First Autonomous AI Operating System\n\
                     Exec={}\n\
                     Icon=utilities-terminal\n\
                     Terminal=false\n\
                     Categories=Development;IDE;Engineering;System;\n\
                     StartupNotify=true\n",
                    target_bin.display()
                );

                fs::write(&desktop_file, desktop_content)?;
                println!(
                    "      \x1b[1;32m[✓]\x1b[0m Registered -> {}",
                    desktop_file.display()
                );
            }
        }

        // Udev rules
        println!("[4/4] Checking embedded hardware udev rules...");
        let udev_target = Path::new("/etc/udev/rules.d/99-probe-rs.rules");
        if !udev_target.exists() {
            println!(
                "      \x1b[1;33m[i]\x1b[0m Run 'oxide-tech-local-agent doctor' with sudo to deploy hardware probe-rs rules if flashing STM32/RISC-V targets."
            );
        } else {
            println!("      \x1b[1;32m[✓]\x1b[0m Hardware probe rules active.");
        }
    }

    #[cfg(target_os = "windows")]
    {
        println!("[2/4] Registering Windows Local Programs directory...");
        if let Ok(current_exe) = std::env::current_exe() {
            if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
                let target_dir = PathBuf::from(local_appdata).join("Programs/Oxide");
                let _ = fs::create_dir_all(&target_dir);
                let target_exe = target_dir.join("oxide-studio.exe");
                if current_exe != target_exe {
                    let _ = fs::copy(&current_exe, &target_exe);
                    println!(
                        "      \x1b[1;32m[✓]\x1b[0m Installed executable -> {}",
                        target_exe.display()
                    );
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        println!("[2/4] Registering macOS Application runtime...");
    }

    println!(
        "\n\x1b[1;32m[✓] Installation complete! Launch Oxide Studio with 'oxide-tech-local-agent'.\x1b[0m\n"
    );
    Ok(())
}
