use crate::engine::{helpers::local_config, msgx::info};
use anyhow::{Context, Result};
use std::os::unix::fs::PermissionsExt;

enum Health {
    ExistsWithPermissions,
    ExistsWithoutPermissions,
    DoesNotExist,
}

use std::fs;

fn check_directory(path: &str) -> Result<Health> {
    let metadata =
        fs::metadata(path).with_context(|| format!("Failed to get metadata for {}", path))?;

    if metadata.is_dir() {
        let permissions = metadata.permissions();
        let mode = permissions.mode();

        // Check if the directory is writable and readable by root
        if mode & 0o600 == 0o600 {
            return Ok(Health::ExistsWithPermissions);
        } else {
            return Ok(Health::ExistsWithoutPermissions);
        }
    }

    Ok(Health::DoesNotExist)
}

async fn check_and_log_directory(dir_name: &str, dir_path: &str) -> Result<()> {
    match check_directory(dir_path)? {
        Health::ExistsWithPermissions => info(
            &format!(
                "{} directory exists with required permissions at:  {} ",
                dir_name, dir_path
            ),
            colored::Color::Cyan,
        ),
        Health::ExistsWithoutPermissions => info(
            &format!(
                "{} directory exists but doesn't have enough permissions at:   {} ",
                dir_name, dir_path
            ),
            colored::Color::Cyan,
        ),
        Health::DoesNotExist => info(
            &format!("{} directory doesn't exist at:   {} ", dir_name, dir_path),
            colored::Color::Cyan,
        ),
    };
    Ok(())
}

pub async fn check_health() -> Result<()> {
    let (_hysp_remote, hysp_data_dir, hysp_bin_dir, _hysp_metadata, _architecture) =
        match local_config().await {
            Ok((remote, data_dir, bin_dir, metadata, architecture)) => {
                (remote, data_dir, bin_dir, metadata, architecture)
            }
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

    check_and_log_directory("Hysp data", &hysp_data_dir).await?;
    check_and_log_directory("Hysp bin", &hysp_bin_dir).await?;

    Ok(())
}
