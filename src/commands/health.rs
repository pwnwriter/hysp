use crate::engine::{helpers::local_config, msgx::info};
use anyhow::Result;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

enum Health {
    ExistsWithPermissions,
    ExistsWithoutPermissions,
    DoesNotExist,
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

    if is_directory_in_path(&hysp_bin_dir) {
        info(
            &format!("Hysp bin directory {}: is in PATH", hysp_bin_dir),
            colored::Color::BrightBlue,
        );
    } else {
        info(
            &format!("Hysp bin directory {}: isn't in PATH", hysp_bin_dir),
            colored::Color::Red,
        );
    }

    Ok(())
}

fn check_directory(path: &str) -> Result<Health> {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                let permissions = metadata.permissions();
                let mode = permissions.mode();

                if mode & 0o600 == 0o600 {
                    Ok(Health::ExistsWithPermissions)
                } else {
                    Ok(Health::ExistsWithoutPermissions)
                }
            } else {
                Err(anyhow::anyhow!(
                    "Path exists but is not a directory: {}",
                    path
                ))
            }
        }
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                info(
                    &format!("Path doesn't exist: {}", path),
                    colored::Color::Cyan,
                );
                Ok(Health::DoesNotExist)
            } else {
                Err(e.into())
            }
        }
    }
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
            colored::Color::Red,
        ),
        Health::DoesNotExist => info(
            &format!("{} directory doesn't exist at:   {} ", dir_name, dir_path),
            colored::Color::BrightRed,
        ),
    };
    Ok(())
}

fn is_directory_in_path(directory: &str) -> bool {
    if let Some(paths) = env::var_os("PATH") {
        if let Some(paths_str) = paths.to_str() {
            let path_list: Vec<&str> = paths_str.split(':').collect();
            let dir_path = Path::new(directory);

            for path in path_list {
                if Path::new(path) == dir_path {
                    return true;
                }
            }
        }
    }
    false
}
