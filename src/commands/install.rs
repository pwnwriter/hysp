use super::seren_helpers::{BAR, RESET};
use crate::engine::dirs::*;
use crate::engine::parser::fetch_package_info;
use crate::engine::{check_conflicts, check_dependencies, InstallArgs};
use crate::log::info;
use spinoff::{spinners, Color, Spinner, Streams};
use std::fs::{self, File, Permissions};
use std::io::copy;
use std::os::unix::fs::PermissionsExt;

pub async fn download_pkgs(install_pkgs: InstallArgs) -> Result<(), anyhow::Error> {
    let pkgname = &install_pkgs.package;

    println!("{}", RESET); // RESET terminal colors

    let toml_info = fetch_package_info(pkgname).await?;
    let binary_url = toml_info.source.url;

    match check_dependencies(&install_pkgs).await {
        Ok(()) => {
            println!("{}", BAR);
            info(
                "Dependencies are satisfied, proceeding ...",
                colored::Color::Blue,
            );
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    match check_conflicts(install_pkgs.clone()).await {
        Ok(()) => {
            println!(); // just print a new line
            info(
                "No conflicts detected, proceeding ...",
                colored::Color::Blue,
            );
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    download_binary(binary_url).await
}

async fn download_binary(binary_url: String) -> Result<(), anyhow::Error> {
    let spinner = Spinner::new_with_stream(
        spinners::Line,
        "Downloading binary please wait... ",
        Color::Yellow,
        Streams::Stderr,
    );
    let response = reqwest::get(&binary_url).await?;
    let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or_default();

    let file_path = SEREN_BIN_DIR.join(fname);
    let mut dest = File::create(&file_path)?;

    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut dest)?;
    spinner.stop_and_persist("  ", "Done");

    let spinner = Spinner::new_with_stream(
        spinners::Line,
        "Setting up file permissions... ",
        Color::Green,
        Streams::Stderr,
    );
    // Set file permissions to be executable for the current user
    let permissions = Permissions::from_mode(0o755);
    fs::set_permissions(&file_path, permissions)?;

    spinner.stop_and_persist("  ", "Done");
    Ok(())
}
