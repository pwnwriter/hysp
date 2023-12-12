use spinoff::{spinners, Color, Spinner, Streams};
use std::path::Path;
use std::{io::Write, os::unix::fs::PermissionsExt};

use crate::engine::config::parse_pkg_info;

use super::config::pkg_config_structure::PackageInfo;
use crate::engine::msgx::info;
use anyhow::Result;
use std::fs::{self, File, Permissions};
use std::io::copy;

/// Downloads content from the provided `download_url` and saves it as a file at the specified `download_location`.
///
/// # Arguments
///
/// * `download_url` - A string slice representing the URL to download the content from.
/// * `download_location` - A string slice representing the path to save the downloaded content.
///
/// # Returns
///
/// A Result indicating the success or failure of the download process.
///
pub async fn download_as_byte(
    download_url: &str,
    download_location: &str,
) -> Result<(), anyhow::Error> {
    let response = reqwest::get(download_url).await?;

    match response.status().as_u16() {
        200 => {
            let mut spinner_binary = Spinner::new_with_stream(
                spinners::Dots,
                "Fetching binary ... ",
                Color::Green,
                Streams::Stderr,
            );
            if Path::new(download_location).exists() {
                std::fs::remove_file(download_location)?; // Already asked to continue or not so, removing it anyway
            }
            let content = response.bytes().await?;
            let mut dest = File::create(download_location)?;
            copy(&mut content.as_ref(), &mut dest)?;

            let permissions = Permissions::from_mode(0o755);
            fs::set_permissions(download_location, permissions)?;

            spinner_binary.stop_and_persist("Downloading binary  ", "Done");

            info(
                &format!("Package binary location : {} ", &download_location),
                colored::Color::Cyan,
            );
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unexpected status code: {}",
                response.status()
            ));
        }
    };

    Ok(())
}

/// Downloads package information from the provided `download_url`, parses it, and saves it as a TOML file at the specified `download_location`.
///
/// # Arguments
///
/// * `download_url` - A string slice representing the URL to download the package information from.
/// * `download_location` - A string slice representing the path to save the downloaded package information.
///
/// # Returns
///
/// A Result containing the parsed `PackageInfo` struct or an error if the download or parsing fails.
///
pub async fn download_and_parse_package(
    download_url: &str,
    download_location: &str,
) -> Result<PackageInfo> {
    let mut spinner_pkginfo = Spinner::new_with_stream(
        spinners::Dots,
        "Fetching package info ... ",
        Color::Green,
        Streams::Stderr,
    );
    let response = reqwest::get(download_url).await?;

    match response.status().as_u16() {
        200 => {
            if Path::new(download_location).exists() {
                std::fs::remove_file(download_location)?;
            }

            let parsed_pkg_info = parse_pkg_info(download_url).await?;
            let toml_string = toml::to_string(&parsed_pkg_info)?;

            let mut dest = File::create(download_location)?;
            dest.write_all(toml_string.as_bytes())?;

            spinner_pkginfo.stop_and_persist("Fetching package info  ", "Done ");
            info(
                &format!("Package data location : {} ", download_location),
                colored::Color::Cyan,
            );
            Ok(parsed_pkg_info)
        }
        404 => {
            println!();
            info("No such package found", colored::Color::Red);
            std::process::exit(1);
        }
        _ => Err(anyhow::anyhow!(
            "Unexpected status code: {}",
            response.status()
        )),
    }
}
