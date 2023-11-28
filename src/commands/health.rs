use crate::commands::remove::remove_trailing_slash;
use crate::engine::config::read_config;
use crate::log::{info, warn};
use anyhow::{anyhow, Result};
use spinoff::{spinners, Color, Spinner, Streams};
use std::fs;

enum Health {
    ExistsWithPermissions,
    ExistsWithoutPermissions,
    DoesNotExist,
}

fn check_directory(path: &str) -> Result<Health> {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                if metadata.permissions().readonly() {
                    // Check permissions
                    return Ok(Health::ExistsWithoutPermissions);
                } else {
                    return Ok(Health::ExistsWithPermissions);
                }
            }
        }
        Err(_) => return Ok(Health::DoesNotExist),
    }
    Ok(Health::DoesNotExist)
}

async fn check_and_log_directory(dir_name: &str, dir_path: &str) -> Result<()> {
    match check_directory(dir_path)? {
        Health::ExistsWithPermissions => info(
            &format!(
                "{} directory exists with required permissions at: {}",
                dir_name, dir_path
            ),
            colored::Color::Cyan,
        ),
        Health::ExistsWithoutPermissions => warn(
            &format!(
                "{} directory exists but doesn't have enough permissions at: {}",
                dir_name, dir_path
            ),
            colored::Color::Red,
        ),
        Health::DoesNotExist => warn(
            &format!("{} directory doesn't exist at: {}", dir_name, dir_path),
            colored::Color::Red,
        ),
    };
    Ok(())
}

pub async fn check_health() -> Result<()> {
    let mut spinner_pkginfo = Spinner::new_with_stream(
        spinners::Dots,
        "Getting config ... ",
        Color::Green,
        Streams::Stderr,
    );
    let hysp_config = read_config().await?;
    spinner_pkginfo.stop_and_persist("Getting config  ", "Done");

    let directories_to_check = [
        (
            "Home",
            hysp_config
                .local
                .home
                .ok_or_else(|| anyhow!("Couldn't get home directory"))?,
        ),
        (
            "Binary",
            hysp_config
                .local
                .bin
                .ok_or_else(|| anyhow!("Couldn't get binary directory"))?,
        ),
        (
            "Data",
            hysp_config
                .local
                .data
                .ok_or_else(|| anyhow!("Couldn't get data directory"))?,
        ),
    ];

    for (dir_name, dir_path) in directories_to_check.iter() {
        let cleaned_dir_path = remove_trailing_slash(dir_path.to_string_lossy().to_string());
        check_and_log_directory(dir_name, &cleaned_dir_path).await?;
    }
    Ok(())
}
