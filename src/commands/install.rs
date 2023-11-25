use crate::commands::hysp_cmd_helper::ask_to_continue;
use crate::engine::config::pkg_config_structure::*;
use crate::engine::helpers::create_directory_if_not_exists;
use crate::engine::{
    args::InstallArgs,
    config::read_config,
    essetial::check_hash,
    helpers::{check_essentials, print_info},
};
use crate::log::{abort, info};
use anyhow::Result;
use spinoff::{spinners, Color, Spinner, Streams};
use std::fs::{self, File, Permissions};
use std::io::copy;
use std::path::Path;
use std::{io::Write, os::unix::fs::PermissionsExt};

pub async fn install_packages(install_pkgs: InstallArgs) -> Result<()> {
    let hysp_config = read_config().await?;
    let pkg_name = install_pkgs.package.clone();

    let remote = hysp_config.source.remote.unwrap();

    let aarch = hysp_config.source.aarch.unwrap();

    let home_location = hysp_config
        .local
        .home
        .unwrap()
        .to_string_lossy()
        .to_string();
    let bin_location = hysp_config.local.bin.unwrap().to_string_lossy().to_string();
    let data_location = hysp_config
        .local
        .data
        .unwrap()
        .to_string_lossy()
        .to_string();

    create_directory_if_not_exists(&home_location);
    create_directory_if_not_exists(&bin_location);
    create_directory_if_not_exists(&data_location);

    let hysp_pkg_url = if remote.ends_with('/') {
        format!(
            "{}/{}/{}.toml",
            remote.trim_end_matches('/'),
            aarch,
            pkg_name
        )
    } else {
        format!("{}/{}/{}.toml", remote, aarch, pkg_name)
    };

    let package_config =
        fetch_pkg(hysp_pkg_url, data_location.clone(), pkg_name, install_pkgs).await;
    match package_config {
        Ok(package_config) => {
            let binary_url = package_config.package.source.clone();
            check_essentials(package_config.clone());
            if let Err(err) = download_binary(
                binary_url,
                package_config.bin.name.clone(),
                package_config.package.sha,
                bin_location,
            )
            .await
            {
                eprintln!("Error downloading binary: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Error fetching package: {}", err);
        }
    }

    Ok(())
}

pub async fn fetch_pkg(
    pkg_url: String,
    pkg_data_location: String,
    pkg_name: String,
    install_pkgs: InstallArgs,
) -> Result<PackageInfo> {
    let pkg_data_location = if pkg_data_location.ends_with('/') {
        let trimmed_location = pkg_data_location.trim_end_matches('/').to_string();
        format!("{}/{}.toml", trimmed_location, pkg_name)
    } else {
        format!("{}/{}.toml", pkg_data_location, pkg_name)
    };

    if !install_pkgs.force {
        if Path::new(&pkg_data_location).exists() {
            info(
                &format!(
                    "There's already a (binary/data) exist:  {}",
                    pkg_data_location
                ),
                colored::Color::Red,
            );
            info(
                "If you say Y, it's going to overwrite the old (binary/data) .. ",
                colored::Color::Red,
            );
            let should_continue = ask_to_continue();
            if !should_continue {
                std::process::exit(1);
            }
        }
    }

    let mut spinner_pkginfo = Spinner::new_with_stream(
        spinners::Dots,
        "Fetching package info ... ",
        Color::Green,
        Streams::Stderr,
    );
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10)) // Allow redirection limit = 10
        .danger_accept_invalid_certs(true) // allow http as well
        .build()?;

    let response = client.get(&pkg_url).send().await?;

    match response.status().is_success() {
        true => {
            let body = response.text().await?;
            let parsed_package_info: PackageInfo = toml::from_str(&body)?;

            let toml_string = toml::to_string(&parsed_package_info)?;
            let mut file = File::create(&pkg_data_location)?;
            file.write_all(toml_string.as_bytes())?;
            spinner_pkginfo.stop_and_persist(" Fetching pkginfo  ", "Done");
            print_info(parsed_package_info.clone());

            info(
                &format!("Data location: {}", pkg_data_location),
                colored::Color::Cyan,
            );
            Ok(parsed_package_info)
        }
        false => abort("Couldn't find package in the db"),
    }
}

#[inline]
async fn download_binary(
    binary_url: String,
    binary_name: String,
    binary_hash: String,
    binary_location: String,
) -> Result<(), anyhow::Error> {
    let mut spinner_binary = Spinner::new_with_stream(
        spinners::Dots,
        "Fetching binary ... ",
        Color::Green,
        Streams::Stderr,
    );
    let response = reqwest::get(&binary_url).await?;

    let hysp_binary_location = if binary_location.ends_with('/') {
        binary_location.trim_end_matches('/')
    } else {
        &binary_location
    };

    let pkg_binary_path = format!("{}/{}", hysp_binary_location, binary_name);
    let mut dest = File::create(&pkg_binary_path)?;

    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut dest)?;

    let permissions = Permissions::from_mode(0o755); // Set file permissions to be executable for the current user
    fs::set_permissions(&pkg_binary_path, permissions)?;

    spinner_binary.stop_and_persist(" Fetching binary  ", "Done");

    info(
        &format!("Binary location: {}", pkg_binary_path),
        colored::Color::Cyan,
    );

    match check_hash(pkg_binary_path.to_string(), binary_hash).await {
        Ok(hash_match) => {
            if !hash_match {
                abort("Couldn't match hash, aborting");
            }
        }
        Err(err) => {
            eprintln!("Error checking hash: {}", err);
        }
    }

    Ok(())
}
